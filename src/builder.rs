use std::collections::HashMap;

use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::diagnostic::Label;
use codespan_reporting::diagnostic::LabelStyle;
use codespan_reporting::files::Error as CodespanError;
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::emit;
use codespan_reporting::term::Chars;
use codespan_reporting::term::Config;
use codespan_reporting::term::DisplayStyle as CodespanDisplayStyle;
use codespan_reporting::term::Styles;
use termcolor::BufferWriter;
use termcolor::Color;
use termcolor::ColorChoice as TermColorChoice;
use termcolor::StandardStream;
use termcolor::WriteColor;

use ara_source::SourceMap;

use crate::annotation::AnnotationType;
use crate::error::Error;
use crate::issue::IssueSeverity;
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
pub enum DisplayStyle {
    Default,
    Comfortable,
    Compact,
}

#[derive(Debug, Clone)]
pub struct ReportBuilder<'a> {
    pub source_map: &'a SourceMap,
    pub report: Report,
    pub colors: ColorChoice,
    pub charset: CharSet,
    pub style: DisplayStyle,
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
            charset: CharSet::Ascii,
            style: DisplayStyle::Default,
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
    /// # use ara_reporting::builder::CharSet;
    /// # use ara_reporting::Report;
    /// # let report = Report::new();
    /// # let source = SourceMap::new(vec![
    /// #     Source::inline(SourceKind::Script, "function main(): void {}"),
    /// # ]);
    /// # let builder = ReportBuilder::new(&source, report);
    /// let builder = builder.with_charset(CharSet::Ascii);
    /// # assert_eq!(builder.charset, CharSet::Ascii);
    /// let builder = builder.with_charset(CharSet::Unicode);
    /// # assert_eq!(builder.charset, CharSet::Unicode);
    /// ```
    pub fn with_charset(mut self, charset: CharSet) -> Self {
        self.charset = charset;

        self
    }

    /// Set the display style
    ///
    /// Example:
    ///
    /// ```rust
    /// # use ara_source::source::Source;
    /// # use ara_source::source::SourceKind;
    /// # use ara_source::SourceMap;
    /// # use ara_reporting::builder::ReportBuilder;
    /// # use ara_reporting::builder::DisplayStyle;
    /// # use ara_reporting::Report;
    /// # let report = Report::new();
    /// # let source = SourceMap::new(vec![
    /// #     Source::inline(SourceKind::Script, "function main(): void {}"),
    /// # ]);
    /// # let builder = ReportBuilder::new(&source, report);
    /// let builder = builder.with_style(DisplayStyle::Default);
    /// # assert_eq!(builder.style, DisplayStyle::Default);
    /// let builder = builder.with_style(DisplayStyle::Comfortable);
    /// # assert_eq!(builder.style, DisplayStyle::Comfortable);
    /// let builder = builder.with_style(DisplayStyle::Compact);
    /// # assert_eq!(builder.style, DisplayStyle::Compact);
    /// ```
    pub fn with_style(mut self, style: DisplayStyle) -> Self {
        self.style = style;

        self
    }

    /// Print the report to stdout.
    pub fn print(&self) -> Result<(), Error> {
        let mut writer = StandardStream::stdout(match self.colors {
            ColorChoice::Always => match self.charset {
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
            ColorChoice::Always => match self.charset {
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
            ColorChoice::Always => match self.charset {
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
        let mut styles = Styles::default();

        styles.secondary_label.set_bold(true);
        styles.line_number.set_fg(Some(Color::Ansi256(8)));
        styles.source_border.set_fg(Some(Color::Ansi256(8)));

        let config = Config {
            display_style: match self.style {
                DisplayStyle::Default => CodespanDisplayStyle::Rich,
                DisplayStyle::Comfortable => CodespanDisplayStyle::Medium,
                DisplayStyle::Compact => CodespanDisplayStyle::Short,
            },
            chars: match self.charset {
                CharSet::Ascii => Chars::ascii(),
                CharSet::Unicode => Chars::box_drawing(),
            },
            tab_width: 2,
            styles,
            start_context_lines: 1,
            end_context_lines: 1,
        };

        let mut files = SimpleFiles::new();
        let mut ids = HashMap::new();
        for source in &self.source_map.sources {
            let name = source.name();
            let file_id = files.add(name, &source.content);

            ids.insert(name.to_string(), file_id);
        }

        for issue in &self.report.issues {
            let diagnostic = Diagnostic::new(issue.severity.into())
                .with_code(&issue.code)
                .with_message(&issue.message)
                .with_labels(vec![Label::primary(
                    *ids.get(&issue.origin).unwrap_or(&0),
                    issue.from..issue.to,
                )])
                .with_notes(issue.notes.clone())
                .with_labels(
                    issue
                        .annotations
                        .iter()
                        .map(|annotation| {
                            let mut label = Label::new(
                                match annotation.r#type {
                                    AnnotationType::Primary => LabelStyle::Primary,
                                    AnnotationType::Secondary => LabelStyle::Secondary,
                                },
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

        if let Some(footer) = &self.report.footer {
            let diagnostic = Diagnostic::new(
                self.report
                    .severity()
                    .unwrap_or(IssueSeverity::Error)
                    .into(),
            )
            .with_message(&footer.message)
            .with_notes(footer.notes.clone());

            emit(&mut w, &config, &files, &diagnostic).ok();
        }

        Ok(())
    }
}
