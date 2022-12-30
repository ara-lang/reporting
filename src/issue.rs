use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::annotation::Annotation;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum IssueKind {
    Error,
    Warning,
    Help,
    Note,
    Bug,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Issue {
    pub kind: IssueKind,
    pub code: String,
    pub message: String,
    pub origin: String,
    pub from: usize,
    pub to: usize,
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
/// let issue = Issue::error("0003", "standalone type `void` cannot be part of a union", "main.ara", 10, 14)
///     .with_annotation(
///         Annotation::new("main.ara", 9, 10)
///             .with_message("union type starts here")
///     )
///    .with_note("`void`, `never`, and `mixed` are standalone types and cannot be part of a union, or an intersection")
///    .with_help("consider using `null` instead of `void`")
/// ;
///
/// assert_eq!(issue.kind, IssueKind::Error);
/// assert_eq!(issue.code, "0003");
/// assert_eq!(issue.message, "standalone type `void` cannot be part of a union");
/// assert_eq!(issue.from, 10);
/// assert_eq!(issue.to, 14);
/// assert_eq!(issue.annotations.len(), 1);
/// assert_eq!(issue.annotations[0].from, 9);
/// assert_eq!(issue.annotations[0].to, 10);
/// assert_eq!(issue.annotations[0].message, Some("union type starts here".to_string()));
/// assert_eq!(issue.note, Some("`void`, `never`, and `mixed` are standalone types and cannot be part of a union, or an intersection".to_string()));
/// assert_eq!(issue.help, Some("consider using `null` instead of `void`".to_string()));
/// ```
impl Issue {
    /// Create a new issue with the given code and message.
    pub fn new<C: Into<String>, M: Into<String>, O: Into<String>>(
        kind: IssueKind,
        code: C,
        message: M,
        origin: O,
        from: usize,
        to: usize,
    ) -> Self {
        Self {
            kind,
            code: code.into(),
            message: message.into(),
            origin: origin.into(),
            from,
            to,
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
    /// let issue = Issue::error("0003", "...", "main.ara", 10, 11);
    ///
    /// assert_eq!(issue.kind, IssueKind::Error);
    /// ```
    pub fn error<C: Into<String>, M: Into<String>, O: Into<String>>(
        code: C,
        message: M,
        origin: O,
        from: usize,
        to: usize,
    ) -> Self {
        Self::new(IssueKind::Error, code, message, origin, from, to)
    }

    /// Create a new warning issue with the given code and message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueKind;
    ///
    /// let issue = Issue::warning("0003", "...", "main.ara", 10, 11);
    ///
    /// assert_eq!(issue.kind, IssueKind::Warning);
    /// ```
    pub fn warning<C: Into<String>, M: Into<String>, O: Into<String>>(
        code: C,
        message: M,
        origin: O,
        from: usize,
        to: usize,
    ) -> Self {
        Self::new(IssueKind::Warning, code, message, origin, from, to)
    }

    /// Create a new help issue with the given code and message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueKind;
    ///
    /// let issue = Issue::help("0003", "...", "main.ara", 10, 11);
    ///
    /// assert_eq!(issue.kind, IssueKind::Help);
    /// ```
    pub fn help<C: Into<String>, M: Into<String>, O: Into<String>>(
        code: C,
        message: M,
        origin: O,
        from: usize,
        to: usize,
    ) -> Self {
        Self::new(IssueKind::Help, code, message, origin, from, to)
    }

    /// Create a new note issue with the given code and message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueKind;
    ///
    /// let issue = Issue::note("0003", "...", "main.ara", 10, 11);
    ///
    /// assert_eq!(issue.kind, IssueKind::Note);
    /// ```
    pub fn note<C: Into<String>, M: Into<String>, O: Into<String>>(
        code: C,
        message: M,
        origin: O,
        from: usize,
        to: usize,
    ) -> Self {
        Self::new(IssueKind::Note, code, message, origin, from, to)
    }

    /// Create a new bug issue with the given code and message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueKind;
    ///
    /// let issue = Issue::bug("0003", "...", "main.ara", 10, 11);
    ///
    /// assert_eq!(issue.kind, IssueKind::Bug);
    /// ```
    pub fn bug<C: Into<String>, M: Into<String>, O: Into<String>>(
        code: C,
        message: M,
        origin: O,
        from: usize,
        to: usize,
    ) -> Self {
        Self::new(IssueKind::Bug, code, message, origin, from, to)
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
/// assert_eq!(IssueKind::Error.to_string(), "error");
/// assert_eq!(IssueKind::Warning.to_string(), "warning");
/// assert_eq!(IssueKind::Help.to_string(), "help");
/// assert_eq!(IssueKind::Note.to_string(), "note");
/// assert_eq!(IssueKind::Bug.to_string(), "bug");
/// ```
impl std::fmt::Display for IssueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueKind::Error => write!(f, "error"),
            IssueKind::Warning => write!(f, "warning"),
            IssueKind::Help => write!(f, "help"),
            IssueKind::Note => write!(f, "note"),
            IssueKind::Bug => write!(f, "bug"),
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
/// let issue = Issue::error("E0231", "unexpected token `{`, expecting `[`", "main.ara", 10, 1);
///
/// assert_eq!(issue.to_string(), "error[E0231]: unexpected token `{`, expecting `[` at main.ara@10:1");
/// ```
impl std::fmt::Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{}]: {} at {}@{}:{}",
            self.kind, self.code, self.message, self.origin, self.from, self.to
        )
    }
}
