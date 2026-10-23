use assert_fs::TempDir;
use std::fs;

use std::path::PathBuf;
use std::sync::OnceLock;

pub type TestResult = Result<(), Box<dyn std::error::Error>>;

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
