[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
[![CI Status](https://github.com/jwodder/julian-rs/actions/workflows/test.yml/badge.svg)](https://github.com/jwodder/julian-rs/actions/workflows/test.yml)
[![codecov.io](https://codecov.io/gh/jwodder/julian-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/jwodder/julian-rs)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.81-orange)](https://www.rust-lang.org)
[![MIT License](https://img.shields.io/github/license/jwodder/julian-rs.svg)](https://opensource.org/licenses/MIT)

This is a Rust [workspace][] containing various packages for converting between
[Julian day numbers][] and dates in the [Gregorian calendar][] (either
proleptic or with the Reformation occurring at a given date) and/or the
proleptic [Julian calendar][].

The packages are:

- [`julian`][] — Rust library for converting between Julian day numbers and
  Julian & Gregorian calendars

- [`julian-cli`][] — CLI command for converting between Julian day numbers and
  Julian & Gregorian calendars

- [`gentests`][] — Internal package for use in generating test code for
  `julian`

[workspace]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[Julian day numbers]: https://en.wikipedia.org/wiki/Julian_day
[Gregorian calendar]: https://en.wikipedia.org/wiki/Gregorian_calendar
[Julian calendar]: https://en.wikipedia.org/wiki/Julian_calendar
[`julian`]: https://github.com/jwodder/julian-rs/tree/master/crates/julian
[`julian-cli`]: https://github.com/jwodder/julian-rs/tree/master/crates/julian-cli
[`gentests`]: https://github.com/jwodder/julian-rs/tree/master/crates/gentests
