use std::path::{Path, PathBuf};

pub type TestResult = Result<(), Box<dyn std::error::Error>>;

#[ctor::ctor]
fn build_cdylib() {
    escargot::CargoBuild::new()
        .manifest_path("Cargo.toml")
        .exec()
        .unwrap();
    // TODO: Logger.
    // TODO: Create ICD.json in temp.
}

pub fn get_cdylib_path() -> PathBuf {
    let mut cdylib_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    cdylib_path.push("target/debug/librust_software_rasterizer.so");
    cdylib_path
}

pub fn get_icd_json(path: &Path) -> String {
    format!(
        r#"
{{
    "file_format_version": "1.0.0",
    "ICD": {{
        "library_path": "{}",
        "api_version": "1.0.0"
    }}
}}"#,
        path.to_string_lossy()
    )
}
