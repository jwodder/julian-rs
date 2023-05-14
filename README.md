[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
[![CI Status](https://github.com/jwodder/julian-rs/actions/workflows/test.yml/badge.svg)](https://github.com/jwodder/julian-rs/actions/workflows/test.yml)
[![codecov.io](https://codecov.io/gh/jwodder/julian-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/jwodder/julian-rs)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.66-orange)](https://www.rust-lang.org)
[![MIT License](https://img.shields.io/github/license/jwodder/julian-rs.svg)](https://opensource.org/licenses/MIT)

[GitHub](https://github.com/jwodder/julian-rs) | [crates.io](https://crates.io/crates/julian) | [Documentation](https://docs.rs/julian) | [Issues](https://github.com/jwodder/julian-rs/issues) | [Changelog](https://github.com/jwodder/julian-rs/blob/master/CHANGELOG.md)

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

`julian` also provides a command of the same name for converting between Julian
day numbers and dates in Julian-style calendars.  To install this command on
your system, run:

    cargo install julian

Usage
-----

```text
julian [<options>] [<date> ...]
```

When invoked without arguments, the `julian` command displays the current date
in the proleptic Gregorian calendar & UTC timezone along with the corresponding
Julian day number.

When `julian` is invoked with one or more arguments, any calendar date
arguments (in the form "YYYY-MM-DD" or "YYYY-JJJ") are converted to Julian day
numbers, and any integer arguments are treated as Julian day numbers and
converted to calendar dates.  By default, dates are read & written using the
proleptic Gregorian calendar, but this can be changed with the `--julian` or
`--reformation` option.

`julian` uses [astronomical year numbering][yzero], where 1 BC (the year
immediately before AD 1) is denoted on input & output as year 0 (displayed as
"0000"), and the year before that (normally called 2 BC) is denoted -1
(displayed as "-0001").  In addition, the start of the year is always taken as
being on January 1, even though [not all users of the Julian calendar
throughout history have followed this convention][NYD].

Options
-------

- `-C`, `--countries` — List the country codes recognized by the
  `-R`/`--reformation` option.  The output is a table with the following
  columns:

    - "Code" — the two-letter country code accepted by `--reformation`
    - "Country" — the country's English name (or a common variation thereof)
    - "Reformation" — the Julian day number of the date on which the country
      first observed the Gregorian calendar
    - "Last Julian" — the Old Style calendar date of the day before the
      reformation
    - "First Gregorian" — the New Style calendar date of the day of the
      reformation

    The database of country reformations dates is drawn from [the Debian
    version of `ncal.c` as of 2023-04-26][src], so blame Debian for any
    historical inaccuracies.

- `-h`, `--help` — Display a summary of the command-line options and exit

- `-J`, `--julian` — Read & write dates using the proleptic Julian calendar

- `-o`, `--ordinal` — Output calendar dates in the form "YYYY-JJJ", where the
  part after the hyphen is the day of the year from 001 to 366 (the ordinal
  date)

- `-q`, `--quiet` — Do not print the input value before each output value.  Do
  not print "JDN" before Julian day numbers.

- `-R <jdn>`, `--reformation <jdn>` — Read & write dates using a reforming
  calendar in which the Gregorian calendar is first observed on the date with
  the given Julian day number

    A two-letter country code (case insensitive) may be given in place of a JDN
    in order to use the calendar reformation as it was observed in that
    country.  Run `julian --countries` to get a list of recognized country
    codes and their corresponding dates.

- `-s`, `--style` — Mark dates in reforming calendars as "O.S." (Old Style) or
  "N.S." (New Style)".  Has no effect when `-R`/`--reformation` is not given or
  when `-o`/`--ordinal` is given.

- `-V`, `--version` — Show the program version and exit

[yzero]: https://en.wikipedia.org/wiki/Astronomical_year_numbering
[NYD]: https://en.wikipedia.org/wiki/Julian_calendar#New_Year's_Day
[src]: https://salsa.debian.org/meskes/bsdmainutils/-/blob/70ff77b0f084de4a14d79bed935e1958020f43dc/usr.bin/ncal/ncal.c
