use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::Error as CodespanError;
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::emit;
use codespan_reporting::term::Chars;
use codespan_reporting::term::Config;
use termcolor::BufferWriter;
use termcolor::ColorChoice as TermColorChoice;
use termcolor::StandardStream;
use termcolor::WriteColor;

use crate::error::Error;
use crate::issue::IssueKind;
use crate::source::Source;
use crate::Report;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CharSet {
    Ascii,
    Unicode,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ColorChoice {
    Always,
    Auto,
    Never,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReportBuilder<'a> {
    pub source: Source<'a>,
    pub report: Report,
    pub colors: ColorChoice,
    pub char_set: CharSet,
}

/// A report builder.
///
/// A report builder is used to build a report.
///
/// Example:
///
/// ```rust
/// use ara_reporting::builder::ReportBuilder;
/// # use ara_reporting::source::Source;
/// # use ara_reporting::Report;
/// #
/// # let source = Source::inline("function main(): void {}");
/// # let report = Report::new();
///
/// let builder = ReportBuilder::new(source, report);
/// # assert_eq!(builder.source.content, "function main(): void {}");
/// ```
impl ReportBuilder<'_> {
    /// Create a new report builder.
    pub fn new(source: Source, report: Report) -> ReportBuilder {
        ReportBuilder {
            source,
            report,
            colors: ColorChoice::Auto,
            char_set: CharSet::Ascii,
        }
    }

    /// Set the color choice.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use ara_reporting::builder::{ColorChoice, ReportBuilder};
    /// # use ara_reporting::source::Source;
    /// # let source = Source::inline("function main(): void {}");
    /// # let builder = ReportBuilder::new(source, Default::default());
    ///
    /// let builder = builder.with_colors(ColorChoice::Never);
    /// assert_eq!(builder.colors, ColorChoice::Never);
    ///
    /// let builder = builder.with_colors(ColorChoice::Always);
    /// assert_eq!(builder.colors, ColorChoice::Always);
    ///
    /// let builder = builder.with_colors(ColorChoice::Auto);
    /// assert_eq!(builder.colors, ColorChoice::Auto);
    /// ```
    pub fn with_colors(mut self, colors: ColorChoice) -> Self {
        self.colors = colors;

        self
    }

    /// Set the character set.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use ara_reporting::builder::{CharSet, ReportBuilder};
    /// # use ara_reporting::source::Source;
    /// # let source = Source::inline("function main(): void {}");
    /// # let builder = ReportBuilder::new(source, Default::default());
    ///
    /// let builder = builder.with_char_set(CharSet::Ascii);
    /// assert_eq!(builder.char_set, CharSet::Ascii);
    ///
    /// let builder = builder.with_char_set(CharSet::Unicode);
    /// assert_eq!(builder.char_set, CharSet::Unicode);
    /// ```
    pub fn with_char_set(mut self, char_set: CharSet) -> Self {
        self.char_set = char_set;

        self
    }

    /// Print the report to stdout.
    pub fn print(&self) -> Result<(), Error> {
        let mut writer = StandardStream::stdout(match self.colors {
            ColorChoice::Always => match self.char_set {
                CharSet::Ascii => TermColorChoice::AlwaysAnsi,
                CharSet::Unicode => TermColorChoice::Always,
            },
            ColorChoice::Auto => TermColorChoice::Auto,
            ColorChoice::Never => TermColorChoice::Never,
        });

        self.write(&mut writer)
    }

    /// Print the report to stderr.
    pub fn eprint(&self) -> Result<(), Error> {
        let mut writer = StandardStream::stderr(match self.colors {
            ColorChoice::Always => match self.char_set {
                CharSet::Ascii => TermColorChoice::AlwaysAnsi,
                CharSet::Unicode => TermColorChoice::Always,
            },
            ColorChoice::Auto => TermColorChoice::Auto,
            ColorChoice::Never => TermColorChoice::Never,
        });

        self.write(&mut writer)
    }

    /// Get the report as a string.
    pub fn as_string(&self) -> Result<String, Error> {
        let buffer = BufferWriter::stderr(match self.colors {
            ColorChoice::Always => match self.char_set {
                CharSet::Ascii => TermColorChoice::AlwaysAnsi,
                CharSet::Unicode => TermColorChoice::Always,
            },
            ColorChoice::Auto => TermColorChoice::Auto,
            ColorChoice::Never => TermColorChoice::Never,
        });

        let mut buffer = buffer.buffer();

        self.write(&mut buffer)?;

        Ok(String::from_utf8_lossy(buffer.as_slice()).to_string())
    }

    /// Write the report to the given writer.
    pub fn write<T: WriteColor>(&self, mut w: T) -> Result<(), Error> {
        let config = Config {
            chars: match self.char_set {
                CharSet::Ascii => Chars::ascii(),
                CharSet::Unicode => Chars::box_drawing(),
            },
            ..Default::default()
        };

        let mut files = SimpleFiles::new();
        let origin = files.add(self.source.origin.unwrap_or("inline"), self.source.content);

        for issue in &self.report.issues {
            let mut diagnostic = match issue.kind {
                IssueKind::Error => Diagnostic::error(),
                IssueKind::Warning => Diagnostic::warning(),
                IssueKind::Note => Diagnostic::note(),
                IssueKind::Help => Diagnostic::help(),
                IssueKind::Bug => Diagnostic::bug(),
            };

            diagnostic = diagnostic
                .with_code(&issue.code)
                .with_message(&issue.message)
                .with_labels(vec![Label::primary(
                    origin,
                    issue.position..(issue.position + issue.length),
                )
                .with_message(&issue.message)]);

            if let Some(note) = &issue.note {
                diagnostic = diagnostic.with_notes(vec![format!("note: {}", note)]);
            }

            if let Some(help) = &issue.help {
                diagnostic = diagnostic.with_notes(vec![format!("help: {}", help)]);
            }

            diagnostic = diagnostic.with_labels(
                issue
                    .annotations
                    .iter()
                    .map(|annotation| {
                        let mut label = Label::secondary(
                            origin,
                            (annotation.position)..(annotation.position + annotation.length),
                        );

                        if let Some(message) = &annotation.message {
                            label = label.with_message(message);
                        }

                        label
                    })
                    .collect(),
            );

            match emit(&mut w, &config, &files, &diagnostic) {
                Ok(_) => (),
                Err(err) => match err {
                    CodespanError::FileMissing => Err(Error::FileMissing)?,
                    CodespanError::IndexTooLarge { given, max } => {
                        Err(Error::IndexTooLarge { given, max })?
                    }
                    CodespanError::LineTooLarge { given, max } => {
                        Err(Error::LineTooLarge { given, max })?
                    }
                    CodespanError::ColumnTooLarge { given, max } => {
                        Err(Error::ColumnTooLarge { given, max })?
                    }
                    CodespanError::InvalidCharBoundary { given } => {
                        Err(Error::InvalidCharBoundary { given })?
                    }
                    CodespanError::Io(err) => Err(Error::Io(err))?,
                    other => Err(Error::CodespanError(other))?,
                },
            }
        }

        Ok(())
    }
}
