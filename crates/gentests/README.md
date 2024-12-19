[![Project Status: Concept – Minimal or no implementation has been done yet, or the repository is only intended to be a limited example, demo, or proof-of-concept.](https://www.repostatus.org/badges/latest/concept.svg)](https://www.repostatus.org/#concept)
[![CI Status](https://github.com/jwodder/julian-rs/actions/workflows/test.yml/badge.svg)](https://github.com/jwodder/julian-rs/actions/workflows/test.yml)
[![codecov.io](https://codecov.io/gh/jwodder/julian-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/jwodder/julian-rs)
[![MIT License](https://img.shields.io/github/license/jwodder/julian-rs.svg)](https://opensource.org/licenses/MIT)

This is an internal binary package for use in generating test case code for the
[`julian`][] package from the CSVs in [`testdata/`][].

Run `gentests` by invoking `cargo run -p gentests` in the root of the
workspace.  The output will be placed in
[`crates/julian/src/tests/autogen/`][autogen].

[`julian`]: https://github.com/jwodder/julian-rs/tree/master/crates/julian
[`testdata/`]: https://github.com/jwodder/julian-rs/tree/master/testdata/
[autogen]: https://github.com/jwodder/julian-rs/tree/master/crates/julian/src/tests/autogen/
