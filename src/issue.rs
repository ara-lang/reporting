use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::annotation::Annotation;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum IssueKind {
    Error,
    Warning,
    Notice,
    Deprecation,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Issue {
    pub kind: IssueKind,
    pub code: String,
    pub message: String,
    pub position: usize,
    pub length: usize,
    pub annotations: Vec<Annotation>,
    pub note: Option<String>,
    pub help: Option<String>,
}

/// A report issue.
///
/// An issue is a single error or warning in a report.
///
/// Example:
///
/// ```rust
/// use ara_reporting::issue::Issue;
/// use ara_reporting::issue::IssueKind;
/// use ara_reporting::annotation::Annotation;
///
/// let issue = Issue::error("0003", "standalone type `void` cannot be part of a union", 10, 4)
///     .with_annotation(
///         Annotation::new(9, 1)
///             .with_message("union type starts here")
///     )
///    .with_note("`void`, `never`, and `mixed` are standalone types and cannot be part of a union, or an intersection")
///    .with_help("consider using `null` instead of `void`")
/// ;
///
/// assert_eq!(issue.kind, IssueKind::Error);
/// assert_eq!(issue.code, "0003");
/// assert_eq!(issue.message, "standalone type `void` cannot be part of a union");
/// assert_eq!(issue.position, 10);
/// assert_eq!(issue.length, 4);
/// assert_eq!(issue.annotations.len(), 1);
/// assert_eq!(issue.annotations[0].position, 9);
/// assert_eq!(issue.annotations[0].length, 1);
/// assert_eq!(issue.annotations[0].message, Some("union type starts here".to_string()));
/// assert_eq!(issue.note, Some("`void`, `never`, and `mixed` are standalone types and cannot be part of a union, or an intersection".to_string()));
/// assert_eq!(issue.help, Some("consider using `null` instead of `void`".to_string()));
/// ```
impl Issue {
    /// Create a new issue with the given code and message.
    pub fn new<S: Into<String>, C: Into<String>>(
        kind: IssueKind,
        code: S,
        message: C,
        position: usize,
        length: usize,
    ) -> Self {
        Self {
            kind,
            code: code.into(),
            message: message.into(),
            position,
            length,
            annotations: Vec::new(),
            note: None,
            help: None,
        }
    }

    /// Create a new error issue with the given code and message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueKind;
    ///
    /// let issue = Issue::error("0003", "unexpected token `{`, expecting `[`", 10, 1);
    ///
    /// assert_eq!(issue.kind, IssueKind::Error);
    /// ```
    pub fn error<S: Into<String>, C: Into<String>>(
        code: S,
        message: C,
        position: usize,
        length: usize,
    ) -> Self {
        Self::new(IssueKind::Error, code, message, position, length)
    }

    /// Create a new warning issue with the given code and message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueKind;
    ///
    /// let issue = Issue::warning("0003", "unexpected token `{`, expecting `[`", 10, 1);
    ///
    /// assert_eq!(issue.kind, IssueKind::Warning);
    /// ```
    pub fn warning<S: Into<String>, C: Into<String>>(
        code: S,
        message: C,
        position: usize,
        length: usize,
    ) -> Self {
        Self::new(IssueKind::Warning, code, message, position, length)
    }

    /// Create a new notice issue with the given code and message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueKind;
    ///
    /// let issue = Issue::notice("0003", "unexpected token `{`, expecting `[`", 10, 1);
    ///
    /// assert_eq!(issue.kind, IssueKind::Notice);
    /// ```
    pub fn notice<S: Into<String>, C: Into<String>>(
        code: S,
        message: C,
        position: usize,
        length: usize,
    ) -> Self {
        Self::new(IssueKind::Notice, code, message, position, length)
    }

    /// Create a new deprecation issue with the given code and message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueKind;
    ///
    /// let issue = Issue::deprecation("0003", "untyped properties are deprecated since version 0.0.0", 10, 1);
    ///
    /// assert_eq!(issue.kind, IssueKind::Deprecation);
    /// ```
    pub fn deprecation<S: Into<String>, C: Into<String>>(
        code: S,
        message: C,
        position: usize,
        length: usize,
    ) -> Self {
        Self::new(IssueKind::Deprecation, code, message, position, length)
    }

    /// Add an annotation to this issue.
    pub fn with_annotation(mut self, annotation: Annotation) -> Self {
        self.annotations.push(annotation);

        self
    }

    /// Add a note to this issue.
    pub fn with_note<S: Into<String>>(mut self, note: S) -> Self {
        self.note = Some(note.into());

        self
    }

    /// Add a help message to this issue.
    pub fn with_help<S: Into<String>>(mut self, help: S) -> Self {
        self.help = Some(help.into());

        self
    }
}

/// Display the issue kind as a string.
///
/// Example:
///
/// ```rust
/// use ara_reporting::issue::IssueKind;
///
/// assert_eq!(IssueKind::Error.to_string(), "Error");
/// assert_eq!(IssueKind::Warning.to_string(), "Warning");
/// assert_eq!(IssueKind::Notice.to_string(), "Notice");
/// assert_eq!(IssueKind::Deprecation.to_string(), "Deprecation");
/// ```
impl std::fmt::Display for IssueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueKind::Error => write!(f, "Error"),
            IssueKind::Warning => write!(f, "Warning"),
            IssueKind::Notice => write!(f, "Notice"),
            IssueKind::Deprecation => write!(f, "Deprecation"),
        }
    }
}

/// Display the issue as a string.
///
/// Example:
///
/// ```rust
/// use ara_reporting::issue::Issue;
///
/// let issue = Issue::error("E0231", "unexpected token `{`, expecting `[`", 10, 1);
///
/// assert_eq!(issue.to_string(), "[E0231] Error: unexpected token `{`, expecting `[` at 10:1");
/// ```
impl std::fmt::Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}: {} at {}:{}",
            self.code, self.kind, self.message, self.position, self.length
        )
    }
}
