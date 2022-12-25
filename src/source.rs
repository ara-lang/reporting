use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

    /// Get the line number for the given position.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::source::Source;
    ///
    /// let source = Source::new("main.ara", "function 👋(): void {}");
    ///
    /// assert_eq!(source.get_character_position(20), 17);
    /// ```
    #[doc(hidden)]
    pub fn get_character_position(&self, position: usize) -> usize {
        let slice = &self.content[..position];

        slice.chars().count()
    }
}
