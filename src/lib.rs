#[cfg(test)]
extern crate rstest_reuse;

mod inner;
pub mod ncal;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator};
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

/// Type used for Julian day numbers in this crate
pub type Jdnum = i32;

/// The Julian day number of the date at which the Gregorian Reformation was
/// first put into effect (1582-10-15, following 1582-10-04 O.S.)
pub const GREGORIAN: Jdnum = 2299161;

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
    pub fn is_common(&self) -> bool {
        use YearKind::*;
        matches!(self, Common | ReformCommon)
    }

    /// Returns true if the year kind is `Leap` or `ReformLeap`
    pub fn is_leap(&self) -> bool {
        use YearKind::*;
        matches!(self, Leap | ReformLeap)
    }

    /// Returns true if the year kind is `ReformCommon` or `ReformLeap`
    pub fn is_reform(&self) -> bool {
        use YearKind::*;
        matches!(self, ReformCommon | ReformLeap)
    }

    /// Returns true if the year kind is `Skipped`
    pub fn is_skipped(&self) -> bool {
        self == &YearKind::Skipped
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
    /// An instance of a reforming calendar with the reformation set at the
    /// date in history at which the Gregorian Reformation was first observed.
    ///
    /// This calendar is equal to
    /// `Calendar::reforming(julian::GREGORIAN).unwrap()`.
    pub const GREGORIAN_REFORM: Calendar = Calendar(inner::Calendar::Reforming {
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

    /// Construct an instance of a proleptic Julian calendar
    pub fn julian() -> Calendar {
        Calendar(inner::Calendar::Julian)
    }

    /// Construct an instance of a proleptic Gregorian calendar
    pub fn gregorian() -> Calendar {
        Calendar(inner::Calendar::Gregorian)
    }

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
        let julian = Calendar::julian();
        let pre_reform = julian.at_jdn(
            reformation
                .checked_sub(1)
                .ok_or(ReformingError::InvalidReformation)?,
        );
        let post_reform = Calendar::gregorian().at_jdn(reformation);
        let mut ordinal = post_reform.ordinal();
        if post_reform.year % 100 == 0
            && post_reform.year % 400 != 0
            && post_reform.month > Month::February
        {
            ordinal += 1;
        }
        if julian.get_jdn(post_reform.year(), ordinal)? <= reformation {
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
    /// of seconds since midnight.  UTC is used as the timezone.
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
    /// along with a count of seconds since midnight.  UTC is used as the
    /// timezone.
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
    /// along with a count of seconds since midnight.  UTC is used as the
    /// timezone.
    ///
    /// [Unix time]: https://en.wikipedia.org/wiki/Unix_time
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

    /// Returns the date of the calendar with the given Julian day number.
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
        let (month, day, day_ordinal) = self.ordinal2ymddo(year, ordinal).unwrap();
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

    /// Returns true if this is a proleptic Julian calendar
    pub fn is_julian(&self) -> bool {
        self.0 == inner::Calendar::Julian
    }

    /// Returns true if this is a proleptic Gregorian calendar
    pub fn is_gregorian(&self) -> bool {
        self.0 == inner::Calendar::Gregorian
    }

    /// Returns true if this is a "reforming" calendar
    pub fn is_reforming(&self) -> bool {
        matches!(self.0, inner::Calendar::Reforming { .. })
    }

    /// Returns true if this is a proleptic Julian or Gregorian calendar, i.e.,
    /// not a "reforming" calendar
    pub fn is_proleptic(&self) -> bool {
        self.is_julian() || self.is_gregorian()
    }

    /// If this is a "reforming" calendar, returns the Julian day number of the
    /// reformation (the first day on which the Gregorian calendar is used)
    pub fn reformation(&self) -> Option<Jdnum> {
        if let inner::Calendar::Reforming { reformation, .. } = self.0 {
            Some(reformation)
        } else {
            None
        }
    }

    /// If this is a "reforming" calendar, returns the last date that follows
    /// the Julian calendar, i.e., the date immediately before the reformation.
    pub fn last_julian_date(&self) -> Option<Date> {
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
    pub fn year_kind(&self, year: i32) -> YearKind {
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
                        if (gap.pre_reform.month, gap.pre_reform.day) == (Month::December, 31) {
                            if inner::is_julian_leap_year(year) {
                                YearKind::Leap
                            } else {
                                YearKind::Common
                            }
                        } else if Month::February < gap.pre_reform.month
                            && inner::is_julian_leap_year(year)
                        {
                            YearKind::ReformLeap
                        } else {
                            YearKind::ReformCommon
                        }
                    }
                    Between => YearKind::Skipped,
                    EqBoth => {
                        if (Month::February < gap.pre_reform.month
                            && inner::is_julian_leap_year(year))
                            || (gap.post_reform.month <= Month::February
                                && inner::is_gregorian_leap_year(year))
                        {
                            YearKind::ReformLeap
                        } else {
                            YearKind::ReformCommon
                        }
                    }
                    EqUpper => {
                        if (gap.post_reform.month, gap.post_reform.day) == (Month::January, 1) {
                            if inner::is_gregorian_leap_year(year) {
                                YearKind::Leap
                            } else {
                                YearKind::Common
                            }
                        } else if gap.post_reform.month <= Month::February
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
    pub fn year_length(&self, year: i32) -> u32 {
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
                    let length = if k == YearKind::ReformCommon {
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
                        let correction =
                            u32::from(year % 100 == 0 && year % 400 != 0 && k.is_leap());
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
    pub fn month_shape(&self, year: i32, month: Month) -> Option<MonthShape> {
        use inner::RangeOrdering::*;
        use Month::*;
        let length = match month {
            January => 31,
            February => {
                if self.year_kind(year).is_leap() {
                    29
                } else if let Some(gap) = self.gap() {
                    if gap.cmp_year_month(year, February) == EqLower {
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
                    if gap.kind == inner::GapKind::IntraMonth {
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
    fn gap(&self) -> Option<inner::ReformGap> {
        match self.0 {
            inner::Calendar::Reforming { gap, .. } => Some(gap),
            _ => None,
        }
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
    year: i32,
    month: Month,
    inner: inner::MonthShape,
}

impl MonthShape {
    /// Returns the year in which the month occurs
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Returns the [`Month`] value for the month
    pub fn month(&self) -> Month {
        self.month
    }

    /// Returns the number of days in the month, not counting days skipped due
    /// to a calendar reformation
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u32 {
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
    pub fn first_day(&self) -> u32 {
        use inner::MonthShape::*;
        match self.inner {
            Headless { min_day, .. } => min_day,
            _ => 1,
        }
    }

    /// Returns the last day of the month.  This can be smaller than the
    /// "traditional" length in cases where one or more days at the end of the
    /// month were skipped due to a calendar reformation.
    pub fn last_day(&self) -> u32 {
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
    pub fn nth_day(&self, day_ordinal: u32) -> Option<u32> {
        use inner::MonthShape::*;
        match self.inner {
            Normal { max_day } => (1..=max_day).contains(&day_ordinal).then_some(day_ordinal),
            Headless { min_day, max_day } => (1..=(max_day - min_day + 1))
                .contains(&day_ordinal)
                .then_some(day_ordinal + min_day - 1),
            Tailless { max_day, .. } => (1..=max_day).contains(&day_ordinal).then_some(day_ordinal),
            Gapped {
                gap_start,
                gap_end,
                max_day,
            } => {
                if day_ordinal == 0 {
                    None
                } else if day_ordinal < gap_start {
                    Some(day_ordinal)
                } else {
                    let day = day_ordinal + (gap_end - gap_start + 1);
                    (day <= max_day).then_some(day)
                }
            }
        }
    }

    /// Returns the range of days of the month that were skipped by a calendar
    /// reformation.
    ///
    /// Returns `None` if the month was not affected by a reformation.
    pub fn gap(&self) -> Option<RangeInclusive<u32>> {
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
    pub fn kind(&self) -> MonthKind {
        use inner::MonthShape::*;
        match self.inner {
            Normal { .. } => MonthKind::Normal,
            Headless { .. } => MonthKind::Headless,
            Tailless { .. } => MonthKind::Tailless,
            Gapped { .. } => MonthKind::Gapped,
        }
    }

    /// Returns an iterator over all the valid days of the month
    pub fn days(&self) -> Days {
        Days::new(*self)
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

/// An iterator over the days of a month.
///
/// A `Days` instance can be acquired by calling [`MonthShape::days()`].
pub struct Days {
    month_shape: MonthShape,
    inner: RangeInclusive<u32>,
}

impl Days {
    fn new(month_shape: MonthShape) -> Self {
        Days {
            month_shape,
            inner: 1..=(month_shape.len()),
        }
    }
}

impl Iterator for Days {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.month_shape.nth_day(self.inner.next()?)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl FusedIterator for Days {}

impl ExactSizeIterator for Days {}

impl DoubleEndedIterator for Days {
    fn next_back(&mut self) -> Option<u32> {
        self.month_shape.nth_day(self.inner.next_back()?)
    }
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
    /// Returns a reference to the [`Calendar`] to which the date belongs
    pub fn calendar(&self) -> &Calendar {
        &self.calendar
    }

    /// Returns the date's year
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Returns the date's month
    pub fn month(&self) -> Month {
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
    /// use julian::{Calendar, GREGORIAN};
    ///
    /// let cal = Calendar::GREGORIAN_REFORM;
    ///
    /// let pre_reform = cal.at_jdn(GREGORIAN - 1);
    /// assert_eq!(pre_reform.day(), 4);
    ///
    /// let post_reform = cal.at_jdn(GREGORIAN);
    /// assert_eq!(post_reform.day(), 15);
    /// ```
    pub fn day(&self) -> u32 {
        self.day
    }

    /// Returns the ordinal number of the day within the month.  This is a
    /// number starting from one that does *not* count days skipped due to a
    /// calendar reformation.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, GREGORIAN};
    ///
    /// let cal = Calendar::GREGORIAN_REFORM;
    ///
    /// let pre_reform = cal.at_jdn(GREGORIAN - 1);
    /// assert_eq!(pre_reform.day_ordinal(), 4);
    ///
    /// let post_reform = cal.at_jdn(GREGORIAN);
    /// assert_eq!(post_reform.day_ordinal(), 5);
    /// ```
    pub fn day_ordinal(&self) -> u32 {
        self.day_ordinal
    }

    /// Returns the zero-based ordinal number of the day within the month.
    /// This is the same as [`Date::day_ordinal()`], except starting from 0
    /// instead of 1.
    pub fn day_ordinal0(&self) -> u32 {
        self.day_ordinal - 1
    }

    /// Returns the ordinal number of the day within the year.  Ordinal date 1
    /// is the first day of the year, ordinal 2 is the second, etc.
    pub fn ordinal(&self) -> u32 {
        self.ordinal
    }

    /// Returns the zero-based ordinal number of the day within the year.  This
    /// is the same as [`Date::ordinal()`], except starting from 0 instead of
    /// 1.
    pub fn ordinal0(&self) -> u32 {
        self.ordinal - 1
    }

    /// Returns the Julian day number of the date.
    ///
    /// # Example
    ///
    /// ```
    /// use julian::{Calendar, Month};
    ///
    /// let date = Calendar::gregorian().at_ymd(2023, Month::May, 1).unwrap();
    /// assert_eq!(date.julian_day_number(), 2460066);
    /// ```
    pub fn julian_day_number(&self) -> Jdnum {
        self.jdn
    }

    /// Returns true if the date is in the Julian calendar (a.k.a. "Old
    /// Style"), i.e., if [`Date::calendar()`] is either a proleptic Julian
    /// calendar or a "reforming" calendar for which the reformation occurs
    /// after the date in question.
    pub fn is_julian(&self) -> bool {
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
    pub fn is_gregorian(&self) -> bool {
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
    /// let gregorian_date = Calendar::gregorian().at_ymd(2023, Month::May, 1).unwrap();
    /// let julian_date = gregorian_date.convert_to(Calendar::julian());
    /// assert_eq!(julian_date.to_string(), "2023-04-18");
    /// ```
    pub fn convert_to(&self, calendar: Calendar) -> Date {
        calendar.at_jdn(self.julian_day_number())
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-", self.year())?;
        if f.alternate() {
            write!(f, "{:03}", self.ordinal())?;
        } else {
            write!(f, "{:02}-{:02}", self.month().number(), self.day())?;
        }
        Ok(())
    }
}

/// An enumeration of the twelve months of "Julian-style" years.
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
    pub fn name(&self) -> &'static str {
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
    pub fn short_name(&self) -> &'static str {
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
    pub fn number(&self) -> u32 {
        *self as u32
    }

    /// Returns the zero-based number of the month, where January is 0.
    pub fn number0(&self) -> u32 {
        self.number() - 1
    }

    /// Returns the month before the month in question.  Returns `None` for
    /// January.
    pub fn pred(&self) -> Option<Month> {
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
    pub fn succ(&self) -> Option<Month> {
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
}

impl fmt::Display for Month {
    /// A `Month` is displayed as its English name by default.  Selecting the
    /// alternate form with `{:#}` instead produces just the first three
    /// letters of the English name.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

/// Error returned when parsing a month fails
#[derive(Clone, Copy, Debug, Default, Error, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[error("invalid month name")]
pub struct ParseMonthError;

macro_rules! impl_try_from {
    ($($t:ty),* $(,)?) => {
      $(
        impl TryFrom<$t> for Month {
            type Error = TryIntoMonthError;

            /// Convert a month number to the corresponding month.
            ///
            /// # Errors
            ///
            /// Returns [`TryIntoMonthError`] if the given number is zero or
            /// greater than twelve.
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

impl_try_from!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

/// Error returned when converting a number to a month fails
#[derive(Clone, Copy, Debug, Default, Error, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[error("value out of range for month number; must be from 1 through 12")]
pub struct TryIntoMonthError;

/// Iterator over the months of the year in order.
///
/// # Example
///
/// ```
/// use julian::{Month, MonthIter};
///
/// let mut iter = MonthIter::new();
/// assert_eq!(iter.next(), Some(Month::January));
/// assert_eq!(iter.next(), Some(Month::February));
/// assert_eq!(iter.next(), Some(Month::March));
/// assert_eq!(iter.next(), Some(Month::April));
/// assert_eq!(iter.next(), Some(Month::May));
/// assert_eq!(iter.next(), Some(Month::June));
/// assert_eq!(iter.next(), Some(Month::July));
/// assert_eq!(iter.next(), Some(Month::August));
/// assert_eq!(iter.next(), Some(Month::September));
/// assert_eq!(iter.next(), Some(Month::October));
/// assert_eq!(iter.next(), Some(Month::November));
/// assert_eq!(iter.next(), Some(Month::December));
/// assert_eq!(iter.next(), None);
/// ```
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct MonthIter(RangeInclusive<u16>);

impl MonthIter {
    /// Construct a new `MonthIter`
    pub fn new() -> MonthIter {
        MonthIter(1..=12)
    }
}

impl Default for MonthIter {
    fn default() -> MonthIter {
        MonthIter::new()
    }
}

impl Iterator for MonthIter {
    type Item = Month;

    fn next(&mut self) -> Option<Month> {
        Some(u32::from(self.0.next()?).try_into().unwrap())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl FusedIterator for MonthIter {}

impl ExactSizeIterator for MonthIter {}

impl DoubleEndedIterator for MonthIter {
    fn next_back(&mut self) -> Option<Month> {
        Some(u32::from(self.0.next_back()?).try_into().unwrap())
    }
}

/// Error returned by [`Calendar::reforming()`] when given an invalid
/// reformation date
#[derive(Copy, Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum ReformingError {
    /// Returned if the reformation date would not cause the calendar to skip
    /// forwards
    #[error("reformation date would not cause calendar to advance")]
    InvalidReformation,

    /// Returned if an internal arithmetic operation encounters numeric
    /// overflow or underflow
    #[error("arithmetic overflow/underflow")]
    Arithmetic,
}

/// Error returned by various date-construction methods on invalid input
#[derive(Copy, Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum DateError {
    /// Returned if an internal arithmetic operation encounters numeric
    /// overflow or underflow
    #[error("arithmetic overflow/underflow")]
    Arithmetic,

    /// Returned by [`Calendar::at_ymd()`] if the given day of month value was
    /// zero or greater than the last day of the given month for the given year
    #[error("day {day} is outside of valid range {min_day}-{max_day} for {year:04} {month}")]
    DayOutOfRange {
        /// The year value supplied
        year: i32,
        /// The month value supplied
        month: Month,
        /// The invalid day of month supplied
        day: u32,
        /// The first valid day of the month
        min_day: u32,
        /// The last valid day of the month
        max_day: u32,
    },

    /// Returned by [`Calendar::at_ordinal_date()`] if the given day of year
    /// value was zero or greater than the length of the given year
    #[error("day-of-year ordinal {ordinal} is outside of valid range 1-{max_ordinal} for year {year:04}")]
    OrdinalOutOfRange {
        /// The year value supplied
        year: i32,
        /// The invalid day of year value supplied
        ordinal: u32,
        /// The maximum valid day of year value
        max_ordinal: u32,
    },

    /// Returned by [`Calendar::at_ymd()`] if the given date was skipped by a
    /// calendar reformation
    #[error("date {year:04}-{:02}-{day:02} was skipped by calendar reform", month.number())]
    SkippedDate { year: i32, month: Month, day: u32 },
}

/// Error returned when an internal arithmetic operation encounters numeric
/// overflow or underflow
#[derive(Clone, Copy, Debug, Default, Error, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[error("arithmetic overflow/underflow")]
pub struct ArithmeticError;

impl From<ArithmeticError> for ReformingError {
    fn from(_: ArithmeticError) -> ReformingError {
        ReformingError::Arithmetic
    }
}

impl From<ArithmeticError> for DateError {
    fn from(_: ArithmeticError) -> DateError {
        DateError::Arithmetic
    }
}

/// Error returned by [`Calendar::parse_date()`] on an invalid input date
/// string
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParseDateError {
    /// Returned if the date specified by the date string does not occur in the
    /// calendar
    #[error("invalid calendar date: {0}")]
    InvalidDate(#[from] DateError),

    /// Returned if the month component of the date string had an invalid
    /// numeric value (i.e., zero or greater than twelve)
    #[error("invalid month number: {value}")]
    InvalidMonth {
        /// The invalid month number
        value: u32,
    },

    /// Returned if the date string had extra trailing characters
    #[error("trailing characters after date")]
    Trailing,

    /// Returned if a non-digit, non-sign character was encountered in the date
    /// string while expecting a signed integer
    #[error("expected signed integer, got {got:?}")]
    InvalidIntStart {
        /// The character encountered
        got: char,
    },

    /// Returned if a non-digit was encountered in the date string while
    /// expecting an unsigned integer
    #[error("expected unsigned integer, got {got:?}")]
    InvalidUIntStart {
        /// The non-digit encountered
        got: char,
    },

    /// Returned if the end of the date string was reached while expecting an
    /// integer
    #[error("expected integer, got end of input")]
    EmptyInt,

    /// Returned if a specific character was expected but a different one was
    /// encountered instead
    #[error("expected {expected:?}, got {got:?}")]
    UnexpectedChar {
        /// The expected character
        expected: char,

        /// The character encountered
        got: char,
    },

    /// Returned if a specific character was expected but the end of the date
    /// string was reached instead
    #[error("expected {expected:?}, got end of input")]
    UnexpectedEnd {
        /// The expected character
        expected: char,
    },

    /// Returned if a numeric component of the date string could not be parsed
    /// as an integer
    #[error("numeric parse error: {0}")]
    ParseInt(#[from] ParseIntError),
}

/// Converts a [`std::time::SystemTime`] instance to the corresponding Julian
/// day number, along with a count of seconds since midnight.  UTC is used as
/// the timezone.
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
/// a count of seconds since midnight.  UTC is used as the timezone.
///
/// [Unix time]: https://en.wikipedia.org/wiki/Unix_time
///
/// # Errors
///
/// Returns [`ArithmeticError`] if numeric overflow/underflow occurs during
/// conversion.  This can only happen if the timestamp is less than
/// -185753453990400 or greater than 185331720383999.
pub fn unix2jdn(unix_time: i64) -> Result<(Jdnum, u32), ArithmeticError> {
    let jd = Jdnum::try_from(unix_time.div_euclid(SECONDS_IN_DAY) + (UNIX_EPOCH_JDN as i64))
        .map_err(|_| ArithmeticError)?;
    let secs = u32::try_from(unix_time.rem_euclid(SECONDS_IN_DAY)).unwrap();
    Ok((jd, secs))
}

/// Converts a Julian day number to the [Unix time][] for midnight UTC on that
/// day.
///
/// [Unix time]: https://en.wikipedia.org/wiki/Unix_time
pub fn jdn2unix(jdn: Jdnum) -> i64 {
    (i64::from(jdn) - (UNIX_EPOCH_JDN as i64)) * SECONDS_IN_DAY
}

#[cfg(test)]
mod tests {
    mod at_ordinal_date;
    mod at_ymd;
    mod autogen;
    mod calendar;
    mod date;
    mod jdn;
    mod month;
    mod parse_date;
    mod reformations;
    mod unix;
    mod year_kind;
}
