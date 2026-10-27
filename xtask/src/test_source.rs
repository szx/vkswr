//! Extracts the dEQP-VK cases that the integration suite runs.
//!
//! The suite is the implementation inventory: a case is covered when a
//! `#[test]` calls `run_deqp_vk` with its name, and `#[ignore]` marks the ones
//! that are known not to pass yet.

use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestCase {
    pub case: String,
    pub ignored: bool,
}

/// Collects every case passed to `run_deqp_vk` by a test function.
///
/// # Errors
///
/// Returns an error if two tests run the same case or a test repeats one.
pub fn parse(source: &str) -> Result<Vec<TestCase>, String> {
    let lines: Vec<&str> = source.lines().collect();
    let mut cases = Vec::new();
    let mut seen = BTreeSet::new();
    let mut attributes: Vec<&str> = Vec::new();
    let mut index = 0;

    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim();
        if trimmed.starts_with("#[") {
            attributes.push(trimmed);
            index += 1;
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with("//") {
            index += 1;
            continue;
        }
        let Some(name) = function_name(line) else {
            attributes.clear();
            index += 1;
            continue;
        };

        let start = index;
        index += 1;
        while index < lines.len() && lines[index] != "}" {
            index += 1;
        }
        if let Some(case) = run_case(&lines[start..index].join("\n")) {
            if !seen.insert(case.clone()) {
                return Err(format!(
                    "{name} runs {case}, which another test already runs"
                ));
            }
            cases.push(TestCase {
                case,
                ignored: attributes
                    .iter()
                    .any(|attribute| attribute.starts_with("#[ignore")),
            });
        }

        attributes.clear();
        index += 1;
    }

    Ok(cases)
}

fn function_name(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("fn ")?;
    let name = rest.split('(').next()?.trim();
    (!name.is_empty()).then_some(name)
}

fn run_case(body: &str) -> Option<String> {
    let marker = "run_deqp_vk(";
    let rest = body.split_once(marker)?.1.trim_start();
    let rest = rest.strip_prefix('"')?;
    Some(rest.split('"').next()?.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_case_and_ignore_flag() {
        let source = r#"
#[test]
fn covered() -> common::TestResult {
    run_deqp_vk(
        "dEQP-VK.api.smoke.triangle",
        false,
    )
}

#[ignore = "not implemented"]
#[test]
fn pending() -> common::TestResult {
    run_deqp_vk("dEQP-VK.api.*", false)
}

#[test]
fn unrelated() -> common::TestResult {
    Ok(())
}
"#;
        assert_eq!(
            parse(source),
            Ok(vec![
                TestCase {
                    case: "dEQP-VK.api.smoke.triangle".to_owned(),
                    ignored: false,
                },
                TestCase {
                    case: "dEQP-VK.api.*".to_owned(),
                    ignored: true,
                },
            ])
        );
    }

    #[test]
    fn rejects_duplicate_cases() {
        let source = r#"
#[test]
fn first() -> common::TestResult {
    run_deqp_vk("dEQP-VK.api.smoke.triangle", false)
}

#[test]
fn second() -> common::TestResult {
    run_deqp_vk("dEQP-VK.api.smoke.triangle", false)
}
"#;
        assert!(parse(source).is_err());
    }

    #[test]
    fn parses_the_repository_suite() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../test_suite/tests/integration_tests.rs");
        let source = std::fs::read_to_string(path).expect("the integration suite is readable");
        let cases = parse(&source).expect("the integration suite parses");
        assert!(
            cases
                .iter()
                .any(|case| case.case == "dEQP-VK.api.smoke.triangle")
        );
        assert!(cases.iter().any(|case| case.case == "dEQP-VK.api.*"));
        assert!(cases.iter().all(|case| case.case.starts_with("dEQP-VK.")));
    }
}
