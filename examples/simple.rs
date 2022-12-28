use ara_reporting::annotation::Annotation;
use ara_reporting::builder::CharSet;
use ara_reporting::builder::ColorChoice;
use ara_reporting::builder::ReportBuilder;
use ara_reporting::error::Error;
use ara_reporting::issue::Issue;
use ara_reporting::source::Source;
use ara_reporting::Report;

fn main() -> Result<(), Error> {
    let source = Source::inline(
        r#"
$b = match $a {
    1 => 2,
    2 => 3,
    default => "string",
};
"#,
    );

    let report = Report::new().with_issue(
        Issue::error("E0417", "`match` arms have incompatible types", 6, 61)
            .with_annotation(
                Annotation::new(26, 1).with_message("this is found to be of type `{int}`"),
            )
            .with_annotation(
                Annotation::new(38, 1).with_message("this is found to be of type `{int}`"),
            )
            .with_annotation(
                Annotation::new(56, 8).with_message("expected `{int}`, found `{string}`"),
            )
            .with_note("for more information about this error, try `ara --explain E0417`"),
    );

    let builder = ReportBuilder::new(source, report)
        .with_colors(ColorChoice::Always)
        .with_char_set(CharSet::Unicode);

    builder.print()
}
