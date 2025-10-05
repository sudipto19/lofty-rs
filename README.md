<img align="right" width="200" height="200" src="doc/lofty.svg" alt="Lofty logo">

# Lofty

*Parse, convert, and write metadata to various audio formats.*

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Serial-ATA/lofty-rs/ci.yml?branch=main&logo=github&style=for-the-badge)](https://github.com/Serial-ATA/lofty-rs/actions/workflows/ci.yml)
[![Downloads](https://img.shields.io/crates/d/lofty?style=for-the-badge&logo=rust)](https://crates.io/crates/lofty)
[![Version](https://img.shields.io/crates/v/lofty?style=for-the-badge&logo=rust)](https://crates.io/crates/lofty)
[![Documentation](https://img.shields.io/badge/docs.rs-lofty-informational?style=for-the-badge&logo=read-the-docs)](https://docs.rs/lofty/)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/Serial-ATA?style=for-the-badge&logo=githubsponsors)](https://github.com/sponsors/Serial-ATA)

⚠️ **LOOKING FOR HELP WITH DOCUMENTATION** ⚠️

I'm looking for help with the refinement of the docs. Any contribution, whether it be typos,
grammar, punctuation, or missing examples is highly appreciated!

## Supported Formats

[See here](./SUPPORTED_FORMATS.md)

## Examples

* [Tag reader](https://github.com/Serial-ATA/lofty-rs/blob/main/examples/tag_reader.rs)
* [Tag stripper](https://github.com/Serial-ATA/lofty-rs/blob/main/examples/tag_stripper.rs)
* [Tag writer](https://github.com/Serial-ATA/lofty-rs/blob/main/examples/tag_writer.rs)
* [Custom resolver](https://github.com/Serial-ATA/lofty-rs/tree/main/examples/custom_resolver)

To try them out, run:

```bash
cargo run --example tag_reader /path/to/file
cargo run --example tag_stripper /path/to/file
cargo run --example tag_writer <options> /path/to/file
cargo run --example custom_resolver
```

## Documentation

Available [here](https://docs.rs/lofty)

## Testing

As some formats are complex, Lofty makes use of [test-log](https://crates.io/crates/test-log) to get
the detailed debug/trace logging for failures. To run the tests, do:

```shell
RUST_LOG=trace cargo test
```

## Benchmarking

There are benchmarks available [here](./benches) with both Linux-specific and cross-platform options:

### Linux (iai-callgrind - high precision)
```shell
cargo bench --bench read_file --bench create_tag
```

### Cross-platform (Criterion - Windows/macOS/Linux)
```shell
cargo bench --bench read_file_criterion --bench create_tag_criterion
```

### All benchmarks
```shell
cargo bench
```

**Note**: The iai-callgrind benchmarks require Linux and Valgrind. The Criterion benchmarks work on all platforms and provide statistical analysis with HTML reports in `target/criterion/`.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
