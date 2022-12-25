# Ara Reporting

Ara reporting is an issue reporting library designed to be used within the Ara compiler.

Internally, Ara reporting uses the [ariadne](https://crates.io/crates/ariadne) library to build a report of the issues found in the code.

> **Note** If you are planning on adding more features to Ara reporting, please consider adding them to [ariadne](https://crates.io/crates/ariadne) instead.

> **Note** Unlike the [ariadne](https://crates.io/crates/ariadne) library, Ara reporting uses byte-based positions instead of character-based positions.
> This is because the Ara parser uses byte-based spans.

## Usage

Add `ara_reporting` to your `Cargo.toml`, and you're good to go!

```toml
[dependencies]
ara_reporting = "0.1.0"
```

## Example

see [examples](examples) directory.

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
* [All contributors](https://github.com/ryangjchandler/php-parser-rs/graphs/contributors)
