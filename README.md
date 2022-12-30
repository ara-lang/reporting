# Ara Reporting

[![Actions Status](https://github.com/ara-lang/reporting/workflows/ci/badge.svg)](https://github.com/ara-lang/reporting/actions)
[![Crates.io](https://img.shields.io/crates/v/ara_reporting.svg)](https://crates.io/crates/ara_reporting)
[![Docs](https://docs.rs/ara_reporting/badge.svg)](https://docs.rs/ara_reporting/latest/ara_reporting/)

A Reporting library for for Ara Programming Language 📃

Internally, Ara reporting uses the [codespan-reporting](https://github.com/brendanzab/codespan) library to build a report of the issues found in the code.

> **Note** If you are planning on adding more features to Ara reporting, please consider adding them to [codespan](https://github.com/brendanzab/codespan) instead if possible.

## Usage

Add `ara_reporting` to your `Cargo.toml`, and you're good to go!

```toml
[dependencies]
ara_reporting = "0.3.0"
```

## Example

```rust
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
    let origin = "example.ara";
    let code = r#"
$b = match $a {
    1 => 2,
    2 => 3,
    default => "string",
};
"#;

    let map = SourceMap::new(vec![Source::new(SourceKind::Script, origin, code)]);

    let report = Report::new().with_issue(
        Issue::error(
            "E0417",
            "`match` arms have incompatible types",
            origin,
            6,
            67,
        )
        .with_annotation(
            Annotation::new(origin, 26, 27).with_message("this is found to be of type `{int}`"),
        )
        .with_annotation(
            Annotation::new(origin, 38, 39).with_message("this is found to be of type `{int}`"),
        )
        .with_annotation(
            Annotation::new(origin, 56, 64).with_message("expected `{int}`, found `{string}`"),
        )
        .with_note("for more information about this error, try `ara --explain E0417`"),
    );

    let builder = ReportBuilder::new(&map, report)
        .with_colors(ColorChoice::Always)
        .with_charset(Charset::Unicode);

    builder.print()
}
```

see [examples](examples) directory for more examples.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

* [Saif Eddin Gmati](https://github.com/azjezz)
* [All contributors](https://github.com/ara-lang/reporting/graphs/contributors)
