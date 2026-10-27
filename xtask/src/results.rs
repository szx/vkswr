//! Reads the outcome of the last test run from `deqp-results.tsv`.
//!
//! The integration suite rewrites the file after every case, so a committed
//! copy makes the generated `README.md` checklist reproducible.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::readme::Status;

/// # Errors
///
/// Returns an error if the file is missing, unreadable, or malformed.
pub fn load(path: &Path) -> Result<BTreeMap<String, Status>, String> {
    let contents =
        fs::read_to_string(path).map_err(|error| format!("reading {}: {error}", path.display()))?;
    let mut recorded = BTreeMap::new();

    for (index, line) in contents.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((case, status)) = line.split_once('\t') else {
            return Err(format!(
                "{}:{}: expected `case<TAB>status`",
                path.display(),
                index + 1
            ));
        };
        let status = Status::parse(status).ok_or_else(|| {
            format!(
                "{}:{}: expected `pass` or `fail`, got {status:?}",
                path.display(),
                index + 1
            )
        })?;
        recorded.insert(case.to_owned(), status);
    }

    Ok(recorded)
}
