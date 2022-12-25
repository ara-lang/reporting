use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Annotation {
    pub message: Option<String>,
    pub position: usize,
    pub length: usize,
}

impl Annotation {
    /// Create a new annotation with the given message.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::annotation::Annotation;
    ///
    /// let annotation = Annotation::new(0, 5);
    ///
    /// assert_eq!(annotation.position, 0);
    /// assert_eq!(annotation.length, 5);
    /// ```
    pub fn new(position: usize, length: usize) -> Self {
        Self {
            message: None,
            position,
            length,
        }
    }

    /// Set the message of this annotation.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_reporting::annotation::Annotation;
    ///
    /// let annotation = Annotation::new(10, 1)
    ///     .with_message("try removing this semicolon");
    ///
    /// assert_eq!(annotation.message, Some("try removing this semicolon".to_string()));
    /// ```
    pub fn with_message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = Some(message.into());

        self
    }
}
