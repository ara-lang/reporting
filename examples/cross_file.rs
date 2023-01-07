use ara_reporting::annotation::Annotation;
use ara_reporting::builder::Charset;
use ara_reporting::builder::ColorChoice;
use ara_reporting::builder::ReportBuilder;
use ara_reporting::error::Error;
use ara_reporting::issue::Issue;
use ara_reporting::Report;
use ara_source::source::Source;
use ara_source::source::SourceKind;
use ara_source::SourceMap;
fn main() -> Result<(), Error> {
    let map = SourceMap::new(vec![
        Source::new(
            SourceKind::Script,
            "src/main.ara",
            r#"
use function Math\add;

function main(): int {
    $value = add(1, '2');

    $value
}
"#,
        ),
        Source::new(
            SourceKind::Script,
            "vendor/some-vendor/some-lib/src/add.ara",
            r#"
namespace Math;

function add(int $a, int $b): int {
    return $a + $b;
}
"#,
        ),
    ]);

    let report = Report::new().with_issue(
        Issue::error(
            "E0417",
            "mismatched types expected `{int}`, found `{string}`",
            "src/main.ara",
            68,
            71,
        )
        .with_annotation(
            Annotation::secondary("src/main.ara", 61, 64)
                .with_message("arguments to this function are incorrect"),
        )
        .with_annotation(
            Annotation::secondary("vendor/some-vendor/some-lib/src/add.ara", 27, 51)
                .with_message("function defined here"),
        )
        .with_note(
            "you can cast a `{string}` to an `{int}` using `Psl\\Str\\to_int(...)` function",
        ),
    );

    let builder = ReportBuilder::new(&map, report)
        .with_colors(ColorChoice::Always)
        .with_charset(Charset::Unicode);

    builder.print()
}
