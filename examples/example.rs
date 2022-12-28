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
    );

    let report = Report::new()
        .with_issue(
            Issue::error("E123", "some error here", 35, 7)
                .with_annotation(Annotation::new(39, 1).with_message("an annotation"))
                .with_help("this is a help")
                .with_note("this is a note"),
        )
        .with_issue(
            Issue::warning("W123", "some warning here", 29, 158)
                .with_annotation(Annotation::new(126, 1).with_message("an annotation"))
                .with_help("this is a help")
                .with_note("this is a note"),
        )
        .with_issue(
            Issue::note("N123", "some note here", 84, 80)
                .with_annotation(Annotation::new(105, 7).with_message("an annotation"))
                .with_annotation(Annotation::new(121, 7).with_message("another annotation"))
                .with_annotation(Annotation::new(137, 20).with_message("and another"))
                .with_help("this is a help")
                .with_note("this is a note"),
        )
        .with_issue(
            Issue::help("H123", "some help here", 137, 20)
                .with_annotation(Annotation::new(35, 7).with_message("an annotation"))
                .with_help("this is a help")
                .with_note("this is a note"),
        )
        .with_issue(
            Issue::bug("E123", "`match` arms have incompatible types", 84, 80)
                .with_annotation(
                    Annotation::new(110, 1).with_message("this is found to be of type `{int}`"),
                )
                .with_annotation(
                    Annotation::new(126, 1).with_message("this is found to be of type `{int}`"),
                )
                .with_annotation(
                    Annotation::new(148, 8).with_message("expected `{int}`, found `{string}`"),
                )
                .with_note("for more information about this error, try `ara --explain E0308`"),
        );

    let builder = ReportBuilder::new(source, report)
        .with_colors(ColorChoice::Never)
        .with_char_set(CharSet::Unicode);

    builder.print()
}
