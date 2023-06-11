use assert_fs::TempDir;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub type TestResult = Result<(), Box<dyn std::error::Error>>;

#[ctor::ctor]
fn setup() {
    // Rebuild cdylib.
    escargot::CargoBuild::new()
        .manifest_path("../Cargo.toml")
        .exec()
        .unwrap();
    // TODO: Logger.
}

pub fn get_cdylib_path() -> PathBuf {
    let mut cdylib_path = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/.."));
    cdylib_path.push("target/debug/libicd.so");
    fs::canonicalize(cdylib_path).unwrap()
}

static TEMP_DIR: OnceLock<TempDir> = OnceLock::new();

fn get_temp_dir() -> &'static TempDir {
    TEMP_DIR.get_or_init(|| assert_fs::TempDir::new().unwrap())
}

pub fn get_icd_json_path() -> PathBuf {
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

    let icd_path = get_temp_dir().to_path_buf().join("icd.json");
    fs::write(&icd_path, icd_json).unwrap();

    icd_path
}
