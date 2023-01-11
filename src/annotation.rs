use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationType {
    Primary,
    Secondary,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Annotation {
    pub message: Option<String>,
    pub r#type: AnnotationType,
    pub origin: String,
    pub from: usize,
    pub to: usize,
}

impl Annotation {
    /// Create a new annotation.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::annotation::Annotation;
    /// use ara_reporting::annotation::AnnotationType;
    ///
    /// let annotation = Annotation::new(AnnotationType::Secondary, "main.ara", 0, 5);
    ///
    /// assert_eq!(annotation.r#type, AnnotationType::Secondary);
    /// assert_eq!(annotation.message, None);
    /// assert_eq!(annotation.origin, "main.ara");
    /// assert_eq!(annotation.from, 0);
    /// assert_eq!(annotation.to, 5);
    /// ```
    pub fn new<O: Into<String>>(r#type: AnnotationType, origin: O, from: usize, to: usize) -> Self {
        Self {
            r#type,
            message: None,
            origin: origin.into(),
            from,
            to,
        }
    }

    /// Create a primary annotation.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::annotation::Annotation;
    /// use ara_reporting::annotation::AnnotationType;
    ///
    /// let annotation = Annotation::primary("main.ara", 0, 5);
    ///
    /// assert_eq!(annotation.r#type, AnnotationType::Primary);
    /// ```
    pub fn primary<O: Into<String>>(origin: O, from: usize, to: usize) -> Self {
        Self::new(AnnotationType::Primary, origin, from, to)
    }

    /// Create a secondary annotation.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::annotation::Annotation;
    /// use ara_reporting::annotation::AnnotationType;
    ///
    /// let annotation = Annotation::secondary("main.ara", 0, 5);
    ///
    /// assert_eq!(annotation.r#type, AnnotationType::Secondary);
    /// ```
    pub fn secondary<O: Into<String>>(origin: O, from: usize, to: usize) -> Self {
        Self::new(AnnotationType::Secondary, origin, from, to)
    }

    /// Set the message of this annotation.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::annotation::{Annotation, AnnotationType};
    ///
    /// let annotation = Annotation::secondary("main.ara", 10, 1)
    ///     .with_message("try removing this semicolon");
    ///
    /// assert_eq!(annotation.message, Some("try removing this semicolon".to_string()));
    /// ```
    #[must_use]
    pub fn with_message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = Some(message.into());

        self
    }
}
