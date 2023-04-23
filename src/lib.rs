//! Unless otherwise specified, all functions use a Gregorian calendar with the
//! Reformation taking place on 1582-10-05/15.

#[cfg(test)]
extern crate rstest_reuse;

use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

pub type YearT = i32;
pub type DaysT = u32;
pub type SecondsT = u32;
pub type JulianDayT = i32;

pub const GREG_REFORM: JulianDayT = 2299161; // noon on 1582-10-15
pub const UK_REFORM: JulianDayT = 2361222; // noon on 1752-09-14

const SECONDS_IN_MINUTE: SecondsT = 60;
const SECONDS_IN_HOUR: SecondsT = 60 * SECONDS_IN_MINUTE;
const SECONDS_IN_HALF_DAY: SecondsT = 12 * SECONDS_IN_HOUR;
const SECONDS_IN_DAY: SecondsT = 24 * SECONDS_IN_HOUR;

const REFORM_YEAR: YearT = 1582;
const REFORM_YDAY: DaysT = 277; // zero-index yday for Oct 5
const REFORM_MONTH: u32 = 10;
const PRE_REFORM_MDAY: u32 = 4;
const POST_REFORM_MDAY: u32 = 15;
const REFORM_GAP: JulianDayT = 10;
const REFORM_MONTH_LENGTH: u32 = 21;
const START1583: JulianDayT = GREG_REFORM + 78; // noon on 1583-01-01
const START1600: JulianDayT = 2305448; // noon on 1600-01-01

// Julian-calendar year in which Julian day 0 occurs
const JD0_YEAR: YearT = -4712;

const COMMON_YEAR_LENGTH: JulianDayT = 365;
const LEAP_YEAR_LENGTH: JulianDayT = 366;

static MONTH_LENGTHS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

const FEBRUARY: u32 = 2;

const GREGORIAN_CYCLE_DAYS: JulianDayT = 146097;
const GREGORIAN_CYCLE_YEARS: YearT = 400;
const GREGORIAN_CYCLE_START_YEAR: YearT = 1600;

const JULIAN_CYCLE_DAYS: JulianDayT = 1461;
const JULIAN_CYCLE_YEARS: YearT = 4;

const UNIX_EPOCH_JD: JulianDayT = 2440588; // noon on 1970-01-01

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
        Date::from(SystemTime::now())
    }

    pub fn from_unix_timestamp(unix_time: u64) -> Date {
        let days =
            JulianDayT::try_from(unix_time / (SECONDS_IN_DAY as u64) + (UNIX_EPOCH_JD as u64))
                .expect("Arithmetic overflow");
        let secs = SecondsT::try_from(unix_time % (SECONDS_IN_DAY as u64)).unwrap();
        let jd = JulianDate {
            days,
            seconds: None,
        };
        jd.attach_seconds(Some(secs)).to_gregorian()
    }

    pub fn is_before_gregorian(&self) -> bool {
        self.year < REFORM_YEAR || (self.year == REFORM_YEAR && self.yday < REFORM_YDAY)
    }

    /// `month` and `mday` are one-indexed
    pub fn from_ymd(
        year: YearT,
        month: u32,
        mday: u32,
        seconds: Option<SecondsT>,
    ) -> Result<Date, DateError> {
        if !(1..=12).contains(&month) {
            return Err(DateError::MonthOutOfRange { month });
        }
        let month_index = usize::try_from(month).unwrap() - 1;
        if mday < 1
            || (mday > MONTH_LENGTHS[month_index]
                && !(is_leap_year(year) && month == FEBRUARY && mday == 29))
        {
            return Err(DateError::MdayOutOfRange { month, mday });
        }
        let mut yday = 0;
        for (&length, i) in MONTH_LENGTHS[0..month_index].iter().zip(1..) {
            yday += length;
            if i == FEBRUARY && is_leap_year(year) {
                yday += 1;
            }
        }
        yday += mday - 1;
        if year == REFORM_YEAR {
            if month > REFORM_MONTH || (month == REFORM_MONTH && mday >= POST_REFORM_MDAY) {
                yday -= REFORM_GAP as DaysT;
            } else if month == REFORM_MONTH
                && ((PRE_REFORM_MDAY + 1)..POST_REFORM_MDAY).contains(&mday)
            {
                return Err(DateError::SkippedDate);
            }
        }
        Ok(Date {
            year,
            yday,
            month,
            mday,
            seconds,
        })
    }

    // Uses Gregorian calendar
    pub fn from_year_yday(
        year: YearT,
        yday: DaysT,
        seconds: Option<SecondsT>,
    ) -> Result<Date, DateError> {
        let year_style = YearStyle::for_gregorian_year(year);
        let (month, mday) = year_style
            .break_yday(yday)
            .ok_or(DateError::YdayOutOfRange { year, yday })?;
        Ok(Date {
            year,
            yday,
            month,
            mday,
            seconds,
        })
    }

    pub fn from_julian_year_yday(
        year: YearT,
        yday: DaysT,
        seconds: Option<SecondsT>,
    ) -> Result<Date, DateError> {
        let year_style = YearStyle::for_julian_year(year);
        let (month, mday) = year_style
            .break_yday(yday)
            .ok_or(DateError::YdayOutOfRange { year, yday })?;
        Ok(Date {
            year,
            yday,
            month,
            mday,
            seconds,
        })
    }

    pub fn to_julian_date(&self) -> JulianDate {
        let idays = JulianDayT::try_from(self.yday).unwrap();
        let jdays: JulianDayT = if self.year < JD0_YEAR {
            let rev_year = JD0_YEAR - self.year;
            idays - (rev_year * COMMON_YEAR_LENGTH + rev_year / JULIAN_CYCLE_YEARS)
        } else if self.is_before_gregorian() {
            (self.year - JD0_YEAR) * COMMON_YEAR_LENGTH
                + (self.year - JD0_YEAR + JULIAN_CYCLE_YEARS - 1) / JULIAN_CYCLE_YEARS
                + idays
        } else if self.year == REFORM_YEAR {
            GREG_REFORM + idays.checked_sub_unsigned(REFORM_YDAY).unwrap()
        } else {
            let base = self.year - 1201;
            let days_since_1582 = (base - 382) * COMMON_YEAR_LENGTH
                + (base - 380) / JULIAN_CYCLE_YEARS
                - (base - 300) / 100
                + base / GREGORIAN_CYCLE_YEARS;
            START1583 + days_since_1582 + idays
        };
        let jd = JulianDate {
            days: jdays,
            seconds: None,
        };
        jd.attach_seconds(self.seconds)
    }
}

impl From<SystemTime> for Date {
    fn from(t: SystemTime) -> Date {
        Date::from_unix_timestamp(
            t.duration_since(UNIX_EPOCH)
                .expect("Current system time is before 1970")
                .as_secs(),
        )
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
            return Err(DateParseError::HasTrailing);
        }
        match diny {
            DayInYear::Yday(yday) => Ok(Date::from_year_yday(year, yday, seconds)?),
            DayInYear::Date { month, mday } => Ok(Date::from_ymd(year, month, mday, seconds)?),
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
pub enum DateError {
    #[error("month {month} is outside of valid range")]
    MonthOutOfRange { month: u32 },
    #[error("mday {mday} is outside of valid range for month {month}")]
    MdayOutOfRange { month: u32, mday: u32 },
    #[error("yday {yday} is outside of valid range for year {year}")]
    YdayOutOfRange { year: YearT, yday: DaysT },
    #[error("date was skipped by calendar reform")]
    SkippedDate,
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum DateParseError {
    #[error("invalid calendar date: {0}")]
    InvalidDate(#[from] DateError),
    #[error("trailing characters after date")]
    HasTrailing,
    #[error("year not terminated by '-'")]
    UnterminatedYear,
    #[error("yday must be three digits long")]
    InvalidYday,
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
    #[error("expected time segment or end of input, got {got:?}")]
    BadTimeStart { got: char },
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
            None => Err(DateParseError::UnterminatedYear),
        }
    }

    fn parse_day_in_year(&mut self) -> Result<DayInYear, DateParseError> {
        if let Some(i) = self.data.find(['-', 'T']) {
            if &self.data[i..(i + 1)] == "T" {
                let yday = Self::parse_yday(&self.data[..i])?;
                self.data = &self.data[i..];
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
            Ok(DayInYear::Yday(s.parse::<DaysT>()? - 1))
        } else {
            Err(DateParseError::InvalidYday)
        }
    }

    fn parse_02d(&mut self) -> Result<u32, DateParseError> {
        match self
            .data
            .char_indices()
            .take_while(|&(_, ch)| ch.is_ascii_digit())
            .last()
            .map(|(i, _)| i + 1)
        {
            Some(2) => {
                let n = self.data[..2].parse::<u32>().unwrap();
                self.data = &self.data[2..];
                Ok(n)
            }
            Some(got) => Err(DateParseError::Invalid02dLength { got }),
            None => match self.data.chars().next() {
                Some(got) => Err(DateParseError::Invalid02dStart { got }),
                None => Err(DateParseError::Invalid02dSuddenEnd),
            },
        }
    }

    fn scan_char(&mut self, ch: char) -> Result<(), DateParseError> {
        if let Some(s) = self.data.strip_prefix(ch) {
            self.data = s;
            Ok(())
        } else {
            match self.data.chars().next() {
                Some(c2) => Err(DateParseError::UnexpectedChar {
                    expected: ch,
                    got: c2,
                }),
                None => Err(DateParseError::UnexpectedEnd { expected: ch }),
            }
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
            let ch = self.data.chars().next().unwrap();
            Err(DateParseError::BadTimeStart { got: ch })
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
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
        if year == REFORM_YEAR {
            YearStyle::Reform
        } else if is_leap_year(year) {
            YearStyle::Leap
        } else {
            YearStyle::Common
        }
    }

    pub fn for_julian_year(year: YearT) -> YearStyle {
        if year % JULIAN_CYCLE_YEARS == 0 {
            YearStyle::Leap
        } else {
            YearStyle::Common
        }
    }

    pub fn break_yday(&self, yday: DaysT) -> Option<(u32, u32)> {
        let mut days = yday;
        for (&length, month) in MONTH_LENGTHS.iter().zip(1..) {
            let length = match (self, month) {
                (YearStyle::Leap, FEBRUARY) => length + 1,
                (YearStyle::Reform, REFORM_MONTH) => {
                    let length = REFORM_MONTH_LENGTH;
                    if days < length {
                        if PRE_REFORM_MDAY <= days {
                            days += REFORM_GAP as DaysT;
                        }
                        return Some((month, days + 1));
                    }
                    length
                }
                _ => length,
            };
            if days < length {
                return Some((month, days + 1));
            }
            days -= length;
        }
        None
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct JulianDate {
    pub days: JulianDayT,
    pub seconds: Option<SecondsT>,
}

impl JulianDate {
    const DEFAULT_PRECISION: usize = 6;

    pub fn attach_seconds(&self, seconds: Option<SecondsT>) -> JulianDate {
        let JulianDate { days, .. } = *self;
        match seconds {
            None => JulianDate {
                days,
                seconds: None,
            },
            Some(s) if s < SECONDS_IN_HALF_DAY => JulianDate {
                days: days - 1,
                seconds: Some(s + SECONDS_IN_HALF_DAY),
            },
            Some(s) => JulianDate {
                days,
                seconds: Some(s - SECONDS_IN_HALF_DAY),
            },
        }
    }

    /// If the Julian date has any seconds, remove them and adjust the date to
    /// point to the "secondless" Julian day that the receiver pointed to
    pub fn detatch_seconds(&self) -> (JulianDate, Option<SecondsT>) {
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
        let (JulianDate { days, .. }, seconds) = self.detatch_seconds();
        if days < START1600 {
            let days = if GREG_REFORM <= days {
                days + REFORM_GAP
            } else {
                days
            };
            let when = (JulianDate {
                days,
                seconds: None,
            })
            .to_julian();
            Date::from_year_yday(
                when.year,
                if GREG_REFORM <= days && days - REFORM_GAP < START1583 {
                    when.yday - (REFORM_GAP as DaysT)
                } else {
                    when.yday
                },
                seconds,
            )
            .unwrap()
        } else {
            let mut days: JulianDayT = days - START1600;
            let mut year: YearT =
                GREGORIAN_CYCLE_START_YEAR + (days / GREGORIAN_CYCLE_DAYS) * GREGORIAN_CYCLE_YEARS;
            days %= GREGORIAN_CYCLE_DAYS;
            // Add a "virtual leap day" to the end of each non-Gregorian
            // centennial year so that `days` can then be handled as in the
            // Julian calendar:
            if days > COMMON_YEAR_LENGTH {
                days += (days - LEAP_YEAR_LENGTH) / 36524;
            }
            year += (days / JULIAN_CYCLE_DAYS) * JULIAN_CYCLE_YEARS;
            days %= JULIAN_CYCLE_DAYS;
            if days > COMMON_YEAR_LENGTH {
                days += (days - LEAP_YEAR_LENGTH) / COMMON_YEAR_LENGTH;
            }
            year += days / LEAP_YEAR_LENGTH;
            days %= LEAP_YEAR_LENGTH;
            Date::from_year_yday(year, DaysT::try_from(days).unwrap(), seconds).unwrap()
        }
    }

    /// Convert a Julian date to a year & yday in the Julian calendar
    pub fn to_julian(&self) -> Date {
        let (JulianDate { days, .. }, seconds) = self.detatch_seconds();
        if days < 0 {
            let alt = JulianDate {
                days: COMMON_YEAR_LENGTH
                    .checked_sub(days)
                    .expect("Arithmetic underflow"),
                seconds: None,
            };
            let when = alt.to_julian();
            let year = JD0_YEAR - (when.year - JD0_YEAR);
            let year_length = if year % JULIAN_CYCLE_YEARS == 0 {
                LEAP_YEAR_LENGTH as DaysT
            } else {
                COMMON_YEAR_LENGTH as DaysT
            };
            let yday = year_length - 1 - when.yday;
            Date::from_julian_year_yday(year, yday, seconds).unwrap()
        } else {
            let mut year: YearT = days / JULIAN_CYCLE_DAYS * JULIAN_CYCLE_YEARS;
            let mut yday: JulianDayT = days % JULIAN_CYCLE_DAYS;
            // Add a "virtual leap day" to the end of each common year so that
            // `yday` can be divided & modded by LEAP_YEAR_LENGTH evenly:
            if yday > COMMON_YEAR_LENGTH {
                yday += (yday - LEAP_YEAR_LENGTH) / COMMON_YEAR_LENGTH;
            }
            year += yday / LEAP_YEAR_LENGTH;
            yday %= LEAP_YEAR_LENGTH;
            year += JD0_YEAR;
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
        let days = days_str
            .parse::<JulianDayT>()
            .map_err(JulianDateParseError::InvalidDay)?;
        let seconds = match m {
            Some((_, ":")) => Some(
                secstr
                    .parse::<SecondsT>()
                    .map_err(JulianDateParseError::InvalidSeconds)?,
            ),
            Some((_, ".")) => {
                let mut secs = 0;
                let mut coef: SecondsT = SECONDS_IN_DAY / 10;
                let mut accum = 0;
                let mut denom = 1;
                for ch in secstr.chars() {
                    let d = ch
                        .to_digit(10)
                        .ok_or(JulianDateParseError::InvalidDigit { ch })?;
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
                let precision = f.precision().unwrap_or(Self::DEFAULT_PRECISION);
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
    #[error("cannot parse Julian day component")]
    InvalidDay(#[source] ParseIntError),
    #[error("invalid digit {ch:?} in Julian date")]
    InvalidDigit { ch: char },
    #[error("cannot parse integer seconds component")]
    InvalidSeconds(#[source] ParseIntError),
}

fn is_leap_year(year: YearT) -> bool {
    year % JULIAN_CYCLE_YEARS == 0
        && (year <= REFORM_YEAR || year % 100 != 0 || year % GREGORIAN_CYCLE_YEARS == 0)
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
    use YearStyle::*;

    #[template]
    #[rstest]
    #[case(-2298701, -11006, 7, 2)]
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
        assert_eq!(cal.seconds, None);
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
    fn test_calendar_with_seconds_before_noon_to_julian() {
        let cal = Date::from_ymd(2023, 4, 20, Some(7 * 3600 + 30 * 60 + 13)).unwrap();
        let jd = JulianDate {
            days: 2460054,
            seconds: Some(19 * 3600 + 30 * 60 + 13),
        };
        assert_eq!(cal.to_julian_date(), jd);
    }

    #[test]
    fn test_calendar_with_seconds_after_noon_to_julian() {
        let cal = Date::from_ymd(2023, 4, 20, Some(16 * 3600 + 18 * 60 + 44)).unwrap();
        let jd = JulianDate {
            days: 2460055,
            seconds: Some(4 * 3600 + 18 * 60 + 44),
        };
        assert_eq!(cal.to_julian_date(), jd);
    }

    #[test]
    fn test_julian_with_seconds_before_midnight_to_calendar() {
        let jd = JulianDate {
            days: 2460055,
            seconds: Some(4 * 3600 + 18 * 60 + 44),
        };
        let cal = jd.to_gregorian();
        assert_eq!(cal.year, 2023);
        assert_eq!(cal.month, 4);
        assert_eq!(cal.mday, 20);
        assert_eq!(cal.yday, 109);
        assert_eq!(cal.seconds, Some(16 * 3600 + 18 * 60 + 44));
    }

    #[test]
    fn test_julian_with_seconds_after_midnight_to_calendar() {
        let jd = JulianDate {
            days: 2460054,
            seconds: Some(19 * 3600 + 30 * 60 + 13),
        };
        let cal = jd.to_gregorian();
        assert_eq!(cal.year, 2023);
        assert_eq!(cal.month, 4);
        assert_eq!(cal.mday, 20);
        assert_eq!(cal.yday, 109);
        assert_eq!(cal.seconds, Some(7 * 3600 + 30 * 60 + 13));
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

    mod parse_date {
        use super::*;

        #[test]
        fn test_parse_ymd() {
            let date = "2023-04-20".parse::<Date>().unwrap();
            assert_eq!(date.year, 2023);
            assert_eq!(date.month, 4);
            assert_eq!(date.mday, 20);
            assert_eq!(date.yday, 109);
            assert_eq!(date.seconds, None);
        }

        #[test]
        fn test_parse_ymd_hms() {
            let date = "2023-04-20T16:39:50".parse::<Date>().unwrap();
            assert_eq!(date.year, 2023);
            assert_eq!(date.month, 4);
            assert_eq!(date.mday, 20);
            assert_eq!(date.yday, 109);
            assert_eq!(date.seconds, Some(16 * 3600 + 39 * 60 + 50));
        }

        #[test]
        fn test_parse_ymd_hms_z() {
            let date = "2023-04-20T16:39:50Z".parse::<Date>().unwrap();
            assert_eq!(date.year, 2023);
            assert_eq!(date.month, 4);
            assert_eq!(date.mday, 20);
            assert_eq!(date.yday, 109);
            assert_eq!(date.seconds, Some(16 * 3600 + 39 * 60 + 50));
        }

        #[test]
        fn test_parse_yj() {
            let date = "2023-110".parse::<Date>().unwrap();
            assert_eq!(date.year, 2023);
            assert_eq!(date.month, 4);
            assert_eq!(date.mday, 20);
            assert_eq!(date.yday, 109);
            assert_eq!(date.seconds, None);
        }

        #[test]
        fn test_parse_yj_padded() {
            let date = "2023-006".parse::<Date>().unwrap();
            assert_eq!(date.year, 2023);
            assert_eq!(date.month, 1);
            assert_eq!(date.mday, 6);
            assert_eq!(date.yday, 5);
            assert_eq!(date.seconds, None);
        }

        #[test]
        fn test_parse_yj_hms() {
            let date = "2023-110T16:39:50".parse::<Date>().unwrap();
            assert_eq!(date.year, 2023);
            assert_eq!(date.month, 4);
            assert_eq!(date.mday, 20);
            assert_eq!(date.yday, 109);
            assert_eq!(date.seconds, Some(16 * 3600 + 39 * 60 + 50));
        }

        #[test]
        fn test_parse_yj_hms_z() {
            let date = "2023-110T16:39:50Z".parse::<Date>().unwrap();
            assert_eq!(date.year, 2023);
            assert_eq!(date.month, 4);
            assert_eq!(date.mday, 20);
            assert_eq!(date.yday, 109);
            assert_eq!(date.seconds, Some(16 * 3600 + 39 * 60 + 50));
        }

        #[test]
        fn test_parse_negative_ymd() {
            let date = "-2023-04-20".parse::<Date>().unwrap();
            assert_eq!(date.year, -2023);
            assert_eq!(date.month, 4);
            assert_eq!(date.mday, 20);
            assert_eq!(date.yday, 109);
            assert_eq!(date.seconds, None);
        }

        #[test]
        fn test_parse_ymd_short_year() {
            // TODO: Should this be an error instead?
            let date = "20-04-20".parse::<Date>().unwrap();
            assert_eq!(date.year, 20);
            assert_eq!(date.month, 4);
            assert_eq!(date.mday, 20);
            assert_eq!(date.yday, 110);
            assert_eq!(date.seconds, None);
        }

        #[test]
        fn test_parse_date_short_yday() {
            let r = "1234-56".parse::<Date>();
            assert_eq!(r, Err(DateParseError::InvalidYday));
            assert_eq!(r.unwrap_err().to_string(), "yday must be three digits long");
        }

        #[test]
        fn test_parse_date_long_yday() {
            let r = "1234-5678".parse::<Date>();
            assert_eq!(r, Err(DateParseError::InvalidYday));
            assert_eq!(r.unwrap_err().to_string(), "yday must be three digits long");
        }

        #[test]
        fn test_parse_ymd_short_month() {
            let r = "2023-4-20".parse::<Date>();
            assert_eq!(r, Err(DateParseError::Invalid02dLength { got: 1 }));
            assert_eq!(
                r.unwrap_err().to_string(),
                "expected two digits, got 1 digits"
            );
        }

        #[test]
        fn test_parse_ymd_long_month() {
            let r = "2023-012-20".parse::<Date>();
            assert_eq!(r, Err(DateParseError::Invalid02dLength { got: 3 }));
            assert_eq!(
                r.unwrap_err().to_string(),
                "expected two digits, got 3 digits"
            );
        }

        #[test]
        fn test_parse_ymd_short_mday() {
            let r = "2023-04-2".parse::<Date>();
            assert_eq!(r, Err(DateParseError::Invalid02dLength { got: 1 }));
            assert_eq!(
                r.unwrap_err().to_string(),
                "expected two digits, got 1 digits"
            );
        }

        #[test]
        fn test_parse_ymd_hms_bad_time_sep() {
            // TODO: Support this format:
            let r = "2023-04-20 16:39:50".parse::<Date>();
            assert_eq!(r, Err(DateParseError::BadTimeStart { got: ' ' }));
            assert_eq!(
                r.unwrap_err().to_string(),
                "expected time segment or end of input, got ' '"
            );
        }

        #[test]
        fn test_parse_ymd_hms_bad_timezone_spec() {
            let r = "2023-04-20T16:39:50+00:00".parse::<Date>();
            assert_eq!(r, Err(DateParseError::HasTrailing));
            assert_eq!(r.unwrap_err().to_string(), "trailing characters after date");
        }

        #[test]
        fn test_parse_year_hyphen() {
            let r = "2023-".parse::<Date>();
            assert_eq!(r, Err(DateParseError::InvalidYday));
            assert_eq!(r.unwrap_err().to_string(), "yday must be three digits long");
        }

        #[test]
        fn test_parse_year_month_hyphen() {
            let r = "2023-04-".parse::<Date>();
            assert_eq!(r, Err(DateParseError::Invalid02dSuddenEnd));
            assert_eq!(
                r.unwrap_err().to_string(),
                "expected two digits, reached end of input"
            );
        }

        #[test]
        fn test_parse_year_hyphen_hyphen_md() {
            let r = "2023--04-20".parse::<Date>();
            assert_eq!(r, Err(DateParseError::Invalid02dStart { got: '-' }));
            assert_eq!(
                r.unwrap_err().to_string(),
                "expected two digits, got non-digit '-'"
            );
        }

        #[test]
        fn test_parse_zero_month() {
            let r = "2023-00-13".parse::<Date>();
            assert_eq!(
                r,
                Err(DateParseError::InvalidDate(DateError::MonthOutOfRange {
                    month: 0
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: month 0 is outside of valid range"
            );
        }

        #[test]
        fn test_parse_smarch() {
            let r = "2023-13-13".parse::<Date>();
            assert_eq!(
                r,
                Err(DateParseError::InvalidDate(DateError::MonthOutOfRange {
                    month: 13
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: month 13 is outside of valid range"
            );
        }

        #[test]
        fn test_parse_mday_0() {
            let r = "2023-04-00".parse::<Date>();
            assert_eq!(
                r,
                Err(DateParseError::InvalidDate(DateError::MdayOutOfRange {
                    month: 4,
                    mday: 0
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: mday 0 is outside of valid range for month 4"
            );
        }

        #[test]
        fn test_parse_mday_32() {
            let r = "2023-04-32".parse::<Date>();
            assert_eq!(
                r,
                Err(DateParseError::InvalidDate(DateError::MdayOutOfRange {
                    month: 4,
                    mday: 32
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: mday 32 is outside of valid range for month 4"
            );
        }

        #[test]
        fn test_parse_sep_31() {
            let r = "2023-09-31".parse::<Date>();
            assert_eq!(
                r,
                Err(DateParseError::InvalidDate(DateError::MdayOutOfRange {
                    month: 9,
                    mday: 31
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: mday 31 is outside of valid range for month 9"
            );
        }

        #[test]
        fn test_parse_invalid_leap_day() {
            let r = "2023-02-29".parse::<Date>();
            assert_eq!(
                r,
                Err(DateParseError::InvalidDate(DateError::MdayOutOfRange {
                    month: 2,
                    mday: 29
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: mday 29 is outside of valid range for month 2"
            );
        }

        #[test]
        fn test_parse_valid_leap_day() {
            let date = "2024-02-29".parse::<Date>().unwrap();
            assert_eq!(date.year, 2024);
            assert_eq!(date.month, 2);
            assert_eq!(date.mday, 29);
            assert_eq!(date.seconds, None);
        }

        #[test]
        fn test_parse_skipped_date() {
            let r = "1582-10-10".parse::<Date>();
            assert_eq!(r, Err(DateParseError::InvalidDate(DateError::SkippedDate)));
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: date was skipped by calendar reform"
            );
        }

        #[test]
        fn test_parse_first_skipped_date() {
            let r = "1582-10-05".parse::<Date>();
            assert_eq!(r, Err(DateParseError::InvalidDate(DateError::SkippedDate)));
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: date was skipped by calendar reform"
            );
        }

        #[test]
        fn test_parse_last_skipped_date() {
            let r = "1582-10-14".parse::<Date>();
            assert_eq!(r, Err(DateParseError::InvalidDate(DateError::SkippedDate)));
            assert_eq!(
                r.unwrap_err().to_string(),
                "invalid calendar date: date was skipped by calendar reform"
            );
        }

        #[rstest]
        #[case(2023, 366)]
        #[case(2023, 999)]
        #[case(2024, 367)]
        #[case(2024, 999)]
        #[case(1582, 356)]
        #[case(1582, 999)]
        fn test_parse_invalid_yj(#[case] year: YearT, #[case] yday: DaysT) {
            let r = format!("{year:04}-{yday:03}").parse::<Date>();
            assert_eq!(
                r,
                Err(DateParseError::InvalidDate(DateError::YdayOutOfRange {
                    year,
                    yday: yday - 1
                }))
            );
            assert_eq!(
                r.unwrap_err().to_string(),
                format!(
                    "invalid calendar date: yday {} is outside of valid range for year {year}",
                    yday - 1
                )
            );
        }
    }

    mod parse_julian_date {
        use super::*;

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

    #[rstest]
    #[case(Common, 0, 1, 1)]
    #[case(Common, 1, 1, 2)]
    #[case(Common, 2, 1, 3)]
    #[case(Common, 3, 1, 4)]
    #[case(Common, 4, 1, 5)]
    #[case(Common, 5, 1, 6)]
    #[case(Common, 6, 1, 7)]
    #[case(Common, 7, 1, 8)]
    #[case(Common, 8, 1, 9)]
    #[case(Common, 9, 1, 10)]
    #[case(Common, 10, 1, 11)]
    #[case(Common, 11, 1, 12)]
    #[case(Common, 12, 1, 13)]
    #[case(Common, 13, 1, 14)]
    #[case(Common, 14, 1, 15)]
    #[case(Common, 15, 1, 16)]
    #[case(Common, 16, 1, 17)]
    #[case(Common, 17, 1, 18)]
    #[case(Common, 18, 1, 19)]
    #[case(Common, 19, 1, 20)]
    #[case(Common, 20, 1, 21)]
    #[case(Common, 21, 1, 22)]
    #[case(Common, 22, 1, 23)]
    #[case(Common, 23, 1, 24)]
    #[case(Common, 24, 1, 25)]
    #[case(Common, 25, 1, 26)]
    #[case(Common, 26, 1, 27)]
    #[case(Common, 27, 1, 28)]
    #[case(Common, 28, 1, 29)]
    #[case(Common, 29, 1, 30)]
    #[case(Common, 30, 1, 31)]
    #[case(Common, 31, 2, 1)]
    #[case(Common, 32, 2, 2)]
    #[case(Common, 33, 2, 3)]
    #[case(Common, 34, 2, 4)]
    #[case(Common, 35, 2, 5)]
    #[case(Common, 36, 2, 6)]
    #[case(Common, 37, 2, 7)]
    #[case(Common, 38, 2, 8)]
    #[case(Common, 39, 2, 9)]
    #[case(Common, 40, 2, 10)]
    #[case(Common, 41, 2, 11)]
    #[case(Common, 42, 2, 12)]
    #[case(Common, 43, 2, 13)]
    #[case(Common, 44, 2, 14)]
    #[case(Common, 45, 2, 15)]
    #[case(Common, 46, 2, 16)]
    #[case(Common, 47, 2, 17)]
    #[case(Common, 48, 2, 18)]
    #[case(Common, 49, 2, 19)]
    #[case(Common, 50, 2, 20)]
    #[case(Common, 51, 2, 21)]
    #[case(Common, 52, 2, 22)]
    #[case(Common, 53, 2, 23)]
    #[case(Common, 54, 2, 24)]
    #[case(Common, 55, 2, 25)]
    #[case(Common, 56, 2, 26)]
    #[case(Common, 57, 2, 27)]
    #[case(Common, 58, 2, 28)]
    #[case(Common, 59, 3, 1)]
    #[case(Common, 60, 3, 2)]
    #[case(Common, 61, 3, 3)]
    #[case(Common, 62, 3, 4)]
    #[case(Common, 63, 3, 5)]
    #[case(Common, 64, 3, 6)]
    #[case(Common, 65, 3, 7)]
    #[case(Common, 66, 3, 8)]
    #[case(Common, 67, 3, 9)]
    #[case(Common, 68, 3, 10)]
    #[case(Common, 69, 3, 11)]
    #[case(Common, 70, 3, 12)]
    #[case(Common, 71, 3, 13)]
    #[case(Common, 72, 3, 14)]
    #[case(Common, 73, 3, 15)]
    #[case(Common, 74, 3, 16)]
    #[case(Common, 75, 3, 17)]
    #[case(Common, 76, 3, 18)]
    #[case(Common, 77, 3, 19)]
    #[case(Common, 78, 3, 20)]
    #[case(Common, 79, 3, 21)]
    #[case(Common, 80, 3, 22)]
    #[case(Common, 81, 3, 23)]
    #[case(Common, 82, 3, 24)]
    #[case(Common, 83, 3, 25)]
    #[case(Common, 84, 3, 26)]
    #[case(Common, 85, 3, 27)]
    #[case(Common, 86, 3, 28)]
    #[case(Common, 87, 3, 29)]
    #[case(Common, 88, 3, 30)]
    #[case(Common, 89, 3, 31)]
    #[case(Common, 90, 4, 1)]
    #[case(Common, 91, 4, 2)]
    #[case(Common, 92, 4, 3)]
    #[case(Common, 93, 4, 4)]
    #[case(Common, 94, 4, 5)]
    #[case(Common, 95, 4, 6)]
    #[case(Common, 96, 4, 7)]
    #[case(Common, 97, 4, 8)]
    #[case(Common, 98, 4, 9)]
    #[case(Common, 99, 4, 10)]
    #[case(Common, 100, 4, 11)]
    #[case(Common, 101, 4, 12)]
    #[case(Common, 102, 4, 13)]
    #[case(Common, 103, 4, 14)]
    #[case(Common, 104, 4, 15)]
    #[case(Common, 105, 4, 16)]
    #[case(Common, 106, 4, 17)]
    #[case(Common, 107, 4, 18)]
    #[case(Common, 108, 4, 19)]
    #[case(Common, 109, 4, 20)]
    #[case(Common, 110, 4, 21)]
    #[case(Common, 111, 4, 22)]
    #[case(Common, 112, 4, 23)]
    #[case(Common, 113, 4, 24)]
    #[case(Common, 114, 4, 25)]
    #[case(Common, 115, 4, 26)]
    #[case(Common, 116, 4, 27)]
    #[case(Common, 117, 4, 28)]
    #[case(Common, 118, 4, 29)]
    #[case(Common, 119, 4, 30)]
    #[case(Common, 120, 5, 1)]
    #[case(Common, 121, 5, 2)]
    #[case(Common, 122, 5, 3)]
    #[case(Common, 123, 5, 4)]
    #[case(Common, 124, 5, 5)]
    #[case(Common, 125, 5, 6)]
    #[case(Common, 126, 5, 7)]
    #[case(Common, 127, 5, 8)]
    #[case(Common, 128, 5, 9)]
    #[case(Common, 129, 5, 10)]
    #[case(Common, 130, 5, 11)]
    #[case(Common, 131, 5, 12)]
    #[case(Common, 132, 5, 13)]
    #[case(Common, 133, 5, 14)]
    #[case(Common, 134, 5, 15)]
    #[case(Common, 135, 5, 16)]
    #[case(Common, 136, 5, 17)]
    #[case(Common, 137, 5, 18)]
    #[case(Common, 138, 5, 19)]
    #[case(Common, 139, 5, 20)]
    #[case(Common, 140, 5, 21)]
    #[case(Common, 141, 5, 22)]
    #[case(Common, 142, 5, 23)]
    #[case(Common, 143, 5, 24)]
    #[case(Common, 144, 5, 25)]
    #[case(Common, 145, 5, 26)]
    #[case(Common, 146, 5, 27)]
    #[case(Common, 147, 5, 28)]
    #[case(Common, 148, 5, 29)]
    #[case(Common, 149, 5, 30)]
    #[case(Common, 150, 5, 31)]
    #[case(Common, 151, 6, 1)]
    #[case(Common, 152, 6, 2)]
    #[case(Common, 153, 6, 3)]
    #[case(Common, 154, 6, 4)]
    #[case(Common, 155, 6, 5)]
    #[case(Common, 156, 6, 6)]
    #[case(Common, 157, 6, 7)]
    #[case(Common, 158, 6, 8)]
    #[case(Common, 159, 6, 9)]
    #[case(Common, 160, 6, 10)]
    #[case(Common, 161, 6, 11)]
    #[case(Common, 162, 6, 12)]
    #[case(Common, 163, 6, 13)]
    #[case(Common, 164, 6, 14)]
    #[case(Common, 165, 6, 15)]
    #[case(Common, 166, 6, 16)]
    #[case(Common, 167, 6, 17)]
    #[case(Common, 168, 6, 18)]
    #[case(Common, 169, 6, 19)]
    #[case(Common, 170, 6, 20)]
    #[case(Common, 171, 6, 21)]
    #[case(Common, 172, 6, 22)]
    #[case(Common, 173, 6, 23)]
    #[case(Common, 174, 6, 24)]
    #[case(Common, 175, 6, 25)]
    #[case(Common, 176, 6, 26)]
    #[case(Common, 177, 6, 27)]
    #[case(Common, 178, 6, 28)]
    #[case(Common, 179, 6, 29)]
    #[case(Common, 180, 6, 30)]
    #[case(Common, 181, 7, 1)]
    #[case(Common, 182, 7, 2)]
    #[case(Common, 183, 7, 3)]
    #[case(Common, 184, 7, 4)]
    #[case(Common, 185, 7, 5)]
    #[case(Common, 186, 7, 6)]
    #[case(Common, 187, 7, 7)]
    #[case(Common, 188, 7, 8)]
    #[case(Common, 189, 7, 9)]
    #[case(Common, 190, 7, 10)]
    #[case(Common, 191, 7, 11)]
    #[case(Common, 192, 7, 12)]
    #[case(Common, 193, 7, 13)]
    #[case(Common, 194, 7, 14)]
    #[case(Common, 195, 7, 15)]
    #[case(Common, 196, 7, 16)]
    #[case(Common, 197, 7, 17)]
    #[case(Common, 198, 7, 18)]
    #[case(Common, 199, 7, 19)]
    #[case(Common, 200, 7, 20)]
    #[case(Common, 201, 7, 21)]
    #[case(Common, 202, 7, 22)]
    #[case(Common, 203, 7, 23)]
    #[case(Common, 204, 7, 24)]
    #[case(Common, 205, 7, 25)]
    #[case(Common, 206, 7, 26)]
    #[case(Common, 207, 7, 27)]
    #[case(Common, 208, 7, 28)]
    #[case(Common, 209, 7, 29)]
    #[case(Common, 210, 7, 30)]
    #[case(Common, 211, 7, 31)]
    #[case(Common, 212, 8, 1)]
    #[case(Common, 213, 8, 2)]
    #[case(Common, 214, 8, 3)]
    #[case(Common, 215, 8, 4)]
    #[case(Common, 216, 8, 5)]
    #[case(Common, 217, 8, 6)]
    #[case(Common, 218, 8, 7)]
    #[case(Common, 219, 8, 8)]
    #[case(Common, 220, 8, 9)]
    #[case(Common, 221, 8, 10)]
    #[case(Common, 222, 8, 11)]
    #[case(Common, 223, 8, 12)]
    #[case(Common, 224, 8, 13)]
    #[case(Common, 225, 8, 14)]
    #[case(Common, 226, 8, 15)]
    #[case(Common, 227, 8, 16)]
    #[case(Common, 228, 8, 17)]
    #[case(Common, 229, 8, 18)]
    #[case(Common, 230, 8, 19)]
    #[case(Common, 231, 8, 20)]
    #[case(Common, 232, 8, 21)]
    #[case(Common, 233, 8, 22)]
    #[case(Common, 234, 8, 23)]
    #[case(Common, 235, 8, 24)]
    #[case(Common, 236, 8, 25)]
    #[case(Common, 237, 8, 26)]
    #[case(Common, 238, 8, 27)]
    #[case(Common, 239, 8, 28)]
    #[case(Common, 240, 8, 29)]
    #[case(Common, 241, 8, 30)]
    #[case(Common, 242, 8, 31)]
    #[case(Common, 243, 9, 1)]
    #[case(Common, 244, 9, 2)]
    #[case(Common, 245, 9, 3)]
    #[case(Common, 246, 9, 4)]
    #[case(Common, 247, 9, 5)]
    #[case(Common, 248, 9, 6)]
    #[case(Common, 249, 9, 7)]
    #[case(Common, 250, 9, 8)]
    #[case(Common, 251, 9, 9)]
    #[case(Common, 252, 9, 10)]
    #[case(Common, 253, 9, 11)]
    #[case(Common, 254, 9, 12)]
    #[case(Common, 255, 9, 13)]
    #[case(Common, 256, 9, 14)]
    #[case(Common, 257, 9, 15)]
    #[case(Common, 258, 9, 16)]
    #[case(Common, 259, 9, 17)]
    #[case(Common, 260, 9, 18)]
    #[case(Common, 261, 9, 19)]
    #[case(Common, 262, 9, 20)]
    #[case(Common, 263, 9, 21)]
    #[case(Common, 264, 9, 22)]
    #[case(Common, 265, 9, 23)]
    #[case(Common, 266, 9, 24)]
    #[case(Common, 267, 9, 25)]
    #[case(Common, 268, 9, 26)]
    #[case(Common, 269, 9, 27)]
    #[case(Common, 270, 9, 28)]
    #[case(Common, 271, 9, 29)]
    #[case(Common, 272, 9, 30)]
    #[case(Common, 273, 10, 1)]
    #[case(Common, 274, 10, 2)]
    #[case(Common, 275, 10, 3)]
    #[case(Common, 276, 10, 4)]
    #[case(Common, 277, 10, 5)]
    #[case(Common, 278, 10, 6)]
    #[case(Common, 279, 10, 7)]
    #[case(Common, 280, 10, 8)]
    #[case(Common, 281, 10, 9)]
    #[case(Common, 282, 10, 10)]
    #[case(Common, 283, 10, 11)]
    #[case(Common, 284, 10, 12)]
    #[case(Common, 285, 10, 13)]
    #[case(Common, 286, 10, 14)]
    #[case(Common, 287, 10, 15)]
    #[case(Common, 288, 10, 16)]
    #[case(Common, 289, 10, 17)]
    #[case(Common, 290, 10, 18)]
    #[case(Common, 291, 10, 19)]
    #[case(Common, 292, 10, 20)]
    #[case(Common, 293, 10, 21)]
    #[case(Common, 294, 10, 22)]
    #[case(Common, 295, 10, 23)]
    #[case(Common, 296, 10, 24)]
    #[case(Common, 297, 10, 25)]
    #[case(Common, 298, 10, 26)]
    #[case(Common, 299, 10, 27)]
    #[case(Common, 300, 10, 28)]
    #[case(Common, 301, 10, 29)]
    #[case(Common, 302, 10, 30)]
    #[case(Common, 303, 10, 31)]
    #[case(Common, 304, 11, 1)]
    #[case(Common, 305, 11, 2)]
    #[case(Common, 306, 11, 3)]
    #[case(Common, 307, 11, 4)]
    #[case(Common, 308, 11, 5)]
    #[case(Common, 309, 11, 6)]
    #[case(Common, 310, 11, 7)]
    #[case(Common, 311, 11, 8)]
    #[case(Common, 312, 11, 9)]
    #[case(Common, 313, 11, 10)]
    #[case(Common, 314, 11, 11)]
    #[case(Common, 315, 11, 12)]
    #[case(Common, 316, 11, 13)]
    #[case(Common, 317, 11, 14)]
    #[case(Common, 318, 11, 15)]
    #[case(Common, 319, 11, 16)]
    #[case(Common, 320, 11, 17)]
    #[case(Common, 321, 11, 18)]
    #[case(Common, 322, 11, 19)]
    #[case(Common, 323, 11, 20)]
    #[case(Common, 324, 11, 21)]
    #[case(Common, 325, 11, 22)]
    #[case(Common, 326, 11, 23)]
    #[case(Common, 327, 11, 24)]
    #[case(Common, 328, 11, 25)]
    #[case(Common, 329, 11, 26)]
    #[case(Common, 330, 11, 27)]
    #[case(Common, 331, 11, 28)]
    #[case(Common, 332, 11, 29)]
    #[case(Common, 333, 11, 30)]
    #[case(Common, 334, 12, 1)]
    #[case(Common, 335, 12, 2)]
    #[case(Common, 336, 12, 3)]
    #[case(Common, 337, 12, 4)]
    #[case(Common, 338, 12, 5)]
    #[case(Common, 339, 12, 6)]
    #[case(Common, 340, 12, 7)]
    #[case(Common, 341, 12, 8)]
    #[case(Common, 342, 12, 9)]
    #[case(Common, 343, 12, 10)]
    #[case(Common, 344, 12, 11)]
    #[case(Common, 345, 12, 12)]
    #[case(Common, 346, 12, 13)]
    #[case(Common, 347, 12, 14)]
    #[case(Common, 348, 12, 15)]
    #[case(Common, 349, 12, 16)]
    #[case(Common, 350, 12, 17)]
    #[case(Common, 351, 12, 18)]
    #[case(Common, 352, 12, 19)]
    #[case(Common, 353, 12, 20)]
    #[case(Common, 354, 12, 21)]
    #[case(Common, 355, 12, 22)]
    #[case(Common, 356, 12, 23)]
    #[case(Common, 357, 12, 24)]
    #[case(Common, 358, 12, 25)]
    #[case(Common, 359, 12, 26)]
    #[case(Common, 360, 12, 27)]
    #[case(Common, 361, 12, 28)]
    #[case(Common, 362, 12, 29)]
    #[case(Common, 363, 12, 30)]
    #[case(Common, 364, 12, 31)]
    #[case(Leap, 0, 1, 1)]
    #[case(Leap, 1, 1, 2)]
    #[case(Leap, 2, 1, 3)]
    #[case(Leap, 3, 1, 4)]
    #[case(Leap, 4, 1, 5)]
    #[case(Leap, 5, 1, 6)]
    #[case(Leap, 6, 1, 7)]
    #[case(Leap, 7, 1, 8)]
    #[case(Leap, 8, 1, 9)]
    #[case(Leap, 9, 1, 10)]
    #[case(Leap, 10, 1, 11)]
    #[case(Leap, 11, 1, 12)]
    #[case(Leap, 12, 1, 13)]
    #[case(Leap, 13, 1, 14)]
    #[case(Leap, 14, 1, 15)]
    #[case(Leap, 15, 1, 16)]
    #[case(Leap, 16, 1, 17)]
    #[case(Leap, 17, 1, 18)]
    #[case(Leap, 18, 1, 19)]
    #[case(Leap, 19, 1, 20)]
    #[case(Leap, 20, 1, 21)]
    #[case(Leap, 21, 1, 22)]
    #[case(Leap, 22, 1, 23)]
    #[case(Leap, 23, 1, 24)]
    #[case(Leap, 24, 1, 25)]
    #[case(Leap, 25, 1, 26)]
    #[case(Leap, 26, 1, 27)]
    #[case(Leap, 27, 1, 28)]
    #[case(Leap, 28, 1, 29)]
    #[case(Leap, 29, 1, 30)]
    #[case(Leap, 30, 1, 31)]
    #[case(Leap, 31, 2, 1)]
    #[case(Leap, 32, 2, 2)]
    #[case(Leap, 33, 2, 3)]
    #[case(Leap, 34, 2, 4)]
    #[case(Leap, 35, 2, 5)]
    #[case(Leap, 36, 2, 6)]
    #[case(Leap, 37, 2, 7)]
    #[case(Leap, 38, 2, 8)]
    #[case(Leap, 39, 2, 9)]
    #[case(Leap, 40, 2, 10)]
    #[case(Leap, 41, 2, 11)]
    #[case(Leap, 42, 2, 12)]
    #[case(Leap, 43, 2, 13)]
    #[case(Leap, 44, 2, 14)]
    #[case(Leap, 45, 2, 15)]
    #[case(Leap, 46, 2, 16)]
    #[case(Leap, 47, 2, 17)]
    #[case(Leap, 48, 2, 18)]
    #[case(Leap, 49, 2, 19)]
    #[case(Leap, 50, 2, 20)]
    #[case(Leap, 51, 2, 21)]
    #[case(Leap, 52, 2, 22)]
    #[case(Leap, 53, 2, 23)]
    #[case(Leap, 54, 2, 24)]
    #[case(Leap, 55, 2, 25)]
    #[case(Leap, 56, 2, 26)]
    #[case(Leap, 57, 2, 27)]
    #[case(Leap, 58, 2, 28)]
    #[case(Leap, 59, 2, 29)]
    #[case(Leap, 60, 3, 1)]
    #[case(Leap, 61, 3, 2)]
    #[case(Leap, 62, 3, 3)]
    #[case(Leap, 63, 3, 4)]
    #[case(Leap, 64, 3, 5)]
    #[case(Leap, 65, 3, 6)]
    #[case(Leap, 66, 3, 7)]
    #[case(Leap, 67, 3, 8)]
    #[case(Leap, 68, 3, 9)]
    #[case(Leap, 69, 3, 10)]
    #[case(Leap, 70, 3, 11)]
    #[case(Leap, 71, 3, 12)]
    #[case(Leap, 72, 3, 13)]
    #[case(Leap, 73, 3, 14)]
    #[case(Leap, 74, 3, 15)]
    #[case(Leap, 75, 3, 16)]
    #[case(Leap, 76, 3, 17)]
    #[case(Leap, 77, 3, 18)]
    #[case(Leap, 78, 3, 19)]
    #[case(Leap, 79, 3, 20)]
    #[case(Leap, 80, 3, 21)]
    #[case(Leap, 81, 3, 22)]
    #[case(Leap, 82, 3, 23)]
    #[case(Leap, 83, 3, 24)]
    #[case(Leap, 84, 3, 25)]
    #[case(Leap, 85, 3, 26)]
    #[case(Leap, 86, 3, 27)]
    #[case(Leap, 87, 3, 28)]
    #[case(Leap, 88, 3, 29)]
    #[case(Leap, 89, 3, 30)]
    #[case(Leap, 90, 3, 31)]
    #[case(Leap, 91, 4, 1)]
    #[case(Leap, 92, 4, 2)]
    #[case(Leap, 93, 4, 3)]
    #[case(Leap, 94, 4, 4)]
    #[case(Leap, 95, 4, 5)]
    #[case(Leap, 96, 4, 6)]
    #[case(Leap, 97, 4, 7)]
    #[case(Leap, 98, 4, 8)]
    #[case(Leap, 99, 4, 9)]
    #[case(Leap, 100, 4, 10)]
    #[case(Leap, 101, 4, 11)]
    #[case(Leap, 102, 4, 12)]
    #[case(Leap, 103, 4, 13)]
    #[case(Leap, 104, 4, 14)]
    #[case(Leap, 105, 4, 15)]
    #[case(Leap, 106, 4, 16)]
    #[case(Leap, 107, 4, 17)]
    #[case(Leap, 108, 4, 18)]
    #[case(Leap, 109, 4, 19)]
    #[case(Leap, 110, 4, 20)]
    #[case(Leap, 111, 4, 21)]
    #[case(Leap, 112, 4, 22)]
    #[case(Leap, 113, 4, 23)]
    #[case(Leap, 114, 4, 24)]
    #[case(Leap, 115, 4, 25)]
    #[case(Leap, 116, 4, 26)]
    #[case(Leap, 117, 4, 27)]
    #[case(Leap, 118, 4, 28)]
    #[case(Leap, 119, 4, 29)]
    #[case(Leap, 120, 4, 30)]
    #[case(Leap, 121, 5, 1)]
    #[case(Leap, 122, 5, 2)]
    #[case(Leap, 123, 5, 3)]
    #[case(Leap, 124, 5, 4)]
    #[case(Leap, 125, 5, 5)]
    #[case(Leap, 126, 5, 6)]
    #[case(Leap, 127, 5, 7)]
    #[case(Leap, 128, 5, 8)]
    #[case(Leap, 129, 5, 9)]
    #[case(Leap, 130, 5, 10)]
    #[case(Leap, 131, 5, 11)]
    #[case(Leap, 132, 5, 12)]
    #[case(Leap, 133, 5, 13)]
    #[case(Leap, 134, 5, 14)]
    #[case(Leap, 135, 5, 15)]
    #[case(Leap, 136, 5, 16)]
    #[case(Leap, 137, 5, 17)]
    #[case(Leap, 138, 5, 18)]
    #[case(Leap, 139, 5, 19)]
    #[case(Leap, 140, 5, 20)]
    #[case(Leap, 141, 5, 21)]
    #[case(Leap, 142, 5, 22)]
    #[case(Leap, 143, 5, 23)]
    #[case(Leap, 144, 5, 24)]
    #[case(Leap, 145, 5, 25)]
    #[case(Leap, 146, 5, 26)]
    #[case(Leap, 147, 5, 27)]
    #[case(Leap, 148, 5, 28)]
    #[case(Leap, 149, 5, 29)]
    #[case(Leap, 150, 5, 30)]
    #[case(Leap, 151, 5, 31)]
    #[case(Leap, 152, 6, 1)]
    #[case(Leap, 153, 6, 2)]
    #[case(Leap, 154, 6, 3)]
    #[case(Leap, 155, 6, 4)]
    #[case(Leap, 156, 6, 5)]
    #[case(Leap, 157, 6, 6)]
    #[case(Leap, 158, 6, 7)]
    #[case(Leap, 159, 6, 8)]
    #[case(Leap, 160, 6, 9)]
    #[case(Leap, 161, 6, 10)]
    #[case(Leap, 162, 6, 11)]
    #[case(Leap, 163, 6, 12)]
    #[case(Leap, 164, 6, 13)]
    #[case(Leap, 165, 6, 14)]
    #[case(Leap, 166, 6, 15)]
    #[case(Leap, 167, 6, 16)]
    #[case(Leap, 168, 6, 17)]
    #[case(Leap, 169, 6, 18)]
    #[case(Leap, 170, 6, 19)]
    #[case(Leap, 171, 6, 20)]
    #[case(Leap, 172, 6, 21)]
    #[case(Leap, 173, 6, 22)]
    #[case(Leap, 174, 6, 23)]
    #[case(Leap, 175, 6, 24)]
    #[case(Leap, 176, 6, 25)]
    #[case(Leap, 177, 6, 26)]
    #[case(Leap, 178, 6, 27)]
    #[case(Leap, 179, 6, 28)]
    #[case(Leap, 180, 6, 29)]
    #[case(Leap, 181, 6, 30)]
    #[case(Leap, 182, 7, 1)]
    #[case(Leap, 183, 7, 2)]
    #[case(Leap, 184, 7, 3)]
    #[case(Leap, 185, 7, 4)]
    #[case(Leap, 186, 7, 5)]
    #[case(Leap, 187, 7, 6)]
    #[case(Leap, 188, 7, 7)]
    #[case(Leap, 189, 7, 8)]
    #[case(Leap, 190, 7, 9)]
    #[case(Leap, 191, 7, 10)]
    #[case(Leap, 192, 7, 11)]
    #[case(Leap, 193, 7, 12)]
    #[case(Leap, 194, 7, 13)]
    #[case(Leap, 195, 7, 14)]
    #[case(Leap, 196, 7, 15)]
    #[case(Leap, 197, 7, 16)]
    #[case(Leap, 198, 7, 17)]
    #[case(Leap, 199, 7, 18)]
    #[case(Leap, 200, 7, 19)]
    #[case(Leap, 201, 7, 20)]
    #[case(Leap, 202, 7, 21)]
    #[case(Leap, 203, 7, 22)]
    #[case(Leap, 204, 7, 23)]
    #[case(Leap, 205, 7, 24)]
    #[case(Leap, 206, 7, 25)]
    #[case(Leap, 207, 7, 26)]
    #[case(Leap, 208, 7, 27)]
    #[case(Leap, 209, 7, 28)]
    #[case(Leap, 210, 7, 29)]
    #[case(Leap, 211, 7, 30)]
    #[case(Leap, 212, 7, 31)]
    #[case(Leap, 213, 8, 1)]
    #[case(Leap, 214, 8, 2)]
    #[case(Leap, 215, 8, 3)]
    #[case(Leap, 216, 8, 4)]
    #[case(Leap, 217, 8, 5)]
    #[case(Leap, 218, 8, 6)]
    #[case(Leap, 219, 8, 7)]
    #[case(Leap, 220, 8, 8)]
    #[case(Leap, 221, 8, 9)]
    #[case(Leap, 222, 8, 10)]
    #[case(Leap, 223, 8, 11)]
    #[case(Leap, 224, 8, 12)]
    #[case(Leap, 225, 8, 13)]
    #[case(Leap, 226, 8, 14)]
    #[case(Leap, 227, 8, 15)]
    #[case(Leap, 228, 8, 16)]
    #[case(Leap, 229, 8, 17)]
    #[case(Leap, 230, 8, 18)]
    #[case(Leap, 231, 8, 19)]
    #[case(Leap, 232, 8, 20)]
    #[case(Leap, 233, 8, 21)]
    #[case(Leap, 234, 8, 22)]
    #[case(Leap, 235, 8, 23)]
    #[case(Leap, 236, 8, 24)]
    #[case(Leap, 237, 8, 25)]
    #[case(Leap, 238, 8, 26)]
    #[case(Leap, 239, 8, 27)]
    #[case(Leap, 240, 8, 28)]
    #[case(Leap, 241, 8, 29)]
    #[case(Leap, 242, 8, 30)]
    #[case(Leap, 243, 8, 31)]
    #[case(Leap, 244, 9, 1)]
    #[case(Leap, 245, 9, 2)]
    #[case(Leap, 246, 9, 3)]
    #[case(Leap, 247, 9, 4)]
    #[case(Leap, 248, 9, 5)]
    #[case(Leap, 249, 9, 6)]
    #[case(Leap, 250, 9, 7)]
    #[case(Leap, 251, 9, 8)]
    #[case(Leap, 252, 9, 9)]
    #[case(Leap, 253, 9, 10)]
    #[case(Leap, 254, 9, 11)]
    #[case(Leap, 255, 9, 12)]
    #[case(Leap, 256, 9, 13)]
    #[case(Leap, 257, 9, 14)]
    #[case(Leap, 258, 9, 15)]
    #[case(Leap, 259, 9, 16)]
    #[case(Leap, 260, 9, 17)]
    #[case(Leap, 261, 9, 18)]
    #[case(Leap, 262, 9, 19)]
    #[case(Leap, 263, 9, 20)]
    #[case(Leap, 264, 9, 21)]
    #[case(Leap, 265, 9, 22)]
    #[case(Leap, 266, 9, 23)]
    #[case(Leap, 267, 9, 24)]
    #[case(Leap, 268, 9, 25)]
    #[case(Leap, 269, 9, 26)]
    #[case(Leap, 270, 9, 27)]
    #[case(Leap, 271, 9, 28)]
    #[case(Leap, 272, 9, 29)]
    #[case(Leap, 273, 9, 30)]
    #[case(Leap, 274, 10, 1)]
    #[case(Leap, 275, 10, 2)]
    #[case(Leap, 276, 10, 3)]
    #[case(Leap, 277, 10, 4)]
    #[case(Leap, 278, 10, 5)]
    #[case(Leap, 279, 10, 6)]
    #[case(Leap, 280, 10, 7)]
    #[case(Leap, 281, 10, 8)]
    #[case(Leap, 282, 10, 9)]
    #[case(Leap, 283, 10, 10)]
    #[case(Leap, 284, 10, 11)]
    #[case(Leap, 285, 10, 12)]
    #[case(Leap, 286, 10, 13)]
    #[case(Leap, 287, 10, 14)]
    #[case(Leap, 288, 10, 15)]
    #[case(Leap, 289, 10, 16)]
    #[case(Leap, 290, 10, 17)]
    #[case(Leap, 291, 10, 18)]
    #[case(Leap, 292, 10, 19)]
    #[case(Leap, 293, 10, 20)]
    #[case(Leap, 294, 10, 21)]
    #[case(Leap, 295, 10, 22)]
    #[case(Leap, 296, 10, 23)]
    #[case(Leap, 297, 10, 24)]
    #[case(Leap, 298, 10, 25)]
    #[case(Leap, 299, 10, 26)]
    #[case(Leap, 300, 10, 27)]
    #[case(Leap, 301, 10, 28)]
    #[case(Leap, 302, 10, 29)]
    #[case(Leap, 303, 10, 30)]
    #[case(Leap, 304, 10, 31)]
    #[case(Leap, 305, 11, 1)]
    #[case(Leap, 306, 11, 2)]
    #[case(Leap, 307, 11, 3)]
    #[case(Leap, 308, 11, 4)]
    #[case(Leap, 309, 11, 5)]
    #[case(Leap, 310, 11, 6)]
    #[case(Leap, 311, 11, 7)]
    #[case(Leap, 312, 11, 8)]
    #[case(Leap, 313, 11, 9)]
    #[case(Leap, 314, 11, 10)]
    #[case(Leap, 315, 11, 11)]
    #[case(Leap, 316, 11, 12)]
    #[case(Leap, 317, 11, 13)]
    #[case(Leap, 318, 11, 14)]
    #[case(Leap, 319, 11, 15)]
    #[case(Leap, 320, 11, 16)]
    #[case(Leap, 321, 11, 17)]
    #[case(Leap, 322, 11, 18)]
    #[case(Leap, 323, 11, 19)]
    #[case(Leap, 324, 11, 20)]
    #[case(Leap, 325, 11, 21)]
    #[case(Leap, 326, 11, 22)]
    #[case(Leap, 327, 11, 23)]
    #[case(Leap, 328, 11, 24)]
    #[case(Leap, 329, 11, 25)]
    #[case(Leap, 330, 11, 26)]
    #[case(Leap, 331, 11, 27)]
    #[case(Leap, 332, 11, 28)]
    #[case(Leap, 333, 11, 29)]
    #[case(Leap, 334, 11, 30)]
    #[case(Leap, 335, 12, 1)]
    #[case(Leap, 336, 12, 2)]
    #[case(Leap, 337, 12, 3)]
    #[case(Leap, 338, 12, 4)]
    #[case(Leap, 339, 12, 5)]
    #[case(Leap, 340, 12, 6)]
    #[case(Leap, 341, 12, 7)]
    #[case(Leap, 342, 12, 8)]
    #[case(Leap, 343, 12, 9)]
    #[case(Leap, 344, 12, 10)]
    #[case(Leap, 345, 12, 11)]
    #[case(Leap, 346, 12, 12)]
    #[case(Leap, 347, 12, 13)]
    #[case(Leap, 348, 12, 14)]
    #[case(Leap, 349, 12, 15)]
    #[case(Leap, 350, 12, 16)]
    #[case(Leap, 351, 12, 17)]
    #[case(Leap, 352, 12, 18)]
    #[case(Leap, 353, 12, 19)]
    #[case(Leap, 354, 12, 20)]
    #[case(Leap, 355, 12, 21)]
    #[case(Leap, 356, 12, 22)]
    #[case(Leap, 357, 12, 23)]
    #[case(Leap, 358, 12, 24)]
    #[case(Leap, 359, 12, 25)]
    #[case(Leap, 360, 12, 26)]
    #[case(Leap, 361, 12, 27)]
    #[case(Leap, 362, 12, 28)]
    #[case(Leap, 363, 12, 29)]
    #[case(Leap, 364, 12, 30)]
    #[case(Leap, 365, 12, 31)]
    #[case(Reform, 0, 1, 1)]
    #[case(Reform, 1, 1, 2)]
    #[case(Reform, 2, 1, 3)]
    #[case(Reform, 3, 1, 4)]
    #[case(Reform, 4, 1, 5)]
    #[case(Reform, 5, 1, 6)]
    #[case(Reform, 6, 1, 7)]
    #[case(Reform, 7, 1, 8)]
    #[case(Reform, 8, 1, 9)]
    #[case(Reform, 9, 1, 10)]
    #[case(Reform, 10, 1, 11)]
    #[case(Reform, 11, 1, 12)]
    #[case(Reform, 12, 1, 13)]
    #[case(Reform, 13, 1, 14)]
    #[case(Reform, 14, 1, 15)]
    #[case(Reform, 15, 1, 16)]
    #[case(Reform, 16, 1, 17)]
    #[case(Reform, 17, 1, 18)]
    #[case(Reform, 18, 1, 19)]
    #[case(Reform, 19, 1, 20)]
    #[case(Reform, 20, 1, 21)]
    #[case(Reform, 21, 1, 22)]
    #[case(Reform, 22, 1, 23)]
    #[case(Reform, 23, 1, 24)]
    #[case(Reform, 24, 1, 25)]
    #[case(Reform, 25, 1, 26)]
    #[case(Reform, 26, 1, 27)]
    #[case(Reform, 27, 1, 28)]
    #[case(Reform, 28, 1, 29)]
    #[case(Reform, 29, 1, 30)]
    #[case(Reform, 30, 1, 31)]
    #[case(Reform, 31, 2, 1)]
    #[case(Reform, 32, 2, 2)]
    #[case(Reform, 33, 2, 3)]
    #[case(Reform, 34, 2, 4)]
    #[case(Reform, 35, 2, 5)]
    #[case(Reform, 36, 2, 6)]
    #[case(Reform, 37, 2, 7)]
    #[case(Reform, 38, 2, 8)]
    #[case(Reform, 39, 2, 9)]
    #[case(Reform, 40, 2, 10)]
    #[case(Reform, 41, 2, 11)]
    #[case(Reform, 42, 2, 12)]
    #[case(Reform, 43, 2, 13)]
    #[case(Reform, 44, 2, 14)]
    #[case(Reform, 45, 2, 15)]
    #[case(Reform, 46, 2, 16)]
    #[case(Reform, 47, 2, 17)]
    #[case(Reform, 48, 2, 18)]
    #[case(Reform, 49, 2, 19)]
    #[case(Reform, 50, 2, 20)]
    #[case(Reform, 51, 2, 21)]
    #[case(Reform, 52, 2, 22)]
    #[case(Reform, 53, 2, 23)]
    #[case(Reform, 54, 2, 24)]
    #[case(Reform, 55, 2, 25)]
    #[case(Reform, 56, 2, 26)]
    #[case(Reform, 57, 2, 27)]
    #[case(Reform, 58, 2, 28)]
    #[case(Reform, 59, 3, 1)]
    #[case(Reform, 60, 3, 2)]
    #[case(Reform, 61, 3, 3)]
    #[case(Reform, 62, 3, 4)]
    #[case(Reform, 63, 3, 5)]
    #[case(Reform, 64, 3, 6)]
    #[case(Reform, 65, 3, 7)]
    #[case(Reform, 66, 3, 8)]
    #[case(Reform, 67, 3, 9)]
    #[case(Reform, 68, 3, 10)]
    #[case(Reform, 69, 3, 11)]
    #[case(Reform, 70, 3, 12)]
    #[case(Reform, 71, 3, 13)]
    #[case(Reform, 72, 3, 14)]
    #[case(Reform, 73, 3, 15)]
    #[case(Reform, 74, 3, 16)]
    #[case(Reform, 75, 3, 17)]
    #[case(Reform, 76, 3, 18)]
    #[case(Reform, 77, 3, 19)]
    #[case(Reform, 78, 3, 20)]
    #[case(Reform, 79, 3, 21)]
    #[case(Reform, 80, 3, 22)]
    #[case(Reform, 81, 3, 23)]
    #[case(Reform, 82, 3, 24)]
    #[case(Reform, 83, 3, 25)]
    #[case(Reform, 84, 3, 26)]
    #[case(Reform, 85, 3, 27)]
    #[case(Reform, 86, 3, 28)]
    #[case(Reform, 87, 3, 29)]
    #[case(Reform, 88, 3, 30)]
    #[case(Reform, 89, 3, 31)]
    #[case(Reform, 90, 4, 1)]
    #[case(Reform, 91, 4, 2)]
    #[case(Reform, 92, 4, 3)]
    #[case(Reform, 93, 4, 4)]
    #[case(Reform, 94, 4, 5)]
    #[case(Reform, 95, 4, 6)]
    #[case(Reform, 96, 4, 7)]
    #[case(Reform, 97, 4, 8)]
    #[case(Reform, 98, 4, 9)]
    #[case(Reform, 99, 4, 10)]
    #[case(Reform, 100, 4, 11)]
    #[case(Reform, 101, 4, 12)]
    #[case(Reform, 102, 4, 13)]
    #[case(Reform, 103, 4, 14)]
    #[case(Reform, 104, 4, 15)]
    #[case(Reform, 105, 4, 16)]
    #[case(Reform, 106, 4, 17)]
    #[case(Reform, 107, 4, 18)]
    #[case(Reform, 108, 4, 19)]
    #[case(Reform, 109, 4, 20)]
    #[case(Reform, 110, 4, 21)]
    #[case(Reform, 111, 4, 22)]
    #[case(Reform, 112, 4, 23)]
    #[case(Reform, 113, 4, 24)]
    #[case(Reform, 114, 4, 25)]
    #[case(Reform, 115, 4, 26)]
    #[case(Reform, 116, 4, 27)]
    #[case(Reform, 117, 4, 28)]
    #[case(Reform, 118, 4, 29)]
    #[case(Reform, 119, 4, 30)]
    #[case(Reform, 120, 5, 1)]
    #[case(Reform, 121, 5, 2)]
    #[case(Reform, 122, 5, 3)]
    #[case(Reform, 123, 5, 4)]
    #[case(Reform, 124, 5, 5)]
    #[case(Reform, 125, 5, 6)]
    #[case(Reform, 126, 5, 7)]
    #[case(Reform, 127, 5, 8)]
    #[case(Reform, 128, 5, 9)]
    #[case(Reform, 129, 5, 10)]
    #[case(Reform, 130, 5, 11)]
    #[case(Reform, 131, 5, 12)]
    #[case(Reform, 132, 5, 13)]
    #[case(Reform, 133, 5, 14)]
    #[case(Reform, 134, 5, 15)]
    #[case(Reform, 135, 5, 16)]
    #[case(Reform, 136, 5, 17)]
    #[case(Reform, 137, 5, 18)]
    #[case(Reform, 138, 5, 19)]
    #[case(Reform, 139, 5, 20)]
    #[case(Reform, 140, 5, 21)]
    #[case(Reform, 141, 5, 22)]
    #[case(Reform, 142, 5, 23)]
    #[case(Reform, 143, 5, 24)]
    #[case(Reform, 144, 5, 25)]
    #[case(Reform, 145, 5, 26)]
    #[case(Reform, 146, 5, 27)]
    #[case(Reform, 147, 5, 28)]
    #[case(Reform, 148, 5, 29)]
    #[case(Reform, 149, 5, 30)]
    #[case(Reform, 150, 5, 31)]
    #[case(Reform, 151, 6, 1)]
    #[case(Reform, 152, 6, 2)]
    #[case(Reform, 153, 6, 3)]
    #[case(Reform, 154, 6, 4)]
    #[case(Reform, 155, 6, 5)]
    #[case(Reform, 156, 6, 6)]
    #[case(Reform, 157, 6, 7)]
    #[case(Reform, 158, 6, 8)]
    #[case(Reform, 159, 6, 9)]
    #[case(Reform, 160, 6, 10)]
    #[case(Reform, 161, 6, 11)]
    #[case(Reform, 162, 6, 12)]
    #[case(Reform, 163, 6, 13)]
    #[case(Reform, 164, 6, 14)]
    #[case(Reform, 165, 6, 15)]
    #[case(Reform, 166, 6, 16)]
    #[case(Reform, 167, 6, 17)]
    #[case(Reform, 168, 6, 18)]
    #[case(Reform, 169, 6, 19)]
    #[case(Reform, 170, 6, 20)]
    #[case(Reform, 171, 6, 21)]
    #[case(Reform, 172, 6, 22)]
    #[case(Reform, 173, 6, 23)]
    #[case(Reform, 174, 6, 24)]
    #[case(Reform, 175, 6, 25)]
    #[case(Reform, 176, 6, 26)]
    #[case(Reform, 177, 6, 27)]
    #[case(Reform, 178, 6, 28)]
    #[case(Reform, 179, 6, 29)]
    #[case(Reform, 180, 6, 30)]
    #[case(Reform, 181, 7, 1)]
    #[case(Reform, 182, 7, 2)]
    #[case(Reform, 183, 7, 3)]
    #[case(Reform, 184, 7, 4)]
    #[case(Reform, 185, 7, 5)]
    #[case(Reform, 186, 7, 6)]
    #[case(Reform, 187, 7, 7)]
    #[case(Reform, 188, 7, 8)]
    #[case(Reform, 189, 7, 9)]
    #[case(Reform, 190, 7, 10)]
    #[case(Reform, 191, 7, 11)]
    #[case(Reform, 192, 7, 12)]
    #[case(Reform, 193, 7, 13)]
    #[case(Reform, 194, 7, 14)]
    #[case(Reform, 195, 7, 15)]
    #[case(Reform, 196, 7, 16)]
    #[case(Reform, 197, 7, 17)]
    #[case(Reform, 198, 7, 18)]
    #[case(Reform, 199, 7, 19)]
    #[case(Reform, 200, 7, 20)]
    #[case(Reform, 201, 7, 21)]
    #[case(Reform, 202, 7, 22)]
    #[case(Reform, 203, 7, 23)]
    #[case(Reform, 204, 7, 24)]
    #[case(Reform, 205, 7, 25)]
    #[case(Reform, 206, 7, 26)]
    #[case(Reform, 207, 7, 27)]
    #[case(Reform, 208, 7, 28)]
    #[case(Reform, 209, 7, 29)]
    #[case(Reform, 210, 7, 30)]
    #[case(Reform, 211, 7, 31)]
    #[case(Reform, 212, 8, 1)]
    #[case(Reform, 213, 8, 2)]
    #[case(Reform, 214, 8, 3)]
    #[case(Reform, 215, 8, 4)]
    #[case(Reform, 216, 8, 5)]
    #[case(Reform, 217, 8, 6)]
    #[case(Reform, 218, 8, 7)]
    #[case(Reform, 219, 8, 8)]
    #[case(Reform, 220, 8, 9)]
    #[case(Reform, 221, 8, 10)]
    #[case(Reform, 222, 8, 11)]
    #[case(Reform, 223, 8, 12)]
    #[case(Reform, 224, 8, 13)]
    #[case(Reform, 225, 8, 14)]
    #[case(Reform, 226, 8, 15)]
    #[case(Reform, 227, 8, 16)]
    #[case(Reform, 228, 8, 17)]
    #[case(Reform, 229, 8, 18)]
    #[case(Reform, 230, 8, 19)]
    #[case(Reform, 231, 8, 20)]
    #[case(Reform, 232, 8, 21)]
    #[case(Reform, 233, 8, 22)]
    #[case(Reform, 234, 8, 23)]
    #[case(Reform, 235, 8, 24)]
    #[case(Reform, 236, 8, 25)]
    #[case(Reform, 237, 8, 26)]
    #[case(Reform, 238, 8, 27)]
    #[case(Reform, 239, 8, 28)]
    #[case(Reform, 240, 8, 29)]
    #[case(Reform, 241, 8, 30)]
    #[case(Reform, 242, 8, 31)]
    #[case(Reform, 243, 9, 1)]
    #[case(Reform, 244, 9, 2)]
    #[case(Reform, 245, 9, 3)]
    #[case(Reform, 246, 9, 4)]
    #[case(Reform, 247, 9, 5)]
    #[case(Reform, 248, 9, 6)]
    #[case(Reform, 249, 9, 7)]
    #[case(Reform, 250, 9, 8)]
    #[case(Reform, 251, 9, 9)]
    #[case(Reform, 252, 9, 10)]
    #[case(Reform, 253, 9, 11)]
    #[case(Reform, 254, 9, 12)]
    #[case(Reform, 255, 9, 13)]
    #[case(Reform, 256, 9, 14)]
    #[case(Reform, 257, 9, 15)]
    #[case(Reform, 258, 9, 16)]
    #[case(Reform, 259, 9, 17)]
    #[case(Reform, 260, 9, 18)]
    #[case(Reform, 261, 9, 19)]
    #[case(Reform, 262, 9, 20)]
    #[case(Reform, 263, 9, 21)]
    #[case(Reform, 264, 9, 22)]
    #[case(Reform, 265, 9, 23)]
    #[case(Reform, 266, 9, 24)]
    #[case(Reform, 267, 9, 25)]
    #[case(Reform, 268, 9, 26)]
    #[case(Reform, 269, 9, 27)]
    #[case(Reform, 270, 9, 28)]
    #[case(Reform, 271, 9, 29)]
    #[case(Reform, 272, 9, 30)]
    #[case(Reform, 273, 10, 1)]
    #[case(Reform, 274, 10, 2)]
    #[case(Reform, 275, 10, 3)]
    #[case(Reform, 276, 10, 4)]
    #[case(Reform, 277, 10, 15)]
    #[case(Reform, 278, 10, 16)]
    #[case(Reform, 279, 10, 17)]
    #[case(Reform, 280, 10, 18)]
    #[case(Reform, 281, 10, 19)]
    #[case(Reform, 282, 10, 20)]
    #[case(Reform, 283, 10, 21)]
    #[case(Reform, 284, 10, 22)]
    #[case(Reform, 285, 10, 23)]
    #[case(Reform, 286, 10, 24)]
    #[case(Reform, 287, 10, 25)]
    #[case(Reform, 288, 10, 26)]
    #[case(Reform, 289, 10, 27)]
    #[case(Reform, 290, 10, 28)]
    #[case(Reform, 291, 10, 29)]
    #[case(Reform, 292, 10, 30)]
    #[case(Reform, 293, 10, 31)]
    #[case(Reform, 294, 11, 1)]
    #[case(Reform, 295, 11, 2)]
    #[case(Reform, 296, 11, 3)]
    #[case(Reform, 297, 11, 4)]
    #[case(Reform, 298, 11, 5)]
    #[case(Reform, 299, 11, 6)]
    #[case(Reform, 300, 11, 7)]
    #[case(Reform, 301, 11, 8)]
    #[case(Reform, 302, 11, 9)]
    #[case(Reform, 303, 11, 10)]
    #[case(Reform, 304, 11, 11)]
    #[case(Reform, 305, 11, 12)]
    #[case(Reform, 306, 11, 13)]
    #[case(Reform, 307, 11, 14)]
    #[case(Reform, 308, 11, 15)]
    #[case(Reform, 309, 11, 16)]
    #[case(Reform, 310, 11, 17)]
    #[case(Reform, 311, 11, 18)]
    #[case(Reform, 312, 11, 19)]
    #[case(Reform, 313, 11, 20)]
    #[case(Reform, 314, 11, 21)]
    #[case(Reform, 315, 11, 22)]
    #[case(Reform, 316, 11, 23)]
    #[case(Reform, 317, 11, 24)]
    #[case(Reform, 318, 11, 25)]
    #[case(Reform, 319, 11, 26)]
    #[case(Reform, 320, 11, 27)]
    #[case(Reform, 321, 11, 28)]
    #[case(Reform, 322, 11, 29)]
    #[case(Reform, 323, 11, 30)]
    #[case(Reform, 324, 12, 1)]
    #[case(Reform, 325, 12, 2)]
    #[case(Reform, 326, 12, 3)]
    #[case(Reform, 327, 12, 4)]
    #[case(Reform, 328, 12, 5)]
    #[case(Reform, 329, 12, 6)]
    #[case(Reform, 330, 12, 7)]
    #[case(Reform, 331, 12, 8)]
    #[case(Reform, 332, 12, 9)]
    #[case(Reform, 333, 12, 10)]
    #[case(Reform, 334, 12, 11)]
    #[case(Reform, 335, 12, 12)]
    #[case(Reform, 336, 12, 13)]
    #[case(Reform, 337, 12, 14)]
    #[case(Reform, 338, 12, 15)]
    #[case(Reform, 339, 12, 16)]
    #[case(Reform, 340, 12, 17)]
    #[case(Reform, 341, 12, 18)]
    #[case(Reform, 342, 12, 19)]
    #[case(Reform, 343, 12, 20)]
    #[case(Reform, 344, 12, 21)]
    #[case(Reform, 345, 12, 22)]
    #[case(Reform, 346, 12, 23)]
    #[case(Reform, 347, 12, 24)]
    #[case(Reform, 348, 12, 25)]
    #[case(Reform, 349, 12, 26)]
    #[case(Reform, 350, 12, 27)]
    #[case(Reform, 351, 12, 28)]
    #[case(Reform, 352, 12, 29)]
    #[case(Reform, 353, 12, 30)]
    #[case(Reform, 354, 12, 31)]
    fn test_break_yday(
        #[case] style: YearStyle,
        #[case] yday: DaysT,
        #[case] month: u32,
        #[case] mday: u32,
    ) {
        assert_eq!(style.break_yday(yday), Some((month, mday)));
    }

    #[rstest]
    #[case(Common, 365)]
    #[case(Common, 999)]
    #[case(Leap, 366)]
    #[case(Leap, 999)]
    #[case(Reform, 355)]
    #[case(Reform, 999)]
    fn test_bad_break_yday(#[case] style: YearStyle, #[case] yday: DaysT) {
        assert_eq!(style.break_yday(yday), None);
    }

    #[test]
    fn test_display_date() {
        let date = Date::from_ymd(2023, 4, 20, None).unwrap();
        assert_eq!(format!("{date}"), "2023-04-20");
    }

    #[test]
    fn test_alternate_display_date() {
        let date = Date::from_ymd(2023, 4, 20, None).unwrap();
        assert_eq!(format!("{date:#}"), "2023-110");
    }

    #[test]
    fn test_alternate_display_date_padded() {
        let date = Date::from_ymd(2023, 3, 15, None).unwrap();
        assert_eq!(format!("{date:#}"), "2023-074");
    }

    #[test]
    fn test_display_date_with_seconds() {
        let date = Date::from_ymd(2023, 4, 20, Some(16 * 3600 + 39 * 60 + 50)).unwrap();
        assert_eq!(format!("{date}"), "2023-04-20T16:39:50Z");
    }

    #[test]
    fn test_display_date_with_seconds_padded() {
        let date = Date::from_ymd(1, 1, 1, Some(0)).unwrap();
        assert_eq!(format!("{date}"), "0001-01-01T00:00:00Z");
    }

    #[test]
    fn test_alternate_display_date_with_seconds() {
        let date = Date::from_ymd(2023, 4, 20, Some(16 * 3600 + 39 * 60 + 50)).unwrap();
        assert_eq!(format!("{date:#}"), "2023-110T16:39:50Z");
    }

    #[test]
    fn test_display_julian_date() {
        let jd = JulianDate {
            days: 2460055,
            seconds: None,
        };
        assert_eq!(format!("{jd}"), "2460055");
        assert_eq!(format!("{jd:#}"), "2460055");
    }

    #[test]
    fn test_display_julian_date_with_seconds() {
        let jd = JulianDate {
            days: 2460055,
            seconds: Some(16 * 3600 + 39 * 60 + 50),
        };
        assert_eq!(format!("{jd}"), "2460055.694329");
    }

    #[test]
    fn test_display_julian_date_with_lower_precision() {
        let jd = JulianDate {
            days: 2460055,
            seconds: Some(16 * 3600 + 39 * 60 + 50),
        };
        assert_eq!(format!("{jd:.1}"), "2460055.7");
    }

    #[test]
    fn test_display_julian_date_with_higher_precision() {
        let jd = JulianDate {
            days: 2460055,
            seconds: Some(16 * 3600 + 39 * 60 + 50),
        };
        assert_eq!(format!("{jd:.8}"), "2460055.69432870");
    }

    #[test]
    fn test_display_julian_date_with_zero_precision() {
        let jd = JulianDate {
            days: 2460055,
            seconds: Some(16 * 3600 + 39 * 60 + 50),
        };
        assert_eq!(format!("{jd:.0}"), "2460055");
    }

    #[test]
    fn test_alternate_display_julian_date_with_seconds() {
        let jd = JulianDate {
            days: 2460055,
            seconds: Some(16 * 3600 + 39 * 60 + 50),
        };
        assert_eq!(format!("{jd:#}"), "2460055:59990");
    }

    #[rstest]
    #[case(0, 1970, 1, 1, 0)]
    #[case(100000000, 1973, 3, 3, 35200)]
    #[case(1000000000, 2001, 9, 9, 6400)]
    #[case(1234567890, 2009, 2, 13, 84690)]
    #[case(1682028168, 2023, 4, 20, 79368)]
    #[case(2147483647, 2038, 1, 19, 11647)]
    fn test_from_unix_timestamp(
        #[case] ts: u64,
        #[case] year: YearT,
        #[case] month: u32,
        #[case] mday: u32,
        #[case] seconds: SecondsT,
    ) {
        let date = Date::from_unix_timestamp(ts);
        assert_eq!(date.year, year);
        assert_eq!(date.month, month);
        assert_eq!(date.mday, mday);
        assert_eq!(date.seconds, Some(seconds));
    }

    #[test]
    fn test_from_ymd_zero_month() {
        let r = Date::from_ymd(2023, 0, 13, None);
        assert_eq!(r, Err(DateError::MonthOutOfRange { month: 0 }));
        assert_eq!(
            r.unwrap_err().to_string(),
            "month 0 is outside of valid range"
        );
    }

    #[test]
    fn test_from_ymd_smarch() {
        let r = Date::from_ymd(2023, 13, 13, None);
        assert_eq!(r, Err(DateError::MonthOutOfRange { month: 13 }));
        assert_eq!(
            r.unwrap_err().to_string(),
            "month 13 is outside of valid range"
        );
    }

    #[test]
    fn test_from_ymd_mday_0() {
        let r = Date::from_ymd(2023, 4, 0, None);
        assert_eq!(r, Err(DateError::MdayOutOfRange { month: 4, mday: 0 }));
        assert_eq!(
            r.unwrap_err().to_string(),
            "mday 0 is outside of valid range for month 4"
        );
    }

    #[test]
    fn test_from_ymd_mday_32() {
        let r = Date::from_ymd(2023, 4, 32, None);
        assert_eq!(r, Err(DateError::MdayOutOfRange { month: 4, mday: 32 }));
        assert_eq!(
            r.unwrap_err().to_string(),
            "mday 32 is outside of valid range for month 4"
        );
    }

    #[test]
    fn test_from_ymd_sep_31() {
        let r = Date::from_ymd(2023, 9, 31, None);
        assert_eq!(r, Err(DateError::MdayOutOfRange { month: 9, mday: 31 }));
        assert_eq!(
            r.unwrap_err().to_string(),
            "mday 31 is outside of valid range for month 9"
        );
    }

    #[test]
    fn test_from_ymd_invalid_leap_day() {
        let r = Date::from_ymd(2023, 2, 29, None);
        assert_eq!(r, Err(DateError::MdayOutOfRange { month: 2, mday: 29 }));
        assert_eq!(
            r.unwrap_err().to_string(),
            "mday 29 is outside of valid range for month 2"
        );
    }

    #[test]
    fn test_from_ymd_valid_leap_day() {
        let date = Date::from_ymd(2024, 2, 29, None).unwrap();
        assert_eq!(date.year, 2024);
        assert_eq!(date.month, 2);
        assert_eq!(date.mday, 29);
        assert_eq!(date.seconds, None);
    }

    #[test]
    fn test_from_ymd_skipped_date() {
        let r = Date::from_ymd(1582, 10, 10, None);
        assert_eq!(r, Err(DateError::SkippedDate));
        assert_eq!(
            r.unwrap_err().to_string(),
            "date was skipped by calendar reform"
        );
    }

    #[test]
    fn test_from_ymd_first_skipped_date() {
        let r = Date::from_ymd(1582, 10, 5, None);
        assert_eq!(r, Err(DateError::SkippedDate));
        assert_eq!(
            r.unwrap_err().to_string(),
            "date was skipped by calendar reform"
        );
    }

    #[test]
    fn test_from_ymd_last_skipped_date() {
        let r = Date::from_ymd(1582, 10, 14, None);
        assert_eq!(r, Err(DateError::SkippedDate));
        assert_eq!(
            r.unwrap_err().to_string(),
            "date was skipped by calendar reform"
        );
    }

    #[rstest]
    #[case(2023, 365)]
    #[case(2023, 999)]
    #[case(2024, 366)]
    #[case(2024, 999)]
    #[case(1582, 355)]
    #[case(1582, 999)]
    fn test_from_year_yday_err(#[case] year: YearT, #[case] yday: DaysT) {
        let r = Date::from_year_yday(year, yday, None);
        assert_eq!(r, Err(DateError::YdayOutOfRange { year, yday }));
        assert_eq!(
            r.unwrap_err().to_string(),
            format!("yday {yday} is outside of valid range for year {year}")
        );
    }
}
