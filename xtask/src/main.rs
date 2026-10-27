//! Regenerates the implementation-progress checklist in `README.md`.
//!
//! Run with `cargo run -p xtask -- readme`; pass `--check` to fail instead of
//! writing when the README is out of date.

mod readme;
mod results;
mod test_source;

use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use readme::{AssetImage, GoalEntry, Status};
use test_source::TestCase;

const RESULTS_FILE: &str = "deqp-results.tsv";
const TEST_SOURCE: &str = "test_suite/tests/integration_tests.rs";
const README_FILE: &str = "README.md";
const ASSET_DIR: &str = "assets/deqp-vk";
const IMAGE_LABELS: [&str; 3] = ["Result", "Reference", "ErrorMask"];

fn main() -> ExitCode {
    let mut arguments = std::env::args().skip(1);
    let command = arguments.next();
    let mut check = false;
    for argument in arguments {
        match argument.as_str() {
            "--check" => check = true,
            other => {
                eprintln!("unknown argument {other:?}");
                return ExitCode::FAILURE;
            }
        }
    }
    match command.as_deref() {
        None | Some("readme") => {}
        Some(other) => {
            eprintln!("unknown command {other:?}; expected `readme`");
            return ExitCode::FAILURE;
        }
    }

    match update_readme(check) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn update_readme(check: bool) -> Result<(), Box<dyn Error>> {
    let root = repository_root();
    let source = fs::read_to_string(root.join(TEST_SOURCE))?;
    let tests = test_source::parse(&source)?;
    let recorded = results::load(&root.join(RESULTS_FILE))?;
    warn_about_stale_records(&tests, &recorded);

    let mut entries: Vec<GoalEntry> = tests
        .iter()
        .map(|test| goal_entry(&root, test, &recorded))
        .collect();
    entries.sort_by(|left, right| left.case.cmp(&right.case));

    let passed = entries
        .iter()
        .filter(|entry| entry.status == Status::Pass)
        .count();
    println!("{passed} of {} cases pass in {RESULTS_FILE}", entries.len());

    let generated = readme::render_goals(&entries);
    let readme_path = root.join(README_FILE);
    let current = fs::read_to_string(&readme_path)?;
    let updated = readme::splice(&current, &generated)?;
    if updated == current {
        return Ok(());
    }
    if check {
        return Err(
            format!("{README_FILE} is out of date; run `cargo run -p xtask -- readme`").into(),
        );
    }
    fs::write(&readme_path, updated)?;
    println!("updated {}", readme_path.display());
    Ok(())
}

fn goal_entry(root: &Path, test: &TestCase, recorded: &BTreeMap<String, Status>) -> GoalEntry {
    let status = recorded.get(&test.case).copied().unwrap_or(Status::NotRun);
    GoalEntry {
        case: test.case.clone(),
        status,
        images: images_for(root, &test.case),
    }
}

fn images_for(root: &Path, case: &str) -> Vec<AssetImage> {
    if case.ends_with('*') {
        return Vec::new();
    }
    IMAGE_LABELS
        .into_iter()
        .filter_map(|label| {
            let path = format!("{ASSET_DIR}/{case}_{label}.PNG");
            root.join(&path)
                .is_file()
                .then_some(AssetImage { label, path })
        })
        .collect()
}

fn warn_about_stale_records(tests: &[TestCase], recorded: &BTreeMap<String, Status>) {
    for test in tests {
        match recorded.get(&test.case) {
            Some(_) if test.ignored => eprintln!(
                "warning: {} is ignored but {RESULTS_FILE} still records a result for it",
                test.case
            ),
            None if !test.ignored => eprintln!("note: {} has not been run yet", test.case),
            _ => {}
        }
    }
    for case in recorded.keys() {
        if !tests.iter().any(|test| &test.case == case) {
            eprintln!("warning: {RESULTS_FILE} records {case}, which no test runs anymore");
        }
    }
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives one level below the repository root")
        .to_path_buf()
}
