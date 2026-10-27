use assert_fs::TempDir;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub type TestResult = Result<(), Box<dyn std::error::Error>>;

const RESULTS_FILE: &str = "deqp-results.tsv";

/// Records the outcome of one case for the generated `README.md` checklist.
///
/// Tests run in parallel, so the file is updated under a lock file and replaced
/// atomically.
pub fn record_deqp_result(case: &str, passed: bool) -> io::Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let lock = File::create(root.join("deqp-results.lock"))?;
    lock.lock()?;

    let path = root.join(RESULTS_FILE);
    let mut results = read_results(&path)?;
    let status = if passed { "pass" } else { "fail" };
    results.insert(case.to_owned(), status.to_owned());

    let mut contents = String::new();
    for (case, status) in &results {
        contents.push_str(case);
        contents.push('\t');
        contents.push_str(status);
        contents.push('\n');
    }
    let temporary = root.join(format!("{RESULTS_FILE}.tmp"));
    fs::write(&temporary, contents)?;
    fs::rename(&temporary, &path)?;
    Ok(())
}

fn read_results(path: &Path) -> io::Result<BTreeMap<String, String>> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(contents
            .lines()
            .filter_map(|line| {
                let (case, status) = line.split_once('\t')?;
                Some((case.to_owned(), status.to_owned()))
            })
            .collect()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(BTreeMap::new()),
        Err(error) => Err(error),
    }
}

pub fn get_cdylib_path() -> PathBuf {
    let mut cdylib_path = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/.."));
    cdylib_path.push("target/debug/libicd.so");
    fs::canonicalize(&cdylib_path)
        .unwrap_or_else(|error| panic!("cannot find {}: {error}", cdylib_path.display()))
}

static TEMP_DIR: OnceLock<TempDir> = OnceLock::new();
static ICD_JSON: OnceLock<PathBuf> = OnceLock::new();

fn get_temp_dir() -> &'static TempDir {
    TEMP_DIR.get_or_init(|| assert_fs::TempDir::new().expect("create temporary directory"))
}

/// Path to the ICD manifest shared by all tests in this process.
///
/// The manifest is written exactly once; rewriting it from concurrent tests
/// would let a subprocess observe a truncated file.
pub fn get_icd_json_path() -> PathBuf {
    ICD_JSON
        .get_or_init(|| {
            let cdylib_path = get_cdylib_path();
            let icd_json = format!(
                r#"
{{
    "file_format_version": "1.0.0",
    "ICD": {{
        "library_path": "{}",
        "api_version": "1.0.0"
    }}
}}"#,
                cdylib_path.to_string_lossy()
            );

            let icd_path = get_temp_dir().join("icd.json");
            fs::write(&icd_path, icd_json).expect("write the ICD manifest");
            icd_path
        })
        .clone()
}
