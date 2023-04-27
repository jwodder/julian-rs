#[cfg(test)]
extern crate rstest_reuse;

mod inner;
pub mod reformations;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator};
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

pub type YearT = i32;
pub type DaysT = u32;
pub type JulianDayT = i32;

/// The Julian day number of the start of the Unix epoch (1970-01-01)
pub const UNIX_EPOCH_JDN: JulianDayT = 2440588;

const SECONDS_IN_DAY: i64 = 24 * 60 * 60;

const COMMON_YEAR_LENGTH: JulianDayT = 365;
const LEAP_YEAR_LENGTH: JulianDayT = 366;

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
    pub fn is_common(&self) -> bool {
        use YearKind::*;
        matches!(self, Common | ReformCommon)
    }

    pub fn is_leap(&self) -> bool {
        use YearKind::*;
        matches!(self, Leap | ReformLeap)
    }

    pub fn is_reform(&self) -> bool {
        use YearKind::*;
        matches!(self, ReformCommon | ReformLeap)
    }

    pub fn is_skipped(&self) -> bool {
        self == &YearKind::Skipped
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Calendar(inner::Calendar);

impl Calendar {
    pub fn julian() -> Calendar {
        Calendar(inner::Calendar::Julian)
    }

    pub fn gregorian() -> Calendar {
        Calendar(inner::Calendar::Gregorian)
    }

    // Errors on reformation dates that would lead to negative or empty
    // reformation gaps (i.e., any Julian day number less than 1830692, the
    // value of which is available as `julian::reformations::MIN_REFORM_JDN`)
    pub fn reforming(reformation: JulianDayT) -> Result<Calendar, Error> {
        let pre_reform = Calendar::julian().at_julian_day_number(
            reformation
                .checked_sub(1)
                .ok_or(Error::ArithmeticOutOfBounds)?,
        )?;
        let post_reform = Calendar::gregorian().at_julian_day_number(reformation)?;
        let post_reform_as_julian = Calendar::julian()
            .at_ymd(post_reform.year(), post_reform.month(), post_reform.day())?
            .julian_day_number();
        if post_reform_as_julian <= reformation {
            return Err(Error::InvalidReformation);
        }
        let gap_length = u32::try_from(post_reform_as_julian - reformation).unwrap();
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
        let post_ordinal = match kind {
            inner::GapKind::IntraMonth | inner::GapKind::CrossMonth => pre_reform.ordinal + 1,
            _ => 1,
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
                gap_length,
                kind,
            },
        }))
    }

    pub fn gregorian_reform() -> Calendar {
        Self::reforming(reformations::GREGORIAN).unwrap()
    }

    pub fn now(&self) -> Result<(Date, u32), ArithmeticOutOfBounds> {
        self.at_system_time(SystemTime::now())
    }

    pub fn at_system_time(&self, t: SystemTime) -> Result<(Date, u32), ArithmeticOutOfBounds> {
        let (jdn, secs) = system_time_to_julian_day_number(t)?;
        Ok((self.at_julian_day_number(jdn)?, secs))
    }

    pub fn at_unix_time(&self, unix_time: i64) -> Result<(Date, u32), ArithmeticOutOfBounds> {
        let (jdn, secs) = unix_time_to_julian_day_number(unix_time)?;
        Ok((self.at_julian_day_number(jdn)?, secs))
    }

    pub fn at_ymd(&self, year: YearT, month: Month, day: u32) -> Result<Date, Error> {
        let ordinal = self.ymd2ordinal(year, month, day)?;
        let jdn = self.get_julian_day_number(year, ordinal, month, day)?;
        let day_ordinal = self.month_shape(year, month).get_day_ordinal(day).unwrap();
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

    pub fn at_ordinal_date(&self, year: YearT, ordinal: DaysT) -> Result<Date, Error> {
        let (month, day) = self.ordinal2ymd(year, ordinal)?;
        let jdn = self.get_julian_day_number(year, ordinal, month, day)?;
        let day_ordinal = self.month_shape(year, month).get_day_ordinal(day).unwrap();
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

    pub fn at_julian_day_number(&self, jdn: JulianDayT) -> Result<Date, ArithmeticOutOfBounds> {
        use inner::Calendar::*;
        let (year, ordinal, month, day);
        if self.0 == Julian || matches!(self.0, Reforming { reformation, .. } if jdn < reformation)
        {
            (year, ordinal) = inner::jd_to_julian_yj(jdn).ok_or(ArithmeticOutOfBounds)?;
            (month, day) = self.ordinal2ymd(year, ordinal).unwrap();
        } else {
            (year, month, day) = inner::jd_to_gregorian_ymd(jdn).ok_or(ArithmeticOutOfBounds)?;
            ordinal = self.ymd2ordinal(year, month, day).unwrap();
        }
        let day_ordinal = self.month_shape(year, month).get_day_ordinal(day).unwrap();
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

    // Formats (fields may be shorter than the given number of digits):
    // - [+/-]YYYY-MM-DD
    // - [+/-]YYYY-DDD
    pub fn parse_date(&self, s: &str) -> Result<Date, ParseDateError> {
        let mut parser = inner::DateParser::new(s);
        let year = parser.parse_year()?;
        let diny = parser.parse_day_in_year()?;
        if !parser.is_empty() {
            return Err(ParseDateError::HasTrailing);
        }
        match diny {
            inner::DayInYear::Ordinal(ordinal) => Ok(self.at_ordinal_date(year, ordinal)?),
            inner::DayInYear::Date { month, day } => Ok(self.at_ymd(year, month, day)?),
        }
    }

    pub fn is_julian(&self) -> bool {
        self.0 == inner::Calendar::Julian
    }

    pub fn is_gregorian(&self) -> bool {
        self.0 == inner::Calendar::Gregorian
    }

    pub fn is_reforming(&self) -> bool {
        matches!(self.0, inner::Calendar::Reforming { .. })
    }

    pub fn is_proleptic(&self) -> bool {
        self.is_julian() || self.is_gregorian()
    }

    pub fn reformation(&self) -> Option<JulianDayT> {
        if let inner::Calendar::Reforming { reformation, .. } = self.0 {
            Some(reformation)
        } else {
            None
        }
    }

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

    // Number of dates *as reckoned in the Julian calendar* (i.e., counting any
    // Julian-only leap days that may have been skipped) between the last
    // Julian date and the first Gregorian date
    pub fn reformation_gap(&self) -> Option<JulianDayT> {
        if let inner::Calendar::Reforming { gap, .. } = self.0 {
            Some(JulianDayT::try_from(gap.gap_length).unwrap())
        } else {
            None
        }
    }

    pub fn year_kind(&self, year: YearT) -> YearKind {
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

    pub fn year_length(&self, year: YearT) -> u32 {
        match self.0 {
            inner::Calendar::Julian | inner::Calendar::Gregorian => match self.year_kind(year) {
                YearKind::Common => COMMON_YEAR_LENGTH as u32,
                YearKind::Leap => LEAP_YEAR_LENGTH as u32,
                _ => unreachable!(),
            },
            inner::Calendar::Reforming { gap, .. } => {
                match self.year_kind(year) {
                    YearKind::Common => COMMON_YEAR_LENGTH as u32,
                    YearKind::Leap => LEAP_YEAR_LENGTH as u32,
                    k @ (YearKind::ReformCommon | YearKind::ReformLeap) => {
                        let mut length = if k == YearKind::ReformCommon {
                            COMMON_YEAR_LENGTH as u32
                        } else {
                            LEAP_YEAR_LENGTH as u32
                        };
                        use inner::GapKind::*;
                        match gap.kind {
                            IntraMonth | CrossMonth => {
                                length -= gap.gap_length;
                                // If the gap crossed a leap day, the year kind
                                // will be ReformCommon, but an extra day will
                                // be included in gap_length, so we need to
                                // add back 1.
                                if gap.cmp_ymd(year, Month::February, 29)
                                    == inner::RangeOrdering::Between
                                {
                                    length += 1;
                                }
                            }
                            CrossYear | MultiYear => {
                                if year == gap.pre_reform.year {
                                    length = gap.pre_reform.ordinal + 1;
                                } else {
                                    debug_assert!(year == gap.post_reform.year);
                                    length -= gap.gap_length
                                        - (self.year_length(year - 1) - gap.pre_reform.ordinal - 1);
                                }
                            }
                        }
                        length
                    }
                    YearKind::Skipped => 0,
                }
            }
        }
    }

    pub fn month_length(&self, year: YearT, month: Month) -> u32 {
        self.month_shape(year, month).len()
    }

    // This uses a one-based ordinal.
    fn ordinal2ymd(&self, year: YearT, ordinal: u32) -> Result<(Month, u32), Error> {
        if ordinal == 0 {
            return Err(Error::OrdinalOutOfRange { year, ordinal });
        }
        let mut days = ordinal;
        for month in MonthIter::new() {
            let shape = self.month_shape(year, month);
            if let Some(day) = shape.get_day_label(days) {
                return Ok((month, day));
            }
            days -= shape.len();
        }
        Err(Error::OrdinalOutOfRange { year, ordinal })
    }

    // This uses a one-based ordinal.
    fn ymd2ordinal(&self, year: YearT, month: Month, day: u32) -> Result<u32, Error> {
        let mord = self.get_day_ordinal(year, month, day)?;
        Ok(MonthIter::new()
            .take_while(|&m| m < month)
            .map(|m| self.month_length(year, m))
            .sum::<u32>()
            + mord)
    }

    fn get_day_ordinal(&self, year: YearT, month: Month, day: u32) -> Result<u32, Error> {
        self.month_shape(year, month).get_day_ordinal(day)
    }

    fn month_shape(&self, year: YearT, month: Month) -> inner::MonthShape {
        use Month::*;
        let length = match month {
            January => 31,
            February => {
                if self.year_kind(year).is_common() {
                    28
                } else {
                    29
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
        if let inner::Calendar::Reforming { gap, .. } = self.0 {
            use inner::RangeOrdering::*;
            return match gap.cmp_year_month(year, month) {
                EqLower | EqBoth => {
                    if gap.kind == inner::GapKind::IntraMonth {
                        inner::MonthShape::HasGap {
                            year,
                            month,
                            gap: (gap.pre_reform.day + 1)..(gap.post_reform.day),
                            max_day: length,
                        }
                    } else {
                        inner::MonthShape::Solid {
                            year,
                            month,
                            range: 1..=(gap.pre_reform.day),
                        }
                    }
                }
                Between => inner::MonthShape::Skipped { year, month },
                EqUpper => inner::MonthShape::Solid {
                    year,
                    month,
                    range: (gap.post_reform.day)..=length,
                },
                _ => inner::MonthShape::Solid {
                    year,
                    month,
                    range: 1..=length,
                },
            };
        }
        inner::MonthShape::Solid {
            year,
            month,
            range: 1..=length,
        }
    }

    fn get_julian_day_number(
        &self,
        year: YearT,
        ordinal: DaysT,
        month: Month,
        day: u32,
    ) -> Result<JulianDayT, ArithmeticOutOfBounds> {
        use inner::Calendar::*;
        if self.0 == Julian
            || matches!(self.0, Reforming {gap, ..} if (year, ordinal) < (gap.post_reform.year, gap.post_reform.ordinal))
        {
            inner::julian_yj_to_jd(year, ordinal)
        } else {
            inner::gregorian_ymd_to_jd(year, month, day)
        }.ok_or(ArithmeticOutOfBounds)
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Date {
    calendar: Calendar,
    year: YearT,    // 0 == 1 BC
    ordinal: DaysT, // ordinal day of year; 1 == Jan 1
    month: Month,
    day: u32,         // one-based
    day_ordinal: u32, // one-based
    jdn: JulianDayT,
}

impl Date {
    pub fn calendar(&self) -> Calendar {
        self.calendar
    }

    pub fn year(&self) -> YearT {
        self.year
    }

    pub fn month(&self) -> Month {
        self.month
    }

    // The "display" day (one-indexed, counting skipped days)
    pub fn day(&self) -> u32 {
        self.day
    }

    // Returns the index of the day within the month, starting from one, not
    // counting days skipped due to reformation
    pub fn day_ordinal(&self) -> u32 {
        self.day_ordinal
    }

    pub fn day_ordinal0(&self) -> u32 {
        self.day_ordinal - 1
    }

    pub fn ordinal(&self) -> DaysT {
        self.ordinal
    }

    pub fn ordinal0(&self) -> DaysT {
        self.ordinal - 1
    }

    pub fn julian_day_number(&self) -> JulianDayT {
        self.jdn
    }

    // a.k.a. "is Old Style"
    pub fn is_julian(&self) -> bool {
        match self.calendar.0 {
            inner::Calendar::Julian => true,
            inner::Calendar::Reforming { reformation, .. } => {
                self.julian_day_number() < reformation
            }
            inner::Calendar::Gregorian => false,
        }
    }

    // a.k.a. "is New Style"
    pub fn is_gregorian(&self) -> bool {
        match self.calendar.0 {
            inner::Calendar::Julian => false,
            inner::Calendar::Reforming { reformation, .. } => {
                reformation <= self.julian_day_number()
            }
            inner::Calendar::Gregorian => true,
        }
    }

    pub fn convert_to(&self, calendar: Calendar) -> Result<Date, ArithmeticOutOfBounds> {
        calendar.at_julian_day_number(self.julian_day_number())
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Date) -> Ordering {
        (self.julian_day_number(), self.calendar())
            .cmp(&(other.julian_day_number(), other.calendar()))
    }
}

impl fmt::Display for Date {
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

    pub fn number(&self) -> u32 {
        *self as u32
    }

    pub fn number0(&self) -> u32 {
        self.number() - 1
    }

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

#[derive(Clone, Copy, Debug, Default, Error, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[error("invalid month name")]
pub struct ParseMonthError;

impl TryFrom<u32> for Month {
    type Error = TryIntoMonthError;

    fn try_from(value: u32) -> Result<Month, TryIntoMonthError> {
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

#[derive(Clone, Copy, Debug, Default, Error, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[error("value out of range for month number; must be from 1 through 12")]
pub struct TryIntoMonthError;

pub struct MonthIter(RangeInclusive<u16>);

impl MonthIter {
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

#[derive(Copy, Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
    #[error("reformation date would not cause calendar to advance")]
    InvalidReformation,
    #[error("arithmetic overflow/underflow")]
    ArithmeticOutOfBounds,
    #[error("day {day} is outside of valid range for {month} {year}")]
    DayOutOfRange { year: YearT, month: Month, day: u32 },
    #[error("day-of-year ordinal {ordinal} is outside of valid range for year {year}")]
    OrdinalOutOfRange { year: YearT, ordinal: DaysT },
    #[error("date {year:04}-{:02}-{day:02} was skipped by calendar reform", month.number())]
    SkippedDate { year: YearT, month: Month, day: u32 },
}

#[derive(Clone, Copy, Debug, Default, Error, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[error("arithmetic overflow/underflow")]
pub struct ArithmeticOutOfBounds;

impl From<ArithmeticOutOfBounds> for Error {
    fn from(_: ArithmeticOutOfBounds) -> Error {
        Error::ArithmeticOutOfBounds
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParseDateError {
    #[error("invalid calendar date: {0}")]
    InvalidDate(#[from] Error),
    #[error("invalid month number: {value}")]
    InvalidMonth { value: u32 },
    #[error("trailing characters after date")]
    HasTrailing,
    #[error("year not terminated by '-'")]
    UnterminatedYear,
    #[error("expected one or more digits, got non-digit {got:?}")]
    InvalidIntStart { got: char },
    #[error("expected one or more digits, reached end of input")]
    EmptyInt,
    #[error("expected {expected:?}, got {got:?}")]
    UnexpectedChar { expected: char, got: char },
    #[error("expected {expected:?}, reached end of input")]
    UnexpectedEnd { expected: char },
    #[error("invalid calendar date: numeric parse error: {0}")]
    ParseInt(#[from] ParseIntError),
}

// Seconds count from midnight, as that's when JDN numbers change
pub fn system_time_to_julian_day_number(
    t: SystemTime,
) -> Result<(JulianDayT, u32), ArithmeticOutOfBounds> {
    let ts = match t.duration_since(UNIX_EPOCH) {
        Ok(d) => i64::try_from(d.as_secs()),
        Err(e) => i64::try_from(e.duration().as_secs()).map(|i| -i),
    }
    .map_err(|_| ArithmeticOutOfBounds)?;
    unix_time_to_julian_day_number(ts)
}

// Seconds count from midnight, as that's when JDN numbers change
pub fn unix_time_to_julian_day_number(
    unix_time: i64,
) -> Result<(JulianDayT, u32), ArithmeticOutOfBounds> {
    let jd = JulianDayT::try_from(unix_time.div_euclid(SECONDS_IN_DAY) + (UNIX_EPOCH_JDN as i64))
        .map_err(|_| ArithmeticOutOfBounds)?;
    let secs = u32::try_from(unix_time.rem_euclid(SECONDS_IN_DAY)).unwrap();
    Ok((jd, secs))
}

#[cfg(test)]
mod tests {
    mod at_ordinal_date;
    mod at_ymd;
    mod date;
    mod gregorian_reform;
    mod jdn;
    mod misc_reforms;
    mod month;
    mod parse_date;
    mod unix;
}
