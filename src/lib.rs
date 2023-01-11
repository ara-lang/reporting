use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::issue::Issue;
use crate::issue::IssueSeverity;

pub mod annotation;
pub mod builder;
pub mod error;
pub mod issue;

pub type ReportCollection<'a> = Vec<&'a Report>;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ReportFooter {
    pub message: String,
    pub notes: Vec<String>,
    pub summary: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Report {
    pub issues: Vec<Issue>,
    pub footer: Option<ReportFooter>,
}

pub trait Reportable {
    fn to_reports(&self) -> Vec<&Report>;
}

/// A report.
///
/// A report is a collection of issues.
///
/// Example:
///
/// ```rust
/// use ara_reporting::Report;
/// use ara_reporting::ReportFooter;
/// use ara_reporting::issue::Issue;
/// use ara_reporting::issue::IssueSeverity;
///
/// let report = Report::new()
///     .with_issue(Issue::error("0003", "standalone type `void` cannot be part of a union", "main.ara", 10, 14))
///     .with_issue(Issue::warning("0023", "...", "some_file.ara", 9, 10))
///     .with_footer(ReportFooter::new("This is a report message"));
///
/// # assert_eq!(report.issues.len(), 2);
/// # let footer = report.footer.unwrap();
/// # assert_eq!(footer.message, "This is a report message");
/// # assert!(footer.notes.is_empty());
/// # assert_eq!(report.issues[0].severity, IssueSeverity::Error);
/// # assert_eq!(report.issues[0].code, "0003");
/// # assert_eq!(report.issues[0].message, "standalone type `void` cannot be part of a union");
/// # assert_eq!(report.issues[0].origin, "main.ara");
/// # assert_eq!(report.issues[0].from, 10);
/// # assert_eq!(report.issues[0].to, 14);
/// # assert_eq!(report.issues[1].severity, IssueSeverity::Warning);
/// # assert_eq!(report.issues[1].code, "0023");
/// # assert_eq!(report.issues[1].message, "...");
/// # assert_eq!(report.issues[1].origin, "some_file.ara");
/// # assert_eq!(report.issues[1].from, 9);
/// # assert_eq!(report.issues[1].to, 10);
/// ```
impl Report {
    /// Create a new report.
    pub fn new() -> Self {
        Self {
            issues: vec![],
            footer: None,
        }
    }

    /// Add an issue to this report.
    #[must_use]
    pub fn with_issue(mut self, issue: Issue) -> Self {
        self.issues.push(issue);

        self
    }

    /// Add a footer to this report.
    #[must_use]
    pub fn with_footer(mut self, footer: ReportFooter) -> Self {
        self.footer = Some(footer);

        self
    }

    /// Returns the highest severity of all issues in this report.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::Report;
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueSeverity;
    ///
    /// let empty_report = Report::new();
    ///
    /// let first_report = Report::new()
    ///     .with_issue(Issue::help("0001", "...", "main.ara", 10, 11))
    ///     .with_issue(Issue::warning("0002", "...", "some_file.ara", 9, 10))
    ///     .with_issue(Issue::note("0003", "...", "main.ara", 10, 11));
    ///
    /// let second_report = Report::new()
    ///     .with_issue(Issue::warning("0001", "...", "some_file.ara", 9, 10))
    ///     .with_issue(Issue::bug("0002", "...", "main.ara", 10, 11))
    ///     .with_issue(Issue::error("0003", "...", "main.ara", 10, 11));
    ///
    /// let third_report = Report::new()
    ///     .with_issue(Issue::note("0001", "...", "main.ara", 10, 11))
    ///     .with_issue(Issue::bug("0002", "...", "main.ara", 10, 11))
    ///     .with_issue(Issue::bug("0003", "...", "main.ara", 10, 11));
    ///
    /// assert_eq!(empty_report.severity(), None);
    /// assert_eq!(first_report.severity().unwrap(), IssueSeverity::Warning);
    /// assert_eq!(second_report.severity().unwrap(), IssueSeverity::Error);
    /// assert_eq!(third_report.severity().unwrap(), IssueSeverity::Note);
    /// ```
    pub fn severity(&self) -> Option<IssueSeverity> {
        self.issues.iter().map(|issue| issue.severity).max()
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

impl From<Issue> for Report {
    fn from(val: Issue) -> Self {
        Report {
            issues: vec![val],
            footer: None,
        }
    }
}

/// A footer for a report.
///
/// A footer is a message that is displayed at the end of a report.
impl ReportFooter {
    /// Create a new footer.
    pub fn new<M: Into<String>>(message: M) -> Self {
        Self {
            message: message.into(),
            notes: vec![],
            summary: true,
        }
    }

    /// Add a note to this footer.
    #[must_use]
    pub fn with_note<S: Into<String>>(mut self, note: S) -> Self {
        self.notes.push(note.into());

        self
    }

    /// Defines if either the summary should be enabled or disabled
    #[must_use]
    pub fn with_summary(mut self, enabled: bool) -> Self {
        self.summary = enabled;

        self
    }
}

impl Reportable for Report {
    fn to_reports(&self) -> Vec<&Report> {
        vec![self]
    }
}

impl Reportable for ReportCollection<'_> {
    fn to_reports(&self) -> Vec<&Report> {
        self.to_vec()
    }
}
