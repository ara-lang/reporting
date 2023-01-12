use codespan_reporting::diagnostic::Severity;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::annotation::Annotation;

#[derive(
    Debug, PartialEq, Eq, Ord, Copy, Clone, Hash, PartialOrd, Deserialize, Serialize, JsonSchema,
)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum IssueSeverity {
    Note,
    Help,
    Warning,
    Error,
    Bug,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Issue {
    pub severity: IssueSeverity,
    pub code: String,
    pub message: String,
    pub origin: String,
    pub from: usize,
    pub to: usize,
    pub annotations: Vec<Annotation>,
    pub notes: Vec<String>,
}

/// A report issue.
///
/// An issue is a single error or warning in a report.
///
/// Example:
///
/// ```rust
/// # use ara_reporting::issue::Issue;
/// # use ara_reporting::issue::IssueSeverity;
/// # use ara_reporting::annotation::Annotation;
/// let issue = Issue::error("0003", "standalone type `void` cannot be part of a union", "main.ara", 10, 14)
///     .with_annotation(
///         Annotation::secondary("main.ara", 9, 10)
///             .with_message("union type starts here")
///     )
///    .with_note("`void`, `never`, and `mixed` are standalone types and cannot be part of a union, or an intersection")
///    .with_note("consider using `null` instead of `void`")
/// ;
/// # assert_eq!(issue.severity, IssueSeverity::Error);
/// # assert_eq!(issue.code, "0003");
/// # assert_eq!(issue.message, "standalone type `void` cannot be part of a union");
/// # assert_eq!(issue.from, 10);
/// # assert_eq!(issue.to, 14);
/// # assert_eq!(issue.annotations.len(), 1);
/// # assert_eq!(issue.annotations[0].from, 9);
/// # assert_eq!(issue.annotations[0].to, 10);
/// # assert_eq!(issue.annotations[0].message, Some("union type starts here".to_string()));
/// # assert_eq!(issue.notes, vec![
/// #     "`void`, `never`, and `mixed` are standalone types and cannot be part of a union, or an intersection".to_string(),
/// #    "consider using `null` instead of `void`".to_string(),
/// # ]);
/// ```
impl Issue {
    /// Create a new issue with the given code and message.
    pub fn new<C: Into<String>, M: Into<String>, O: Into<String>>(
        severity: IssueSeverity,
        code: C,
        message: M,
        origin: O,
        from: usize,
        to: usize,
    ) -> Self {
        Self {
            severity,
            code: code.into(),
            message: message.into(),
            origin: origin.into(),
            from,
            to,
            annotations: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// Create a new error issue with the given code and message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueSeverity;
    ///
    /// let issue = Issue::error("0003", "...", "main.ara", 10, 11);
    ///
    /// assert_eq!(issue.severity, IssueSeverity::Error);
    /// ```
    pub fn error<C: Into<String>, M: Into<String>, O: Into<String>>(
        code: C,
        message: M,
        origin: O,
        from: usize,
        to: usize,
    ) -> Self {
        Self::new(IssueSeverity::Error, code, message, origin, from, to)
    }

    /// Create a new warning issue with the given code and message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueSeverity;
    ///
    /// let issue = Issue::warning("0003", "...", "main.ara", 10, 11);
    ///
    /// assert_eq!(issue.severity, IssueSeverity::Warning);
    /// ```
    pub fn warning<C: Into<String>, M: Into<String>, O: Into<String>>(
        code: C,
        message: M,
        origin: O,
        from: usize,
        to: usize,
    ) -> Self {
        Self::new(IssueSeverity::Warning, code, message, origin, from, to)
    }

    /// Create a new help issue with the given code and message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueSeverity;
    ///
    /// let issue = Issue::help("0003", "...", "main.ara", 10, 11);
    ///
    /// assert_eq!(issue.severity, IssueSeverity::Help);
    /// ```
    pub fn help<C: Into<String>, M: Into<String>, O: Into<String>>(
        code: C,
        message: M,
        origin: O,
        from: usize,
        to: usize,
    ) -> Self {
        Self::new(IssueSeverity::Help, code, message, origin, from, to)
    }

    /// Create a new note issue with the given code and message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueSeverity;
    ///
    /// let issue = Issue::note("0003", "...", "main.ara", 10, 11);
    ///
    /// assert_eq!(issue.severity, IssueSeverity::Note);
    /// ```
    pub fn note<C: Into<String>, M: Into<String>, O: Into<String>>(
        code: C,
        message: M,
        origin: O,
        from: usize,
        to: usize,
    ) -> Self {
        Self::new(IssueSeverity::Note, code, message, origin, from, to)
    }

    /// Create a new bug issue with the given code and message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::issue::Issue;
    /// use ara_reporting::issue::IssueSeverity;
    ///
    /// let issue = Issue::bug("0003", "...", "main.ara", 10, 11);
    ///
    /// assert_eq!(issue.severity, IssueSeverity::Bug);
    /// ```
    pub fn bug<C: Into<String>, M: Into<String>, O: Into<String>>(
        code: C,
        message: M,
        origin: O,
        from: usize,
        to: usize,
    ) -> Self {
        Self::new(IssueSeverity::Bug, code, message, origin, from, to)
    }

    /// Add an annotation to this issue.
    #[must_use]
    pub fn with_annotation(mut self, annotation: Annotation) -> Self {
        self.annotations.push(annotation);

        self
    }

    /// Add a note to this issue.
    #[must_use]
    pub fn with_note<S: Into<String>>(mut self, note: S) -> Self {
        self.notes.push(note.into());

        self
    }
}

/// Returns `Severity` from `IssueSeverity`
///
/// Example:
///
/// ```rust
/// use codespan_reporting::diagnostic::Severity;
/// use ara_reporting::issue::IssueSeverity;
///
/// assert_eq!(Severity::Error, IssueSeverity::Error.into());
/// ```
#[doc(hidden)]
impl From<IssueSeverity> for Severity {
    fn from(severity: IssueSeverity) -> Self {
        match severity {
            IssueSeverity::Error => Severity::Error,
            IssueSeverity::Warning => Severity::Warning,
            IssueSeverity::Note => Severity::Note,
            IssueSeverity::Help => Severity::Help,
            IssueSeverity::Bug => Severity::Bug,
        }
    }
}

/// Display the issue kind as a string.
///
/// Example:
///
/// ```rust
/// use ara_reporting::issue::IssueSeverity;
///
/// assert_eq!(IssueSeverity::Error.to_string(), "error");
/// assert_eq!(IssueSeverity::Warning.to_string(), "warning");
/// assert_eq!(IssueSeverity::Help.to_string(), "help");
/// assert_eq!(IssueSeverity::Note.to_string(), "note");
/// assert_eq!(IssueSeverity::Bug.to_string(), "bug");
/// ```
impl std::fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueSeverity::Error => write!(f, "error"),
            IssueSeverity::Warning => write!(f, "warning"),
            IssueSeverity::Help => write!(f, "help"),
            IssueSeverity::Note => write!(f, "note"),
            IssueSeverity::Bug => write!(f, "bug"),
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
            self.severity, self.code, self.message, self.origin, self.from, self.to
        )
    }
}
