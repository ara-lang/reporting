use std::io::Result;
use std::io::Write;

use ariadne::{
    CharSet, Color, Config, Label, Report as AriadneReport, ReportKind, Source as AriadneSource,
};

use crate::issue::IssueKind;
use crate::source::Source;
use crate::Report;

pub struct ReportBuilder<'a> {
    pub source: Source<'a>,
    pub report: Report,
    pub colors: bool,
    pub ascii: bool,
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
            colors: true,
            ascii: false,
        }
    }

    /// Set whether or not to use colors.
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
    /// let builder = ReportBuilder::new(source, report).with_colors(false);
    ///
    /// assert_eq!(builder.colors, false);
    /// ```
    pub fn with_colors(mut self, colors: bool) -> Self {
        self.colors = colors;
        self
    }

    /// Set whether or not to use ASCII characters.
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
    /// let builder = ReportBuilder::new(source, report).with_ascii(true);
    ///
    /// assert_eq!(builder.ascii, true);
    /// ```
    pub fn with_ascii(mut self, ascii: bool) -> Self {
        self.ascii = ascii;
        self
    }

    /// Print the report to stdout.
    pub fn print(&self) -> Result<()> {
        self.write(std::io::stdout())
    }

    /// Print the report to stderr.
    pub fn eprint(&self) -> Result<()> {
        self.write(std::io::stderr())
    }

    /// Get the report as a string.
    pub fn as_string(&self) -> Result<String> {
        let mut buffer = Vec::new();
        self.write(&mut buffer)?;

        Ok(unsafe {
            // SAFETY: The buffer is always valid UTF-8.
            String::from_utf8_unchecked(buffer)
        })
    }

    /// Write the report to the given writer.
    pub fn write<T: Write>(&self, mut w: T) -> Result<()> {
        let origin = self.source.origin.unwrap_or("inline");

        let config = Config::default()
            .with_color(self.colors)
            .with_char_set(if self.ascii {
                CharSet::Ascii
            } else {
                CharSet::Unicode
            });

        for issue in &self.report.issues {
            let color = match issue.kind {
                IssueKind::Error => Color::Fixed(9),
                IssueKind::Warning => Color::Fixed(11),
                IssueKind::Notice => Color::Fixed(12),
                IssueKind::Deprecation => Color::Fixed(14),
            };

            let kind = match issue.kind {
                IssueKind::Error => ReportKind::Custom("Error", color),
                IssueKind::Warning => ReportKind::Custom("Warning", color),
                IssueKind::Notice => ReportKind::Custom("Notice", color),
                IssueKind::Deprecation => ReportKind::Custom("Deprecation", color),
            };

            let mut report = AriadneReport::build(
                kind,
                origin,
                self.source.get_character_position(issue.position),
            )
            .with_code(&issue.code)
            .with_message(&issue.message)
            .with_config(config);

            let mut label = Label::new((
                origin,
                self.source.get_character_position(issue.position)
                    ..self
                        .source
                        .get_character_position(issue.position + issue.length),
            ))
            .with_order(0);

            if self.colors {
                label = label.with_color(color);
            }

            report = report.with_label(label);

            if let Some(note) = &issue.note {
                report = report.with_note(note);
            }

            if let Some(help) = &issue.help {
                report = report.with_help(help);
            }

            for (order, annotation) in issue.annotations.iter().enumerate() {
                let mut label = Label::new((
                    origin,
                    self.source.get_character_position(annotation.position)
                        ..self
                            .source
                            .get_character_position(annotation.position + annotation.length),
                ))
                .with_order((order + 1).try_into().unwrap());

                if self.colors {
                    label = label.with_color(Color::White);
                }

                if let Some(message) = &annotation.message {
                    label = label.with_message(message);
                }

                report = report.with_label(label);
            }

            report
                .finish()
                .write((origin, AriadneSource::from(self.source.content)), &mut w)?;

            writeln!(&mut w)?;
        }

        Ok(())
    }
}
