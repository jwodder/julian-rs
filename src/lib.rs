#![cfg_attr(docsrs, feature(doc_cfg))]
//! `julian` is a Rust library for converting between [Julian day numbers][jdn]
//! and dates in the [Gregorian calendar][] (either proleptic or with the
//! Reformation occurring at a given date) and/or the proleptic [Julian
//! calendar][].  There are also features for querying details about years &
//! months in a "reforming" Gregorian calendar and how they are affected by the
//! calendar reformation date of your choice.
//!
//! [Gregorian calendar]: https://en.wikipedia.org/wiki/Gregorian_calendar
//! [Julian calendar]: https://en.wikipedia.org/wiki/Julian_calendar
//!
//! Features
//! ========
//!
//! The `julian` crate has the following optional features:
//!
//! - `cli` — Adds dependencies needed by the `julian` binary.  This is enabled
//!   by default, but if you are using `julian` as a library, it is recommended
//!   that you remove it by disabling default features.
//!
//! - `chrono` — Enables converting values of certain `julian` types to the
//!   corresponding [`chrono`] types and *vice versa*.
//!
//! - `gentests` — Used internally to install dependencies needed for a utility
//!   binary.  Do not enable.
//!
//! Examples
//! ========
//!
//! Before you construct a date, you must first choose a [calendar][Calendar]
//! in which to reckon [dates][Date].  [`Calendar::GREGORIAN`] is the proleptic
//! Gregorian calendar, which should be both simple and useful enough for most
//! basic purposes.
//!
//! To convert a Julian day number to a date in a calendar, use the
//! [`Calendar::at_jdn()`] method, like so:
//!
//! ```
//! use julian::{Calendar, Month};
//!
//! let cal = Calendar::GREGORIAN;
//! let date = cal.at_jdn(2460065);
//! assert_eq!(date.year(), 2023);
//! assert_eq!(date.month(), Month::April);
//! assert_eq!(date.day(), 30);
//! ```
//!
//! So JDN 2460065 is April 30, 2023, in the proleptic Gregorian calendar.
//!
//! To convert a date to a Julian day number, use [`Calendar::at_ymd()`] to
//! construct the date, and then call its
//! [`julian_day_number()`][Date::julian_day_number] method:
//!
//! ```
//! use julian::{Calendar, Month};
//!
//! let cal = Calendar::GREGORIAN;
//! let date = cal.at_ymd(2023, Month::April, 30).unwrap();
//! assert_eq!(date.julian_day_number(), 2460065);
//! ```
//!
//! See the documentation below for more things you can do!
//!
//! Terminology and Conventions Used by This Crate
//! ==============================================
//!
//! Julian Day Numbers
//! ------------------
//!
//! A [Julian day number][jdn] is an integer assigned to each day, with day 0
//! being January 1, 4713 BC, in the proleptic Julian calendar (or November 24,
//! 4714 BC, in the proleptic Gregorian calendar), ticking over at midnight
//! UTC.  For example, the Julian day number for the day I'm writing this,
//! April 30, 2023, in the Gregorian calendar is 2460065.
//!
//! Years
//! -----
//!
//! `julian` uses [astronomical year numbering][yzero], where 1 BC (the year
//! immediately before AD 1) is denoted on input & output as year 0 (or "0000"
//! when displaying a date), and the year before that (normally called 2 BC) is
//! denoted -1 (displayed as "-0001").  Thus, 4713 BC is represented by this
//! crate as -4712.
//!
//! In addition, the start of the year is always taken as being on January 1,
//! even though [not all users of the Julian calendar throughout history have
//! followed this convention][NYD].
//!
//! Calendars
//! ---------
//!
//! `julian` works with three types of calendars.  The first is the [proleptic
//! Julian calendar][julian], in which a leap day is inserted at the end of
//! February in every fourth year.  "Proleptic" means that this system is
//! extended backwards before its historical introduction, including pretending
//! that [early inaccuracies in applying the leap year rule][leap-error] never
//! happened.  If you wish to determine the actual Julian day number of an
//! event recorded before AD 8, you may need to consult external sources.
//!
//! However, the Julian calendar's rule for inserting leap years proved
//! insufficiently accurate to the solar year, causing the dates of the
//! solstices and equinoctes to drift earlier in the calendar over the
//! centuries.  This was a problem for the Catholic Church, which set the date
//! of Easter based on the first full moon after March 21, which was expected
//! to be the date of the vernal equinox by the First Council of Nicaea in AD
//! 325.  Eventually, Pope Gregory XIII proclaimed a reformation to the
//! calendar to address this problem: Leap years would thereafter only be
//! observed in years divisible by four, excluding years divisible by 100 that
//! were not also divisible by 400.  Moreover, in order to align the calendar
//! so that the vernal equinox once again fell on March 21, ten days were to be
//! skipped in October 1582, with the day after October 4 being October 15.
//!
//! This gives us the second calendar that `julian` works with: the [proleptic
//! Gregorian calendar][gregorian], which uses the leap year rule given above
//! and which is only aligned with the Julian calendar from March 1, AD 200,
//! through February 28, AD 300.
//!
//! For political and other reasons, not every jurisdiction immediately adopted
//! the Gregorian calendar.  For example, Great Britain and its colonies did
//! not switch until 1752, when September 2 was followed by September 14, and
//! Russia (arguably the last major country to switch) did not reform its
//! calendar until 1918.  During these reforms, when one could be expected to
//! communicate with people using a calendar slightly off from one's own, it
//! was conventional to give the date in both the Julian and Gregorian
//! calendars, with the Julian date marked as "Old Style" or "O.S." and the
//! Gregorian date marked as "New Style" or "N.S."  This documentation follows
//! the same convention for most dates where appropriate.
//!
//! Because of these changes in calendar, there is a third calendar — or
//! rather, a class of infinitely many calendars[^infinite] — that `julian`
//! must deal with: a calendar that follows the Julian rule for leap days for
//! some time before undergoing a calendar reformation, switching to the
//! Gregorian rule and skipping some number of dates in order to align with the
//! proleptic Gregorian calendar.  `julian` calls these "reforming" calendars,
//! and each one is defined by the date (as a Julian day number) on which the
//! Gregorian calendar was first used.
//!
//! Days and Day Ordinals
//! ---------------------
//!
//! Because "reforming" calendars (described above) necessarily skip some dates
//! at some point, it is not always the case that day *n* of a month is the
//! *n*-th date of that month.  For example, for the initial reformation in
//! 1582, October 4 (the fourth day of the month) was followed by October 15
//! (the fifth day of the month).  `julian` calls the actual day numbers that
//! are written in dates "days" (queried with [`Date::day()`]), and a given
//! day's one-based ordinal index in its month is called the "day ordinal"
//! (queried with [`Date::day_ordinal()`]).
//!
//! [jdn]: https://en.wikipedia.org/wiki/Julian_day
//! [yzero]: https://en.wikipedia.org/wiki/Astronomical_year_numbering
//! [NYD]: https://en.wikipedia.org/wiki/Julian_calendar#New_Year's_Day
//! [julian]: https://en.wikipedia.org/wiki/Proleptic_Julian_calendar
//! [leap-error]: https://en.wikipedia.org/wiki/Julian_calendar#Leap_year_error
//! [gregorian]: https://en.wikipedia.org/wiki/Proleptic_Gregorian_calendar
//!
//! [^infinite]: Technically, as far as this library (limited by finite integer
//!     arithmetic) is concerned, 2,145,608,897 calendars.
//!
//! Valid Input Value Ranges
//! ========================
//!
//! Care has been taken to support converting to & from all Julian day numbers
//! that fit in an [`i32`].  Other forms of input — i.e., dates and Unix
//! timestamps — can only be supported so long as their corresponding Julian
//! day numbers fit in this type.  Thus, the ranges of accepted input values
//! are:
//!
//! |                         | Minimum          | Maximum         |
//! | ----------------------- | ---------------- | --------------- |
//! | Julian day number       | -2147483648      | 2147483647      |
//! | Julian calendar date    | -5884202-03-16   | 5874777-10-17   |
//! | Gregorian calendar date | -5884323-05-15   | 5874898-06-03   |
//! | Unix timestamp          | -185753453990400 | 185331720383999 |

#[cfg(test)]
extern crate rstest_reuse;

pub mod errors;
mod inner;
pub mod iter;
pub mod ncal;
use crate::errors::*;
use crate::iter::*;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

/// Type used for Julian day numbers in this crate
pub type Jdnum = i32;

/// The Julian day number of the date at which the Gregorian Reformation was
/// first put into effect (1582-10-15, following 1582-10-04 O.S.)
pub const REFORM1582_JDN: Jdnum = 2299161;

/// The Julian day number of the start of the Unix epoch (1970-01-01)
pub const UNIX_EPOCH_JDN: Jdnum = 2440588;

/// The Julian day number of the zero [Rata Die][] day number, i.e., 0000-12-31
/// in the proleptic Gregorian calendar.
///
/// The Rata Die day number for a given date can be determined by subtracting
/// this constant from the date's Julian day number.
///
/// [Rata Die]: https://en.wikipedia.org/wiki/Rata_Die
pub const RATA_DIE_ZERO_JDN: Jdnum = 1721425;

const SECONDS_IN_DAY: i64 = 24 * 60 * 60;

const COMMON_YEAR_LENGTH: Jdnum = 365;
const LEAP_YEAR_LENGTH: Jdnum = 366;

/// A classification of calendar years.
///
/// A year can be common or leap, and a year in a "reforming" calendar can be
/// shortened or skipped entirely.
///
/// A `YearKind` can be obtained by calling [`Calendar::year_kind()`].
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum YearKind {
    /// A full-length year not containing February 29
    Common,

    /// A full-length year containing February 29
    Leap,

    /// A year that was shortened (either at the beginning or the end) or has a
    /// gap in it due to a calendar reformation and which (after taking the
    /// reformation into account) does not contain February 29.
    ///
    /// Note that, if a leap day (either Julian or Gregorian) was skipped due
    /// to a reformation, the year will be counted as `ReformCommon` rather
    /// than `ReformLeap`.
    ReformCommon,

    /// A year that was shortened (either at the beginning or the end) or has a
    /// gap in it due to a calendar reformation and which (after taking the
    /// reformation into account) contains February 29.
    ReformLeap,

    /// A year that was skipped entirely by a calendar reformation.
    ///
    /// This can only happen for reformations of at least JDN 19582149
    /// (48902-01-01 in the Gregorian calendar).
    Skipped,
}

impl YearKind {
    /// Returns true if the year kind is `Common` or `ReformCommon`
    pub const fn is_common(&self) -> bool {
        use YearKind::*;
        matches!(self, Common | ReformCommon)
    }

    /// Returns true if the year kind is `Leap` or `ReformLeap`
    pub const fn is_leap(&self) -> bool {
        use YearKind::*;
        matches!(self, Leap | ReformLeap)
    }

    /// Returns true if the year kind is `ReformCommon` or `ReformLeap`
    pub const fn is_reform(&self) -> bool {
        use YearKind::*;
        matches!(self, ReformCommon | ReformLeap)
    }

    /// Returns true if the year kind is `Skipped`
    pub const fn is_skipped(&self) -> bool {
        matches!(self, YearKind::Skipped)
    }
}

/// A "Julian-style" calendar, featuring twelve months and occasionally a leap
/// day at the end of February.
///
/// A calendar may be a proleptic Julian calendar (in which leap years happen
/// exactly every four years), a proleptic Gregorian calendar (in which leap
/// years happen every four years, excepting centennial years not divisible by
/// 400), or a "reforming" calendar that starts out as Julian and changes to
/// Gregorian at some date, with the reformation involving skipping a number of
/// calendar days in order to align with the proleptic Gregorian calendar.
///
/// The `Ord` implementation is such that the proleptic Julian calendar is
/// smaller than all other calendars; it is followed by "reforming" calendars
/// in ascending order of reformation date, and then the proleptic Gregorian
/// calendar is larger than all other calendars.
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Calendar(inner::Calendar);

impl Calendar {
    /// A proleptic Julian calendar
    pub const JULIAN: Calendar = Calendar(inner::Calendar::Julian);

    /// A proleptic Gregorian calendar
    pub const GREGORIAN: Calendar = Calendar(inner::Calendar::Gregorian);

    /// An instance of a reforming calendar with the reformation set at the
    /// date in history at which the Gregorian Reformation was first observed
    /// (i.e., 1582-10-15, following 1582-10-04 O.S.).
    ///
    /// This calendar is equal to
    /// `Calendar::reforming(julian::REFORM1582_JDN).unwrap()`.
    pub const REFORM1582: Calendar = Calendar(inner::Calendar::Reforming {
        reformation: 2299161,
        gap: inner::ReformGap {
            pre_reform: inner::Date {
                year: 1582,
                ordinal: 277,
                month: Month::October,
                day: 4,
            },
            post_reform: inner::Date {
                year: 1582,
                ordinal: 278,
                month: Month::October,
                day: 15,
            },
            kind: inner::GapKind::IntraMonth,
            ordinal_gap_start: 287,
            ordinal_gap: 10,
        },
    });

    /// Construct an instance of a reforming calendar.  `reformation` is the
    /// Julian day number of the first day on which the Gregorian calendar is
    /// used.
    ///
    /// A selection of per-country (not always historically accurate)
    /// reformation constants is available in the [`ncal`] module.
    ///
    /// # Errors
    ///
    /// Returns [`ReformingError::InvalidReformation`] if observing a
    /// reformation at the given date would not cause the calendar to skip
    /// forwards.  This can only happen for Julian day numbers less than
    /// 1830692 (corresponding to the date 0300-03-01 N.S. or 0300-02-29 O.S.).
    ///
    /// Returns [`ReformingError::Arithmetic`] if numeric overflow/underflow
    /// occurs while converting `reformation` to a calendar date.  This can
    /// only happen for Julian day numbers greater than 2147439588
    /// (corresponding to the date 5874777-10-17 N.S. or 5874657-03-02 O.S.).
    pub fn reforming(reformation: Jdnum) -> Result<Calendar, ReformingError> {
        let pre_reform = Calendar::JULIAN.at_jdn(
            reformation
                .checked_sub(1)
                .ok_or(ReformingError::InvalidReformation)?,
        );
        let post_reform = Calendar::GREGORIAN.at_jdn(reformation);
        let mut ordinal = post_reform.ordinal();
        if post_reform.year % 100 == 0
            && post_reform.year % 400 != 0
            && post_reform.month > Month::February
        {
            ordinal += 1;
        }
        if Calendar::JULIAN.get_jdn(post_reform.year(), ordinal)? <= reformation {
            return Err(ReformingError::InvalidReformation);
        }
        let kind = inner::GapKind::for_dates(
            pre_reform.year,
            pre_reform.month,
            post_reform.year,
            post_reform.month,
        );
        let pre_reform = inner::Date {
            year: pre_reform.year,
            ordinal: pre_reform.ordinal,
            month: pre_reform.month,
            day: pre_reform.day,
        };
        let (post_ordinal, ordinal_gap_start, ordinal_gap) = match kind {
            inner::GapKind::IntraMonth | inner::GapKind::CrossMonth => (
                pre_reform.ordinal + 1,
                post_reform.ordinal - 1,
                post_reform.ordinal - pre_reform.ordinal - 1,
            ),
            _ => (1, 0, post_reform.ordinal - 1),
        };
        let post_reform = inner::Date {
            year: post_reform.year,
            ordinal: post_ordinal,
            month: post_reform.month,
            day: post_reform.day,
        };
        Ok(Calendar(inner::Calendar::Reforming {
            reformation,
            gap: inner::ReformGap {
                pre_reform,
                post_reform,
                kind,
                ordinal_gap_start,
                ordinal_gap,
            },
        }))
    }

    /// Returns the current date according to the calendar, along with a count
    /// of seconds since midnight UTC.
    ///
    /// # Errors
    ///
    /// Returns [`ArithmeticError`] if numeric overflow/underflow occurs while
    /// converting the time.  This can only happen if the system time in UTC is
    /// before -5884323-05-15 (-5884202-03-16 O.S.) or after 5874898-06-03
    /// (5874777-10-17 O.S.).
    pub fn now(&self) -> Result<(Date, u32), ArithmeticError> {
        self.at_system_time(SystemTime::now())
    }

    /// Returns the date according to the calendar for the given system time,
    /// along with a count of seconds since midnight UTC.
    ///
    /// # Errors
    ///
    /// Returns [`ArithmeticError`] if numeric overflow/underflow occurs while
    /// converting the time.  This can only happen if the system time in UTC is
    /// before -5884323-05-15 (-5884202-03-16 O.S.) or after 5874898-06-03
    /// (5874777-10-17 O.S.).
    pub fn at_system_time(&self, t: SystemTime) -> Result<(Date, u32), ArithmeticError> {
        let (jdn, secs) = system2jdn(t)?;
        Ok((self.at_jdn(jdn), secs))
    }

    /// Returns the date according to the calendar for the given [Unix time][],
    /// along with a count of seconds since midnight UTC.
    ///
    /// [Unix time]: https://en.wikipedia.org/wiki/Unix_time
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::GREGORIAN;
    /// let (date, seconds) = cal.at_unix_time(1682906621).unwrap();
    /// assert_eq!(date.year(), 2023);
    /// assert_eq!(date.month(), Month::May);
    /// assert_eq!(date.day(), 1);
    /// assert_eq!(seconds, 7421);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ArithmeticError`] if numeric overflow/underflow occurs while
    /// converting the time.  This can only happen if the timestamp is less
    /// than -185753453990400 or greater than 185331720383999.
    pub fn at_unix_time(&self, unix_time: i64) -> Result<(Date, u32), ArithmeticError> {
        let (jdn, secs) = unix2jdn(unix_time)?;
        Ok((self.at_jdn(jdn), secs))
    }

    /// Returns the date of the calendar with the given year, month, and day of
    /// month.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::GREGORIAN;
    /// let date = cal.at_ymd(2023, Month::April, 30).unwrap();
    /// assert_eq!(date.to_string(), "2023-04-30");
    ///
    /// assert!(cal.at_ymd(2023, Month::April, 31).is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`DateError::DayOutOfRange`] if `day` is zero or greater than
    /// the last day of the given month for the given year.
    ///
    /// Returns [`DateError::SkippedDate`] if the given date was skipped by a
    /// calendar reformation.  This error takes precedence over
    /// `DayOutOfRange`, i.e., if one or more days at the end of given month
    /// were skipped due to a calendar reformation, and the given day falls
    /// into this range, `SkippedDate` will be returned instead of
    /// `DayOutOfRange`.  However, any `DayOutOfRange` errors returned for the
    /// same month will show the last non-skipped day as the maximum valid day
    /// of the month.
    ///
    /// Returns [`DateError::Arithmetic`] if numeric overflow/underflow occurs
    /// while calculating the date's Julian day number.  This can only happen
    /// for dates before -5884323-05-15 (-5884202-03-16 O.S.) or after
    /// 5874898-06-03 (5874777-10-17 O.S.).
    pub fn at_ymd(&self, year: i32, month: Month, day: u32) -> Result<Date, DateError> {
        let day_ordinal = self.get_day_ordinal(year, month, day)?;
        let ordinal = self.ymdo2ordinal(year, month, day_ordinal);
        let jdn = self.get_jdn(year, ordinal)?;
        Ok(Date {
            calendar: *self,
            year,
            ordinal,
            month,
            day,
            day_ordinal,
            jdn,
        })
    }

    /// Returns the date of the calendar with the given year and day-of-year
    /// (starting counting from 1 at January 1).
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::GREGORIAN;
    /// let date = cal.at_ordinal_date(2023, 120).unwrap();
    /// assert_eq!(date.year(), 2023);
    /// assert_eq!(date.month(), Month::April);
    /// assert_eq!(date.day(), 30);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`DateError::OrdinalOutOfRange`] if `ordinal` is zero or
    /// greater than the length of the year.
    ///
    /// Returns [`DateError::Arithmetic`] if numeric overflow/underflow occurs
    /// while calculating the date's Julian day number.  This can only happen
    /// for dates before -5884323-135 (-5884202-075 O.S.) or after 5874898-154
    /// (5874777-290 O.S.).
    pub fn at_ordinal_date(&self, year: i32, ordinal: u32) -> Result<Date, DateError> {
        let (month, day, day_ordinal) = self.ordinal2ymddo(year, ordinal)?;
        let jdn = self.get_jdn(year, ordinal)?;
        Ok(Date {
            calendar: *self,
            year,
            ordinal,
            month,
            day,
            day_ordinal,
            jdn,
        })
    }

    /// Returns the date with the given Julian day number under the calendar.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::GREGORIAN;
    /// let date = cal.at_jdn(2460065);
    /// assert_eq!(date.year(), 2023);
    /// assert_eq!(date.month(), Month::April);
    /// assert_eq!(date.day(), 30);
    /// ```
    pub fn at_jdn(&self, jdn: Jdnum) -> Date {
        use inner::Calendar::*;
        let (year, mut ordinal) = if self.0 == Julian
            || matches!(self.0, Reforming { reformation, .. } if jdn < reformation)
        {
            inner::jdn2julian(jdn)
        } else {
            inner::jdn2gregorian(jdn)
        };
        if let Some(gap) = self.gap() {
            if year == gap.post_reform.year && ordinal > gap.ordinal_gap_start {
                ordinal -= gap.ordinal_gap;
            }
        }
        let Ok((month, day, day_ordinal)) = self.ordinal2ymddo(year, ordinal) else {
            unreachable!("ordinal should be within range for year");
        };
        Date {
            calendar: *self,
            year,
            ordinal,
            month,
            day,
            day_ordinal,
            jdn,
        }
    }

    /// Parse a calendar date from a string.
    ///
    /// The following string formats are accepted:
    ///
    /// - `YYYY-MM-DD` — year, month number, and day
    /// - `YYYY-JJJ` — year and day-of-year
    ///
    /// In both forms, the year component may be preceded by a `+` or `-`, and
    /// each component may be any number of digits long, not just the
    /// "conventional" length shown here.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::GREGORIAN;
    /// let date = cal.parse_date("2023-04-30").unwrap();
    /// assert_eq!(date.year(), 2023);
    /// assert_eq!(date.month(), Month::April);
    /// assert_eq!(date.day(), 30);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ParseDateError`] if the string or the date it represents is
    /// invalid
    pub fn parse_date(&self, s: &str) -> Result<Date, ParseDateError> {
        let mut parser = inner::DateParser::new(s);
        let year = parser.parse_int()?;
        parser.scan_char('-')?;
        let diny = parser.parse_day_in_year()?;
        if !parser.is_empty() {
            return Err(ParseDateError::Trailing);
        }
        match diny {
            inner::DayInYear::Ordinal(ordinal) => Ok(self.at_ordinal_date(year, ordinal)?),
            inner::DayInYear::Date { month, day } => Ok(self.at_ymd(year, month, day)?),
        }
    }

    /// Returns true if this is a proleptic Julian or Gregorian calendar, i.e.,
    /// not a "reforming" calendar
    ///
    /// # Example
    ///
    /// ```
    /// use julian::Calendar;
    ///
    /// assert!(Calendar::JULIAN.is_proleptic());
    /// assert!(Calendar::GREGORIAN.is_proleptic());
    /// assert!(!Calendar::REFORM1582.is_proleptic());
    /// ```
    pub const fn is_proleptic(&self) -> bool {
        matches!(self.0, inner::Calendar::Julian | inner::Calendar::Gregorian)
    }

    /// Returns true if this is a "reforming" calendar
    ///
    /// # Example
    ///
    /// ```
    /// use julian::Calendar;
    ///
    /// assert!(!Calendar::JULIAN.is_reforming());
    /// assert!(!Calendar::GREGORIAN.is_reforming());
    /// assert!(Calendar::REFORM1582.is_reforming());
    /// ```
    pub const fn is_reforming(&self) -> bool {
        matches!(self.0, inner::Calendar::Reforming { .. })
    }

    /// If this is a "reforming" calendar, returns the Julian day number of the
    /// reformation (the first day on which the Gregorian calendar is used)
    pub const fn reformation(&self) -> Option<Jdnum> {
        if let inner::Calendar::Reforming { reformation, .. } = self.0 {
            Some(reformation)
        } else {
            None
        }
    }

    /// If this is a "reforming" calendar, returns the last date that follows
    /// the Julian calendar, i.e., the date immediately before the reformation.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::REFORM1582;
    /// let date = cal.last_julian_date().unwrap();
    /// assert_eq!(date.year(), 1582);
    /// assert_eq!(date.month(), Month::October);
    /// assert_eq!(date.day(), 4);
    /// ```
    pub const fn last_julian_date(&self) -> Option<Date> {
        if let inner::Calendar::Reforming { reformation, gap } = self.0 {
            Some(Date {
                calendar: *self,
                year: gap.pre_reform.year,
                ordinal: gap.pre_reform.ordinal,
                month: gap.pre_reform.month,
                day: gap.pre_reform.day,
                day_ordinal: gap.pre_reform.day,
                jdn: reformation - 1,
            })
        } else {
            None
        }
    }

    /// If this is a "reforming" calendar, returns the first date that follows
    /// the Gregorian calendar, i.e., the date of the reformation.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::REFORM1582;
    /// let date = cal.first_gregorian_date().unwrap();
    /// assert_eq!(date.year(), 1582);
    /// assert_eq!(date.month(), Month::October);
    /// assert_eq!(date.day(), 15);
    /// ```
    pub fn first_gregorian_date(&self) -> Option<Date> {
        if let inner::Calendar::Reforming { reformation, gap } = self.0 {
            let day_ordinal = if gap.kind == inner::GapKind::IntraMonth {
                gap.pre_reform.day + 1
            } else {
                1
            };
            Some(Date {
                calendar: *self,
                year: gap.post_reform.year,
                ordinal: gap.post_reform.ordinal,
                month: gap.post_reform.month,
                day: gap.post_reform.day,
                day_ordinal,
                jdn: reformation,
            })
        } else {
            None
        }
    }

    /// Returns the [`YearKind`] for the given year in the calendar
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, YearKind};
    ///
    /// let cal = Calendar::REFORM1582;
    /// assert_eq!(cal.year_kind(1582), YearKind::ReformCommon);
    /// assert_eq!(cal.year_kind(1900), YearKind::Common);
    /// assert_eq!(cal.year_kind(2000), YearKind::Leap);
    ///
    /// let cal2 = Calendar::reforming(19582149).unwrap();
    /// assert_eq!(cal2.year_kind(48900), YearKind::Leap);
    /// assert_eq!(cal2.year_kind(48901), YearKind::Skipped);
    /// assert_eq!(cal2.year_kind(48902), YearKind::Common);
    /// ```
    pub const fn year_kind(&self, year: i32) -> YearKind {
        match self.0 {
            inner::Calendar::Julian => {
                if inner::is_julian_leap_year(year) {
                    YearKind::Leap
                } else {
                    YearKind::Common
                }
            }
            inner::Calendar::Gregorian => {
                if inner::is_gregorian_leap_year(year) {
                    YearKind::Leap
                } else {
                    YearKind::Common
                }
            }
            inner::Calendar::Reforming { gap, .. } => {
                use inner::RangeOrdering::*;
                match gap.cmp_year(year) {
                    Less => {
                        if inner::is_julian_leap_year(year) {
                            YearKind::Leap
                        } else {
                            YearKind::Common
                        }
                    }
                    EqLower => {
                        if matches!(
                            (gap.pre_reform.month, gap.pre_reform.day),
                            (Month::December, 31)
                        ) {
                            if inner::is_julian_leap_year(year) {
                                YearKind::Leap
                            } else {
                                YearKind::Common
                            }
                        } else if Month::February.lt(gap.pre_reform.month)
                            && inner::is_julian_leap_year(year)
                        {
                            YearKind::ReformLeap
                        } else {
                            YearKind::ReformCommon
                        }
                    }
                    Between => YearKind::Skipped,
                    EqBoth => {
                        if (Month::February.lt(gap.pre_reform.month)
                            && inner::is_julian_leap_year(year))
                            || (gap.post_reform.month.le(Month::February)
                                && inner::is_gregorian_leap_year(year))
                        {
                            YearKind::ReformLeap
                        } else {
                            YearKind::ReformCommon
                        }
                    }
                    EqUpper => {
                        if matches!(
                            (gap.post_reform.month, gap.post_reform.day),
                            (Month::January, 1)
                        ) {
                            if inner::is_gregorian_leap_year(year) {
                                YearKind::Leap
                            } else {
                                YearKind::Common
                            }
                        } else if gap.post_reform.month.le(Month::February)
                            && inner::is_gregorian_leap_year(year)
                        {
                            YearKind::ReformLeap
                        } else {
                            YearKind::ReformCommon
                        }
                    }
                    Greater => {
                        if inner::is_gregorian_leap_year(year) {
                            YearKind::Leap
                        } else {
                            YearKind::Common
                        }
                    }
                }
            }
        }
    }

    /// Returns the number of days in the given year in the calendar.
    ///
    /// If the year was skipped in its entirety due to a calendar reformation,
    /// this method will return 0.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, YearKind};
    ///
    /// let cal = Calendar::REFORM1582;
    /// assert_eq!(cal.year_length(1582), 355);
    /// assert_eq!(cal.year_length(1900), 365);
    /// assert_eq!(cal.year_length(2000), 366);
    ///
    /// let cal2 = Calendar::reforming(19582149).unwrap();
    /// assert_eq!(cal2.year_length(48900), 366);
    /// assert_eq!(cal2.year_length(48901), 0);
    /// assert_eq!(cal2.year_length(48902), 365);
    /// ```
    pub const fn year_length(&self, year: i32) -> u32 {
        match self.0 {
            inner::Calendar::Julian | inner::Calendar::Gregorian => match self.year_kind(year) {
                YearKind::Common => COMMON_YEAR_LENGTH as u32,
                YearKind::Leap => LEAP_YEAR_LENGTH as u32,
                _ => unreachable!(),
            },
            inner::Calendar::Reforming { gap, .. } => match self.year_kind(year) {
                YearKind::Common => COMMON_YEAR_LENGTH as u32,
                YearKind::Leap => LEAP_YEAR_LENGTH as u32,
                k @ (YearKind::ReformCommon | YearKind::ReformLeap) => {
                    let length = if matches!(k, YearKind::ReformCommon) {
                        COMMON_YEAR_LENGTH as u32
                    } else {
                        LEAP_YEAR_LENGTH as u32
                    };
                    if year == gap.post_reform.year {
                        // If this is a Julian-only leap year and the year kind
                        // is ReformLeap, then the year contains a Julian-only
                        // leap day and we need to add 1 to `ordinal_gap`
                        // because the `pre_reform.ordinal` subtrahend that
                        // produced it counted the leap day but the
                        // `post_reform.ordinal` minuend did not.
                        let correction = (year % 100 == 0 && year % 400 != 0 && k.is_leap()) as u32;
                        length - gap.ordinal_gap - correction
                    } else {
                        debug_assert!(year == gap.pre_reform.year);
                        gap.pre_reform.ordinal
                    }
                }
                YearKind::Skipped => 0,
            },
        }
    }

    /// Returns information on the "shape" of the given month of the given
    /// year.
    ///
    /// Returns `None` if the month was completely skipped by a calendar
    /// reformation.  This can only happen for reformations of at least JDN
    /// 3145930 (3901-03-01 in the Gregorian calendar).
    pub const fn month_shape(&self, year: i32, month: Month) -> Option<MonthShape> {
        use inner::RangeOrdering::*;
        use Month::*;
        let length = match month {
            January => 31,
            February => {
                if self.year_kind(year).is_leap() {
                    29
                } else if let Some(gap) = self.gap() {
                    if matches!(gap.cmp_year_month(year, February), EqLower) {
                        29
                    } else {
                        28
                    }
                } else {
                    28
                }
            }
            March => 31,
            April => 30,
            May => 31,
            June => 30,
            July => 31,
            August => 31,
            September => 30,
            October => 31,
            November => 30,
            December => 31,
        };
        let inshape = if let Some(gap) = self.gap() {
            match gap.cmp_year_month(year, month) {
                EqLower | EqBoth => {
                    if matches!(gap.kind, inner::GapKind::IntraMonth) {
                        inner::MonthShape::Gapped {
                            gap_start: gap.pre_reform.day + 1,
                            gap_end: gap.post_reform.day - 1,
                            max_day: length,
                        }
                    } else if gap.pre_reform.day == length {
                        inner::MonthShape::Normal { max_day: length }
                    } else {
                        inner::MonthShape::Tailless {
                            max_day: gap.pre_reform.day,
                            natural_max_day: length,
                        }
                    }
                }
                Between => return None,
                EqUpper if gap.post_reform.day > 1 => inner::MonthShape::Headless {
                    min_day: gap.post_reform.day,
                    max_day: length,
                },
                _ => inner::MonthShape::Normal { max_day: length },
            }
        } else {
            inner::MonthShape::Normal { max_day: length }
        };
        Some(MonthShape {
            calendar: *self,
            year,
            month,
            inner: inshape,
        })
    }

    /// [Private] Calculate the month, day of month, and ordinal day of month
    /// for a given year and day of year.
    ///
    /// # Errors
    ///
    /// Returns [`DateError::OrdinalOutOfRange`] if `ordinal` is zero or
    /// greater than the length of the year.
    fn ordinal2ymddo(&self, year: i32, ordinal: u32) -> Result<(Month, u32, u32), DateError> {
        let max_ordinal = self.year_length(year);
        if !(1..=max_ordinal).contains(&ordinal) {
            return Err(DateError::OrdinalOutOfRange {
                year,
                ordinal,
                max_ordinal,
            });
        }
        let mut days = ordinal;
        for month in MonthIter::new() {
            if let Some(shape) = self.month_shape(year, month) {
                if let Some(day) = shape.nth_day(days) {
                    return Ok((month, day, days));
                }
                days -= shape.len();
            }
        }
        unreachable!()
    }

    /// [Private] Calculate the day of year for a given year, month, and day
    /// ordinal of month.  The day ordinal must be valid for the given month;
    /// otherwise, the result will be garbage.
    fn ymdo2ordinal(&self, year: i32, month: Month, day_ordinal: u32) -> u32 {
        MonthIter::new()
            .take_while(|&m| m < month)
            .filter_map(|m| self.month_shape(year, m).map(|ms| ms.len()))
            .sum::<u32>()
            + day_ordinal
    }

    /// [Private] Calculate the day ordinal for a given year, month, and day of
    /// month.
    ///
    /// # Errors
    ///
    /// Returns [`DateError::DayOutOfRange`] if `day` is zero or greater than
    /// the last day of the given month for the given year.
    ///
    /// Returns [`DateError::SkippedDate`] if the given date (or the entirety
    /// of the month) was skipped by a calendar reformation.
    fn get_day_ordinal(&self, year: i32, month: Month, day: u32) -> Result<u32, DateError> {
        if let Some(shape) = self.month_shape(year, month) {
            shape.day_ordinal_err(day)
        } else {
            Err(DateError::SkippedDate { year, month, day })
        }
    }

    /// [Private] Calculates the Julian day number of the calendar date with
    /// the given year and day of year.
    ///
    /// # Errors
    ///
    /// Returns [`ArithmeticError`] if numeric overflow/underflow occurs.
    fn get_jdn(&self, year: i32, mut ordinal: u32) -> Result<Jdnum, ArithmeticError> {
        use inner::Calendar::*;
        if let Some(gap) = self.gap() {
            if year == gap.post_reform.year && ordinal >= gap.post_reform.ordinal {
                ordinal += gap.ordinal_gap;
            }
        }
        if self.0 == Julian
            || matches!(self.0, Reforming {gap, ..} if (year, ordinal) < (gap.post_reform.year, gap.post_reform.ordinal))
        {
            inner::julian2jdn(year, ordinal)
        } else {
            inner::gregorian2jdn(year, ordinal)
        }.ok_or(ArithmeticError)
    }

    /// [Private] If this is a "reforming" calendar, returns the inner
    /// `ReformGap` field.
    const fn gap(&self) -> Option<inner::ReformGap> {
        match self.0 {
            inner::Calendar::Reforming { gap, .. } => Some(gap),
            _ => None,
        }
    }

    /// [Private] Returns the next year after `year`, skipping any skipped
    /// years.  `year` itself must not be a skipped year or else the result
    /// will be garbage.
    const fn next_year_after(&self, year: i32) -> i32 {
        if let Some(gap) = self.gap() {
            if year == gap.pre_reform.year && gap.post_reform.year > gap.pre_reform.year {
                return gap.post_reform.year;
            }
        }
        year + 1
    }

    /// [Private] Returns the year immediately before `year`, skipping any
    /// skipped years.  `year` itself must not be a skipped year or else the
    /// result will be garbage.
    const fn prev_year_before(&self, year: i32) -> i32 {
        if let Some(gap) = self.gap() {
            if year == gap.post_reform.year && gap.post_reform.year > gap.pre_reform.year {
                return gap.pre_reform.year;
            }
        }
        year - 1
    }
}

/// Information about the days of a month.
///
/// Due to calendar reformations, not every month ranges from 1 to some number
/// from 28 to 31.  A reformation can cause a month to lose days at the
/// beginning, middle, or end, or even to be skipped entirely (though months of
/// this last kind do not have corresponding `MonthShape`s).  When days of a
/// month are skipped due to a reformation, actual day numbers (the values that
/// one would write in a date) will be skipped, resulting in, for example, a
/// month that goes from day 5 to day 15 or a month that starts on day 14.
///
/// A `MonthShape` can be obtained by calling [`Calendar::month_shape()`].
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct MonthShape {
    calendar: Calendar,
    year: i32,
    month: Month,
    inner: inner::MonthShape,
}

impl MonthShape {
    /// Returns the [`Calendar`] to which the month shape belongs
    pub const fn calendar(&self) -> Calendar {
        self.calendar
    }

    /// Returns the year in which the month occurs
    pub const fn year(&self) -> i32 {
        self.year
    }

    /// Returns the [`Month`] value for the month
    pub const fn month(&self) -> Month {
        self.month
    }

    /// Returns the number of days in the month, not counting days skipped due
    /// to a calendar reformation
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::REFORM1582;
    /// let shape = cal.month_shape(1582, Month::October).unwrap();
    /// assert_eq!(shape.len(), 21);
    /// ```
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> u32 {
        use inner::MonthShape::*;
        match self.inner {
            Normal { max_day } => max_day,
            Headless { min_day, max_day } => max_day - min_day + 1,
            Tailless { max_day, .. } => max_day,
            Gapped {
                gap_start,
                gap_end,
                max_day,
            } => max_day - (gap_end - gap_start + 1),
        }
    }

    /// Returns true if the given day occurs in the month.  False may be
    /// returned for a value between [`MonthShape::first_day()`] and
    /// [`MonthShape::last_day()`] if one or more days in the middle of the
    /// month were skipped by a calendar reformation.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::REFORM1582;
    /// let shape = cal.month_shape(1582, Month::October).unwrap();
    /// assert!(shape.contains(1));
    /// assert!(shape.contains(4));
    /// assert!(!shape.contains(5));
    /// assert!(!shape.contains(14));
    /// assert!(shape.contains(15));
    /// assert!(shape.contains(31));
    /// assert!(!shape.contains(32));
    /// ```
    pub fn contains(&self, day: u32) -> bool {
        use inner::MonthShape::*;
        match self.inner {
            Normal { max_day } => (1..=max_day).contains(&day),
            Headless { min_day, max_day } => (min_day..=max_day).contains(&day),
            Tailless { max_day, .. } => (1..=max_day).contains(&day),
            Gapped {
                gap_start,
                gap_end,
                max_day,
            } => (1..=max_day).contains(&day) && !(gap_start..=gap_end).contains(&day),
        }
    }

    /// Returns the first day of the month.  This can be larger than 1 in cases
    /// where one or more days at the beginning of the month were skipped due
    /// to a calendar reformation.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::REFORM1582;
    /// let shape = cal.month_shape(1582, Month::October).unwrap();
    /// assert_eq!(shape.first_day(), 1);
    /// ```
    pub const fn first_day(&self) -> u32 {
        use inner::MonthShape::*;
        match self.inner {
            Headless { min_day, .. } => min_day,
            _ => 1,
        }
    }

    /// Returns the last day of the month.  This can be smaller than the
    /// "traditional" length in cases where one or more days at the end of the
    /// month were skipped due to a calendar reformation.
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::REFORM1582;
    /// let shape = cal.month_shape(1582, Month::October).unwrap();
    /// assert_eq!(shape.last_day(), 31);
    /// ```
    pub const fn last_day(&self) -> u32 {
        use inner::MonthShape::*;
        match self.inner {
            Normal { max_day } => max_day,
            Headless { max_day, .. } => max_day,
            Tailless { max_day, .. } => max_day,
            Gapped { max_day, .. } => max_day,
        }
    }

    /// Converts a day of the month to the corresponding ordinal number of the
    /// day within the month.  This is a number starting from one that does
    /// *not* count days skipped due to a calendar reformation.
    ///
    /// Returns `None` if the given day does not occur in the month.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::REFORM1582;
    /// let shape = cal.month_shape(1582, Month::October).unwrap();
    /// assert_eq!(shape.day_ordinal(1), Some(1));
    /// assert_eq!(shape.day_ordinal(4), Some(4));
    /// assert_eq!(shape.day_ordinal(5), None);
    /// assert_eq!(shape.day_ordinal(14), None);
    /// assert_eq!(shape.day_ordinal(15), Some(5));
    /// assert_eq!(shape.day_ordinal(31), Some(21));
    /// assert_eq!(shape.day_ordinal(32), None);
    /// ```
    pub fn day_ordinal(&self, day: u32) -> Option<u32> {
        self.day_ordinal_err(day).ok()
    }

    /// [Private] Converts a day of the month to the corresponding ordinal
    /// number of the day within the month.  This is a number starting from one
    /// that does *not* count days skipped due to a calendar reformation.
    ///
    /// # Errors
    ///
    /// Returns [`DateError::DayOutOfRange`] if `day` is zero or greater than
    /// the last normal day of the month.
    ///
    /// Returns [`DateError::SkippedDate`] if the given date was skipped by a
    /// calendar reformation.
    fn day_ordinal_err(&self, day: u32) -> Result<u32, DateError> {
        use inner::MonthShape::*;
        match self.inner {
            Normal { max_day } => {
                if (1..=max_day).contains(&day) {
                    Ok(day)
                } else {
                    Err(DateError::DayOutOfRange {
                        year: self.year,
                        month: self.month,
                        day,
                        min_day: 1,
                        max_day,
                    })
                }
            }
            Headless { min_day, max_day } => {
                if (min_day..=max_day).contains(&day) {
                    Ok(day - min_day + 1)
                } else if (1..min_day).contains(&day) {
                    Err(DateError::SkippedDate {
                        year: self.year,
                        month: self.month,
                        day,
                    })
                } else {
                    Err(DateError::DayOutOfRange {
                        year: self.year,
                        month: self.month,
                        day,
                        min_day,
                        max_day,
                    })
                }
            }
            Tailless {
                max_day,
                natural_max_day,
            } => {
                if (1..=max_day).contains(&day) {
                    Ok(day)
                } else if ((max_day + 1)..=natural_max_day).contains(&day) {
                    Err(DateError::SkippedDate {
                        year: self.year,
                        month: self.month,
                        day,
                    })
                } else {
                    Err(DateError::DayOutOfRange {
                        year: self.year,
                        month: self.month,
                        day,
                        min_day: 1,
                        max_day,
                    })
                }
            }
            Gapped {
                gap_start,
                gap_end,
                max_day,
            } => {
                if day == 0 || day > max_day {
                    Err(DateError::DayOutOfRange {
                        year: self.year,
                        month: self.month,
                        day,
                        min_day: 1,
                        max_day,
                    })
                } else if day < gap_start {
                    Ok(day)
                } else if day <= gap_end {
                    Err(DateError::SkippedDate {
                        year: self.year,
                        month: self.month,
                        day,
                    })
                } else {
                    Ok(day - (gap_end - gap_start + 1))
                }
            }
        }
    }

    /// Converts a one-based ordinal number to the corresponding day of the
    /// month.
    ///
    /// Returns `None` if `day` is 0 or larger than [`MonthShape::len()`].
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::REFORM1582;
    /// let shape = cal.month_shape(1582, Month::October).unwrap();
    /// assert_eq!(shape.nth_day(0), None);
    /// assert_eq!(shape.nth_day(1), Some(1));
    /// assert_eq!(shape.nth_day(2), Some(2));
    /// assert_eq!(shape.nth_day(4), Some(4));
    /// assert_eq!(shape.nth_day(5), Some(15));
    /// assert_eq!(shape.nth_day(21), Some(31));
    /// assert_eq!(shape.nth_day(22), None);
    /// ```
    #[allow(clippy::if_then_some_else_none)] // then_some() isn't const
    pub const fn nth_day(&self, day_ordinal: u32) -> Option<u32> {
        use inner::MonthShape::*;
        match self.inner {
            Normal { max_day } | Tailless { max_day, .. }
                if 1 <= day_ordinal && day_ordinal <= max_day =>
            {
                Some(day_ordinal)
            }
            Headless { min_day, max_day }
                if 1 <= day_ordinal && day_ordinal <= (max_day - min_day + 1) =>
            {
                Some(day_ordinal + min_day - 1)
            }
            Gapped { .. } if day_ordinal == 0 => None,
            Gapped { gap_start, .. } if day_ordinal < gap_start => Some(day_ordinal),
            Gapped {
                gap_start,
                gap_end,
                max_day,
            } => {
                let day = day_ordinal + (gap_end - gap_start + 1);
                if day <= max_day {
                    Some(day)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Converts a one-based ordinal number to the corresponding [`Date`]
    /// within the month.
    ///
    /// Returns `None` if `day` is 0 or larger than [`MonthShape::len()`].
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::REFORM1582;
    /// let shape = cal.month_shape(1582, Month::October).unwrap();
    /// let date = shape.nth_date(5).unwrap();
    /// assert_eq!(date.to_string(), "1582-10-15");
    /// ```
    pub fn nth_date(&self, day_ordinal: u32) -> Option<Date> {
        let day = self.nth_day(day_ordinal)?;
        let Ok(date) = self.calendar.at_ymd(self.year, self.month, day) else {
            unreachable!("day should be within range for month");
        };
        Some(date)
    }

    /// Returns the range of days of the month that were skipped by a calendar
    /// reformation.
    ///
    /// Returns `None` if the month was not affected by a reformation.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::REFORM1582;
    /// let shape = cal.month_shape(1582, Month::October).unwrap();
    /// assert_eq!(shape.gap(), Some(5..=14));
    /// ```
    #[allow(clippy::range_minus_one)]
    pub const fn gap(&self) -> Option<RangeInclusive<u32>> {
        use inner::MonthShape::*;
        match self.inner {
            Normal { .. } => None,
            Headless { min_day, .. } => Some(1..=(min_day - 1)),
            Tailless {
                max_day,
                natural_max_day,
            } => Some((max_day + 1)..=natural_max_day),
            Gapped {
                gap_start, gap_end, ..
            } => Some(gap_start..=gap_end),
        }
    }

    /// Returns the [`MonthKind`] for the month
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month, MonthKind};
    ///
    /// let cal = Calendar::REFORM1582;
    /// let shape = cal.month_shape(1582, Month::October).unwrap();
    /// assert_eq!(shape.kind(), MonthKind::Gapped);
    /// ```
    pub const fn kind(&self) -> MonthKind {
        use inner::MonthShape::*;
        match self.inner {
            Normal { .. } => MonthKind::Normal,
            Headless { .. } => MonthKind::Headless,
            Tailless { .. } => MonthKind::Tailless,
            Gapped { .. } => MonthKind::Gapped,
        }
    }

    /// Returns an iterator over all the valid days of the month
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month, MonthKind};
    ///
    /// let cal = Calendar::REFORM1582;
    /// let shape = cal.month_shape(1582, Month::October).unwrap();
    /// let days = shape.days().collect::<Vec<u32>>();
    /// assert_eq!(days, [
    ///      1,  2,  3,  4, 15, 16, 17,
    ///     18, 19, 20, 21, 22, 23, 24,
    ///     25, 26, 27, 28, 29, 30, 31,
    /// ]);
    /// ```
    pub const fn days(&self) -> Days {
        Days::new(*self)
    }

    /// Returns an iterator over all [`Date`s][Date] within the month
    pub const fn dates(&self) -> Dates {
        Dates::new(*self)
    }
}

/// A description of how a calendar month was affected by a calendar
/// reformation.
///
/// A month can be either affected or unaffected by a calendar reformation,
/// and, if it is affected, it can have days at the beginning, end, or middle
/// skipped.  (A month can also be skipped in its entirety by a calendar
/// reformation, but such months do not get `MonthKind` values.)
///
/// A `MonthKind` can be obtained by calling [`MonthShape::kind()`].
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum MonthKind {
    /// The month was unaffected by a calendar reformation
    Normal,

    /// The month had one or more days at the beginning skipped by a calendar
    /// reformation
    Headless,

    /// The month had one or more days at the end skipped by a calendar
    /// reformation
    Tailless,

    /// The month had one or more days in the middle skipped by a calendar
    /// reformation
    Gapped,
}

/// A date (year, month, and day of month) in a certain calendar.
///
/// Instances of `Date` can be constructed through various methods of
/// [`Calendar`], including [`Calendar::at_ymd()`] and [`Calendar::at_jdn()`].
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Date {
    calendar: Calendar,
    year: i32,
    ordinal: u32,
    month: Month,
    day: u32,
    day_ordinal: u32,
    jdn: Jdnum,
}

impl Date {
    /// Returns the [`Calendar`] to which the date belongs
    pub const fn calendar(&self) -> Calendar {
        self.calendar
    }

    /// Returns the date's year
    pub const fn year(&self) -> i32 {
        self.year
    }

    /// Returns the date's month
    pub const fn month(&self) -> Month {
        self.month
    }

    /// Returns the date's day of month (the value that one would write down as
    /// part of a date).  This is a number starting from 1 that counts any
    /// intervening days that may have been skipped due to a calendar
    /// reformation.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, REFORM1582_JDN};
    ///
    /// let cal = Calendar::REFORM1582;
    ///
    /// let pre_reform = cal.at_jdn(REFORM1582_JDN - 1);
    /// assert_eq!(pre_reform.day(), 4);
    ///
    /// let post_reform = cal.at_jdn(REFORM1582_JDN);
    /// assert_eq!(post_reform.day(), 15);
    /// ```
    pub const fn day(&self) -> u32 {
        self.day
    }

    /// Returns the ordinal number of the day within the month.  This is a
    /// number starting from one that does *not* count days skipped due to a
    /// calendar reformation.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, REFORM1582_JDN};
    ///
    /// let cal = Calendar::REFORM1582;
    ///
    /// let pre_reform = cal.at_jdn(REFORM1582_JDN - 1);
    /// assert_eq!(pre_reform.day_ordinal(), 4);
    ///
    /// let post_reform = cal.at_jdn(REFORM1582_JDN);
    /// assert_eq!(post_reform.day_ordinal(), 5);
    /// ```
    pub const fn day_ordinal(&self) -> u32 {
        self.day_ordinal
    }

    /// Returns the zero-based ordinal number of the day within the month.
    /// This is the same as [`Date::day_ordinal()`], except starting from 0
    /// instead of 1.
    pub const fn day_ordinal0(&self) -> u32 {
        self.day_ordinal - 1
    }

    /// Returns the ordinal number of the day within the year.  Ordinal date 1
    /// is the first day of the year, ordinal 2 is the second, etc.
    ///
    /// # Examples
    ///
    /// Ordinal for a regular date:
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::GREGORIAN;
    /// let date = cal.at_ymd(2023, Month::May, 1).unwrap();
    /// assert_eq!(date.ordinal(), 121);
    /// ```
    ///
    /// Ordinal around a calendar reformation:
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::REFORM1582;
    ///
    /// let pre_reform = cal.at_ymd(1582, Month::October, 4).unwrap();
    /// assert_eq!(pre_reform.ordinal(), 277);
    ///
    /// let post_reform = cal.at_ymd(1582, Month::October, 15).unwrap();
    /// assert_eq!(post_reform.ordinal(), 278);
    /// ```
    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    /// Returns the zero-based ordinal number of the day within the year.  This
    /// is the same as [`Date::ordinal()`], except starting from 0 instead of
    /// 1.
    pub const fn ordinal0(&self) -> u32 {
        self.ordinal - 1
    }

    /// Returns the Julian day number of the date.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let date = Calendar::GREGORIAN.at_ymd(2023, Month::May, 1).unwrap();
    /// assert_eq!(date.julian_day_number(), 2460066);
    /// ```
    pub const fn julian_day_number(&self) -> Jdnum {
        self.jdn
    }

    /// Returns the date's day of the week.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month, Weekday};
    ///
    /// let date = Calendar::GREGORIAN.at_ymd(2023, Month::May, 1).unwrap();
    /// assert_eq!(date.weekday(), Weekday::Monday);
    /// ```
    pub fn weekday(&self) -> Weekday {
        Weekday::for_jdn(self.jdn)
    }

    /// Returns true if the date is in the Julian calendar (a.k.a. "Old
    /// Style"), i.e., if [`Date::calendar()`] is either a proleptic Julian
    /// calendar or a "reforming" calendar for which the reformation occurs
    /// after the date in question.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::REFORM1582;
    ///
    /// let pre_reform = cal.at_ymd(1582, Month::October, 4).unwrap();
    /// assert!(pre_reform.is_julian());
    ///
    /// let post_reform = cal.at_ymd(1582, Month::October, 15).unwrap();
    /// assert!(!post_reform.is_julian());
    /// ```
    pub const fn is_julian(&self) -> bool {
        match self.calendar.0 {
            inner::Calendar::Julian => true,
            inner::Calendar::Reforming { reformation, .. } => {
                self.julian_day_number() < reformation
            }
            inner::Calendar::Gregorian => false,
        }
    }

    /// Returns true if the date is in the Gregorian calendar (a.k.a. "New
    /// Style"), i.e., if [`Date::calendar()`] is either a proleptic Gregorian
    /// calendar or a "reforming" calendar for which the reformation occurs at
    /// or before the date in question.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let cal = Calendar::REFORM1582;
    ///
    /// let pre_reform = cal.at_ymd(1582, Month::October, 4).unwrap();
    /// assert!(!pre_reform.is_gregorian());
    ///
    /// let post_reform = cal.at_ymd(1582, Month::October, 15).unwrap();
    /// assert!(post_reform.is_gregorian());
    /// ```
    pub const fn is_gregorian(&self) -> bool {
        match self.calendar.0 {
            inner::Calendar::Julian => false,
            inner::Calendar::Reforming { reformation, .. } => {
                reformation <= self.julian_day_number()
            }
            inner::Calendar::Gregorian => true,
        }
    }

    /// Convert to the date with the same Julian day number in the given
    /// calendar.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let gregorian_date = Calendar::GREGORIAN.at_ymd(2023, Month::May, 1).unwrap();
    /// let julian_date = gregorian_date.convert_to(Calendar::JULIAN);
    /// assert_eq!(julian_date.to_string(), "2023-04-18");
    /// ```
    pub fn convert_to(&self, calendar: Calendar) -> Date {
        calendar.at_jdn(self.julian_day_number())
    }

    /// Returns the next date in the calendar.  Returns `None` if numeric
    /// overflow occurs while calculating the next date's Julian day number
    /// (See [`Calendar::at_ymd()`] for when this can happen).
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let date = Calendar::REFORM1582.at_ymd(1582, Month::October, 4).unwrap();
    /// let next_date = date.succ().unwrap();
    /// assert_eq!(next_date.to_string(), "1582-10-15");
    /// ```
    pub fn succ(&self) -> Option<Date> {
        let jdn = self.jdn.checked_add(1)?;
        let mut year = self.year;
        let mut ordinal = self.ordinal + 1;
        let (month, day, day_ordinal) = match self.calendar().ordinal2ymddo(year, ordinal) {
            Ok(values) => values,
            Err(DateError::OrdinalOutOfRange { .. }) => {
                year = self.calendar().next_year_after(year);
                ordinal = 1;
                // Erroring here shouldn't happen, but just in case...
                self.calendar().ordinal2ymddo(year, ordinal).ok()?
            }
            // This shouldn't happen, but just in case...
            _ => return None,
        };
        Some(Date {
            calendar: self.calendar,
            year,
            ordinal,
            month,
            day,
            day_ordinal,
            jdn,
        })
    }

    /// Returns the previous date in the calendar.  Returns `None` if numeric
    /// underflow occurs while calculating the previous date's Julian day
    /// number (See [`Calendar::at_ymd()`] for when this can happen).
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let date = Calendar::REFORM1582.at_ymd(1582, Month::October, 15).unwrap();
    /// let next_date = date.pred().unwrap();
    /// assert_eq!(next_date.to_string(), "1582-10-04");
    /// ```
    pub fn pred(&self) -> Option<Date> {
        let jdn = self.jdn.checked_sub(1)?;
        let mut year = self.year;
        let ordinal = if self.ordinal > 1 {
            self.ordinal - 1
        } else {
            year = self.calendar().prev_year_before(year);
            self.calendar().year_length(year)
        };
        // Erroring here shouldn't happen, but just in case...
        let (month, day, day_ordinal) = self.calendar().ordinal2ymddo(year, ordinal).ok()?;
        Some(Date {
            calendar: self.calendar,
            year,
            ordinal,
            month,
            day,
            day_ordinal,
            jdn,
        })
    }

    /// Returns an iterator over all later days in the calendar, starting after
    /// (and not including) the receiver date.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let date = Calendar::REFORM1582.at_ymd(1582, Month::October, 2).unwrap();
    /// let mut iter = date.later();
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-03");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-04");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-15");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-16");
    /// ```
    pub const fn later(&self) -> Later {
        Later::new(*self)
    }

    /// Returns an iterator over all earlier days in the calendar, starting
    /// before (and not including) the receiver date.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let date = Calendar::REFORM1582.at_ymd(1582, Month::October, 17).unwrap();
    /// let mut iter = date.earlier();
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-16");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-15");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-04");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-03");
    /// ```
    pub const fn earlier(&self) -> Earlier {
        Earlier::new(*self)
    }

    /// Returns an iterator that yields the receiver date followed by all later
    /// days in the calendar.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let date = Calendar::REFORM1582.at_ymd(1582, Month::October, 2).unwrap();
    /// let mut iter = date.and_later();
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-02");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-03");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-04");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-15");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-16");
    /// ```
    pub const fn and_later(&self) -> AndLater {
        AndLater::new(*self)
    }

    /// Returns an iterator that yields the receiver date followed by all
    /// earlier days in the calendar.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let date = Calendar::REFORM1582.at_ymd(1582, Month::October, 17).unwrap();
    /// let mut iter = date.and_earlier();
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-17");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-16");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-15");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-04");
    /// assert_eq!(iter.next().unwrap().to_string(), "1582-10-03");
    /// ```
    pub const fn and_earlier(&self) -> AndEarlier {
        AndEarlier::new(*self)
    }
}

impl PartialOrd for Date {
    /// `Date` instances are ordered first by Julian day number, then by
    /// `Calendar`.
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    /// `Date` instances are ordered first by Julian day number, then by
    /// `Calendar`.
    fn cmp(&self, other: &Date) -> Ordering {
        (self.julian_day_number(), self.calendar())
            .cmp(&(other.julian_day_number(), other.calendar()))
    }
}

impl fmt::Display for Date {
    /// A `Date` is displayed in the format `YYYY-MM-DD` (year, month number,
    /// and day of month) by default.  Selecting the alternate form with `{:#}`
    /// instead produces a string of the form `YYYY-JJJ` (year and day of
    /// year).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-", self.year())?;
        if f.alternate() {
            write!(f, "{:03}", self.ordinal())?;
        } else {
            write!(f, "{:02}-{:02}", self.month().number(), self.day())?;
        }
        Ok(())
    }
}

#[cfg(feature = "chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl<T: chrono::Datelike> From<T> for Date {
    /// Convert a [`chrono::Datelike`] value to the corresponding `Date` in the
    /// proleptic Gregorian calendar
    fn from(date: T) -> Date {
        Calendar::GREGORIAN
            .at_ymd(
                date.year(),
                Month::try_from(date.month())
                    .expect("chrono month value should be within range for Month"),
                date.day(),
            )
            .expect("chrono date should be valid for proleptic Gregorian calendar")
    }
}

#[cfg(feature = "chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl TryFrom<Date> for chrono::naive::NaiveDate {
    type Error = TryFromDateError;

    /// Convert a [`Date`] to a [`chrono::naive::NaiveDate`].  The source date
    /// is converted to the proleptic Gregorian calendar first, if necessary.
    ///
    /// # Errors
    ///
    /// Returns [`TryFromDateError`] if the source date is outside the valid
    /// range for a `chrono::naive::NaiveDate`.
    fn try_from(mut date: Date) -> Result<chrono::naive::NaiveDate, TryFromDateError> {
        if !date.is_gregorian() {
            date = date.convert_to(Calendar::GREGORIAN);
        }
        chrono::naive::NaiveDate::from_ymd_opt(date.year(), date.month().number(), date.day())
            .ok_or(TryFromDateError)
    }
}

/// An enumeration of the twelve months of "Julian-style" years.
///
/// An iterator over the months of the year is available as
/// [`iter::MonthIter`].
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl Month {
    /// Returns the English name of the month.  This is the same as the month's
    /// Rust identifier.
    pub const fn name(&self) -> &'static str {
        use Month::*;
        match self {
            January => "January",
            February => "February",
            March => "March",
            April => "April",
            May => "May",
            June => "June",
            July => "July",
            August => "August",
            September => "September",
            October => "October",
            November => "November",
            December => "December",
        }
    }

    /// Returns the first three letters of the English name of the month
    pub const fn short_name(&self) -> &'static str {
        use Month::*;
        match self {
            January => "Jan",
            February => "Feb",
            March => "Mar",
            April => "Apr",
            May => "May",
            June => "Jun",
            July => "Jul",
            August => "Aug",
            September => "Sep",
            October => "Oct",
            November => "Nov",
            December => "Dec",
        }
    }

    /// Returns the number of the month, where January is 1.
    ///
    /// These values are also available as the enumeration discriminants and
    /// can be accessed by casting, e.g., `Month::January as u32`.
    pub const fn number(&self) -> u32 {
        *self as u32
    }

    /// Returns the zero-based number of the month, where January is 0.
    pub const fn number0(&self) -> u32 {
        self.number() - 1
    }

    /// Returns the month before the month in question.  Returns `None` for
    /// January.
    pub const fn pred(&self) -> Option<Month> {
        use Month::*;
        match self {
            January => None,
            February => Some(January),
            March => Some(February),
            April => Some(March),
            May => Some(April),
            June => Some(May),
            July => Some(June),
            August => Some(July),
            September => Some(August),
            October => Some(September),
            November => Some(October),
            December => Some(November),
        }
    }

    /// Returns the month after the month in question.  Returns `None` for
    /// December.
    pub const fn succ(&self) -> Option<Month> {
        use Month::*;
        match self {
            January => Some(February),
            February => Some(March),
            March => Some(April),
            April => Some(May),
            May => Some(June),
            June => Some(July),
            July => Some(August),
            August => Some(September),
            September => Some(October),
            October => Some(November),
            November => Some(December),
            December => None,
        }
    }

    /// [Private] `const` equivalent of `==`
    const fn eq(&self, other: Month) -> bool {
        (*self as u32) == (other as u32)
    }

    /// [Private] `const` equivalent of `<`
    const fn lt(&self, other: Month) -> bool {
        (*self as u32) < (other as u32)
    }

    /// [Private] `const` equivalent of `<=`
    const fn le(&self, other: Month) -> bool {
        (*self as u32) <= (other as u32)
    }
}

impl fmt::Display for Month {
    /// A `Month` is displayed as its English name by default.  Selecting the
    /// alternate form with `{:#}` instead produces just the first three
    /// letters of the English name.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.short_name())
        } else {
            write!(f, "{}", self.name())
        }
    }
}

impl FromStr for Month {
    type Err = ParseMonthError;

    /// Parses a month from either its English name or just the first three
    /// letters of the name.  Input is treated case-insensitively.
    fn from_str(s: &str) -> Result<Month, ParseMonthError> {
        use Month::*;
        match s.to_ascii_lowercase().as_str() {
            "january" | "jan" => Ok(January),
            "february" | "feb" => Ok(February),
            "march" | "mar" => Ok(March),
            "april" | "apr" => Ok(April),
            "may" => Ok(May),
            "june" | "jun" => Ok(June),
            "july" | "jul" => Ok(July),
            "august" | "aug" => Ok(August),
            "september" | "sep" => Ok(September),
            "october" | "oct" => Ok(October),
            "november" | "nov" => Ok(November),
            "december" | "dec" => Ok(December),
            _ => Err(ParseMonthError),
        }
    }
}

macro_rules! impl_month_try_from {
    ($($t:ty),* $(,)?) => {
      $(
        impl TryFrom<$t> for Month {
            type Error = TryIntoMonthError;

            /// Convert a month number to the corresponding month.
            ///
            /// # Errors
            ///
            /// Returns [`TryIntoMonthError`] if the given number is less than
            /// one or greater than twelve.
            fn try_from(value: $t) -> Result<Month, TryIntoMonthError> {
                use Month::*;
                match value {
                    1 => Ok(January),
                    2 => Ok(February),
                    3 => Ok(March),
                    4 => Ok(April),
                    5 => Ok(May),
                    6 => Ok(June),
                    7 => Ok(July),
                    8 => Ok(August),
                    9 => Ok(September),
                    10 => Ok(October),
                    11 => Ok(November),
                    12 => Ok(December),
                    _ => Err(TryIntoMonthError),
                }
            }
        }
      )*
    }
}

impl_month_try_from!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

#[cfg(feature = "chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl From<chrono::Month> for Month {
    /// Convert a [`chrono::Month`] to a [`Month`]
    fn from(m: chrono::Month) -> Month {
        match m {
            chrono::Month::January => Month::January,
            chrono::Month::February => Month::February,
            chrono::Month::March => Month::March,
            chrono::Month::April => Month::April,
            chrono::Month::May => Month::May,
            chrono::Month::June => Month::June,
            chrono::Month::July => Month::July,
            chrono::Month::August => Month::August,
            chrono::Month::September => Month::September,
            chrono::Month::October => Month::October,
            chrono::Month::November => Month::November,
            chrono::Month::December => Month::December,
        }
    }
}

#[cfg(feature = "chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl From<Month> for chrono::Month {
    /// Convert a [`Month`] to a [`chrono::Month`]
    fn from(m: Month) -> chrono::Month {
        match m {
            Month::January => chrono::Month::January,
            Month::February => chrono::Month::February,
            Month::March => chrono::Month::March,
            Month::April => chrono::Month::April,
            Month::May => chrono::Month::May,
            Month::June => chrono::Month::June,
            Month::July => chrono::Month::July,
            Month::August => chrono::Month::August,
            Month::September => chrono::Month::September,
            Month::October => chrono::Month::October,
            Month::November => chrono::Month::November,
            Month::December => chrono::Month::December,
        }
    }
}

/// An enumeration of the seven days of the week.
///
/// This type follows the ISO convention of designating Monday as the first day
/// of the week and Sunday as the last.
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum Weekday {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

impl Weekday {
    /// Returns the English name of the weekday.  This is the same as the
    /// weekday's Rust identifier.
    pub const fn name(&self) -> &'static str {
        use Weekday::*;
        match self {
            Monday => "Monday",
            Tuesday => "Tuesday",
            Wednesday => "Wednesday",
            Thursday => "Thursday",
            Friday => "Friday",
            Saturday => "Saturday",
            Sunday => "Sunday",
        }
    }

    /// Returns the first three letters of the English name of the weekday
    pub const fn short_name(&self) -> &'static str {
        use Weekday::*;
        match self {
            Monday => "Mon",
            Tuesday => "Tue",
            Wednesday => "Wed",
            Thursday => "Thu",
            Friday => "Fri",
            Saturday => "Sat",
            Sunday => "Sun",
        }
    }

    /// Returns the number of the weekday, where Monday is 1 and Sunday is 7.
    ///
    /// These values are also available as the enumeration discriminants and
    /// can be accessed by casting, e.g., `Weekday::Monday as u32`.
    pub const fn number(&self) -> u32 {
        *self as u32
    }

    /// Returns the zero-based number of the weekday, where Monday is 0 and
    /// Sunday is 6.
    pub const fn number0(&self) -> u32 {
        self.number() - 1
    }

    /// Returns the weekday for the given Julian day number
    pub fn for_jdn(jdn: Jdnum) -> Weekday {
        match Weekday::try_from(jdn.rem_euclid(7) + 1) {
            Ok(wd) => wd,
            Err(_) => unreachable!("JDN computation should produce valid weekday number"),
        }
    }

    /// Returns the day of the week before this one.  Returns `None` for
    /// Monday.
    pub const fn pred(&self) -> Option<Weekday> {
        use Weekday::*;
        match self {
            Monday => None,
            Tuesday => Some(Monday),
            Wednesday => Some(Tuesday),
            Thursday => Some(Wednesday),
            Friday => Some(Thursday),
            Saturday => Some(Friday),
            Sunday => Some(Saturday),
        }
    }

    /// Returns the day of the week after this one.  Returns `None` for
    /// Sunday.
    pub const fn succ(&self) -> Option<Weekday> {
        use Weekday::*;
        match self {
            Monday => Some(Tuesday),
            Tuesday => Some(Wednesday),
            Wednesday => Some(Thursday),
            Thursday => Some(Friday),
            Friday => Some(Saturday),
            Saturday => Some(Sunday),
            Sunday => None,
        }
    }
}

impl fmt::Display for Weekday {
    /// A `Weekday` is displayed as its English name by default.  Selecting the
    /// alternate form with `{:#}` instead produces just the first three
    /// letters of the English name.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.short_name())
        } else {
            write!(f, "{}", self.name())
        }
    }
}

impl FromStr for Weekday {
    type Err = ParseWeekdayError;

    /// Parses a weekday from either its English name or just the first three
    /// letters of the name.  Input is treated case-insensitively.
    fn from_str(s: &str) -> Result<Weekday, ParseWeekdayError> {
        use Weekday::*;
        match s.to_ascii_lowercase().as_str() {
            "sunday" | "sun" => Ok(Sunday),
            "monday" | "mon" => Ok(Monday),
            "tuesday" | "tue" => Ok(Tuesday),
            "wednesday" | "wed" => Ok(Wednesday),
            "thursday" | "thu" => Ok(Thursday),
            "friday" | "fri" => Ok(Friday),
            "saturday" | "sat" => Ok(Saturday),
            _ => Err(ParseWeekdayError),
        }
    }
}

macro_rules! impl_weekday_try_from {
    ($($t:ty),* $(,)?) => {
      $(
        impl TryFrom<$t> for Weekday {
            type Error = TryIntoWeekdayError;

            /// Convert a number from 1 to 7 to a weekday, where 1 is Monday
            /// and 7 is Sunday.
            ///
            /// # Errors
            ///
            /// Returns [`TryIntoWeekdayError`] if the given number is less
            /// than one or greater than seven.
            fn try_from(value: $t) -> Result<Weekday, TryIntoWeekdayError> {
                use Weekday::*;
                match value {
                    1 => Ok(Monday),
                    2 => Ok(Tuesday),
                    3 => Ok(Wednesday),
                    4 => Ok(Thursday),
                    5 => Ok(Friday),
                    6 => Ok(Saturday),
                    7 => Ok(Sunday),
                    _ => Err(TryIntoWeekdayError),
                }
            }
        }
      )*
    }
}

impl_weekday_try_from!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

#[cfg(feature = "chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl From<chrono::Weekday> for Weekday {
    /// Convert a [`chrono::Weekday`] to a [`Weekday`]
    fn from(wd: chrono::Weekday) -> Weekday {
        match wd {
            chrono::Weekday::Sun => Weekday::Sunday,
            chrono::Weekday::Mon => Weekday::Monday,
            chrono::Weekday::Tue => Weekday::Tuesday,
            chrono::Weekday::Wed => Weekday::Wednesday,
            chrono::Weekday::Thu => Weekday::Thursday,
            chrono::Weekday::Fri => Weekday::Friday,
            chrono::Weekday::Sat => Weekday::Saturday,
        }
    }
}

#[cfg(feature = "chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl From<Weekday> for chrono::Weekday {
    /// Convert a [`Weekday`] to a [`chrono::Weekday`]
    fn from(wd: Weekday) -> chrono::Weekday {
        match wd {
            Weekday::Sunday => chrono::Weekday::Sun,
            Weekday::Monday => chrono::Weekday::Mon,
            Weekday::Tuesday => chrono::Weekday::Tue,
            Weekday::Wednesday => chrono::Weekday::Wed,
            Weekday::Thursday => chrono::Weekday::Thu,
            Weekday::Friday => chrono::Weekday::Fri,
            Weekday::Saturday => chrono::Weekday::Sat,
        }
    }
}

/// Converts a [`std::time::SystemTime`] instance to the corresponding Julian
/// day number, along with a count of seconds since midnight UTC.
///
/// # Errors
///
/// Returns [`ArithmeticError`] if numeric overflow/underflow occurs during
/// conversion.  This can only happen if the system time in UTC is before
/// -5884323-05-15 (-5884202-03-16 O.S.) or after 5874898-06-03 (5874777-10-17
/// O.S.).
pub fn system2jdn(t: SystemTime) -> Result<(Jdnum, u32), ArithmeticError> {
    let ts = match t.duration_since(UNIX_EPOCH) {
        Ok(d) => i64::try_from(d.as_secs()),
        Err(e) => i64::try_from(e.duration().as_secs()).map(|i| -i),
    }
    .map_err(|_| ArithmeticError)?;
    unix2jdn(ts)
}

/// Converts a [Unix time][] to the corresponding Julian day number, along with
/// a count of seconds since midnight UTC.
///
/// [Unix time]: https://en.wikipedia.org/wiki/Unix_time
///
/// # Example
///
/// ```
/// use julian::unix2jdn;
///
/// let (jdn, seconds) = unix2jdn(1682906621).unwrap();
/// assert_eq!(jdn, 2460066);
/// assert_eq!(seconds, 7421);
/// ```
///
/// # Errors
///
/// Returns [`ArithmeticError`] if numeric overflow/underflow occurs during
/// conversion.  This can only happen if the timestamp is less than
/// -185753453990400 or greater than 185331720383999.
pub fn unix2jdn(unix_time: i64) -> Result<(Jdnum, u32), ArithmeticError> {
    let jd = Jdnum::try_from(unix_time.div_euclid(SECONDS_IN_DAY) + (UNIX_EPOCH_JDN as i64))
        .map_err(|_| ArithmeticError)?;
    let Ok(secs) = u32::try_from(unix_time.rem_euclid(SECONDS_IN_DAY)) else {
        unreachable!("Unix time modulo seconds in day should fit in u32");
    };
    Ok((jd, secs))
}

/// Converts a Julian day number to the [Unix time][] for midnight UTC on that
/// day.
///
/// [Unix time]: https://en.wikipedia.org/wiki/Unix_time
///
/// # Example
///
/// ```
/// use julian::jdn2unix;
///
/// let ts = jdn2unix(2460066);
/// assert_eq!(ts, 1682899200);
/// ```
pub fn jdn2unix(jdn: Jdnum) -> i64 {
    (i64::from(jdn) - (UNIX_EPOCH_JDN as i64)) * SECONDS_IN_DAY
}

#[cfg(test)]
mod tests {
    mod at_ordinal_date;
    mod at_ymd;
    mod autogen;
    mod calendar;
    mod chrono;
    mod date;
    mod jdn;
    mod month;
    mod parse_date;
    mod reformations;
    mod unix;
    mod weekday;
    mod year_kind;
}
