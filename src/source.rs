#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Source<'a> {
    pub origin: Option<&'a str>,
    pub content: &'a str,
}

/// A report source.
///
/// A source is a single source file in a report.
///
/// Example:
///
/// ```rust
/// use ara_reporting::source::Source;
/// use ara_reporting::issue::Issue;
/// use ara_reporting::issue::IssueKind;
/// use ara_reporting::annotation::Annotation;
///
/// let source = Source::new("main.ara", "function main(): void {}");
///
/// assert_eq!(source.origin, Some("main.ara"));
/// assert_eq!(source.content, "function main(): void {}");
/// ```
impl Source<'_> {
    /// Create a new source with the given content.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::source::Source;
    ///
    /// let source = Source::new("main.ara", "function main(): void {}");
    ///
    /// assert_eq!(source.origin, Some("main.ara"));
    /// assert_eq!(source.content, "function main(): void {}");
    /// ```
    pub fn new<'a>(origin: &'a str, content: &'a str) -> Source<'a> {
        Source {
            origin: Some(origin),
            content,
        }
    }

    /// Create a new source with the given content.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::source::Source;
    ///
    /// let source = Source::inline("function main(): void {}");
    ///
    /// assert_eq!(source.origin, None);
    /// assert_eq!(source.content, "function main(): void {}");
    /// ```
    pub fn inline(content: &str) -> Source {
        Source {
            origin: None,
            content,
        }
    }
}
