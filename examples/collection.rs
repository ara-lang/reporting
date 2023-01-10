use ara_reporting::annotation::Annotation;
use ara_reporting::builder::CharSet;
use ara_reporting::builder::ColorChoice;
use ara_reporting::builder::ReportBuilder;
use ara_reporting::error::Error;
use ara_reporting::issue::Issue;
use ara_reporting::Report;
use ara_reporting::ReportCollection;
use ara_reporting::ReportFooter;
use ara_source::source::Source;
use ara_source::source::SourceKind;
use ara_source::SourceMap;

fn main() -> Result<(), Error> {
    let first_origin = "example.ara";
    let first_code = r#"
$b = match $a {
    1 => 2,
    2 => 3,
    default => "string",
};
"#;

    let second_origin = "example-2.ara";
    let second_code = r#"
function foo(Bar&float) {}
"#;

    let map = SourceMap::new(vec![
        Source::new(SourceKind::Script, first_origin, first_code),
        Source::new(SourceKind::Script, second_origin, second_code),
    ]);

    let first_report = Report::new()
        .with_issue(
            Issue::error(
                "E0417",
                "`match` arms have incompatible types",
                first_origin,
                6,
                67,
            )
            .with_annotation(
                Annotation::secondary(first_origin, 26, 27)
                    .with_message("this is found to be of type `{int}`"),
            )
            .with_annotation(
                Annotation::secondary(first_origin, 38, 39)
                    .with_message("this is found to be of type `{int}`"),
            )
            .with_annotation(
                Annotation::secondary(first_origin, 56, 64)
                    .with_message("expected `{int}`, found `{string}`"),
            )
            .with_note("for more information about this error, try `ara --explain E0417`"),
        )
        .with_footer(
            ReportFooter::new("this is a report footer message")
                .with_note("this is a note message"),
        );

    let second_report = Report::new()
        .with_issue(
            Issue::error(
                "P0015",
                "scalar type `float` cannot be used in an intersection",
                second_origin,
                18,
                23,
            )
            .with_annotation(
                Annotation::secondary(second_origin, 17, 19)
                    .with_message("scalar type `float` cannot be used in an intersection"),
            )
            .with_note("a scalar type is either `int`, `float`, `string`, or `bool`.")
            .with_note("try using a different type for the intersection."),
        )
        .with_footer(
            ReportFooter::new("this is a report footer message")
                .with_note("this is a note message"),
        );

    let reports: ReportCollection = vec![&first_report, &second_report];

    let builder = ReportBuilder::new(&map)
        .with_colors(ColorChoice::Always)
        .with_charset(CharSet::Unicode);

    builder.print(&reports)
}
