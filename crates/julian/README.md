[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
[![CI Status](https://github.com/jwodder/julian-rs/actions/workflows/test.yml/badge.svg)](https://github.com/jwodder/julian-rs/actions/workflows/test.yml)
[![codecov.io](https://codecov.io/gh/jwodder/julian-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/jwodder/julian-rs)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.67-orange)](https://www.rust-lang.org)
[![MIT License](https://img.shields.io/github/license/jwodder/julian-rs.svg)](https://opensource.org/licenses/MIT)

[GitHub](https://github.com/jwodder/julian-rs) | [crates.io](https://crates.io/crates/julian) | [Documentation](https://docs.rs/julian) | [Issues](https://github.com/jwodder/julian-rs/issues) | [Changelog](https://github.com/jwodder/julian-rs/blob/master/crates/julian/CHANGELOG.md)

`julian` is a Rust library for converting between [Julian day numbers][] and
dates in the [Gregorian calendar][] (either proleptic or with the Reformation
occurring at a given date) and/or the proleptic [Julian calendar][].  There are
also features for querying details about years & months in a "reforming"
Gregorian calendar and how they are affected by the calendar reformation date
of your choice.

[Julian day numbers]: https://en.wikipedia.org/wiki/Julian_day
[Gregorian calendar]: https://en.wikipedia.org/wiki/Gregorian_calendar
[Julian calendar]: https://en.wikipedia.org/wiki/Julian_calendar

Examples
========

Before you construct a date, you must first choose a calendar in which to
reckon dates.  `Calendar::GREGORIAN` is the proleptic Gregorian calendar, which
should be both simple and useful enough for most basic purposes.

To convert a Julian day number to a date in a calendar, use the
`Calendar::at_jdn()` method, like so:

```rust
use julian::{Calendar, Month};

let cal = Calendar::GREGORIAN;
let date = cal.at_jdn(2460065);
assert_eq!(date.year(), 2023);
assert_eq!(date.month(), Month::April);
assert_eq!(date.day(), 30);
```

So JDN 2460065 is April 30, 2023, in the proleptic Gregorian calendar.

To convert a date to a Julian day number, use `Calendar::at_ymd()` to construct
the date, and then call its `julian_day_number()` method:

```rust
use julian::{Calendar, Month};

let cal = Calendar::GREGORIAN;
let date = cal.at_ymd(2023, Month::April, 30).unwrap();
assert_eq!(date.julian_day_number(), 2460065);
```

See [the documentation](https://docs.rs/julian) for more things you can do!

Command
=======

There is also an accompanying binary package
[`julian-cli`](https://crates.io/crates/julian-cli) that provides a CLI command
named `julian` for converting between Julian day numbers and dates in
Julian-style calendars.  Feel free to install it if you're interested!
