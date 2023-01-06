use std::collections::HashMap;

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

use ara_source::SourceMap;

use crate::error::Error;
use crate::issue::IssueSeverity;
use crate::Report;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Charset {
    Ascii,
    Unicode,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ColorChoice {
    Always,
    Auto,
    Never,
}

#[derive(Debug, Clone)]
pub struct ReportBuilder<'a> {
    pub source_map: &'a SourceMap,
    pub report: Report,
    pub colors: ColorChoice,
    pub charset: Charset,
}

/// A report builder.
///
/// A report builder is used to build a report.
///
/// Example:
///
/// ```rust
/// use ara_source::source::Source;
/// use ara_source::source::SourceKind;
/// use ara_source::SourceMap;
///
/// use ara_reporting::builder::ReportBuilder;
/// use ara_reporting::Report;
///
/// let report = Report::new();
/// let source = SourceMap::new(vec![
///     Source::inline(SourceKind::Script, "function main(): void {}"),
/// ]);
///
/// let builder = ReportBuilder::new(&source, report);
/// # assert_eq!(builder.source_map.sources[0].content, "function main(): void {}");
/// ```
impl ReportBuilder<'_> {
    /// Create a new report builder.
    pub fn new(source_map: &SourceMap, report: Report) -> ReportBuilder {
        ReportBuilder {
            source_map,
            report,
            colors: ColorChoice::Auto,
            charset: Charset::Ascii,
        }
    }

    /// Set the color choice.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use ara_source::source::Source;
    /// # use ara_source::source::SourceKind;
    /// # use ara_source::SourceMap;
    /// # use ara_reporting::builder::ReportBuilder;
    /// # use ara_reporting::builder::ColorChoice;
    /// # use ara_reporting::Report;
    /// # let report = Report::new();
    /// # let source = SourceMap::new(vec![
    /// #     Source::inline(SourceKind::Script, "function main(): void {}"),
    /// # ]);
    /// # let builder = ReportBuilder::new(&source, report);
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
    /// # use ara_source::source::Source;
    /// # use ara_source::source::SourceKind;
    /// # use ara_source::SourceMap;
    /// # use ara_reporting::builder::ReportBuilder;
    /// # use ara_reporting::builder::Charset;
    /// # use ara_reporting::Report;
    /// # let report = Report::new();
    /// # let source = SourceMap::new(vec![
    /// #     Source::inline(SourceKind::Script, "function main(): void {}"),
    /// # ]);
    /// # let builder = ReportBuilder::new(&source, report);
    ///
    /// let builder = builder.with_charset(Charset::Ascii);
    /// assert_eq!(builder.charset, Charset::Ascii);
    ///
    /// let builder = builder.with_charset(Charset::Unicode);
    /// assert_eq!(builder.charset, Charset::Unicode);
    /// ```
    pub fn with_charset(mut self, charset: Charset) -> Self {
        self.charset = charset;

        self
    }

    /// Print the report to stdout.
    pub fn print(&self) -> Result<(), Error> {
        let mut writer = StandardStream::stdout(match self.colors {
            ColorChoice::Always => match self.charset {
                Charset::Ascii => TermColorChoice::AlwaysAnsi,
                Charset::Unicode => TermColorChoice::Always,
            },
            ColorChoice::Auto => TermColorChoice::Auto,
            ColorChoice::Never => TermColorChoice::Never,
        });

        self.write(&mut writer)
    }

    /// Print the report to stderr.
    pub fn eprint(&self) -> Result<(), Error> {
        let mut writer = StandardStream::stderr(match self.colors {
            ColorChoice::Always => match self.charset {
                Charset::Ascii => TermColorChoice::AlwaysAnsi,
                Charset::Unicode => TermColorChoice::Always,
            },
            ColorChoice::Auto => TermColorChoice::Auto,
            ColorChoice::Never => TermColorChoice::Never,
        });

        self.write(&mut writer)
    }

    /// Get the report as a string.
    pub fn as_string(&self) -> Result<String, Error> {
        let buffer = BufferWriter::stderr(match self.colors {
            ColorChoice::Always => match self.charset {
                Charset::Ascii => TermColorChoice::AlwaysAnsi,
                Charset::Unicode => TermColorChoice::Always,
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
            chars: match self.charset {
                Charset::Ascii => Chars::ascii(),
                Charset::Unicode => Chars::box_drawing(),
            },
            ..Default::default()
        };

        let mut files = SimpleFiles::new();
        let mut ids = HashMap::new();
        for source in &self.source_map.sources {
            let name = source.name();
            let file_id = files.add(name, &source.content);

            ids.insert(name.to_string(), file_id);
        }

        for issue in &self.report.issues {
            let mut diagnostic = match issue.severity {
                IssueSeverity::Error => Diagnostic::error(),
                IssueSeverity::Warning => Diagnostic::warning(),
                IssueSeverity::Note => Diagnostic::note(),
                IssueSeverity::Help => Diagnostic::help(),
                IssueSeverity::Bug => Diagnostic::bug(),
            };

            diagnostic = diagnostic
                .with_code(&issue.code)
                .with_message(&issue.message)
                .with_labels(vec![Label::primary(
                    *ids.get(&issue.origin).unwrap_or(&0),
                    issue.from..issue.to,
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
                            *ids.get(&annotation.origin).unwrap_or(&0),
                            annotation.from..annotation.to,
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
