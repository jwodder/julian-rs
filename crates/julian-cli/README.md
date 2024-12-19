[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
[![CI Status](https://github.com/jwodder/julian-rs/actions/workflows/test.yml/badge.svg)](https://github.com/jwodder/julian-rs/actions/workflows/test.yml)
[![codecov.io](https://codecov.io/gh/jwodder/julian-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/jwodder/julian-rs)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.81-orange)](https://www.rust-lang.org)
[![MIT License](https://img.shields.io/github/license/jwodder/julian-rs.svg)](https://opensource.org/licenses/MIT)

[GitHub](https://github.com/jwodder/julian-rs) | [crates.io](https://crates.io/crates/julian-cli) | [Issues](https://github.com/jwodder/julian-rs/issues) | [Changelog](https://github.com/jwodder/julian-rs/blob/master/crates/julian-cli/CHANGELOG.md)

The `julian-cli` package provides a `julian` command for converting between
[Julian day numbers][] and dates in the [Gregorian calendar][] (either
proleptic or with the Reformation occurring at a given date) and/or the
proleptic [Julian calendar][].  It is built on top of the
[`julian`](https://crates.io/crates/julian) library package by the same author.

[Julian day numbers]: https://en.wikipedia.org/wiki/Julian_day
[Gregorian calendar]: https://en.wikipedia.org/wiki/Gregorian_calendar
[Julian calendar]: https://en.wikipedia.org/wiki/Julian_calendar

Installation
============

In order to install the `julian` command, you first need to have [Rust and
Cargo installed](https://www.rust-lang.org/tools/install).  You can then build
the latest release of `julian-cli` and install it in `~/.cargo/bin` by running:

    cargo install julian-cli

Usage
=====

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

- `-c`, `--countries` — List the country codes recognized by the
  `-r`/`--reformation` option.  The output is a table with the following
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

- `-j`, `--julian` — Read & write dates using the proleptic Julian calendar

- `-J`, `--json` — Output JSON.  See [JSON Output](#json-output) below for more
  information.

- `-o`, `--ordinal` — Output calendar dates in the form "YYYY-JJJ", where the
  part after the hyphen is the day of the year from 001 to 366 (the ordinal
  date)

- `-q`, `--quiet` — Do not print the input value before each output value.  Do
  not print "JDN" before Julian day numbers.

- `-r <jdn>`, `--reformation <jdn>` — Read & write dates using a reforming
  calendar in which the Gregorian calendar is first observed on the date with
  the given Julian day number.

    A two-letter country code (case insensitive) may be given in place of a JDN
    in order to use the calendar reformation as it was observed in that
    country.  Run `julian --countries` to get a list of recognized country
    codes and their corresponding dates.

- `-s`, `--style` — Mark dates in reforming calendars as "O.S." (Old Style) or
  "N.S." (New Style)".  Has no effect when `-r`/`--reformation` is not given or
  when `-o`/`--ordinal` is given.

- `-V`, `--version` — Show the program version and exit

[yzero]: https://en.wikipedia.org/wiki/Astronomical_year_numbering
[NYD]: https://en.wikipedia.org/wiki/Julian_calendar#New_Year's_Day
[src]: https://salsa.debian.org/meskes/bsdmainutils/-/blob/70ff77b0f084de4a14d79bed935e1958020f43dc/usr.bin/ncal/ncal.c

JSON Output
-----------

When `julian` is invoked with the `-J`/`--json` option, it outputs a JSON
breakdown of the chosen calendar and input & output values.  The output
structure is an object with two keys, `"calendar"` and `"dates"`.

- `"calendar"` — Denotes the type of calendar selected for the `julian`
  invocation.  This is an object that always contains at least one key,
  `"type"`, the value of which is `"gregorian"` (for the default proleptic
  Gregorian calendar), `"julian"` (for the proleptic Julian calendar), or
  `"reforming"` (for a reforming calendar).  When `"type"` is `"reforming"`,
  there will be an additional field, `"reformation"`, whose value is the Julian
  day number of the date on which the calendar first follows the Gregorian
  calendar.

- `"dates"` — A list of objects, one per argument (or, if no arguments were
  given, one object for the current date).  Each object contains the following
  fields describing the date indicated by the argument, regardless of whether
  the argument was a calendar date or a Julian day number:

    - `"julian_day_number"` — the date's Julian day number
    - `"year"` — the date's year
    - `"month"` — the number (1-12) of the date's month
    - `"day"` — the date's day-of-month (1-31)
    - `"ordinal"` — the date's one-based day-of-year ordinal (1-366)
    - `"display"` — the date in "YYYY-MM-DD" form
    - `"ordinal_display"` — the date in "YYYY-JJJ" form
    - `"old_style"` — This field is only present if the calendar in use is a
      reforming calendar.  It is `true` if the date occurs before the calendar
      reformation, `false` otherwise.
