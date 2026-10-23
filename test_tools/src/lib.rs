//! Download (and, for the CTS, build) the external tools used by the test
//! suites.
//!
//! Keeping the tool acquisition here means the tests never depend on whatever
//! happens to be on the user's `PATH` or in `VULKAN_SDK`/`VULKAN_CTS_PATH`:
//! they always invoke absolute paths from a project-local cache under
//! `target/vkswr-tools`.
//!
//! The cache directory can be moved with `VKSWR_TOOLS_DIR`. The first test run
//! downloads the LunarG Vulkan SDK, downloads the Vulkan CTS sources, fetches
//! the CTS submodules, and builds `deqp-vk`. Building requires `python3`,
//! `cmake`, and `ninja` to be available.

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use fs2::FileExt;
use sha2::{Digest, Sha256};

/// Pinned Vulkan SDK release.
const VULKAN_SDK_VERSION: &str = "1.4.363.0";
const VULKAN_SDK_URL: &str =
    "https://sdk.lunarg.com/sdk/download/1.4.363.0/linux/vulkan-sdk-linux.tar.gz";
const VULKAN_SDK_SHA256: &str = "197962f5cbf80baf2775a03336a01cee7c8745686c65aaa70d3f751ade4d7e43";
const VULKAN_SDK_BIN: &str = "x86_64/bin";

/// Pinned Vulkan CTS release.
const CTS_VERSION: &str = "vulkan-cts-1.4.7.0";
const CTS_URL: &str = concat!(
    "https://github.com/KhronosGroup/VK-GL-CTS/archive/refs/tags/",
    "vulkan-cts-1.4.7.0.tar.gz"
);
const CTS_SHA256: &str = "f6cc5bf35adf6e6260444fe5ecbd3d74dfc74915a7779f8c0d7d0c95316bf51e";
/// Directory name of the CTS source tree in the downloaded archive.
const CTS_SRC_DIR: &str = "VK-GL-CTS-vulkan-cts-1.4.7.0";
const CTS_DEQP_VK: &str = "external/vulkancts/modules/vulkan/deqp-vk";

static SDK: OnceLock<Result<PathBuf, String>> = OnceLock::new();
static CTS: OnceLock<Result<PathBuf, String>> = OnceLock::new();

/// Absolute path of a binary shipped in the Vulkan SDK.
///
/// On first use this downloads and extracts the pinned Vulkan SDK. It does not
/// build the CTS.
pub fn sdk_bin(name: &str) -> Result<PathBuf> {
    let bin = match SDK.get_or_init(|| prepare_sdk().map_err(|error| format!("{error:#}"))) {
        Ok(bin) => bin,
        Err(message) => bail!("could not prepare the Vulkan SDK:\n{message}"),
    };
    let path = bin.join(name);
    if !path.is_file() {
        bail!("Vulkan SDK does not contain {name} ({})", path.display());
    }
    Ok(path)
}

/// Absolute path of the `deqp-vk` binary built from the Vulkan CTS.
///
/// On first use this downloads the CTS sources, fetches their dependencies, and
/// builds `deqp-vk`.
pub fn deqp_vk() -> Result<PathBuf> {
    match CTS.get_or_init(|| prepare_cts().map_err(|error| format!("{error:#}"))) {
        Ok(deqp_vk) => Ok(deqp_vk.clone()),
        Err(message) => bail!("could not prepare the Vulkan CTS:\n{message}"),
    }
}

fn prepare_sdk() -> Result<PathBuf> {
    if !cfg!(target_os = "linux") {
        bail!("automatic Vulkan SDK downloads are only implemented for Linux");
    }

    let root = tools_root();
    let _lock = lock(&root)?;
    ensure_vulkan_sdk(&root)
}

fn prepare_cts() -> Result<PathBuf> {
    if !cfg!(target_os = "linux") {
        bail!("automatic Vulkan CTS builds are only implemented for Linux");
    }

    let root = tools_root();
    let _lock = lock(&root)?;
    ensure_vulkan_cts(&root)
}

fn lock(root: &Path) -> Result<File> {
    fs::create_dir_all(root).with_context(|| format!("creating {}", root.display()))?;
    let lock_path = root.join(".lock");
    let lock = File::create(&lock_path)
        .with_context(|| format!("creating lock file {}", lock_path.display()))?;
    lock.lock_exclusive()
        .with_context(|| format!("locking {}", lock_path.display()))?;
    Ok(lock)
}

fn tools_root() -> PathBuf {
    if let Some(path) = std::env::var_os("VKSWR_TOOLS_DIR") {
        return PathBuf::from(path);
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("test_tools crate lives in the workspace")
        .join("target")
        .join("vkswr-tools")
}

fn ensure_vulkan_sdk(root: &Path) -> Result<PathBuf> {
    let sdk = root.join("sdk").join(VULKAN_SDK_VERSION);
    let bin = sdk.join(VULKAN_SDK_BIN);
    if bin.join("glslangValidator").is_file()
        && bin.join("vulkaninfo").is_file()
        && bin.join("vkcube").is_file()
    {
        return Ok(bin);
    }

    let archive = download(
        root,
        VULKAN_SDK_URL,
        &format!("vulkan-sdk-{VULKAN_SDK_VERSION}.tar.xz"),
        VULKAN_SDK_SHA256,
    )?;

    eprintln!("[vkswr] extracting Vulkan SDK {VULKAN_SDK_VERSION}");
    let staging = root
        .join("sdk")
        .join(format!(".{VULKAN_SDK_VERSION}.partial"));
    remove_dir_if_exists(&staging)?;
    fs::create_dir_all(&staging)?;
    extract_tar_xz(&archive, &staging)?;

    let downloaded = staging.join(VULKAN_SDK_VERSION);
    if !downloaded.is_dir() {
        bail!(
            "unexpected Vulkan SDK archive layout (missing {})",
            downloaded.display()
        );
    }
    remove_dir_if_exists(&sdk)?;
    fs::rename(&downloaded, &sdk).with_context(|| format!("moving SDK into {}", sdk.display()))?;
    remove_dir_if_exists(&staging)?;

    if !bin.join("glslangValidator").is_file() {
        bail!("Vulkan SDK extraction did not produce glslangValidator");
    }
    Ok(bin)
}

fn ensure_vulkan_cts(root: &Path) -> Result<PathBuf> {
    let cts_root = root.join("cts");
    let source = cts_root.join(CTS_SRC_DIR);
    let build = cts_root.join("build");
    let deqp_vk = build.join(CTS_DEQP_VK);

    let version_marker = build.join("vkswr-cts-version");
    let matches_pinned_version = fs::read_to_string(&version_marker)
        .map(|version| version.trim() == CTS_VERSION)
        .unwrap_or(false);
    if deqp_vk.is_file() && matches_pinned_version {
        return Ok(deqp_vk);
    }

    if !source.is_dir() {
        let archive = download(root, CTS_URL, &format!("{CTS_SRC_DIR}.tar.gz"), CTS_SHA256)?;
        eprintln!("[vkswr] extracting Vulkan CTS {CTS_VERSION}");
        let staging = cts_root.join(format!(".{CTS_SRC_DIR}.partial"));
        remove_dir_if_exists(&staging)?;
        fs::create_dir_all(&staging)?;
        extract_tar_gz(&archive, &staging)?;
        fs::create_dir_all(&cts_root)?;
        remove_dir_if_exists(&source)?;
        fs::rename(staging.join(CTS_SRC_DIR), &source)
            .with_context(|| format!("moving CTS sources into {}", source.display()))?;
        remove_dir_if_exists(&staging)?;
    }

    if !source.join("external/glslang/src").is_dir() {
        eprintln!("[vkswr] fetching Vulkan CTS external sources");
        run(
            Command::new(python())
                .arg("external/fetch_sources.py")
                .current_dir(&source),
            "Vulkan CTS fetch_sources.py",
        )?;
    }

    if !matches_pinned_version {
        remove_dir_if_exists(&build)?;
    }

    if !build.join("build.ninja").is_file() {
        eprintln!("[vkswr] configuring Vulkan CTS");
        fs::create_dir_all(&build)?;
        let mut cmake = Command::new(cmake());
        cmake
            .arg("-S")
            .arg(&source)
            .arg("-B")
            .arg(&build)
            .arg("-G")
            .arg("Ninja")
            .arg("-DCMAKE_BUILD_TYPE=Release")
            .arg("-DDEQP_DISABLE_VK_VIDEO_TESTS=ON");
        run(&mut cmake, "Vulkan CTS configure")?;
        fs::write(&version_marker, CTS_VERSION)
            .with_context(|| format!("writing {}", version_marker.display()))?;
    }

    eprintln!("[vkswr] building deqp-vk (this can take a while)");
    let jobs = std::env::var("VKSWR_CTS_JOBS").unwrap_or_else(|_| {
        std::thread::available_parallelism()
            .map_or(4, |parallelism| parallelism.get().min(8))
            .to_string()
    });
    let mut cmake = Command::new(cmake());
    cmake
        .arg("--build")
        .arg(&build)
        .arg("--target")
        .arg("deqp-vk")
        .arg("--parallel")
        .arg(&jobs);
    run(&mut cmake, "Vulkan CTS build")?;

    if !deqp_vk.is_file() {
        bail!(
            "Vulkan CTS build finished without producing {}",
            deqp_vk.display()
        );
    }
    Ok(deqp_vk)
}

fn download(root: &Path, url: &str, filename: &str, expected_sha256: &str) -> Result<PathBuf> {
    let downloads = root.join("downloads");
    fs::create_dir_all(&downloads)?;
    let destination = downloads.join(filename);
    if destination.is_file() && sha256_file(&destination)? == expected_sha256 {
        return Ok(destination);
    }

    eprintln!("[vkswr] downloading {url}");
    let partial = downloads.join(format!("{filename}.partial"));
    let _ = fs::remove_file(&partial);
    let response = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(30))
        .timeout_read(Duration::from_secs(300))
        .build()
        .get(url)
        .call()
        .with_context(|| format!("downloading {url}"))?;
    let mut reader = response.into_reader();
    let mut file =
        File::create(&partial).with_context(|| format!("creating {}", partial.display()))?;
    io::copy(&mut reader, &mut file).with_context(|| format!("writing {}", partial.display()))?;
    drop(file);

    let actual = sha256_file(&partial)?;
    if actual != expected_sha256 {
        let _ = fs::remove_file(&partial);
        bail!("checksum mismatch for {url}: expected {expected_sha256}, got {actual}");
    }
    fs::rename(&partial, &destination)
        .with_context(|| format!("moving download into {}", destination.display()))?;
    Ok(destination)
}

fn sha256_file(path: &Path) -> Result<String> {
    let mut file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 1024 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .with_context(|| format!("reading {}", path.display()))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn extract_tar_xz(archive: &Path, destination: &Path) -> Result<()> {
    let file = File::open(archive).with_context(|| format!("opening {}", archive.display()))?;
    let decoder = xz2::read::XzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    archive
        .unpack(destination)
        .with_context(|| format!("extracting into {}", destination.display()))
}

fn extract_tar_gz(archive: &Path, destination: &Path) -> Result<()> {
    let file = File::open(archive).with_context(|| format!("opening {}", archive.display()))?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    archive
        .unpack(destination)
        .with_context(|| format!("extracting into {}", destination.display()))
}

fn run(command: &mut Command, what: &str) -> Result<()> {
    eprintln!("[vkswr] {what}: {command:?}");
    let status = command
        .status()
        .with_context(|| format!("starting {what}"))?;
    if !status.success() {
        bail!("{what} failed with {status}");
    }
    Ok(())
}

fn python() -> String {
    std::env::var("VKSWR_PYTHON").unwrap_or_else(|_| "python3".to_owned())
}

fn cmake() -> String {
    std::env::var("VKSWR_CMAKE").unwrap_or_else(|_| "cmake".to_owned())
}

fn remove_dir_if_exists(path: &Path) -> Result<()> {
    match fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("removing {}", path.display())),
    }
}
