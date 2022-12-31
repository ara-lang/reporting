use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::issue::Issue;

pub mod annotation;
pub mod builder;
pub mod error;
pub mod issue;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Report {
    pub issues: Vec<Issue>,
}

/// A report.
///
/// A report is a collection of issues.
///
/// Example:
///
/// ```rust
/// use ara_reporting::Report;
/// use ara_reporting::issue::Issue;
/// use ara_reporting::issue::IssueKind;
///
///
/// let report = Report::new()
///     .with_issue(Issue::error("0003", "standalone type `void` cannot be part of a union", "main.ara", 10, 14))
///     .with_issue(Issue::warning("0023", "...", "some_file.ara", 9, 10))
/// ;
///
/// assert_eq!(report.issues.len(), 2);
///
/// assert_eq!(report.issues[0].kind, IssueKind::Error);
/// assert_eq!(report.issues[0].code, "0003");
/// assert_eq!(report.issues[0].message, "standalone type `void` cannot be part of a union");
/// assert_eq!(report.issues[0].origin, "main.ara");
/// assert_eq!(report.issues[0].from, 10);
/// assert_eq!(report.issues[0].to, 14);
///
/// assert_eq!(report.issues[1].kind, IssueKind::Warning);
/// assert_eq!(report.issues[1].code, "0023");
/// assert_eq!(report.issues[1].message, "...");
/// assert_eq!(report.issues[1].origin, "some_file.ara");
/// assert_eq!(report.issues[1].from, 9);
/// assert_eq!(report.issues[1].to, 10);
/// ```
impl Report {
    /// Create a new report.
    pub fn new() -> Self {
        Self { issues: vec![] }
    }

    /// Add an issue to this report.
    pub fn with_issue(mut self, issue: Issue) -> Self {
        self.issues.push(issue);
        self
    }
}

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for issue in &self.issues {
            writeln!(f, "{}", issue)?;
        }

        Ok(())
    }
}

impl Into<Report> for Issue {
    fn into(self) -> Report {
        Report { issues: vec![self] }
    }
}
