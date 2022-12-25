use std::io::Result;

use ara_reporting::annotation::Annotation;
use ara_reporting::builder::ReportBuilder;
use ara_reporting::issue::Issue;
use ara_reporting::source::Source;
use ara_reporting::Report;

fn main() -> Result<()> {
    let source = Source::inline(
        "
function main(): int|string {
    $a = 1;
    $b = 2;

    $c = $a + $b;

    (string) $c;
}

function fo⭕(): int|void {
    $e = 1;
}
",
    );

    let report = Report::new()
        .with_issue(
            Issue::error("E0413", "standalone type `void` cannot be used in a union", 117, 4)
                .with_annotation(
                    Annotation::new(116, 1)
                        .with_message("union is declared here")
                )
                .with_help("consider using `null` instead")
                .with_note("`void`, and `never` are bottom types that cannot be used in unions, intersections, or type parameters")
        )
       .with_issue(
           Issue::notice("T0413", "the inferred return type `string` for function `main` is more specific than the declared return type `int|string`", 10, 4)
               .with_annotation(
                   Annotation::new(18, 10)
                       .with_message("`int|string` is declared here")
               )
               .with_annotation(
                   Annotation::new(79, 11)
                       .with_message("`string` is inferred here")
               )
               .with_help("Consider changing the declared return type to `string`")
       )
       .with_issue(
           Issue::deprecation("D0013", "using PHP's casting is deprecated", 79, 8)
               .with_help("consider using the `as` keyword instead")
       )
       .with_issue(
           Issue::warning("W0003", "variable `$e` is never used", 128, 2)
               .with_help("consider removing the variable")
               .with_note("if this is intentional, consider renaming the variable to `$_e`")
       )
        ;

    let builder = ReportBuilder::new(source, report)
        .with_colors(true)
        .with_ascii(false);

    builder.print()
}
