use ara_reporting::annotation::Annotation;
use ara_reporting::builder::Charset;
use ara_reporting::builder::ColorChoice;
use ara_reporting::builder::ReportBuilder;
use ara_reporting::error::Error;
use ara_reporting::issue::Issue;
use ara_reporting::Report;
use ara_source::source::Source;
use ara_source::source::SourceKind;
use ara_source::source::DEFAULT_NAME;
use ara_source::SourceMap;

fn main() -> Result<(), Error> {
    let map = SourceMap::new(vec![Source::inline(
        SourceKind::Script,
        r#"
function main(): int|string {
    $a = 1;
    $b = 2;

    $c = $a + $b;

    $b = match ($a) {
        1 => 2,
        2 => 3,
        default => "string",
    };

    return $c + $b;
}
"#,
    )]);

    let report = Report::new()
        .with_issue(
            Issue::error("E123", "some error here", DEFAULT_NAME, 35, 7)
                .with_annotation(Annotation::new(DEFAULT_NAME, 39, 1).with_message("an annotation"))
                .with_help("this is a help")
                .with_note("this is a note"),
        )
        .with_issue(
            Issue::warning("W123", "some warning here", DEFAULT_NAME, 29, 158)
                .with_annotation(
                    Annotation::new(DEFAULT_NAME, 126, 1).with_message("an annotation"),
                )
                .with_help("this is a help")
                .with_note("this is a note"),
        )
        .with_issue(
            Issue::note("N123", "some note here", DEFAULT_NAME, 84, 80)
                .with_annotation(
                    Annotation::new(DEFAULT_NAME, 105, 7).with_message("an annotation"),
                )
                .with_annotation(
                    Annotation::new(DEFAULT_NAME, 121, 7).with_message("another annotation"),
                )
                .with_annotation(Annotation::new(DEFAULT_NAME, 137, 20).with_message("and another"))
                .with_help("this is a help")
                .with_note("this is a note"),
        )
        .with_issue(
            Issue::help("H123", "some help here", DEFAULT_NAME, 137, 20)
                .with_annotation(Annotation::new(DEFAULT_NAME, 35, 7).with_message("an annotation"))
                .with_help("this is a help")
                .with_note("this is a note"),
        )
        .with_issue(
            Issue::bug(
                "E123",
                "`match` arms have incompatible types",
                DEFAULT_NAME,
                84,
                80,
            )
            .with_annotation(
                Annotation::new(DEFAULT_NAME, 110, 1)
                    .with_message("this is found to be of type `{int}`"),
            )
            .with_annotation(
                Annotation::new(DEFAULT_NAME, 126, 1)
                    .with_message("this is found to be of type `{int}`"),
            )
            .with_annotation(
                Annotation::new(DEFAULT_NAME, 148, 8)
                    .with_message("expected `{int}`, found `{string}`"),
            )
            .with_note("for more information about this error, try `ara --explain E0308`"),
        );

    let builder = ReportBuilder::new(&map, report)
        .with_colors(ColorChoice::Always)
        .with_charset(Charset::Unicode);

    builder.print()
}
