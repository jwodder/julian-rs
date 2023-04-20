//! Unless otherwise specified, all functions use a Gregorian calendar with the
//! Reformation taking place on 1582-10-05/15.

#[cfg(test)]
extern crate rstest_reuse;

use chrono::naive::NaiveDate;
use chrono::{Datelike, Timelike, Utc};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

pub type ProlepticGregorianDate = NaiveDate;
pub type YearT = i32;
pub type DaysT = u32;
pub type SecondsT = u32;
pub type JulianDayT = i32;

const JD_PRECISION: usize = 6;

pub const SECONDS_IN_MINUTE: u32 = 60;
pub const SECONDS_IN_HOUR: u32 = 60 * 60;

pub const SECONDS_IN_HALF_DAY: SecondsT = 12 * SECONDS_IN_HOUR;
pub const SECONDS_IN_DAY: SecondsT = 24 * SECONDS_IN_HOUR;

pub const GREG_REFORM: JulianDayT = 2299161; // noon on 1582-10-15

pub const YDAY_REFORM: DaysT = 277; // zero-index yday for Oct 5

pub const START1583: JulianDayT = GREG_REFORM + 78; // noon on 1583-01-01
pub const START1600: JulianDayT = 2305448; // noon on 1600-01-01
pub const UK_REFORM: JulianDayT = 2361222; // noon on 1752-09-14

pub static MONTH_LENGTHS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Date {
    pub year: YearT,               // 0 == 1 BC
    pub yday: DaysT,               // days from the start of the year; 0 == Jan 01
    pub month: u32,                // one-based
    pub mday: u32,                 // one-based
    pub seconds: Option<SecondsT>, // seconds after midnight
}

impl Date {
    pub fn now() -> Date {
        let nowts = Utc::now();
        Date {
            year: nowts.year(),
            yday: nowts.ordinal0(),
            month: nowts.month(),
            mday: nowts.day(),
            seconds: Some(nowts.num_seconds_from_midnight()),
        }
    }

    pub fn is_before_gregorian(&self) -> bool {
        self.year < 1582 || (self.year == 1582 && self.yday < YDAY_REFORM)
    }

    /// `month` and `mday` are one-indexed
    pub fn from_ymd(year: YearT, month: u32, mday: u32, seconds: Option<SecondsT>) -> Option<Date> {
        let month_usize = usize::try_from(month).ok()?;
        if !(1..=12).contains(&month)
            || mday < 1
            || (mday > MONTH_LENGTHS[month_usize - 1]
                && !(is_leap_year(year) && month == 2 && mday == 29))
        {
            return None;
        }
        let mut yday = 0;
        for (&length, i) in MONTH_LENGTHS[0..month_usize - 1].iter().zip(1..) {
            yday += length;
            if i == 2 && is_leap_year(year) {
                yday += 1;
            }
        }
        yday += mday - 1;
        if year == 1582 && (month > 10 || (month == 10 && mday >= 15)) {
            // If someone enters a date that was skipped by the Gregorian
            // Reformation, just assume it's Old Style.
            yday -= 10;
        }
        Some(Date {
            year,
            yday,
            month,
            mday,
            seconds,
        })
    }

    // Uses Gregorian calendar
    pub fn from_year_yday(year: YearT, yday: DaysT, seconds: Option<SecondsT>) -> Option<Date> {
        Self::from_styled_year_yday(year, yday, seconds, YearStyle::for_gregorian_year(year))
    }

    pub fn from_julian_year_yday(
        year: YearT,
        yday: DaysT,
        seconds: Option<SecondsT>,
    ) -> Option<Date> {
        Self::from_styled_year_yday(year, yday, seconds, YearStyle::for_julian_year(year))
    }

    fn from_styled_year_yday(
        year: YearT,
        yday: DaysT,
        seconds: Option<SecondsT>,
        year_style: YearStyle,
    ) -> Option<Date> {
        let mut days = yday;
        for (&length, month) in MONTH_LENGTHS.iter().zip(1..) {
            let length = match (year_style, month) {
                (YearStyle::Leap, 2) => length + 1,
                (YearStyle::Reform, 10) => {
                    let length = 21;
                    if days < length {
                        return Some(Date {
                            year,
                            yday,
                            month,
                            mday: days + (if days < 4 { 1 } else { 11 }),
                            seconds,
                        });
                    }
                    length
                }
                _ => length,
            };
            if days < length {
                return Some(Date {
                    year,
                    yday,
                    month,
                    mday: days + 1,
                    seconds,
                });
            }
            days -= length;
        }
        None
    }

    pub fn to_julian_date(&self) -> JulianDate {
        let idays = JulianDayT::try_from(self.yday).unwrap();
        let jdays: JulianDayT = if self.year < -4712 {
            let rev_year = -4712 - self.year;
            idays - (rev_year * 365 + rev_year / 4)
        } else if self.is_before_gregorian() {
            (self.year + 4712) * 365 + (self.year + 4712 + 3) / 4 + idays
        } else if self.year == 1582 {
            GREG_REFORM + idays.checked_sub_unsigned(YDAY_REFORM).unwrap()
        } else {
            START1583 + days_since_1582(self.year) + idays
        };
        match self.seconds {
            None => JulianDate {
                days: jdays,
                seconds: None,
            },
            Some(s) if s < SECONDS_IN_HALF_DAY => JulianDate {
                days: jdays - 1,
                seconds: Some(s + SECONDS_IN_HALF_DAY),
            },
            Some(s) => JulianDate {
                days: jdays,
                seconds: Some(s - SECONDS_IN_HALF_DAY),
            },
        }
    }
}

impl FromStr for Date {
    type Err = DateParseError;

    // Formats:
    // - [+/-]YYYY-MM-DD[Thh:mm:ss[Z]]
    // - [+/-]YYYY-DDD[Thh:mm:ss[Z]]
    fn from_str(s: &str) -> Result<Date, DateParseError> {
        let mut parser = DateParser::new(s);
        let year = parser.parse_year()?;
        let diny = parser.parse_day_in_year()?;
        let seconds = parser.parse_time()?;
        if !parser.is_empty() {
            return Err(DateParseError::Generic);
        }
        match diny {
            DayInYear::Yday(yday) => {
                Date::from_year_yday(year, yday, seconds).ok_or(DateParseError::Generic)
            }
            DayInYear::Date { month, mday } => {
                Date::from_ymd(year, month, mday, seconds).ok_or(DateParseError::Generic)
            }
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-", self.year)?;
        if f.alternate() {
            write!(f, "{:03}", self.yday + 1)?;
        } else {
            write!(f, "{:02}-{:02}", self.month, self.mday)?;
        }
        if let Some(s) = self.seconds {
            let (hour, min, sec) = break_seconds(s);
            write!(f, "T{:02}:{:02}:{:02}Z", hour, min, sec)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum DateParseError {
    #[error("invalid calendar date")]
    Generic,
    #[error("invalid calendar date: numeric parse error: {0}")]
    ParseInt(#[from] ParseIntError),
}

struct DateParser<'a> {
    data: &'a str,
}

impl<'a> DateParser<'a> {
    fn new(data: &'a str) -> Self {
        DateParser { data }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn parse_year(&mut self) -> Result<YearT, DateParseError> {
        match self.data.match_indices('-').find(|&(i, _)| i > 0) {
            Some((i, _)) => {
                let year = self.data[..i].parse::<YearT>()?;
                self.data = &self.data[i + 1..];
                Ok(year)
            }
            None => Err(DateParseError::Generic),
        }
    }

    fn parse_day_in_year(&mut self) -> Result<DayInYear, DateParseError> {
        if let Some(i) = self.data.find(['-', 'T']) {
            if &self.data[i..(i + 1)] == "T" {
                let yday = Self::parse_yday(&self.data[..i])?;
                self.data = &self.data[(i + 1)..];
                Ok(yday)
            } else {
                let month = self.parse_02d()?;
                self.scan_char('-')?;
                let mday = self.parse_02d()?;
                Ok(DayInYear::Date { month, mday })
            }
        } else {
            let yday = Self::parse_yday(self.data)?;
            self.data = "";
            Ok(yday)
        }
    }

    fn parse_yday(s: &str) -> Result<DayInYear, DateParseError> {
        if s.len() == 3 && s.chars().all(|c| c.is_ascii_digit()) {
            Ok(DayInYear::Yday(s.parse::<DaysT>()?))
        } else {
            Err(DateParseError::Generic)
        }
    }

    fn parse_02d(&mut self) -> Result<u32, DateParseError> {
        match self.data.get(..2) {
            Some(s) => {
                let n = s.parse::<u32>()?;
                self.data = &self.data[2..];
                Ok(n)
            }
            None => Err(DateParseError::Generic),
        }
    }

    fn scan_char(&mut self, ch: char) -> Result<(), DateParseError> {
        if let Some(s) = self.data.strip_prefix(ch) {
            self.data = s;
            Ok(())
        } else {
            Err(DateParseError::Generic)
        }
    }

    fn parse_time(&mut self) -> Result<Option<SecondsT>, DateParseError> {
        if self.data.is_empty() {
            Ok(None)
        } else if let Some(s) = self.data.strip_prefix('T') {
            self.data = s;
            let hour = self.parse_02d()?;
            self.scan_char(':')?;
            let min = self.parse_02d()?;
            self.scan_char(':')?;
            let sec = self.parse_02d()?;
            self.data = self.data.strip_prefix('Z').unwrap_or(self.data);
            Ok(Some(hour * SECONDS_IN_HOUR + min * SECONDS_IN_MINUTE + sec))
        } else {
            Err(DateParseError::Generic)
        }
    }
}

enum DayInYear {
    Yday(DaysT),
    Date { month: u32, mday: u32 },
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum YearStyle {
    Common,
    Leap,
    Reform,
}

impl YearStyle {
    pub fn for_gregorian_year(year: YearT) -> YearStyle {
        if year == 1582 {
            YearStyle::Reform
        } else if is_leap_year(year) {
            YearStyle::Leap
        } else {
            YearStyle::Common
        }
    }

    pub fn for_julian_year(year: YearT) -> YearStyle {
        if year % 4 == 0 {
            YearStyle::Leap
        } else {
            YearStyle::Common
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct JulianDate {
    pub days: JulianDayT,
    pub seconds: Option<SecondsT>,
}

impl JulianDate {
    /// If the Julian date has any seconds, remove them and adjust the date to
    /// point to the "secondless" Julian day that the receiver pointed to
    pub fn split_seconds(&self) -> (JulianDate, Option<SecondsT>) {
        let (days, seconds) = match self.seconds {
            None => (self.days, None),
            Some(s) if s >= SECONDS_IN_HALF_DAY => (self.days + 1, Some(s - SECONDS_IN_HALF_DAY)),
            Some(s) => (self.days, Some(s + SECONDS_IN_HALF_DAY)),
        };
        (
            JulianDate {
                days,
                seconds: None,
            },
            seconds,
        )
    }

    pub fn to_gregorian(&self) -> Date {
        let (JulianDate { days, .. }, seconds) = self.split_seconds();
        if days < START1600 {
            let days = if GREG_REFORM <= days { days + 10 } else { days };
            let when = (JulianDate {
                days,
                seconds: None,
            })
            .to_julian();
            Date::from_year_yday(
                when.year,
                if GREG_REFORM <= days && days - 10 < START1583 {
                    when.yday - 10
                } else {
                    when.yday
                },
                seconds,
            )
            .unwrap()
        } else {
            let mut days: JulianDayT = days - START1600;
            let mut year: YearT = 1600 + (days / 146097) * 400;
            days %= 146097;
            // Add a "virtual leap day" to the end of each non-Gregorian
            // centennial year so that `days` can then be handled as in the
            // Julian calendar:
            if days > 365 {
                days += (days - 366) / 36524;
            }
            year += (days / 1461) * 4;
            days %= 1461;
            if days > 365 {
                days += (days - 366) / 365;
            }
            year += days / 366;
            days %= 366;
            Date::from_year_yday(year, DaysT::try_from(days).unwrap(), seconds).unwrap()
        }
    }

    /// Convert a Julian date to a year & yday in the Julian calendar
    pub fn to_julian(&self) -> Date {
        let (JulianDate { days, .. }, seconds) = self.split_seconds();
        if days < 0 {
            let alt = JulianDate {
                days: 365i32.checked_sub(days).expect("Arithmetic underflow"),
                seconds: None,
            };
            let when = alt.to_julian();
            let year = -4712 - (when.year + 4712);
            let yday = year_length(when.year) - 1 - when.yday;
            Date::from_julian_year_yday(year, yday, seconds).unwrap()
        } else {
            let mut year: YearT = days / 1461 * 4;
            let mut yday: JulianDayT = days % 1461;
            // Add a "virtual leap day" to the end of each common year so that
            // `yday` can be divided & modded by 366 evenly:
            if yday > 365 {
                yday += (yday - 366) / 365;
            }
            year += yday / 366;
            yday %= 366;
            year -= 4712;
            Date::from_julian_year_yday(year, DaysT::try_from(yday).unwrap(), seconds).unwrap()
        }
    }
}

impl FromStr for JulianDate {
    type Err = JulianDateParseError;

    // Formats:
    // - %d
    // - %d.%s
    // - %d:%d
    fn from_str(s: &str) -> Result<JulianDate, JulianDateParseError> {
        let m = s.match_indices(['.', ':']).next();
        let (days_str, secstr) = match m {
            Some((i, _)) => (&s[..i], &s[(i + 1)..]),
            None => (s, ""),
        };
        let days = days_str.parse::<JulianDayT>()?;
        let seconds = match m {
            Some((_, ":")) => Some(secstr.parse::<SecondsT>()?),
            Some((_, ".")) => {
                let mut secs = 0;
                let mut coef: SecondsT = SECONDS_IN_DAY / 10;
                let mut accum = 0;
                let mut denom = 1;
                for ch in secstr.chars() {
                    let d = ch.to_digit(10).ok_or(JulianDateParseError::Generic)?;
                    accum += coef * d;
                    secs += accum / denom;
                    accum %= denom;
                    if coef % 10 != 0 {
                        accum *= 10;
                        denom *= 10;
                    } else {
                        coef /= 10;
                    }
                }
                if accum * 2 >= denom {
                    secs += 1;
                }
                Some(secs)
            }
            Some(_) => unreachable!(),
            None => None,
        };
        Ok(JulianDate { days, seconds })
    }
}

impl fmt::Display for JulianDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.days)?;
        if let Some(mut s) = self.seconds {
            if f.alternate() {
                write!(f, ":{:05}", s)?;
            } else {
                let precision = f.precision().unwrap_or(JD_PRECISION);
                if precision > 0 {
                    write!(f, ".")?;
                    for i in 0..precision {
                        s *= 10;
                        let mut dig = s / SECONDS_IN_DAY;
                        s %= SECONDS_IN_DAY;
                        // TODO: What should happen when the last digit is a 9
                        // that should be rounded up?
                        if i == precision - 1 && s * 2 >= SECONDS_IN_DAY && dig < 9 {
                            dig += 1;
                        }
                        write!(f, "{dig}")?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum JulianDateParseError {
    #[error("invalid Julian date")]
    Generic,
    #[error("invalid Julian date: numeric parse error: {0}")]
    ParseInt(#[from] ParseIntError),
}

pub fn year_length(year: YearT) -> u32 {
    if year == 1582 {
        355
    } else {
        365 + u32::from(is_leap_year(year))
    }
}

fn is_leap_year(year: YearT) -> bool {
    year % 4 == 0 && (year <= 1582 || year % 100 != 0 || year % 400 == 0)
}

/// Returns the number of days in the years [1583..year-1] combined.  Returns 0
/// for years < 1583.
fn days_since_1582(year: YearT) -> JulianDayT {
    if year < 1583 {
        return 0;
    }
    let base = year - 1201;
    (base - 382) * 365 + (base - 380) / 4 - (base - 300) / 100 + base / 400
}

fn break_seconds(mut seconds: SecondsT) -> (u32, u32, u32) {
    // TODO: Check for too-large values
    let hour = seconds / SECONDS_IN_HOUR;
    seconds %= SECONDS_IN_HOUR;
    let min = seconds / SECONDS_IN_MINUTE;
    let sec = seconds % SECONDS_IN_MINUTE;
    (hour, min, sec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{apply, template};

    #[template]
    #[rstest]
    #[case(-1403, -4716, 2, 28)]
    #[case(-1402, -4716, 2, 29)]
    #[case(-1401, -4716, 3, 1)]
    #[case(-1095, -4715, 1, 1)]
    #[case(-730, -4714, 1, 1)]
    #[case(-365, -4713, 1, 1)]
    #[case(-1, -4713, 12, 31)]
    #[case(0, -4712, 1, 1)]
    #[case(366, -4711, 1, 1)]
    #[case(1719656, -4, 2, 29)]
    #[case(2159358, 1200, 1, 1)]
    #[case(2195883, 1300, 1, 1)]
    #[case(2232408, 1400, 1, 1)]
    #[case(2268932, 1499, 12, 31)]
    #[case(2268933, 1500, 1, 1)]
    #[case(2268993, 1500, 3, 1)]
    #[case(2269115, 1500, 7, 1)]
    #[case(2269146, 1500, 8, 1)]
    #[case(2269177, 1500, 9, 1)]
    #[case(2269204, 1500, 9, 28)]
    #[case(2269205, 1500, 9, 29)]
    #[case(2269207, 1500, 10, 1)]
    #[case(2269298, 1500, 12, 31)]
    #[case(2269299, 1501, 1, 1)]
    #[case(2269663, 1501, 12, 31)]
    #[case(2270123, 1503, 4, 5)]
    #[case(2270489, 1504, 4, 5)]
    #[case(2270854, 1505, 4, 5)]
    #[case(2271219, 1506, 4, 5)]
    #[case(2271584, 1507, 4, 5)]
    #[case(2298796, 1581, 10, 5)]
    #[case(2298883, 1581, 12, 31)]
    #[case(2298884, 1582, 1, 1)]
    #[case(2299159, 1582, 10, 3)]
    #[case(2299160, 1582, 10, 4)]
    #[case(2299161, 1582, 10, 15)]
    #[case(2299162, 1582, 10, 16)]
    #[case(2299238, 1582, 12, 31)]
    #[case(2299239, 1583, 1, 1)]
    #[case(2305448, 1600, 1, 1)]
    #[case(2341972, 1699, 12, 31)]
    #[case(2341973, 1700, 1, 1)]
    #[case(2342337, 1700, 12, 31)]
    #[case(2342338, 1701, 1, 1)]
    #[case(2378496, 1799, 12, 31)]
    #[case(2378497, 1800, 1, 1)]
    #[case(2378861, 1800, 12, 31)]
    #[case(2378862, 1801, 1, 1)]
    #[case(2415020, 1899, 12, 31)]
    #[case(2415021, 1900, 1, 1)]
    #[case(2415385, 1900, 12, 31)]
    #[case(2415386, 1901, 1, 1)]
    #[case(2440588, 1970, 1, 1)]
    #[case(2451544, 1999, 12, 31)]
    #[case(2451545, 2000, 1, 1)]
    #[case(2451605, 2000, 3, 1)]
    #[case(2451910, 2000, 12, 31)]
    #[case(2451911, 2001, 1, 1)]
    #[case(2453066, 2004, 3, 1)]
    #[case(2456746, 2014, 3, 29)]
    #[case(2460055, 2023, 4, 20)]
    fn julian_days(
        #[case] days: JulianDayT,
        #[case] year: YearT,
        #[case] month: u32,
        #[case] mday: u32,
    ) {
    }

    #[apply(julian_days)]
    fn test_julian_to_calendar(
        #[case] days: JulianDayT,
        #[case] year: YearT,
        #[case] month: u32,
        #[case] mday: u32,
    ) {
        let jd = JulianDate {
            days,
            seconds: None,
        };
        let cal = jd.to_gregorian();
        assert_eq!(cal.year, year);
        assert_eq!(cal.month, month);
        assert_eq!(cal.mday, mday);
    }

    #[apply(julian_days)]
    fn test_calendar_to_julian(
        #[case] days: JulianDayT,
        #[case] year: YearT,
        #[case] month: u32,
        #[case] mday: u32,
    ) {
        let cal = Date::from_ymd(year, month, mday, None).unwrap();
        let jd = JulianDate {
            days,
            seconds: None,
        };
        assert_eq!(cal.to_julian_date(), jd);
    }

    #[test]
    fn test_calendar_with_seconds_to_julian() {
        let cal = Date::from_ymd(2023, 4, 20, Some(16 * 3600 + 18 * 60 + 44)).unwrap();
        let jd = JulianDate {
            days: 2460055,
            seconds: Some(4 * 3600 + 18 * 60 + 44),
        };
        assert_eq!(cal.to_julian_date(), jd);
    }

    #[rstest]
    #[case(0, (0, 0, 0))]
    #[case(27013, (7, 30, 13))]
    #[case(43200, (12, 0, 0))]
    #[case(45296, (12, 34, 56))]
    #[case(47583, (13, 13, 3))]
    #[case(61504, (17, 5, 4))]
    #[case(86399, (23, 59, 59))]
    fn test_break_seconds(#[case] seconds: SecondsT, #[case] hms: (u32, u32, u32)) {
        assert_eq!(break_seconds(seconds), hms);
    }

    #[test]
    fn test_parse_ymd() {
        let date = "2023-04-20".parse::<Date>().unwrap();
        assert_eq!(date.year, 2023);
        assert_eq!(date.month, 4);
        assert_eq!(date.mday, 20);
        assert_eq!(date.seconds, None);
    }

    #[test]
    fn test_parse_ymd_hms() {
        let date = "2023-04-20T16:39:50".parse::<Date>().unwrap();
        assert_eq!(date.year, 2023);
        assert_eq!(date.month, 4);
        assert_eq!(date.mday, 20);
        assert_eq!(date.seconds, Some(16 * 3600 + 39 * 60 + 50));
    }

    #[test]
    fn test_parse_ymd_hms_z() {
        let date = "2023-04-20T16:39:50Z".parse::<Date>().unwrap();
        assert_eq!(date.year, 2023);
        assert_eq!(date.month, 4);
        assert_eq!(date.mday, 20);
        assert_eq!(date.seconds, Some(16 * 3600 + 39 * 60 + 50));
    }

    #[test]
    fn test_parse_julian_date() {
        let jd = "2460055".parse::<JulianDate>().unwrap();
        assert_eq!(
            jd,
            JulianDate {
                days: 2460055,
                seconds: None
            }
        );
    }

    #[test]
    fn test_parse_julian_date_dot_seconds() {
        let jd = "2460055.1962".parse::<JulianDate>().unwrap();
        assert_eq!(
            jd,
            JulianDate {
                days: 2460055,
                seconds: Some(16952)
            }
        );
    }

    #[test]
    fn test_parse_julian_date_colon_seconds() {
        let jd = "2460055:16952".parse::<JulianDate>().unwrap();
        assert_eq!(
            jd,
            JulianDate {
                days: 2460055,
                seconds: Some(16952)
            }
        );
    }
}
