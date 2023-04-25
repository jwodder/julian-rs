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

const SECONDS_IN_DAY: i64 = 24 * 60 * 60;

const COMMON_YEAR_LENGTH: JulianDayT = 365;
const LEAP_YEAR_LENGTH: JulianDayT = 366;

const UNIX_EPOCH_JD: JulianDayT = 2440588; // noon on 1970-01-01

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

    /// A year that was skipped entirely by a calendar reformation
    // TODO: Give the smallest such reformation date
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
    // reformation gaps
    pub fn reforming(reformation: JulianDayT) -> Result<Calendar, Error> {
        let pre_reform = Calendar::julian().at_julian_day(
            reformation
                .checked_sub(1)
                .ok_or(Error::ArithmeticOutOfBounds)?,
        )?;
        let post_reform = Calendar::gregorian().at_julian_day(reformation)?;
        let post_reform_as_julian = Calendar::julian()
            .at_ymd(post_reform.year(), post_reform.month(), post_reform.mday())?
            .julian_day();
        if post_reform_as_julian <= reformation {
            return Err(Error::BadReformation);
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
            mday: pre_reform.mday,
        };
        let post_ordinal = match kind {
            inner::GapKind::IntraMonth | inner::GapKind::CrossMonth => pre_reform.ordinal + 1,
            _ => 0,
        };
        let post_reform = inner::Date {
            year: post_reform.year,
            ordinal: post_ordinal,
            month: post_reform.month,
            mday: post_reform.mday,
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

    pub fn now(&self) -> Result<(Date, u32), Error> {
        self.at_system_time(SystemTime::now())
    }

    pub fn at_system_time(&self, t: SystemTime) -> Result<(Date, u32), Error> {
        let ts = match t.duration_since(UNIX_EPOCH) {
            Ok(d) => i64::try_from(d.as_secs()).map_err(|_| Error::ArithmeticOutOfBounds)?,
            Err(e) => {
                -i64::try_from(e.duration().as_secs()).map_err(|_| Error::ArithmeticOutOfBounds)?
            }
        };
        self.at_unix_timestamp(ts)
    }

    pub fn at_unix_timestamp(&self, unix_time: i64) -> Result<(Date, u32), Error> {
        let jd = JulianDayT::try_from(unix_time / SECONDS_IN_DAY + (UNIX_EPOCH_JD as i64))
            .map_err(|_| Error::ArithmeticOutOfBounds)?;
        let secs = u32::try_from(unix_time % SECONDS_IN_DAY).unwrap();
        Ok((self.at_julian_day(jd)?, secs))
    }

    // `mday` is one-indexed
    pub fn at_ymd(&self, year: YearT, month: Month, mday: u32) -> Result<Date, Error> {
        let ordinal = self.ymd2ordinal(year, month, mday)?;
        let julian_day = self.get_julian_day(year, ordinal, month, mday);
        let mday_ordinal = self
            .month_shape(year, month)
            .get_mday_ordinal(mday)
            .unwrap();
        Ok(Date {
            calendar: *self,
            year,
            ordinal,
            month,
            mday,
            mday_ordinal,
            julian_day,
        })
    }

    // TODO: Rethink name
    pub fn at_year_ordinal(&self, year: YearT, ordinal: DaysT) -> Result<Date, Error> {
        let (month, mday) = self.ordinal2ymd(year, ordinal)?;
        let julian_day = self.get_julian_day(year, ordinal, month, mday);
        let mday_ordinal = self
            .month_shape(year, month)
            .get_mday_ordinal(mday)
            .unwrap();
        Ok(Date {
            calendar: *self,
            year,
            ordinal,
            month,
            mday,
            mday_ordinal,
            julian_day,
        })
    }

    pub fn at_julian_day(&self, julian_day: JulianDayT) -> Result<Date, Error> {
        match self.0 {
            inner::Calendar::Julian => {
                let (year, ordinal) = inner::jd_to_julian_yj(julian_day);
                let (month, mday) = self.ordinal2ymd(year, ordinal)?;
                let mday_ordinal = self
                    .month_shape(year, month)
                    .get_mday_ordinal(mday)
                    .unwrap();
                Ok(Date {
                    calendar: *self,
                    year,
                    ordinal,
                    month,
                    mday,
                    mday_ordinal,
                    julian_day,
                })
            }
            inner::Calendar::Reforming { reformation, .. } if julian_day < reformation => {
                let (year, ordinal) = inner::jd_to_julian_yj(julian_day);
                let (month, mday) = self.ordinal2ymd(year, ordinal)?;
                let mday_ordinal = self
                    .month_shape(year, month)
                    .get_mday_ordinal(mday)
                    .unwrap();
                Ok(Date {
                    calendar: *self,
                    year,
                    ordinal,
                    month,
                    mday,
                    mday_ordinal,
                    julian_day,
                })
            }
            _ => {
                let (year, month, mday) = inner::jd_to_gregorian_ymd(julian_day);
                let ordinal = self.ymd2ordinal(year, month, mday)?;
                let mday_ordinal = self
                    .month_shape(year, month)
                    .get_mday_ordinal(mday)
                    .unwrap();
                Ok(Date {
                    calendar: *self,
                    year,
                    ordinal,
                    month,
                    mday,
                    mday_ordinal,
                    julian_day,
                })
            }
        }
    }

    // Formats:
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
            inner::DayInYear::Ordinal(ordinal) => Ok(self.at_year_ordinal(year, ordinal)?),
            inner::DayInYear::Date { month, mday } => Ok(self.at_ymd(year, month, mday)?),
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
                mday: gap.pre_reform.mday,
                mday_ordinal: gap.pre_reform.mday,
                julian_day: reformation - 1,
            })
        } else {
            None
        }
    }

    pub fn first_gregorian_date(&self) -> Option<Date> {
        if let inner::Calendar::Reforming { reformation, gap } = self.0 {
            let mday_ordinal = if gap.kind == inner::GapKind::IntraMonth {
                gap.pre_reform.mday + 1
            } else {
                1
            };
            Some(Date {
                calendar: *self,
                year: gap.post_reform.year,
                ordinal: gap.post_reform.ordinal,
                month: gap.post_reform.month,
                mday: gap.post_reform.mday,
                mday_ordinal,
                julian_day: reformation,
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
                match (
                    gap.pre_reform.year.cmp(&year),
                    year.cmp(&gap.post_reform.year),
                ) {
                    (Ordering::Greater, _) => {
                        if inner::is_julian_leap_year(year) {
                            YearKind::Leap
                        } else {
                            YearKind::Common
                        }
                    }
                    (Ordering::Less, Ordering::Less) => YearKind::Skipped,
                    (Ordering::Equal, Ordering::Equal) => {
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
                    (Ordering::Equal, _) => {
                        if Month::February < gap.pre_reform.month
                            && inner::is_julian_leap_year(year)
                        {
                            YearKind::ReformLeap
                        } else {
                            YearKind::ReformCommon
                        }
                    }
                    (_, Ordering::Equal) => {
                        if gap.post_reform.month <= Month::February
                            && inner::is_gregorian_leap_year(year)
                        {
                            YearKind::ReformLeap
                        } else {
                            YearKind::ReformCommon
                        }
                    }
                    (_, Ordering::Greater) => {
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
                                if (gap.pre_reform.month, gap.pre_reform.mday)
                                    < (Month::February, 29)
                                    && (Month::February, 29)
                                        < (gap.post_reform.month, gap.post_reform.mday)
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
            if let Some(mday) = shape.get_mday_label(days) {
                return Ok((month, mday));
            }
            days -= shape.len();
        }
        Err(Error::OrdinalOutOfRange { year, ordinal })
    }

    // This uses a one-based ordinal.
    fn ymd2ordinal(&self, year: YearT, month: Month, mday: u32) -> Result<u32, Error> {
        let mord = self.get_mday_ordinal(year, month, mday)?;
        Ok(MonthIter::new()
            .take_while(|&m| m < month)
            .map(|m| self.month_length(year, m))
            .sum::<u32>()
            + mord)
    }

    fn get_mday_ordinal(&self, year: YearT, month: Month, mday: u32) -> Result<u32, Error> {
        self.month_shape(year, month).get_mday_ordinal(mday)
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
            if (year, month) == (gap.pre_reform.year, gap.pre_reform.month) {
                if gap.kind == inner::GapKind::IntraMonth {
                    return inner::MonthShape::HasGap {
                        year,
                        month,
                        gap: (gap.pre_reform.mday + 1)..(gap.post_reform.mday),
                        max_mday: length,
                    };
                } else {
                    return inner::MonthShape::Solid {
                        year,
                        month,
                        range: 1..=(gap.pre_reform.mday),
                    };
                }
            } else if (gap.pre_reform.year, gap.pre_reform.month) < (year, month)
                && (year, month) < (gap.post_reform.year, gap.post_reform.month)
            {
                return inner::MonthShape::Skipped { year, month };
            } else if (year, month) == (gap.post_reform.year, gap.post_reform.month) {
                return inner::MonthShape::Solid {
                    year,
                    month,
                    range: (gap.post_reform.mday)..=length,
                };
            }
        }
        inner::MonthShape::Solid {
            year,
            month,
            range: 1..=length,
        }
    }

    fn get_julian_day(&self, year: YearT, ordinal: DaysT, month: Month, mday: u32) -> JulianDayT {
        match self.0 {
            inner::Calendar::Julian => inner::julian_yj_to_jd(year, ordinal),
            inner::Calendar::Gregorian => inner::gregorian_ymd_to_jd(year, month, mday),
            inner::Calendar::Reforming { gap, .. } => {
                if (year, ordinal) < (gap.post_reform.year, gap.post_reform.ordinal) {
                    inner::julian_yj_to_jd(year, ordinal)
                } else {
                    inner::gregorian_ymd_to_jd(year, month, mday)
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Date {
    calendar: Calendar,
    year: YearT,    // 0 == 1 BC
    ordinal: DaysT, // ordinal day of year; 1 == Jan 1
    month: Month,
    mday: u32,         // one-based
    mday_ordinal: u32, // one-based
    julian_day: JulianDayT,
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

    // The "display" mday (one-indexed, counting skipped days)
    // TODO: Name this "day()" to match chrono?
    pub fn mday(&self) -> u32 {
        self.mday
    }

    // Returns the index of the day within the month, starting from one, not
    // counting days skipped due to reformation
    pub fn mday_ordinal(&self) -> u32 {
        self.mday_ordinal
    }

    pub fn ordinal(&self) -> DaysT {
        self.ordinal
    }

    pub fn ordinal0(&self) -> DaysT {
        self.ordinal - 1
    }

    pub fn julian_day(&self) -> JulianDayT {
        self.julian_day
    }

    // a.k.a. "is Old Style"
    pub fn is_julian(&self) -> bool {
        match self.calendar.0 {
            inner::Calendar::Julian => true,
            inner::Calendar::Reforming { reformation, .. } => self.julian_day() < reformation,
            inner::Calendar::Gregorian => false,
        }
    }

    // a.k.a. "is New Style"
    pub fn is_gregorian(&self) -> bool {
        match self.calendar.0 {
            inner::Calendar::Julian => false,
            inner::Calendar::Reforming { reformation, .. } => reformation <= self.julian_day(),
            inner::Calendar::Gregorian => true,
        }
    }

    pub fn convert_to(&self, calendar: Calendar) -> Result<Date, Error> {
        calendar.at_julian_day(self.julian_day())
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Date) -> Ordering {
        (self.julian_day(), self.calendar()).cmp(&(other.julian_day(), other.calendar()))
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-", self.year())?;
        if f.alternate() {
            write!(f, "{:03}", self.ordinal())?;
        } else {
            write!(f, "{:02}-{:02}", self.month().number(), self.mday())?;
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
        write!(f, "{}", self.name())
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

#[derive(Copy, Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    #[error("reformation date would not cause calendar to advance")]
    BadReformation,
    #[error("arithmetic overflow/underflow")]
    ArithmeticOutOfBounds,
    #[error("mday {mday} is outside of valid range for {month} {year}")]
    MdayOutOfRange {
        year: YearT,
        month: Month,
        mday: u32,
    },
    #[error("day-of-year ordinal {ordinal} is outside of valid range for year {year}")]
    OrdinalOutOfRange { year: YearT, ordinal: DaysT },
    #[error("date {year:04}-{:02}-{mday:02} was skipped by calendar reform", month.number())]
    SkippedDate {
        year: YearT,
        month: Month,
        mday: u32,
    },
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
    #[error("ordinal must be three digits long")]
    InvalidOrdinal,
    #[error("expected two digits, got {got} digits")]
    Invalid02dLength { got: usize },
    #[error("expected two digits, got non-digit {got:?}")]
    Invalid02dStart { got: char },
    #[error("expected two digits, reached end of input")]
    Invalid02dSuddenEnd,
    #[error("expected {expected:?}, got {got:?}")]
    UnexpectedChar { expected: char, got: char },
    #[error("expected {expected:?}, reached end of input")]
    UnexpectedEnd { expected: char },
    #[error("invalid calendar date: numeric parse error: {0}")]
    ParseInt(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use rstest::rstest;
    use rstest_reuse::{apply, template};

    #[template]
    #[rstest]
    #[case(-2298701, -11006, Month::July, 2)]
    #[case(-1403, -4716, Month::February, 28)]
    #[case(-1402, -4716, Month::February, 29)]
    #[case(-1401, -4716, Month::March, 1)]
    #[case(-1095, -4715, Month::January, 1)]
    #[case(-730, -4714, Month::January, 1)]
    #[case(-365, -4713, Month::January, 1)]
    #[case(-1, -4713, Month::December, 31)]
    #[case(0, -4712, Month::January, 1)]
    #[case(366, -4711, Month::January, 1)]
    #[case(1719656, -4, Month::February, 29)]
    #[case(2159358, 1200, Month::January, 1)]
    #[case(2195883, 1300, Month::January, 1)]
    #[case(2232408, 1400, Month::January, 1)]
    #[case(2268932, 1499, Month::December, 31)]
    #[case(2268933, 1500, Month::January, 1)]
    #[case(2268993, 1500, Month::March, 1)]
    #[case(2269115, 1500, Month::July, 1)]
    #[case(2269146, 1500, Month::August, 1)]
    #[case(2269177, 1500, Month::September, 1)]
    #[case(2269204, 1500, Month::September, 28)]
    #[case(2269205, 1500, Month::September, 29)]
    #[case(2269207, 1500, Month::October, 1)]
    #[case(2269298, 1500, Month::December, 31)]
    #[case(2269299, 1501, Month::January, 1)]
    #[case(2269663, 1501, Month::December, 31)]
    #[case(2270123, 1503, Month::April, 5)]
    #[case(2270489, 1504, Month::April, 5)]
    #[case(2270854, 1505, Month::April, 5)]
    #[case(2271219, 1506, Month::April, 5)]
    #[case(2271584, 1507, Month::April, 5)]
    #[case(2298796, 1581, Month::October, 5)]
    #[case(2298883, 1581, Month::December, 31)]
    #[case(2298884, 1582, Month::January, 1)]
    #[case(2299159, 1582, Month::October, 3)]
    #[case(2299160, 1582, Month::October, 4)]
    #[case(2299161, 1582, Month::October, 15)]
    #[case(2299162, 1582, Month::October, 16)]
    #[case(2299238, 1582, Month::December, 31)]
    #[case(2299239, 1583, Month::January, 1)]
    #[case(2305448, 1600, Month::January, 1)]
    #[case(2341972, 1699, Month::December, 31)]
    #[case(2341973, 1700, Month::January, 1)]
    #[case(2342337, 1700, Month::December, 31)]
    #[case(2342338, 1701, Month::January, 1)]
    #[case(2378496, 1799, Month::December, 31)]
    #[case(2378497, 1800, Month::January, 1)]
    #[case(2378861, 1800, Month::December, 31)]
    #[case(2378862, 1801, Month::January, 1)]
    #[case(2415020, 1899, Month::December, 31)]
    #[case(2415021, 1900, Month::January, 1)]
    #[case(2415385, 1900, Month::December, 31)]
    #[case(2415386, 1901, Month::January, 1)]
    #[case(2440588, 1970, Month::January, 1)]
    #[case(2451544, 1999, Month::December, 31)]
    #[case(2451545, 2000, Month::January, 1)]
    #[case(2451605, 2000, Month::March, 1)]
    #[case(2451910, 2000, Month::December, 31)]
    #[case(2451911, 2001, Month::January, 1)]
    #[case(2453066, 2004, Month::March, 1)]
    #[case(2456746, 2014, Month::March, 29)]
    #[case(2460055, 2023, Month::April, 20)]
    fn julian_days(
        #[case] days: JulianDayT,
        #[case] year: YearT,
        #[case] month: Month,
        #[case] mday: u32,
    ) {
    }

    #[apply(julian_days)]
    fn test_julian_day_to_gregorian(
        #[case] days: JulianDayT,
        #[case] year: YearT,
        #[case] month: Month,
        #[case] mday: u32,
    ) {
        let date = Calendar::gregorian_reform().at_julian_day(days).unwrap();
        assert_eq!(date.year(), year);
        assert_eq!(date.month(), month);
        assert_eq!(date.mday(), mday);
    }

    #[apply(julian_days)]
    fn test_calendar_to_julian(
        #[case] days: JulianDayT,
        #[case] year: YearT,
        #[case] month: Month,
        #[case] mday: u32,
    ) {
        eprintln!("year = {year}, month = {month}, mday = {mday}, days = {days}");
        let date = Calendar::gregorian_reform()
            .at_ymd(year, month, mday)
            .unwrap();
        assert_eq!(date.julian_day(), days);
    }

    mod parse_date {
        use super::*;

        #[test]
        fn test_parse_ymd() {
            let date = Calendar::gregorian_reform()
                .parse_date("2023-04-20")
                .unwrap();
            assert_eq!(date.year(), 2023);
            assert_eq!(date.month(), Month::April);
            assert_eq!(date.mday(), 20);
            assert_eq!(date.ordinal(), 110);
        }

        #[test]
        fn test_parse_yj() {
            let date = Calendar::gregorian_reform().parse_date("2023-110").unwrap();
            assert_eq!(date.year(), 2023);
            assert_eq!(date.month(), Month::April);
            assert_eq!(date.mday(), 20);
            assert_eq!(date.ordinal(), 110);
        }

        #[test]
        fn test_parse_yj_padded() {
            let date = Calendar::gregorian_reform().parse_date("2023-006").unwrap();
            assert_eq!(date.year(), 2023);
            assert_eq!(date.month(), Month::January);
            assert_eq!(date.mday(), 6);
            assert_eq!(date.ordinal(), 6);
        }

        #[test]
        fn test_parse_negative_ymd() {
            let date = Calendar::gregorian_reform()
                .parse_date("-2023-04-20")
                .unwrap();
            assert_eq!(date.year(), -2023);
            assert_eq!(date.month(), Month::April);
            assert_eq!(date.mday(), 20);
            assert_eq!(date.ordinal(), 110);
        }

        #[test]
        fn test_parse_ymd_short_year() {
            // TODO: Should this be an error instead?
            let date = Calendar::gregorian_reform().parse_date("20-04-20").unwrap();
            assert_eq!(date.year(), 20);
            assert_eq!(date.month(), Month::April);
            assert_eq!(date.mday(), 20);
            assert_eq!(date.ordinal(), 111);
        }

        #[test]
        fn test_parse_date_short_ordinal() {
            let r = Calendar::gregorian_reform().parse_date("1234-56");
            assert_eq!(r, Err(ParseDateError::InvalidOrdinal));
            assert_eq!(
                r.unwrap_err().to_string(),
                "ordinal must be three digits long"
            );
        }

        #[test]
        fn test_parse_date_long_ordinal() {
            let r = Calendar::gregorian_reform().parse_date("1234-5678");
            assert_eq!(r, Err(ParseDateError::InvalidOrdinal));
            assert_eq!(
                r.unwrap_err().to_string(),
                "ordinal must be three digits long"
            );
        }

        #[test]
        fn test_parse_ymd_short_month() {
            let r = Calendar::gregorian_reform().parse_date("2023-4-20");
            assert_eq!(r, Err(ParseDateError::Invalid02dLength { got: 1 }));
            assert_eq!(
                r.unwrap_err().to_string(),
                "expected two digits, got 1 digits"
            );
        }

        #[test]
        fn test_parse_ymd_long_month() {
            let r = Calendar::gregorian_reform().parse_date("2023-012-20");
            assert_eq!(r, Err(ParseDateError::Invalid02dLength { got: 3 }));
            assert_eq!(
                r.unwrap_err().to_string(),
                "expected two digits, got 3 digits"
            );
        }

        #[test]
        fn test_parse_ymd_short_mday() {
            let r = Calendar::gregorian_reform().parse_date("2023-04-2");
            assert_eq!(r, Err(ParseDateError::Invalid02dLength { got: 1 }));
            assert_eq!(
                r.unwrap_err().to_string(),
                "expected two digits, got 1 digits"
            );
        }

        #[test]
        fn test_parse_year_hyphen() {
            let r = Calendar::gregorian_reform().parse_date("2023-");
            assert_eq!(r, Err(ParseDateError::InvalidOrdinal));
            assert_eq!(
                r.unwrap_err().to_string(),
                "ordinal must be three digits long"
            );
        }

        #[test]
        fn test_parse_year_month_hyphen() {
            let r = Calendar::gregorian_reform().parse_date("2023-04-");
            assert_eq!(r, Err(ParseDateError::Invalid02dSuddenEnd));
            assert_eq!(
                r.unwrap_err().to_string(),
                "expected two digits, reached end of input"
            );
        }

        #[test]
        fn test_parse_year_hyphen_hyphen_md() {
            let r = Calendar::gregorian_reform().parse_date("2023--04-20");
            assert_eq!(r, Err(ParseDateError::Invalid02dStart { got: '-' }));
            assert_eq!(
                r.unwrap_err().to_string(),
                "expected two digits, got non-digit '-'"
            );
        }

        #[test]
        fn test_parse_zero_month() {
            let r = Calendar::gregorian_reform().parse_date("2023-00-13");
            assert_eq!(r, Err(ParseDateError::InvalidMonth { value: 0 }));
            assert_eq!(r.unwrap_err().to_string(), "invalid month number: 0");
        }

        #[test]
        fn test_parse_smarch() {
            let r = Calendar::gregorian_reform().parse_date("2023-13-13");
            assert_eq!(r, Err(ParseDateError::InvalidMonth { value: 13 }));
            assert_eq!(r.unwrap_err().to_string(), "invalid month number: 13");
        }

        #[test]
        fn test_parse_mday_0() {
            let r = Calendar::gregorian_reform().parse_date("2023-04-00");
            assert_eq!(
                r,
                Err(ParseDateError::InvalidDate(Error::MdayOutOfRange {
                    year: 2023,
                    month: Month::April,
                    mday: 0
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: mday 0 is outside of valid range for April 2023"
            );
        }

        #[test]
        fn test_parse_mday_32() {
            let r = Calendar::gregorian_reform().parse_date("2023-04-32");
            assert_eq!(
                r,
                Err(ParseDateError::InvalidDate(Error::MdayOutOfRange {
                    year: 2023,
                    month: Month::April,
                    mday: 32
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: mday 32 is outside of valid range for April 2023"
            );
        }

        #[test]
        fn test_parse_sep_31() {
            let r = Calendar::gregorian_reform().parse_date("2023-09-31");
            assert_eq!(
                r,
                Err(ParseDateError::InvalidDate(Error::MdayOutOfRange {
                    year: 2023,
                    month: Month::September,
                    mday: 31
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: mday 31 is outside of valid range for September 2023"
            );
        }

        #[test]
        fn test_parse_invalid_leap_day() {
            let r = Calendar::gregorian_reform().parse_date("2023-02-29");
            assert_eq!(
                r,
                Err(ParseDateError::InvalidDate(Error::MdayOutOfRange {
                    year: 2023,
                    month: Month::February,
                    mday: 29
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: mday 29 is outside of valid range for February 2023"
            );
        }

        #[test]
        fn test_parse_valid_leap_day() {
            let date = Calendar::gregorian_reform()
                .parse_date("2024-02-29")
                .unwrap();
            assert_eq!(date.year(), 2024);
            assert_eq!(date.month(), Month::February);
            assert_eq!(date.mday(), 29);
        }

        #[test]
        fn test_parse_skipped_date() {
            let r = Calendar::gregorian_reform().parse_date("1582-10-10");
            assert_eq!(
                r,
                Err(ParseDateError::InvalidDate(Error::SkippedDate {
                    year: 1582,
                    month: Month::October,
                    mday: 10
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: date 1582-10-10 was skipped by calendar reform"
            );
        }

        #[test]
        fn test_parse_first_skipped_date() {
            let r = Calendar::gregorian_reform().parse_date("1582-10-05");
            assert_eq!(
                r,
                Err(ParseDateError::InvalidDate(Error::SkippedDate {
                    year: 1582,
                    month: Month::October,
                    mday: 5
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: date 1582-10-05 was skipped by calendar reform"
            );
        }

        #[test]
        fn test_parse_last_skipped_date() {
            let r = Calendar::gregorian_reform().parse_date("1582-10-14");
            assert_eq!(
                r,
                Err(ParseDateError::InvalidDate(Error::SkippedDate {
                    year: 1582,
                    month: Month::October,
                    mday: 14
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: date 1582-10-14 was skipped by calendar reform"
            );
        }

        #[rstest]
        #[case(2023, 366)]
        #[case(2023, 999)]
        #[case(2024, 367)]
        #[case(2024, 999)]
        #[case(1582, 356)]
        #[case(1582, 999)]
        fn test_parse_invalid_yj(#[case] year: YearT, #[case] ordinal: DaysT) {
            let r = Calendar::gregorian_reform().parse_date(&format!("{year:04}-{ordinal:03}"));
            assert_eq!(
                r,
                Err(ParseDateError::InvalidDate(Error::OrdinalOutOfRange {
                    year,
                    ordinal
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                format!("invalid calendar date: day-of-year ordinal {ordinal} is outside of valid range for year {year}")
            );
        }

        #[test]
        fn test_parse_bad_month_mday_sep() {
            // TODO: Try to make this return a more helpful error
            let r = Calendar::gregorian_reform().parse_date("2023-04:20");
            assert_eq!(r, Err(ParseDateError::InvalidOrdinal));
            assert_eq!(
                r.unwrap_err().to_string(),
                "ordinal must be three digits long"
            );
        }

        #[test]
        fn test_parse_just_year() {
            let r = Calendar::gregorian_reform().parse_date("2023");
            assert_eq!(r, Err(ParseDateError::UnterminatedYear));
            assert_eq!(r.unwrap_err().to_string(), "year not terminated by '-'");
        }

        #[test]
        fn test_parse_nonint_year() {
            use std::num::IntErrorKind::InvalidDigit;
            let r = Calendar::gregorian_reform().parse_date("202e-04-20");
            assert_matches!(r, Err(ParseDateError::ParseInt(ref e)) if e.kind() == &InvalidDigit);
            assert!(r
                .unwrap_err()
                .to_string()
                .starts_with("invalid calendar date: numeric parse error: "));
        }

        #[test]
        fn test_parse_empty() {
            let r = Calendar::gregorian_reform().parse_date("");
            assert_eq!(r, Err(ParseDateError::UnterminatedYear));
            assert_eq!(r.unwrap_err().to_string(), "year not terminated by '-'");
        }
    }

    #[rstest]
    #[case(2023, 1, Month::January, 1)]
    #[case(2023, 2, Month::January, 2)]
    #[case(2023, 3, Month::January, 3)]
    #[case(2023, 4, Month::January, 4)]
    #[case(2023, 5, Month::January, 5)]
    #[case(2023, 6, Month::January, 6)]
    #[case(2023, 7, Month::January, 7)]
    #[case(2023, 8, Month::January, 8)]
    #[case(2023, 9, Month::January, 9)]
    #[case(2023, 10, Month::January, 10)]
    #[case(2023, 11, Month::January, 11)]
    #[case(2023, 12, Month::January, 12)]
    #[case(2023, 13, Month::January, 13)]
    #[case(2023, 14, Month::January, 14)]
    #[case(2023, 15, Month::January, 15)]
    #[case(2023, 16, Month::January, 16)]
    #[case(2023, 17, Month::January, 17)]
    #[case(2023, 18, Month::January, 18)]
    #[case(2023, 19, Month::January, 19)]
    #[case(2023, 20, Month::January, 20)]
    #[case(2023, 21, Month::January, 21)]
    #[case(2023, 22, Month::January, 22)]
    #[case(2023, 23, Month::January, 23)]
    #[case(2023, 24, Month::January, 24)]
    #[case(2023, 25, Month::January, 25)]
    #[case(2023, 26, Month::January, 26)]
    #[case(2023, 27, Month::January, 27)]
    #[case(2023, 28, Month::January, 28)]
    #[case(2023, 29, Month::January, 29)]
    #[case(2023, 30, Month::January, 30)]
    #[case(2023, 31, Month::January, 31)]
    #[case(2023, 32, Month::February, 1)]
    #[case(2023, 33, Month::February, 2)]
    #[case(2023, 34, Month::February, 3)]
    #[case(2023, 35, Month::February, 4)]
    #[case(2023, 36, Month::February, 5)]
    #[case(2023, 37, Month::February, 6)]
    #[case(2023, 38, Month::February, 7)]
    #[case(2023, 39, Month::February, 8)]
    #[case(2023, 40, Month::February, 9)]
    #[case(2023, 41, Month::February, 10)]
    #[case(2023, 42, Month::February, 11)]
    #[case(2023, 43, Month::February, 12)]
    #[case(2023, 44, Month::February, 13)]
    #[case(2023, 45, Month::February, 14)]
    #[case(2023, 46, Month::February, 15)]
    #[case(2023, 47, Month::February, 16)]
    #[case(2023, 48, Month::February, 17)]
    #[case(2023, 49, Month::February, 18)]
    #[case(2023, 50, Month::February, 19)]
    #[case(2023, 51, Month::February, 20)]
    #[case(2023, 52, Month::February, 21)]
    #[case(2023, 53, Month::February, 22)]
    #[case(2023, 54, Month::February, 23)]
    #[case(2023, 55, Month::February, 24)]
    #[case(2023, 56, Month::February, 25)]
    #[case(2023, 57, Month::February, 26)]
    #[case(2023, 58, Month::February, 27)]
    #[case(2023, 59, Month::February, 28)]
    #[case(2023, 60, Month::March, 1)]
    #[case(2023, 61, Month::March, 2)]
    #[case(2023, 62, Month::March, 3)]
    #[case(2023, 63, Month::March, 4)]
    #[case(2023, 64, Month::March, 5)]
    #[case(2023, 65, Month::March, 6)]
    #[case(2023, 66, Month::March, 7)]
    #[case(2023, 67, Month::March, 8)]
    #[case(2023, 68, Month::March, 9)]
    #[case(2023, 69, Month::March, 10)]
    #[case(2023, 70, Month::March, 11)]
    #[case(2023, 71, Month::March, 12)]
    #[case(2023, 72, Month::March, 13)]
    #[case(2023, 73, Month::March, 14)]
    #[case(2023, 74, Month::March, 15)]
    #[case(2023, 75, Month::March, 16)]
    #[case(2023, 76, Month::March, 17)]
    #[case(2023, 77, Month::March, 18)]
    #[case(2023, 78, Month::March, 19)]
    #[case(2023, 79, Month::March, 20)]
    #[case(2023, 80, Month::March, 21)]
    #[case(2023, 81, Month::March, 22)]
    #[case(2023, 82, Month::March, 23)]
    #[case(2023, 83, Month::March, 24)]
    #[case(2023, 84, Month::March, 25)]
    #[case(2023, 85, Month::March, 26)]
    #[case(2023, 86, Month::March, 27)]
    #[case(2023, 87, Month::March, 28)]
    #[case(2023, 88, Month::March, 29)]
    #[case(2023, 89, Month::March, 30)]
    #[case(2023, 90, Month::March, 31)]
    #[case(2023, 91, Month::April, 1)]
    #[case(2023, 92, Month::April, 2)]
    #[case(2023, 93, Month::April, 3)]
    #[case(2023, 94, Month::April, 4)]
    #[case(2023, 95, Month::April, 5)]
    #[case(2023, 96, Month::April, 6)]
    #[case(2023, 97, Month::April, 7)]
    #[case(2023, 98, Month::April, 8)]
    #[case(2023, 99, Month::April, 9)]
    #[case(2023, 100, Month::April, 10)]
    #[case(2023, 101, Month::April, 11)]
    #[case(2023, 102, Month::April, 12)]
    #[case(2023, 103, Month::April, 13)]
    #[case(2023, 104, Month::April, 14)]
    #[case(2023, 105, Month::April, 15)]
    #[case(2023, 106, Month::April, 16)]
    #[case(2023, 107, Month::April, 17)]
    #[case(2023, 108, Month::April, 18)]
    #[case(2023, 109, Month::April, 19)]
    #[case(2023, 110, Month::April, 20)]
    #[case(2023, 111, Month::April, 21)]
    #[case(2023, 112, Month::April, 22)]
    #[case(2023, 113, Month::April, 23)]
    #[case(2023, 114, Month::April, 24)]
    #[case(2023, 115, Month::April, 25)]
    #[case(2023, 116, Month::April, 26)]
    #[case(2023, 117, Month::April, 27)]
    #[case(2023, 118, Month::April, 28)]
    #[case(2023, 119, Month::April, 29)]
    #[case(2023, 120, Month::April, 30)]
    #[case(2023, 121, Month::May, 1)]
    #[case(2023, 122, Month::May, 2)]
    #[case(2023, 123, Month::May, 3)]
    #[case(2023, 124, Month::May, 4)]
    #[case(2023, 125, Month::May, 5)]
    #[case(2023, 126, Month::May, 6)]
    #[case(2023, 127, Month::May, 7)]
    #[case(2023, 128, Month::May, 8)]
    #[case(2023, 129, Month::May, 9)]
    #[case(2023, 130, Month::May, 10)]
    #[case(2023, 131, Month::May, 11)]
    #[case(2023, 132, Month::May, 12)]
    #[case(2023, 133, Month::May, 13)]
    #[case(2023, 134, Month::May, 14)]
    #[case(2023, 135, Month::May, 15)]
    #[case(2023, 136, Month::May, 16)]
    #[case(2023, 137, Month::May, 17)]
    #[case(2023, 138, Month::May, 18)]
    #[case(2023, 139, Month::May, 19)]
    #[case(2023, 140, Month::May, 20)]
    #[case(2023, 141, Month::May, 21)]
    #[case(2023, 142, Month::May, 22)]
    #[case(2023, 143, Month::May, 23)]
    #[case(2023, 144, Month::May, 24)]
    #[case(2023, 145, Month::May, 25)]
    #[case(2023, 146, Month::May, 26)]
    #[case(2023, 147, Month::May, 27)]
    #[case(2023, 148, Month::May, 28)]
    #[case(2023, 149, Month::May, 29)]
    #[case(2023, 150, Month::May, 30)]
    #[case(2023, 151, Month::May, 31)]
    #[case(2023, 152, Month::June, 1)]
    #[case(2023, 153, Month::June, 2)]
    #[case(2023, 154, Month::June, 3)]
    #[case(2023, 155, Month::June, 4)]
    #[case(2023, 156, Month::June, 5)]
    #[case(2023, 157, Month::June, 6)]
    #[case(2023, 158, Month::June, 7)]
    #[case(2023, 159, Month::June, 8)]
    #[case(2023, 160, Month::June, 9)]
    #[case(2023, 161, Month::June, 10)]
    #[case(2023, 162, Month::June, 11)]
    #[case(2023, 163, Month::June, 12)]
    #[case(2023, 164, Month::June, 13)]
    #[case(2023, 165, Month::June, 14)]
    #[case(2023, 166, Month::June, 15)]
    #[case(2023, 167, Month::June, 16)]
    #[case(2023, 168, Month::June, 17)]
    #[case(2023, 169, Month::June, 18)]
    #[case(2023, 170, Month::June, 19)]
    #[case(2023, 171, Month::June, 20)]
    #[case(2023, 172, Month::June, 21)]
    #[case(2023, 173, Month::June, 22)]
    #[case(2023, 174, Month::June, 23)]
    #[case(2023, 175, Month::June, 24)]
    #[case(2023, 176, Month::June, 25)]
    #[case(2023, 177, Month::June, 26)]
    #[case(2023, 178, Month::June, 27)]
    #[case(2023, 179, Month::June, 28)]
    #[case(2023, 180, Month::June, 29)]
    #[case(2023, 181, Month::June, 30)]
    #[case(2023, 182, Month::July, 1)]
    #[case(2023, 183, Month::July, 2)]
    #[case(2023, 184, Month::July, 3)]
    #[case(2023, 185, Month::July, 4)]
    #[case(2023, 186, Month::July, 5)]
    #[case(2023, 187, Month::July, 6)]
    #[case(2023, 188, Month::July, 7)]
    #[case(2023, 189, Month::July, 8)]
    #[case(2023, 190, Month::July, 9)]
    #[case(2023, 191, Month::July, 10)]
    #[case(2023, 192, Month::July, 11)]
    #[case(2023, 193, Month::July, 12)]
    #[case(2023, 194, Month::July, 13)]
    #[case(2023, 195, Month::July, 14)]
    #[case(2023, 196, Month::July, 15)]
    #[case(2023, 197, Month::July, 16)]
    #[case(2023, 198, Month::July, 17)]
    #[case(2023, 199, Month::July, 18)]
    #[case(2023, 200, Month::July, 19)]
    #[case(2023, 201, Month::July, 20)]
    #[case(2023, 202, Month::July, 21)]
    #[case(2023, 203, Month::July, 22)]
    #[case(2023, 204, Month::July, 23)]
    #[case(2023, 205, Month::July, 24)]
    #[case(2023, 206, Month::July, 25)]
    #[case(2023, 207, Month::July, 26)]
    #[case(2023, 208, Month::July, 27)]
    #[case(2023, 209, Month::July, 28)]
    #[case(2023, 210, Month::July, 29)]
    #[case(2023, 211, Month::July, 30)]
    #[case(2023, 212, Month::July, 31)]
    #[case(2023, 213, Month::August, 1)]
    #[case(2023, 214, Month::August, 2)]
    #[case(2023, 215, Month::August, 3)]
    #[case(2023, 216, Month::August, 4)]
    #[case(2023, 217, Month::August, 5)]
    #[case(2023, 218, Month::August, 6)]
    #[case(2023, 219, Month::August, 7)]
    #[case(2023, 220, Month::August, 8)]
    #[case(2023, 221, Month::August, 9)]
    #[case(2023, 222, Month::August, 10)]
    #[case(2023, 223, Month::August, 11)]
    #[case(2023, 224, Month::August, 12)]
    #[case(2023, 225, Month::August, 13)]
    #[case(2023, 226, Month::August, 14)]
    #[case(2023, 227, Month::August, 15)]
    #[case(2023, 228, Month::August, 16)]
    #[case(2023, 229, Month::August, 17)]
    #[case(2023, 230, Month::August, 18)]
    #[case(2023, 231, Month::August, 19)]
    #[case(2023, 232, Month::August, 20)]
    #[case(2023, 233, Month::August, 21)]
    #[case(2023, 234, Month::August, 22)]
    #[case(2023, 235, Month::August, 23)]
    #[case(2023, 236, Month::August, 24)]
    #[case(2023, 237, Month::August, 25)]
    #[case(2023, 238, Month::August, 26)]
    #[case(2023, 239, Month::August, 27)]
    #[case(2023, 240, Month::August, 28)]
    #[case(2023, 241, Month::August, 29)]
    #[case(2023, 242, Month::August, 30)]
    #[case(2023, 243, Month::August, 31)]
    #[case(2023, 244, Month::September, 1)]
    #[case(2023, 245, Month::September, 2)]
    #[case(2023, 246, Month::September, 3)]
    #[case(2023, 247, Month::September, 4)]
    #[case(2023, 248, Month::September, 5)]
    #[case(2023, 249, Month::September, 6)]
    #[case(2023, 250, Month::September, 7)]
    #[case(2023, 251, Month::September, 8)]
    #[case(2023, 252, Month::September, 9)]
    #[case(2023, 253, Month::September, 10)]
    #[case(2023, 254, Month::September, 11)]
    #[case(2023, 255, Month::September, 12)]
    #[case(2023, 256, Month::September, 13)]
    #[case(2023, 257, Month::September, 14)]
    #[case(2023, 258, Month::September, 15)]
    #[case(2023, 259, Month::September, 16)]
    #[case(2023, 260, Month::September, 17)]
    #[case(2023, 261, Month::September, 18)]
    #[case(2023, 262, Month::September, 19)]
    #[case(2023, 263, Month::September, 20)]
    #[case(2023, 264, Month::September, 21)]
    #[case(2023, 265, Month::September, 22)]
    #[case(2023, 266, Month::September, 23)]
    #[case(2023, 267, Month::September, 24)]
    #[case(2023, 268, Month::September, 25)]
    #[case(2023, 269, Month::September, 26)]
    #[case(2023, 270, Month::September, 27)]
    #[case(2023, 271, Month::September, 28)]
    #[case(2023, 272, Month::September, 29)]
    #[case(2023, 273, Month::September, 30)]
    #[case(2023, 274, Month::October, 1)]
    #[case(2023, 275, Month::October, 2)]
    #[case(2023, 276, Month::October, 3)]
    #[case(2023, 277, Month::October, 4)]
    #[case(2023, 278, Month::October, 5)]
    #[case(2023, 279, Month::October, 6)]
    #[case(2023, 280, Month::October, 7)]
    #[case(2023, 281, Month::October, 8)]
    #[case(2023, 282, Month::October, 9)]
    #[case(2023, 283, Month::October, 10)]
    #[case(2023, 284, Month::October, 11)]
    #[case(2023, 285, Month::October, 12)]
    #[case(2023, 286, Month::October, 13)]
    #[case(2023, 287, Month::October, 14)]
    #[case(2023, 288, Month::October, 15)]
    #[case(2023, 289, Month::October, 16)]
    #[case(2023, 290, Month::October, 17)]
    #[case(2023, 291, Month::October, 18)]
    #[case(2023, 292, Month::October, 19)]
    #[case(2023, 293, Month::October, 20)]
    #[case(2023, 294, Month::October, 21)]
    #[case(2023, 295, Month::October, 22)]
    #[case(2023, 296, Month::October, 23)]
    #[case(2023, 297, Month::October, 24)]
    #[case(2023, 298, Month::October, 25)]
    #[case(2023, 299, Month::October, 26)]
    #[case(2023, 300, Month::October, 27)]
    #[case(2023, 301, Month::October, 28)]
    #[case(2023, 302, Month::October, 29)]
    #[case(2023, 303, Month::October, 30)]
    #[case(2023, 304, Month::October, 31)]
    #[case(2023, 305, Month::November, 1)]
    #[case(2023, 306, Month::November, 2)]
    #[case(2023, 307, Month::November, 3)]
    #[case(2023, 308, Month::November, 4)]
    #[case(2023, 309, Month::November, 5)]
    #[case(2023, 310, Month::November, 6)]
    #[case(2023, 311, Month::November, 7)]
    #[case(2023, 312, Month::November, 8)]
    #[case(2023, 313, Month::November, 9)]
    #[case(2023, 314, Month::November, 10)]
    #[case(2023, 315, Month::November, 11)]
    #[case(2023, 316, Month::November, 12)]
    #[case(2023, 317, Month::November, 13)]
    #[case(2023, 318, Month::November, 14)]
    #[case(2023, 319, Month::November, 15)]
    #[case(2023, 320, Month::November, 16)]
    #[case(2023, 321, Month::November, 17)]
    #[case(2023, 322, Month::November, 18)]
    #[case(2023, 323, Month::November, 19)]
    #[case(2023, 324, Month::November, 20)]
    #[case(2023, 325, Month::November, 21)]
    #[case(2023, 326, Month::November, 22)]
    #[case(2023, 327, Month::November, 23)]
    #[case(2023, 328, Month::November, 24)]
    #[case(2023, 329, Month::November, 25)]
    #[case(2023, 330, Month::November, 26)]
    #[case(2023, 331, Month::November, 27)]
    #[case(2023, 332, Month::November, 28)]
    #[case(2023, 333, Month::November, 29)]
    #[case(2023, 334, Month::November, 30)]
    #[case(2023, 335, Month::December, 1)]
    #[case(2023, 336, Month::December, 2)]
    #[case(2023, 337, Month::December, 3)]
    #[case(2023, 338, Month::December, 4)]
    #[case(2023, 339, Month::December, 5)]
    #[case(2023, 340, Month::December, 6)]
    #[case(2023, 341, Month::December, 7)]
    #[case(2023, 342, Month::December, 8)]
    #[case(2023, 343, Month::December, 9)]
    #[case(2023, 344, Month::December, 10)]
    #[case(2023, 345, Month::December, 11)]
    #[case(2023, 346, Month::December, 12)]
    #[case(2023, 347, Month::December, 13)]
    #[case(2023, 348, Month::December, 14)]
    #[case(2023, 349, Month::December, 15)]
    #[case(2023, 350, Month::December, 16)]
    #[case(2023, 351, Month::December, 17)]
    #[case(2023, 352, Month::December, 18)]
    #[case(2023, 353, Month::December, 19)]
    #[case(2023, 354, Month::December, 20)]
    #[case(2023, 355, Month::December, 21)]
    #[case(2023, 356, Month::December, 22)]
    #[case(2023, 357, Month::December, 23)]
    #[case(2023, 358, Month::December, 24)]
    #[case(2023, 359, Month::December, 25)]
    #[case(2023, 360, Month::December, 26)]
    #[case(2023, 361, Month::December, 27)]
    #[case(2023, 362, Month::December, 28)]
    #[case(2023, 363, Month::December, 29)]
    #[case(2023, 364, Month::December, 30)]
    #[case(2023, 365, Month::December, 31)]
    #[case(2024, 1, Month::January, 1)]
    #[case(2024, 2, Month::January, 2)]
    #[case(2024, 3, Month::January, 3)]
    #[case(2024, 4, Month::January, 4)]
    #[case(2024, 5, Month::January, 5)]
    #[case(2024, 6, Month::January, 6)]
    #[case(2024, 7, Month::January, 7)]
    #[case(2024, 8, Month::January, 8)]
    #[case(2024, 9, Month::January, 9)]
    #[case(2024, 10, Month::January, 10)]
    #[case(2024, 11, Month::January, 11)]
    #[case(2024, 12, Month::January, 12)]
    #[case(2024, 13, Month::January, 13)]
    #[case(2024, 14, Month::January, 14)]
    #[case(2024, 15, Month::January, 15)]
    #[case(2024, 16, Month::January, 16)]
    #[case(2024, 17, Month::January, 17)]
    #[case(2024, 18, Month::January, 18)]
    #[case(2024, 19, Month::January, 19)]
    #[case(2024, 20, Month::January, 20)]
    #[case(2024, 21, Month::January, 21)]
    #[case(2024, 22, Month::January, 22)]
    #[case(2024, 23, Month::January, 23)]
    #[case(2024, 24, Month::January, 24)]
    #[case(2024, 25, Month::January, 25)]
    #[case(2024, 26, Month::January, 26)]
    #[case(2024, 27, Month::January, 27)]
    #[case(2024, 28, Month::January, 28)]
    #[case(2024, 29, Month::January, 29)]
    #[case(2024, 30, Month::January, 30)]
    #[case(2024, 31, Month::January, 31)]
    #[case(2024, 32, Month::February, 1)]
    #[case(2024, 33, Month::February, 2)]
    #[case(2024, 34, Month::February, 3)]
    #[case(2024, 35, Month::February, 4)]
    #[case(2024, 36, Month::February, 5)]
    #[case(2024, 37, Month::February, 6)]
    #[case(2024, 38, Month::February, 7)]
    #[case(2024, 39, Month::February, 8)]
    #[case(2024, 40, Month::February, 9)]
    #[case(2024, 41, Month::February, 10)]
    #[case(2024, 42, Month::February, 11)]
    #[case(2024, 43, Month::February, 12)]
    #[case(2024, 44, Month::February, 13)]
    #[case(2024, 45, Month::February, 14)]
    #[case(2024, 46, Month::February, 15)]
    #[case(2024, 47, Month::February, 16)]
    #[case(2024, 48, Month::February, 17)]
    #[case(2024, 49, Month::February, 18)]
    #[case(2024, 50, Month::February, 19)]
    #[case(2024, 51, Month::February, 20)]
    #[case(2024, 52, Month::February, 21)]
    #[case(2024, 53, Month::February, 22)]
    #[case(2024, 54, Month::February, 23)]
    #[case(2024, 55, Month::February, 24)]
    #[case(2024, 56, Month::February, 25)]
    #[case(2024, 57, Month::February, 26)]
    #[case(2024, 58, Month::February, 27)]
    #[case(2024, 59, Month::February, 28)]
    #[case(2024, 60, Month::February, 29)]
    #[case(2024, 61, Month::March, 1)]
    #[case(2024, 62, Month::March, 2)]
    #[case(2024, 63, Month::March, 3)]
    #[case(2024, 64, Month::March, 4)]
    #[case(2024, 65, Month::March, 5)]
    #[case(2024, 66, Month::March, 6)]
    #[case(2024, 67, Month::March, 7)]
    #[case(2024, 68, Month::March, 8)]
    #[case(2024, 69, Month::March, 9)]
    #[case(2024, 70, Month::March, 10)]
    #[case(2024, 71, Month::March, 11)]
    #[case(2024, 72, Month::March, 12)]
    #[case(2024, 73, Month::March, 13)]
    #[case(2024, 74, Month::March, 14)]
    #[case(2024, 75, Month::March, 15)]
    #[case(2024, 76, Month::March, 16)]
    #[case(2024, 77, Month::March, 17)]
    #[case(2024, 78, Month::March, 18)]
    #[case(2024, 79, Month::March, 19)]
    #[case(2024, 80, Month::March, 20)]
    #[case(2024, 81, Month::March, 21)]
    #[case(2024, 82, Month::March, 22)]
    #[case(2024, 83, Month::March, 23)]
    #[case(2024, 84, Month::March, 24)]
    #[case(2024, 85, Month::March, 25)]
    #[case(2024, 86, Month::March, 26)]
    #[case(2024, 87, Month::March, 27)]
    #[case(2024, 88, Month::March, 28)]
    #[case(2024, 89, Month::March, 29)]
    #[case(2024, 90, Month::March, 30)]
    #[case(2024, 91, Month::March, 31)]
    #[case(2024, 92, Month::April, 1)]
    #[case(2024, 93, Month::April, 2)]
    #[case(2024, 94, Month::April, 3)]
    #[case(2024, 95, Month::April, 4)]
    #[case(2024, 96, Month::April, 5)]
    #[case(2024, 97, Month::April, 6)]
    #[case(2024, 98, Month::April, 7)]
    #[case(2024, 99, Month::April, 8)]
    #[case(2024, 100, Month::April, 9)]
    #[case(2024, 101, Month::April, 10)]
    #[case(2024, 102, Month::April, 11)]
    #[case(2024, 103, Month::April, 12)]
    #[case(2024, 104, Month::April, 13)]
    #[case(2024, 105, Month::April, 14)]
    #[case(2024, 106, Month::April, 15)]
    #[case(2024, 107, Month::April, 16)]
    #[case(2024, 108, Month::April, 17)]
    #[case(2024, 109, Month::April, 18)]
    #[case(2024, 110, Month::April, 19)]
    #[case(2024, 111, Month::April, 20)]
    #[case(2024, 112, Month::April, 21)]
    #[case(2024, 113, Month::April, 22)]
    #[case(2024, 114, Month::April, 23)]
    #[case(2024, 115, Month::April, 24)]
    #[case(2024, 116, Month::April, 25)]
    #[case(2024, 117, Month::April, 26)]
    #[case(2024, 118, Month::April, 27)]
    #[case(2024, 119, Month::April, 28)]
    #[case(2024, 120, Month::April, 29)]
    #[case(2024, 121, Month::April, 30)]
    #[case(2024, 122, Month::May, 1)]
    #[case(2024, 123, Month::May, 2)]
    #[case(2024, 124, Month::May, 3)]
    #[case(2024, 125, Month::May, 4)]
    #[case(2024, 126, Month::May, 5)]
    #[case(2024, 127, Month::May, 6)]
    #[case(2024, 128, Month::May, 7)]
    #[case(2024, 129, Month::May, 8)]
    #[case(2024, 130, Month::May, 9)]
    #[case(2024, 131, Month::May, 10)]
    #[case(2024, 132, Month::May, 11)]
    #[case(2024, 133, Month::May, 12)]
    #[case(2024, 134, Month::May, 13)]
    #[case(2024, 135, Month::May, 14)]
    #[case(2024, 136, Month::May, 15)]
    #[case(2024, 137, Month::May, 16)]
    #[case(2024, 138, Month::May, 17)]
    #[case(2024, 139, Month::May, 18)]
    #[case(2024, 140, Month::May, 19)]
    #[case(2024, 141, Month::May, 20)]
    #[case(2024, 142, Month::May, 21)]
    #[case(2024, 143, Month::May, 22)]
    #[case(2024, 144, Month::May, 23)]
    #[case(2024, 145, Month::May, 24)]
    #[case(2024, 146, Month::May, 25)]
    #[case(2024, 147, Month::May, 26)]
    #[case(2024, 148, Month::May, 27)]
    #[case(2024, 149, Month::May, 28)]
    #[case(2024, 150, Month::May, 29)]
    #[case(2024, 151, Month::May, 30)]
    #[case(2024, 152, Month::May, 31)]
    #[case(2024, 153, Month::June, 1)]
    #[case(2024, 154, Month::June, 2)]
    #[case(2024, 155, Month::June, 3)]
    #[case(2024, 156, Month::June, 4)]
    #[case(2024, 157, Month::June, 5)]
    #[case(2024, 158, Month::June, 6)]
    #[case(2024, 159, Month::June, 7)]
    #[case(2024, 160, Month::June, 8)]
    #[case(2024, 161, Month::June, 9)]
    #[case(2024, 162, Month::June, 10)]
    #[case(2024, 163, Month::June, 11)]
    #[case(2024, 164, Month::June, 12)]
    #[case(2024, 165, Month::June, 13)]
    #[case(2024, 166, Month::June, 14)]
    #[case(2024, 167, Month::June, 15)]
    #[case(2024, 168, Month::June, 16)]
    #[case(2024, 169, Month::June, 17)]
    #[case(2024, 170, Month::June, 18)]
    #[case(2024, 171, Month::June, 19)]
    #[case(2024, 172, Month::June, 20)]
    #[case(2024, 173, Month::June, 21)]
    #[case(2024, 174, Month::June, 22)]
    #[case(2024, 175, Month::June, 23)]
    #[case(2024, 176, Month::June, 24)]
    #[case(2024, 177, Month::June, 25)]
    #[case(2024, 178, Month::June, 26)]
    #[case(2024, 179, Month::June, 27)]
    #[case(2024, 180, Month::June, 28)]
    #[case(2024, 181, Month::June, 29)]
    #[case(2024, 182, Month::June, 30)]
    #[case(2024, 183, Month::July, 1)]
    #[case(2024, 184, Month::July, 2)]
    #[case(2024, 185, Month::July, 3)]
    #[case(2024, 186, Month::July, 4)]
    #[case(2024, 187, Month::July, 5)]
    #[case(2024, 188, Month::July, 6)]
    #[case(2024, 189, Month::July, 7)]
    #[case(2024, 190, Month::July, 8)]
    #[case(2024, 191, Month::July, 9)]
    #[case(2024, 192, Month::July, 10)]
    #[case(2024, 193, Month::July, 11)]
    #[case(2024, 194, Month::July, 12)]
    #[case(2024, 195, Month::July, 13)]
    #[case(2024, 196, Month::July, 14)]
    #[case(2024, 197, Month::July, 15)]
    #[case(2024, 198, Month::July, 16)]
    #[case(2024, 199, Month::July, 17)]
    #[case(2024, 200, Month::July, 18)]
    #[case(2024, 201, Month::July, 19)]
    #[case(2024, 202, Month::July, 20)]
    #[case(2024, 203, Month::July, 21)]
    #[case(2024, 204, Month::July, 22)]
    #[case(2024, 205, Month::July, 23)]
    #[case(2024, 206, Month::July, 24)]
    #[case(2024, 207, Month::July, 25)]
    #[case(2024, 208, Month::July, 26)]
    #[case(2024, 209, Month::July, 27)]
    #[case(2024, 210, Month::July, 28)]
    #[case(2024, 211, Month::July, 29)]
    #[case(2024, 212, Month::July, 30)]
    #[case(2024, 213, Month::July, 31)]
    #[case(2024, 214, Month::August, 1)]
    #[case(2024, 215, Month::August, 2)]
    #[case(2024, 216, Month::August, 3)]
    #[case(2024, 217, Month::August, 4)]
    #[case(2024, 218, Month::August, 5)]
    #[case(2024, 219, Month::August, 6)]
    #[case(2024, 220, Month::August, 7)]
    #[case(2024, 221, Month::August, 8)]
    #[case(2024, 222, Month::August, 9)]
    #[case(2024, 223, Month::August, 10)]
    #[case(2024, 224, Month::August, 11)]
    #[case(2024, 225, Month::August, 12)]
    #[case(2024, 226, Month::August, 13)]
    #[case(2024, 227, Month::August, 14)]
    #[case(2024, 228, Month::August, 15)]
    #[case(2024, 229, Month::August, 16)]
    #[case(2024, 230, Month::August, 17)]
    #[case(2024, 231, Month::August, 18)]
    #[case(2024, 232, Month::August, 19)]
    #[case(2024, 233, Month::August, 20)]
    #[case(2024, 234, Month::August, 21)]
    #[case(2024, 235, Month::August, 22)]
    #[case(2024, 236, Month::August, 23)]
    #[case(2024, 237, Month::August, 24)]
    #[case(2024, 238, Month::August, 25)]
    #[case(2024, 239, Month::August, 26)]
    #[case(2024, 240, Month::August, 27)]
    #[case(2024, 241, Month::August, 28)]
    #[case(2024, 242, Month::August, 29)]
    #[case(2024, 243, Month::August, 30)]
    #[case(2024, 244, Month::August, 31)]
    #[case(2024, 245, Month::September, 1)]
    #[case(2024, 246, Month::September, 2)]
    #[case(2024, 247, Month::September, 3)]
    #[case(2024, 248, Month::September, 4)]
    #[case(2024, 249, Month::September, 5)]
    #[case(2024, 250, Month::September, 6)]
    #[case(2024, 251, Month::September, 7)]
    #[case(2024, 252, Month::September, 8)]
    #[case(2024, 253, Month::September, 9)]
    #[case(2024, 254, Month::September, 10)]
    #[case(2024, 255, Month::September, 11)]
    #[case(2024, 256, Month::September, 12)]
    #[case(2024, 257, Month::September, 13)]
    #[case(2024, 258, Month::September, 14)]
    #[case(2024, 259, Month::September, 15)]
    #[case(2024, 260, Month::September, 16)]
    #[case(2024, 261, Month::September, 17)]
    #[case(2024, 262, Month::September, 18)]
    #[case(2024, 263, Month::September, 19)]
    #[case(2024, 264, Month::September, 20)]
    #[case(2024, 265, Month::September, 21)]
    #[case(2024, 266, Month::September, 22)]
    #[case(2024, 267, Month::September, 23)]
    #[case(2024, 268, Month::September, 24)]
    #[case(2024, 269, Month::September, 25)]
    #[case(2024, 270, Month::September, 26)]
    #[case(2024, 271, Month::September, 27)]
    #[case(2024, 272, Month::September, 28)]
    #[case(2024, 273, Month::September, 29)]
    #[case(2024, 274, Month::September, 30)]
    #[case(2024, 275, Month::October, 1)]
    #[case(2024, 276, Month::October, 2)]
    #[case(2024, 277, Month::October, 3)]
    #[case(2024, 278, Month::October, 4)]
    #[case(2024, 279, Month::October, 5)]
    #[case(2024, 280, Month::October, 6)]
    #[case(2024, 281, Month::October, 7)]
    #[case(2024, 282, Month::October, 8)]
    #[case(2024, 283, Month::October, 9)]
    #[case(2024, 284, Month::October, 10)]
    #[case(2024, 285, Month::October, 11)]
    #[case(2024, 286, Month::October, 12)]
    #[case(2024, 287, Month::October, 13)]
    #[case(2024, 288, Month::October, 14)]
    #[case(2024, 289, Month::October, 15)]
    #[case(2024, 290, Month::October, 16)]
    #[case(2024, 291, Month::October, 17)]
    #[case(2024, 292, Month::October, 18)]
    #[case(2024, 293, Month::October, 19)]
    #[case(2024, 294, Month::October, 20)]
    #[case(2024, 295, Month::October, 21)]
    #[case(2024, 296, Month::October, 22)]
    #[case(2024, 297, Month::October, 23)]
    #[case(2024, 298, Month::October, 24)]
    #[case(2024, 299, Month::October, 25)]
    #[case(2024, 300, Month::October, 26)]
    #[case(2024, 301, Month::October, 27)]
    #[case(2024, 302, Month::October, 28)]
    #[case(2024, 303, Month::October, 29)]
    #[case(2024, 304, Month::October, 30)]
    #[case(2024, 305, Month::October, 31)]
    #[case(2024, 306, Month::November, 1)]
    #[case(2024, 307, Month::November, 2)]
    #[case(2024, 308, Month::November, 3)]
    #[case(2024, 309, Month::November, 4)]
    #[case(2024, 310, Month::November, 5)]
    #[case(2024, 311, Month::November, 6)]
    #[case(2024, 312, Month::November, 7)]
    #[case(2024, 313, Month::November, 8)]
    #[case(2024, 314, Month::November, 9)]
    #[case(2024, 315, Month::November, 10)]
    #[case(2024, 316, Month::November, 11)]
    #[case(2024, 317, Month::November, 12)]
    #[case(2024, 318, Month::November, 13)]
    #[case(2024, 319, Month::November, 14)]
    #[case(2024, 320, Month::November, 15)]
    #[case(2024, 321, Month::November, 16)]
    #[case(2024, 322, Month::November, 17)]
    #[case(2024, 323, Month::November, 18)]
    #[case(2024, 324, Month::November, 19)]
    #[case(2024, 325, Month::November, 20)]
    #[case(2024, 326, Month::November, 21)]
    #[case(2024, 327, Month::November, 22)]
    #[case(2024, 328, Month::November, 23)]
    #[case(2024, 329, Month::November, 24)]
    #[case(2024, 330, Month::November, 25)]
    #[case(2024, 331, Month::November, 26)]
    #[case(2024, 332, Month::November, 27)]
    #[case(2024, 333, Month::November, 28)]
    #[case(2024, 334, Month::November, 29)]
    #[case(2024, 335, Month::November, 30)]
    #[case(2024, 336, Month::December, 1)]
    #[case(2024, 337, Month::December, 2)]
    #[case(2024, 338, Month::December, 3)]
    #[case(2024, 339, Month::December, 4)]
    #[case(2024, 340, Month::December, 5)]
    #[case(2024, 341, Month::December, 6)]
    #[case(2024, 342, Month::December, 7)]
    #[case(2024, 343, Month::December, 8)]
    #[case(2024, 344, Month::December, 9)]
    #[case(2024, 345, Month::December, 10)]
    #[case(2024, 346, Month::December, 11)]
    #[case(2024, 347, Month::December, 12)]
    #[case(2024, 348, Month::December, 13)]
    #[case(2024, 349, Month::December, 14)]
    #[case(2024, 350, Month::December, 15)]
    #[case(2024, 351, Month::December, 16)]
    #[case(2024, 352, Month::December, 17)]
    #[case(2024, 353, Month::December, 18)]
    #[case(2024, 354, Month::December, 19)]
    #[case(2024, 355, Month::December, 20)]
    #[case(2024, 356, Month::December, 21)]
    #[case(2024, 357, Month::December, 22)]
    #[case(2024, 358, Month::December, 23)]
    #[case(2024, 359, Month::December, 24)]
    #[case(2024, 360, Month::December, 25)]
    #[case(2024, 361, Month::December, 26)]
    #[case(2024, 362, Month::December, 27)]
    #[case(2024, 363, Month::December, 28)]
    #[case(2024, 364, Month::December, 29)]
    #[case(2024, 365, Month::December, 30)]
    #[case(2024, 366, Month::December, 31)]
    #[case(1582, 1, Month::January, 1)]
    #[case(1582, 2, Month::January, 2)]
    #[case(1582, 3, Month::January, 3)]
    #[case(1582, 4, Month::January, 4)]
    #[case(1582, 5, Month::January, 5)]
    #[case(1582, 6, Month::January, 6)]
    #[case(1582, 7, Month::January, 7)]
    #[case(1582, 8, Month::January, 8)]
    #[case(1582, 9, Month::January, 9)]
    #[case(1582, 10, Month::January, 10)]
    #[case(1582, 11, Month::January, 11)]
    #[case(1582, 12, Month::January, 12)]
    #[case(1582, 13, Month::January, 13)]
    #[case(1582, 14, Month::January, 14)]
    #[case(1582, 15, Month::January, 15)]
    #[case(1582, 16, Month::January, 16)]
    #[case(1582, 17, Month::January, 17)]
    #[case(1582, 18, Month::January, 18)]
    #[case(1582, 19, Month::January, 19)]
    #[case(1582, 20, Month::January, 20)]
    #[case(1582, 21, Month::January, 21)]
    #[case(1582, 22, Month::January, 22)]
    #[case(1582, 23, Month::January, 23)]
    #[case(1582, 24, Month::January, 24)]
    #[case(1582, 25, Month::January, 25)]
    #[case(1582, 26, Month::January, 26)]
    #[case(1582, 27, Month::January, 27)]
    #[case(1582, 28, Month::January, 28)]
    #[case(1582, 29, Month::January, 29)]
    #[case(1582, 30, Month::January, 30)]
    #[case(1582, 31, Month::January, 31)]
    #[case(1582, 32, Month::February, 1)]
    #[case(1582, 33, Month::February, 2)]
    #[case(1582, 34, Month::February, 3)]
    #[case(1582, 35, Month::February, 4)]
    #[case(1582, 36, Month::February, 5)]
    #[case(1582, 37, Month::February, 6)]
    #[case(1582, 38, Month::February, 7)]
    #[case(1582, 39, Month::February, 8)]
    #[case(1582, 40, Month::February, 9)]
    #[case(1582, 41, Month::February, 10)]
    #[case(1582, 42, Month::February, 11)]
    #[case(1582, 43, Month::February, 12)]
    #[case(1582, 44, Month::February, 13)]
    #[case(1582, 45, Month::February, 14)]
    #[case(1582, 46, Month::February, 15)]
    #[case(1582, 47, Month::February, 16)]
    #[case(1582, 48, Month::February, 17)]
    #[case(1582, 49, Month::February, 18)]
    #[case(1582, 50, Month::February, 19)]
    #[case(1582, 51, Month::February, 20)]
    #[case(1582, 52, Month::February, 21)]
    #[case(1582, 53, Month::February, 22)]
    #[case(1582, 54, Month::February, 23)]
    #[case(1582, 55, Month::February, 24)]
    #[case(1582, 56, Month::February, 25)]
    #[case(1582, 57, Month::February, 26)]
    #[case(1582, 58, Month::February, 27)]
    #[case(1582, 59, Month::February, 28)]
    #[case(1582, 60, Month::March, 1)]
    #[case(1582, 61, Month::March, 2)]
    #[case(1582, 62, Month::March, 3)]
    #[case(1582, 63, Month::March, 4)]
    #[case(1582, 64, Month::March, 5)]
    #[case(1582, 65, Month::March, 6)]
    #[case(1582, 66, Month::March, 7)]
    #[case(1582, 67, Month::March, 8)]
    #[case(1582, 68, Month::March, 9)]
    #[case(1582, 69, Month::March, 10)]
    #[case(1582, 70, Month::March, 11)]
    #[case(1582, 71, Month::March, 12)]
    #[case(1582, 72, Month::March, 13)]
    #[case(1582, 73, Month::March, 14)]
    #[case(1582, 74, Month::March, 15)]
    #[case(1582, 75, Month::March, 16)]
    #[case(1582, 76, Month::March, 17)]
    #[case(1582, 77, Month::March, 18)]
    #[case(1582, 78, Month::March, 19)]
    #[case(1582, 79, Month::March, 20)]
    #[case(1582, 80, Month::March, 21)]
    #[case(1582, 81, Month::March, 22)]
    #[case(1582, 82, Month::March, 23)]
    #[case(1582, 83, Month::March, 24)]
    #[case(1582, 84, Month::March, 25)]
    #[case(1582, 85, Month::March, 26)]
    #[case(1582, 86, Month::March, 27)]
    #[case(1582, 87, Month::March, 28)]
    #[case(1582, 88, Month::March, 29)]
    #[case(1582, 89, Month::March, 30)]
    #[case(1582, 90, Month::March, 31)]
    #[case(1582, 91, Month::April, 1)]
    #[case(1582, 92, Month::April, 2)]
    #[case(1582, 93, Month::April, 3)]
    #[case(1582, 94, Month::April, 4)]
    #[case(1582, 95, Month::April, 5)]
    #[case(1582, 96, Month::April, 6)]
    #[case(1582, 97, Month::April, 7)]
    #[case(1582, 98, Month::April, 8)]
    #[case(1582, 99, Month::April, 9)]
    #[case(1582, 100, Month::April, 10)]
    #[case(1582, 101, Month::April, 11)]
    #[case(1582, 102, Month::April, 12)]
    #[case(1582, 103, Month::April, 13)]
    #[case(1582, 104, Month::April, 14)]
    #[case(1582, 105, Month::April, 15)]
    #[case(1582, 106, Month::April, 16)]
    #[case(1582, 107, Month::April, 17)]
    #[case(1582, 108, Month::April, 18)]
    #[case(1582, 109, Month::April, 19)]
    #[case(1582, 110, Month::April, 20)]
    #[case(1582, 111, Month::April, 21)]
    #[case(1582, 112, Month::April, 22)]
    #[case(1582, 113, Month::April, 23)]
    #[case(1582, 114, Month::April, 24)]
    #[case(1582, 115, Month::April, 25)]
    #[case(1582, 116, Month::April, 26)]
    #[case(1582, 117, Month::April, 27)]
    #[case(1582, 118, Month::April, 28)]
    #[case(1582, 119, Month::April, 29)]
    #[case(1582, 120, Month::April, 30)]
    #[case(1582, 121, Month::May, 1)]
    #[case(1582, 122, Month::May, 2)]
    #[case(1582, 123, Month::May, 3)]
    #[case(1582, 124, Month::May, 4)]
    #[case(1582, 125, Month::May, 5)]
    #[case(1582, 126, Month::May, 6)]
    #[case(1582, 127, Month::May, 7)]
    #[case(1582, 128, Month::May, 8)]
    #[case(1582, 129, Month::May, 9)]
    #[case(1582, 130, Month::May, 10)]
    #[case(1582, 131, Month::May, 11)]
    #[case(1582, 132, Month::May, 12)]
    #[case(1582, 133, Month::May, 13)]
    #[case(1582, 134, Month::May, 14)]
    #[case(1582, 135, Month::May, 15)]
    #[case(1582, 136, Month::May, 16)]
    #[case(1582, 137, Month::May, 17)]
    #[case(1582, 138, Month::May, 18)]
    #[case(1582, 139, Month::May, 19)]
    #[case(1582, 140, Month::May, 20)]
    #[case(1582, 141, Month::May, 21)]
    #[case(1582, 142, Month::May, 22)]
    #[case(1582, 143, Month::May, 23)]
    #[case(1582, 144, Month::May, 24)]
    #[case(1582, 145, Month::May, 25)]
    #[case(1582, 146, Month::May, 26)]
    #[case(1582, 147, Month::May, 27)]
    #[case(1582, 148, Month::May, 28)]
    #[case(1582, 149, Month::May, 29)]
    #[case(1582, 150, Month::May, 30)]
    #[case(1582, 151, Month::May, 31)]
    #[case(1582, 152, Month::June, 1)]
    #[case(1582, 153, Month::June, 2)]
    #[case(1582, 154, Month::June, 3)]
    #[case(1582, 155, Month::June, 4)]
    #[case(1582, 156, Month::June, 5)]
    #[case(1582, 157, Month::June, 6)]
    #[case(1582, 158, Month::June, 7)]
    #[case(1582, 159, Month::June, 8)]
    #[case(1582, 160, Month::June, 9)]
    #[case(1582, 161, Month::June, 10)]
    #[case(1582, 162, Month::June, 11)]
    #[case(1582, 163, Month::June, 12)]
    #[case(1582, 164, Month::June, 13)]
    #[case(1582, 165, Month::June, 14)]
    #[case(1582, 166, Month::June, 15)]
    #[case(1582, 167, Month::June, 16)]
    #[case(1582, 168, Month::June, 17)]
    #[case(1582, 169, Month::June, 18)]
    #[case(1582, 170, Month::June, 19)]
    #[case(1582, 171, Month::June, 20)]
    #[case(1582, 172, Month::June, 21)]
    #[case(1582, 173, Month::June, 22)]
    #[case(1582, 174, Month::June, 23)]
    #[case(1582, 175, Month::June, 24)]
    #[case(1582, 176, Month::June, 25)]
    #[case(1582, 177, Month::June, 26)]
    #[case(1582, 178, Month::June, 27)]
    #[case(1582, 179, Month::June, 28)]
    #[case(1582, 180, Month::June, 29)]
    #[case(1582, 181, Month::June, 30)]
    #[case(1582, 182, Month::July, 1)]
    #[case(1582, 183, Month::July, 2)]
    #[case(1582, 184, Month::July, 3)]
    #[case(1582, 185, Month::July, 4)]
    #[case(1582, 186, Month::July, 5)]
    #[case(1582, 187, Month::July, 6)]
    #[case(1582, 188, Month::July, 7)]
    #[case(1582, 189, Month::July, 8)]
    #[case(1582, 190, Month::July, 9)]
    #[case(1582, 191, Month::July, 10)]
    #[case(1582, 192, Month::July, 11)]
    #[case(1582, 193, Month::July, 12)]
    #[case(1582, 194, Month::July, 13)]
    #[case(1582, 195, Month::July, 14)]
    #[case(1582, 196, Month::July, 15)]
    #[case(1582, 197, Month::July, 16)]
    #[case(1582, 198, Month::July, 17)]
    #[case(1582, 199, Month::July, 18)]
    #[case(1582, 200, Month::July, 19)]
    #[case(1582, 201, Month::July, 20)]
    #[case(1582, 202, Month::July, 21)]
    #[case(1582, 203, Month::July, 22)]
    #[case(1582, 204, Month::July, 23)]
    #[case(1582, 205, Month::July, 24)]
    #[case(1582, 206, Month::July, 25)]
    #[case(1582, 207, Month::July, 26)]
    #[case(1582, 208, Month::July, 27)]
    #[case(1582, 209, Month::July, 28)]
    #[case(1582, 210, Month::July, 29)]
    #[case(1582, 211, Month::July, 30)]
    #[case(1582, 212, Month::July, 31)]
    #[case(1582, 213, Month::August, 1)]
    #[case(1582, 214, Month::August, 2)]
    #[case(1582, 215, Month::August, 3)]
    #[case(1582, 216, Month::August, 4)]
    #[case(1582, 217, Month::August, 5)]
    #[case(1582, 218, Month::August, 6)]
    #[case(1582, 219, Month::August, 7)]
    #[case(1582, 220, Month::August, 8)]
    #[case(1582, 221, Month::August, 9)]
    #[case(1582, 222, Month::August, 10)]
    #[case(1582, 223, Month::August, 11)]
    #[case(1582, 224, Month::August, 12)]
    #[case(1582, 225, Month::August, 13)]
    #[case(1582, 226, Month::August, 14)]
    #[case(1582, 227, Month::August, 15)]
    #[case(1582, 228, Month::August, 16)]
    #[case(1582, 229, Month::August, 17)]
    #[case(1582, 230, Month::August, 18)]
    #[case(1582, 231, Month::August, 19)]
    #[case(1582, 232, Month::August, 20)]
    #[case(1582, 233, Month::August, 21)]
    #[case(1582, 234, Month::August, 22)]
    #[case(1582, 235, Month::August, 23)]
    #[case(1582, 236, Month::August, 24)]
    #[case(1582, 237, Month::August, 25)]
    #[case(1582, 238, Month::August, 26)]
    #[case(1582, 239, Month::August, 27)]
    #[case(1582, 240, Month::August, 28)]
    #[case(1582, 241, Month::August, 29)]
    #[case(1582, 242, Month::August, 30)]
    #[case(1582, 243, Month::August, 31)]
    #[case(1582, 244, Month::September, 1)]
    #[case(1582, 245, Month::September, 2)]
    #[case(1582, 246, Month::September, 3)]
    #[case(1582, 247, Month::September, 4)]
    #[case(1582, 248, Month::September, 5)]
    #[case(1582, 249, Month::September, 6)]
    #[case(1582, 250, Month::September, 7)]
    #[case(1582, 251, Month::September, 8)]
    #[case(1582, 252, Month::September, 9)]
    #[case(1582, 253, Month::September, 10)]
    #[case(1582, 254, Month::September, 11)]
    #[case(1582, 255, Month::September, 12)]
    #[case(1582, 256, Month::September, 13)]
    #[case(1582, 257, Month::September, 14)]
    #[case(1582, 258, Month::September, 15)]
    #[case(1582, 259, Month::September, 16)]
    #[case(1582, 260, Month::September, 17)]
    #[case(1582, 261, Month::September, 18)]
    #[case(1582, 262, Month::September, 19)]
    #[case(1582, 263, Month::September, 20)]
    #[case(1582, 264, Month::September, 21)]
    #[case(1582, 265, Month::September, 22)]
    #[case(1582, 266, Month::September, 23)]
    #[case(1582, 267, Month::September, 24)]
    #[case(1582, 268, Month::September, 25)]
    #[case(1582, 269, Month::September, 26)]
    #[case(1582, 270, Month::September, 27)]
    #[case(1582, 271, Month::September, 28)]
    #[case(1582, 272, Month::September, 29)]
    #[case(1582, 273, Month::September, 30)]
    #[case(1582, 274, Month::October, 1)]
    #[case(1582, 275, Month::October, 2)]
    #[case(1582, 276, Month::October, 3)]
    #[case(1582, 277, Month::October, 4)]
    #[case(1582, 278, Month::October, 15)]
    #[case(1582, 279, Month::October, 16)]
    #[case(1582, 280, Month::October, 17)]
    #[case(1582, 281, Month::October, 18)]
    #[case(1582, 282, Month::October, 19)]
    #[case(1582, 283, Month::October, 20)]
    #[case(1582, 284, Month::October, 21)]
    #[case(1582, 285, Month::October, 22)]
    #[case(1582, 286, Month::October, 23)]
    #[case(1582, 287, Month::October, 24)]
    #[case(1582, 288, Month::October, 25)]
    #[case(1582, 289, Month::October, 26)]
    #[case(1582, 290, Month::October, 27)]
    #[case(1582, 291, Month::October, 28)]
    #[case(1582, 292, Month::October, 29)]
    #[case(1582, 293, Month::October, 30)]
    #[case(1582, 294, Month::October, 31)]
    #[case(1582, 295, Month::November, 1)]
    #[case(1582, 296, Month::November, 2)]
    #[case(1582, 297, Month::November, 3)]
    #[case(1582, 298, Month::November, 4)]
    #[case(1582, 299, Month::November, 5)]
    #[case(1582, 300, Month::November, 6)]
    #[case(1582, 301, Month::November, 7)]
    #[case(1582, 302, Month::November, 8)]
    #[case(1582, 303, Month::November, 9)]
    #[case(1582, 304, Month::November, 10)]
    #[case(1582, 305, Month::November, 11)]
    #[case(1582, 306, Month::November, 12)]
    #[case(1582, 307, Month::November, 13)]
    #[case(1582, 308, Month::November, 14)]
    #[case(1582, 309, Month::November, 15)]
    #[case(1582, 310, Month::November, 16)]
    #[case(1582, 311, Month::November, 17)]
    #[case(1582, 312, Month::November, 18)]
    #[case(1582, 313, Month::November, 19)]
    #[case(1582, 314, Month::November, 20)]
    #[case(1582, 315, Month::November, 21)]
    #[case(1582, 316, Month::November, 22)]
    #[case(1582, 317, Month::November, 23)]
    #[case(1582, 318, Month::November, 24)]
    #[case(1582, 319, Month::November, 25)]
    #[case(1582, 320, Month::November, 26)]
    #[case(1582, 321, Month::November, 27)]
    #[case(1582, 322, Month::November, 28)]
    #[case(1582, 323, Month::November, 29)]
    #[case(1582, 324, Month::November, 30)]
    #[case(1582, 325, Month::December, 1)]
    #[case(1582, 326, Month::December, 2)]
    #[case(1582, 327, Month::December, 3)]
    #[case(1582, 328, Month::December, 4)]
    #[case(1582, 329, Month::December, 5)]
    #[case(1582, 330, Month::December, 6)]
    #[case(1582, 331, Month::December, 7)]
    #[case(1582, 332, Month::December, 8)]
    #[case(1582, 333, Month::December, 9)]
    #[case(1582, 334, Month::December, 10)]
    #[case(1582, 335, Month::December, 11)]
    #[case(1582, 336, Month::December, 12)]
    #[case(1582, 337, Month::December, 13)]
    #[case(1582, 338, Month::December, 14)]
    #[case(1582, 339, Month::December, 15)]
    #[case(1582, 340, Month::December, 16)]
    #[case(1582, 341, Month::December, 17)]
    #[case(1582, 342, Month::December, 18)]
    #[case(1582, 343, Month::December, 19)]
    #[case(1582, 344, Month::December, 20)]
    #[case(1582, 345, Month::December, 21)]
    #[case(1582, 346, Month::December, 22)]
    #[case(1582, 347, Month::December, 23)]
    #[case(1582, 348, Month::December, 24)]
    #[case(1582, 349, Month::December, 25)]
    #[case(1582, 350, Month::December, 26)]
    #[case(1582, 351, Month::December, 27)]
    #[case(1582, 352, Month::December, 28)]
    #[case(1582, 353, Month::December, 29)]
    #[case(1582, 354, Month::December, 30)]
    #[case(1582, 355, Month::December, 31)]
    fn test_gregorian_reform_at_year_ordinal(
        #[case] year: YearT,
        #[case] ordinal: DaysT,
        #[case] month: Month,
        #[case] mday: u32,
    ) {
        let date = Calendar::gregorian_reform()
            .at_year_ordinal(year, ordinal)
            .unwrap();
        assert_eq!(date.year(), year);
        assert_eq!(date.ordinal(), ordinal);
        assert_eq!(date.month(), month);
        assert_eq!(date.mday(), mday);
    }

    #[test]
    fn test_display_date() {
        let date = Calendar::gregorian_reform()
            .at_ymd(2023, Month::April, 20)
            .unwrap();
        assert_eq!(format!("{date}"), "2023-04-20");
    }

    #[test]
    fn test_alternate_display_date() {
        let date = Calendar::gregorian_reform()
            .at_ymd(2023, Month::April, 20)
            .unwrap();
        assert_eq!(format!("{date:#}"), "2023-110");
    }

    #[test]
    fn test_alternate_display_date_padded() {
        let date = Calendar::gregorian_reform()
            .at_ymd(2023, Month::March, 15)
            .unwrap();
        assert_eq!(format!("{date:#}"), "2023-074");
    }

    // TODO: Test with negative timestamps
    #[rstest]
    #[case(0, 1970, Month::January, 1, 0)]
    #[case(100000000, 1973, Month::March, 3, 35200)]
    #[case(1000000000, 2001, Month::September, 9, 6400)]
    #[case(1234567890, 2009, Month::February, 13, 84690)]
    #[case(1682028168, 2023, Month::April, 20, 79368)]
    #[case(2147483647, 2038, Month::January, 19, 11647)]
    fn test_from_unix_timestamp(
        #[case] ts: i64,
        #[case] year: YearT,
        #[case] month: Month,
        #[case] mday: u32,
        #[case] seconds: u32,
    ) {
        let (date, s) = Calendar::gregorian_reform().at_unix_timestamp(ts).unwrap();
        assert_eq!(date.year(), year);
        assert_eq!(date.month(), month);
        assert_eq!(date.mday(), mday);
        assert_eq!(s, seconds);
    }

    #[test]
    fn test_month_try_from_zero() {
        let r = Month::try_from(0);
        assert_eq!(r, Err(TryIntoMonthError));
        assert_eq!(
            r.unwrap_err().to_string(),
            "value out of range for month number; must be from 1 through 12"
        );
    }

    #[test]
    fn test_month_try_from_thirteen() {
        let r = Month::try_from(13);
        assert_eq!(r, Err(TryIntoMonthError));
        assert_eq!(
            r.unwrap_err().to_string(),
            "value out of range for month number; must be from 1 through 12"
        );
    }

    #[test]
    fn test_from_ymd_mday_0() {
        let r = Calendar::gregorian_reform().at_ymd(2023, Month::April, 0);
        assert_eq!(
            r,
            Err(Error::MdayOutOfRange {
                year: 2023,
                month: Month::April,
                mday: 0
            })
        );
        assert_eq!(
            r.unwrap_err().to_string(),
            "mday 0 is outside of valid range for April 2023"
        );
    }

    #[test]
    fn test_from_ymd_mday_32() {
        let r = Calendar::gregorian_reform().at_ymd(2023, Month::April, 32);
        assert_eq!(
            r,
            Err(Error::MdayOutOfRange {
                year: 2023,
                month: Month::April,
                mday: 32
            })
        );
        assert_eq!(
            r.unwrap_err().to_string(),
            "mday 32 is outside of valid range for April 2023"
        );
    }

    #[test]
    fn test_from_ymd_sep_31() {
        let r = Calendar::gregorian_reform().at_ymd(2023, Month::September, 31);
        assert_eq!(
            r,
            Err(Error::MdayOutOfRange {
                year: 2023,
                month: Month::September,
                mday: 31
            })
        );
        assert_eq!(
            r.unwrap_err().to_string(),
            "mday 31 is outside of valid range for September 2023"
        );
    }

    #[test]
    fn test_from_ymd_invalid_leap_day() {
        let r = Calendar::gregorian_reform().at_ymd(2023, Month::February, 29);
        assert_eq!(
            r,
            Err(Error::MdayOutOfRange {
                year: 2023,
                month: Month::February,
                mday: 29
            })
        );
        assert_eq!(
            r.unwrap_err().to_string(),
            "mday 29 is outside of valid range for February 2023"
        );
    }

    #[test]
    fn test_from_ymd_valid_leap_day() {
        let date = Calendar::gregorian_reform()
            .at_ymd(2024, Month::February, 29)
            .unwrap();
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.mday(), 29);
    }

    #[test]
    fn test_from_ymd_skipped_date() {
        let r = Calendar::gregorian_reform().at_ymd(1582, Month::October, 10);
        assert_eq!(
            r,
            Err(Error::SkippedDate {
                year: 1582,
                month: Month::October,
                mday: 10
            })
        );
        assert_eq!(
            r.unwrap_err().to_string(),
            "date 1582-10-10 was skipped by calendar reform"
        );
    }

    #[test]
    fn test_from_ymd_first_skipped_date() {
        let r = Calendar::gregorian_reform().at_ymd(1582, Month::October, 5);
        assert_eq!(
            r,
            Err(Error::SkippedDate {
                year: 1582,
                month: Month::October,
                mday: 5
            })
        );
        assert_eq!(
            r.unwrap_err().to_string(),
            "date 1582-10-05 was skipped by calendar reform"
        );
    }

    #[test]
    fn test_from_ymd_last_skipped_date() {
        let r = Calendar::gregorian_reform().at_ymd(1582, Month::October, 14);
        assert_eq!(
            r,
            Err(Error::SkippedDate {
                year: 1582,
                month: Month::October,
                mday: 14
            })
        );
        assert_eq!(
            r.unwrap_err().to_string(),
            "date 1582-10-14 was skipped by calendar reform"
        );
    }

    #[rstest]
    #[case(2023, 366)]
    #[case(2023, 1000)]
    #[case(2024, 367)]
    #[case(2024, 1000)]
    #[case(1582, 356)]
    #[case(1582, 1000)]
    fn test_gregorian_reform_at_year_ordinal_err(#[case] year: YearT, #[case] ordinal: DaysT) {
        let r = Calendar::gregorian_reform().at_year_ordinal(year, ordinal);
        assert_eq!(r, Err(Error::OrdinalOutOfRange { year, ordinal }));
        assert_eq!(
            r.unwrap_err().to_string(),
            format!("day-of-year ordinal {ordinal} is outside of valid range for year {year}")
        );
    }

    #[test]
    fn test_init_gregorian_reform() {
        let cal = Calendar::gregorian_reform();
        // Use assert_matches! instead of assert_eq! because Calendar's Eq
        // implementation ignores `gap`
        assert_matches!(
            cal.0,
            inner::Calendar::Reforming {
                reformation: 2299161,
                gap: inner::ReformGap {
                    pre_reform: inner::Date {
                        year: 1582,
                        ordinal: 277,
                        month: Month::October,
                        mday: 4
                    },
                    post_reform: inner::Date {
                        year: 1582,
                        ordinal: 278,
                        month: Month::October,
                        mday: 15
                    },
                    gap_length: 10,
                    kind: inner::GapKind::IntraMonth
                }
            }
        );
    }

    #[rstest]
    #[case(2023, Month::January, 31)]
    #[case(2023, Month::February, 28)]
    #[case(2023, Month::March, 31)]
    #[case(2023, Month::April, 30)]
    #[case(2023, Month::May, 31)]
    #[case(2023, Month::June, 30)]
    #[case(2023, Month::July, 31)]
    #[case(2023, Month::August, 31)]
    #[case(2023, Month::September, 30)]
    #[case(2023, Month::October, 31)]
    #[case(2023, Month::November, 30)]
    #[case(2023, Month::December, 31)]
    #[case(2024, Month::January, 31)]
    #[case(2024, Month::February, 29)]
    #[case(2024, Month::March, 31)]
    #[case(2024, Month::April, 30)]
    #[case(2024, Month::May, 31)]
    #[case(2024, Month::June, 30)]
    #[case(2024, Month::July, 31)]
    #[case(2024, Month::August, 31)]
    #[case(2024, Month::September, 30)]
    #[case(2024, Month::October, 31)]
    #[case(2024, Month::November, 30)]
    #[case(2024, Month::December, 31)]
    #[case(1582, Month::January, 31)]
    #[case(1582, Month::February, 28)]
    #[case(1582, Month::March, 31)]
    #[case(1582, Month::April, 30)]
    #[case(1582, Month::May, 31)]
    #[case(1582, Month::June, 30)]
    #[case(1582, Month::July, 31)]
    #[case(1582, Month::August, 31)]
    #[case(1582, Month::September, 30)]
    #[case(1582, Month::October, 21)]
    #[case(1582, Month::November, 30)]
    #[case(1582, Month::December, 31)]
    fn test_gregorian_reform_month_length(
        #[case] year: YearT,
        #[case] month: Month,
        #[case] length: u32,
    ) {
        let cal = Calendar::gregorian_reform();
        assert_eq!(cal.month_length(year, month), length);
    }

    #[test]
    fn test_gregorian_reform_reformation_month_shape() {
        use Month::October;
        let cal = Calendar::gregorian_reform();
        let shape = cal.month_shape(1582, October);
        assert_eq!(
            shape,
            inner::MonthShape::HasGap {
                year: 1582,
                month: October,
                gap: 5..15,
                max_mday: 31
            }
        );
        assert_eq!(shape.len(), 21);
        assert!(!shape.has_mday(0));
        assert!(shape.has_mday(1));
        assert!(shape.has_mday(4));
        assert!(!shape.has_mday(5));
        assert!(!shape.has_mday(14));
        assert!(shape.has_mday(15));
        assert!(shape.has_mday(31));
        assert!(!shape.has_mday(32));
        assert_eq!(
            shape.get_mday_ordinal(0),
            Err(Error::MdayOutOfRange {
                year: 1582,
                month: October,
                mday: 0
            })
        );
        assert_eq!(shape.get_mday_ordinal(1), Ok(1));
        assert_eq!(shape.get_mday_ordinal(4), Ok(4));
        assert_eq!(
            shape.get_mday_ordinal(5),
            Err(Error::SkippedDate {
                year: 1582,
                month: October,
                mday: 5
            })
        );
        assert_eq!(
            shape.get_mday_ordinal(14),
            Err(Error::SkippedDate {
                year: 1582,
                month: October,
                mday: 14
            })
        );
        assert_eq!(shape.get_mday_ordinal(15), Ok(5));
        assert_eq!(shape.get_mday_ordinal(31), Ok(21));
        assert_eq!(
            shape.get_mday_ordinal(32),
            Err(Error::MdayOutOfRange {
                year: 1582,
                month: October,
                mday: 32
            })
        );
        assert_eq!(shape.get_mday_label(1), Some(1));
        assert_eq!(shape.get_mday_label(2), Some(2));
        assert_eq!(shape.get_mday_label(4), Some(4));
        assert_eq!(shape.get_mday_label(5), Some(15));
        assert_eq!(shape.get_mday_label(21), Some(31));
        assert_eq!(shape.get_mday_label(22), None);
    }

    #[test]
    fn test_init_german_reformation() {
        let cal = Calendar::reforming(2342032).unwrap();
        // Use assert_matches! instead of assert_eq! because Calendar's Eq
        // implementation ignores `gap`
        assert_matches!(
            cal.0,
            inner::Calendar::Reforming {
                reformation: 2342032,
                gap: inner::ReformGap {
                    pre_reform: inner::Date {
                        year: 1700,
                        ordinal: 49,
                        month: Month::February,
                        mday: 18
                    },
                    post_reform: inner::Date {
                        year: 1700,
                        ordinal: 50,
                        month: Month::March,
                        mday: 1
                    },
                    gap_length: 11,
                    kind: inner::GapKind::CrossMonth
                }
            }
        );
    }

    #[test]
    fn test_german_reformation_year() {
        let cal = Calendar::reforming(2342032).unwrap();
        assert_eq!(cal.year_kind(1700), YearKind::ReformCommon);
        assert_eq!(cal.year_length(1700), 355);
        let shape_feb = cal.month_shape(1700, Month::February);
        assert_eq!(
            shape_feb,
            inner::MonthShape::Solid {
                year: 1700,
                month: Month::February,
                range: 1..=18
            }
        );
        let shape_mar = cal.month_shape(1700, Month::March);
        assert_eq!(
            shape_mar,
            inner::MonthShape::Solid {
                year: 1700,
                month: Month::March,
                range: 1..=31
            }
        );
    }
}
