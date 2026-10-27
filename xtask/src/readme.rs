//! Renders the implementation-progress checklist in `README.md`.
//!
//! The checklist lives between [`BEGIN_MARKER`] and [`END_MARKER`]; everything
//! outside that region is written by hand.


pub const BEGIN_MARKER: &str = "<!-- goals:begin -->";
pub const END_MARKER: &str = "<!-- goals:end -->";

/// Outcome recorded by the last run of the test suite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pass,
    Fail,
    /// The suite has not run the case since it was added or last changed.
    NotRun,
}

impl Status {
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "pass" => Some(Self::Pass),
            "fail" => Some(Self::Fail),
            _ => None,
        }
    }

    fn mark(self) -> char {
        match self {
            Self::Pass => 'x',
            Self::Fail | Self::NotRun => ' ',
        }
    }
}

/// A screenshot that `deqp-vk` wrote next to the case it belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetImage {
    pub label: &'static str,
    pub path: String,
}

impl AssetImage {
    #[must_use]
    pub fn markdown(&self) -> String {
        format!("![{}]({})", self.label, self.path)
    }
}

/// One test from the integration suite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoalEntry {
    /// The dEQP pattern the test runs. A trailing `*` means the test runs a
    /// whole group and reports one result for it.
    pub case: String,
    pub status: Status,
    pub images: Vec<AssetImage>,
}

/// Renders the checklist that `xtask` splices into `README.md`.
///
/// The returned text is inserted verbatim between the markers above, so it must
/// be a markdown list; callers sort `entries` by case name. Each entry is one
/// `#[test]` from the integration suite, marked `[x]` when the last run recorded
/// [`Status::Pass`], `[]` when it failed or has not run.
///
/// A case ending in `*` runs a whole group. Every case starting with that
/// group's prefix (the name without the trailing `*`) nests below it, and when
/// several groups cover a case the most specific one wins. A group can itself
/// be covered by a wider group, so nesting is not limited to one level.
pub fn render_goals(entries: &[GoalEntry]) -> String {
    fn split(ent: &GoalEntry) -> Vec<&str> {
        let mut res: Vec<_> = ent.case.split(".").collect();
        let len = res.len();
        if res[len - 1] == "*" {
            res.truncate(len - 1);
        }
        res
    }

    fn common(xs: &[&str], ys: &[&str]) -> usize {
        xs.iter().zip(ys).take_while(|(x, y)| x == y).count()
    }

    let mut s = String::default();
    let mut indent = 0;
    let mut prefixes = vec![vec![]];
    for ent in entries {
        while let Some(prefix) = prefixes.pop() {
            let split = split(ent);
            let curr = common(&prefix, &split);
            if curr == prefix.len() {
                prefixes.push(prefix);
                prefixes.push(split);
                indent += 2;
                break;
            }
            indent -= 2;
        }
        // The empty root prefix counts as a level of its own.
        s += &render_entry(ent, indent - 2);
    }
    s
}

/// Renders one entry, indented by `indent` spaces.
fn render_entry(entry: &GoalEntry, indent: usize) -> String {
    let padding = " ".repeat(indent);
    let images: String = entry.images.iter().map(AssetImage::markdown).collect();
    let line = format!("{padding}- [{}] {}\n", entry.status.mark(), entry.case);
    if images.is_empty() {
        line
    } else {
        format!("{line}{padding}  {images}\n")
    }
}

/// Replaces the region between the markers with `generated`.
///
/// # Errors
///
/// Returns an error if `README.md` does not contain both markers in order.
pub fn splice(readme: &str, generated: &str) -> Result<String, String> {
    let begin = readme
        .find(BEGIN_MARKER)
        .ok_or_else(|| format!("README.md is missing {BEGIN_MARKER}"))?;
    let after_begin = begin + BEGIN_MARKER.len();
    let end = readme[after_begin..]
        .find(END_MARKER)
        .map(|offset| offset + after_begin)
        .ok_or_else(|| format!("README.md is missing {END_MARKER} after {BEGIN_MARKER}"))?;

    let mut updated = String::with_capacity(readme.len() + generated.len());
    updated.push_str(&readme[..after_begin]);
    updated.push('\n');
    updated.push_str(generated.trim_end());
    updated.push('\n');
    updated.push_str(&readme[end..]);
    Ok(updated)
}

#[cfg(test)]
mod tests {
    use super::*;

    const README: &str =
        "# VkSWR\n\n<!-- goals:begin -->\nstale\n<!-- goals:end -->\n\n## Testing\n";

    #[test]
    fn replaces_the_marked_region() {
        let updated =
            splice(README, "- [x] dEQP-VK.api.smoke.triangle\n").expect("README has both markers");
        assert_eq!(
            updated,
            "# VkSWR\n\n<!-- goals:begin -->\n- [x] dEQP-VK.api.smoke.triangle\n<!-- goals:end -->\n\n## Testing\n"
        );
        assert_eq!(
            splice(&updated, "- [x] dEQP-VK.api.smoke.triangle\n")
                .expect("README has both markers"),
            updated
        );
    }

    #[test]
    fn rejects_readme_without_markers() {
        assert!(splice("# VkSWR\n", "x").is_err());
    }

    #[test]
    fn indents_an_entry_and_its_images() {
        let entry = GoalEntry {
            case: "dEQP-VK.api.smoke.triangle".to_owned(),
            status: Status::Pass,
            images: vec![AssetImage {
                label: "Result",
                path: "triangle_Result.PNG".to_owned(),
            }],
        };
        assert_eq!(
            render_entry(&entry, 2),
            "  - [x] dEQP-VK.api.smoke.triangle\n    ![Result](triangle_Result.PNG)\n"
        );
    }

    #[test]
    fn nests_cases_under_their_group() {
        let entry = |case: &str, status| GoalEntry {
            case: case.to_owned(),
            status,
            images: Vec::new(),
        };
        let entries = [
            entry("dEQP-VK.api.*", Status::NotRun),
            entry("dEQP-VK.api.info.*", Status::Pass),
            entry("dEQP-VK.api.smoke.triangle", Status::Pass),
            entry("dEQP-VK.memory.*", Status::Fail),
            entry("dEQP-VK.memory.requirements.*", Status::Pass),
            entry("dEQP-VK.memory.requirements.core.buffer.regular", Status::Pass),
        ];
        assert_eq!(
            render_goals(&entries),
            "- [ ] dEQP-VK.api.*\n  - [x] dEQP-VK.api.info.*\n  - [x] dEQP-VK.api.smoke.triangle\n- [ ] dEQP-VK.memory.*\n  - [x] dEQP-VK.memory.requirements.*\n    - [x] dEQP-VK.memory.requirements.core.buffer.regular\n"
        );
    }
}
