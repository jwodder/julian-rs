use super::{
    DaysT, Error, JulianDayT, Month, ParseDateError, YearT, COMMON_YEAR_LENGTH, LEAP_YEAR_LENGTH,
};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Range, RangeInclusive};

// Julian-calendar year in which Julian day 0 occurs
const JD0_YEAR: YearT = -4712;

const GREGORIAN_CYCLE_DAYS: JulianDayT = 146097;
const GREGORIAN_CYCLE_YEARS: YearT = 400;

const JULIAN_LEAP_CYCLE_DAYS: JulianDayT = 1461;
const JULIAN_LEAP_CYCLE_YEARS: YearT = 4;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Calendar {
    Julian,
    Gregorian,
    Reforming {
        reformation: JulianDayT,
        // Ignored by comparison traits and Hash, as it's a function of
        // `reformation`:
        gap: ReformGap,
    },
}

impl PartialEq for Calendar {
    fn eq(&self, other: &Calendar) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Calendar {}

impl PartialOrd for Calendar {
    fn partial_cmp(&self, other: &Calendar) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Calendar {
    fn cmp(&self, other: &Calendar) -> Ordering {
        use Calendar::*;
        match (self, other) {
            (Julian, Julian) => Ordering::Equal,
            (Julian, _) => Ordering::Less,
            (Reforming { .. }, Julian) => Ordering::Greater,
            (
                Reforming {
                    reformation: r1, ..
                },
                Reforming {
                    reformation: r2, ..
                },
            ) => r1.cmp(r2),
            (Reforming { .. }, Gregorian) => Ordering::Less,
            (Gregorian, Gregorian) => Ordering::Equal,
            (Gregorian, _) => Ordering::Greater,
        }
    }
}

// <https://stackoverflow.com/a/69192884/744178>
impl Hash for Calendar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Calendar::Julian => state.write_u8(1),
            Calendar::Gregorian => state.write_u8(2),
            Calendar::Reforming { reformation, .. } => {
                state.write_u8(3);
                reformation.hash(state);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub(crate) struct ReformGap {
    pub(crate) pre_reform: Date,
    pub(crate) post_reform: Date,
    pub(crate) gap_length: u32,
    pub(crate) kind: GapKind,
}

impl ReformGap {
    pub(crate) fn cmp_year(&self, year: YearT) -> RangeOrdering {
        cmp_range(year, self.pre_reform.year, self.post_reform.year)
    }

    pub(crate) fn cmp_year_month(&self, year: YearT, month: Month) -> RangeOrdering {
        cmp_range(
            (year, month),
            (self.pre_reform.year, self.pre_reform.month),
            (self.post_reform.year, self.post_reform.month),
        )
    }

    pub(crate) fn cmp_ymd(&self, year: YearT, month: Month, mday: u32) -> RangeOrdering {
        cmp_range(
            (year, month, mday),
            (
                self.pre_reform.year,
                self.pre_reform.month,
                self.pre_reform.mday,
            ),
            (
                self.post_reform.year,
                self.post_reform.month,
                self.post_reform.mday,
            ),
        )
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub(crate) struct Date {
    pub(crate) year: YearT,
    pub(crate) ordinal: DaysT,
    pub(crate) month: Month,
    pub(crate) mday: u32,
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum GapKind {
    IntraMonth,
    CrossMonth,
    CrossYear,
    MultiYear,
}

impl GapKind {
    pub(crate) fn for_dates(
        pre_year: YearT,
        pre_month: Month,
        post_year: YearT,
        post_month: Month,
    ) -> GapKind {
        if pre_year == post_year {
            if pre_month == post_month {
                GapKind::IntraMonth
            } else {
                GapKind::CrossMonth
            }
        } else if pre_year + 1 == post_year {
            GapKind::CrossYear
        } else {
            GapKind::MultiYear
        }
    }
}

pub(crate) struct DateParser<'a> {
    data: &'a str,
}

impl<'a> DateParser<'a> {
    pub(crate) fn new(data: &'a str) -> Self {
        DateParser { data }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub(crate) fn parse_year(&mut self) -> Result<YearT, ParseDateError> {
        match self.data.match_indices('-').find(|&(i, _)| i > 0) {
            Some((i, _)) => {
                let year = self.data[..i].parse::<YearT>()?;
                self.data = &self.data[i + 1..];
                Ok(year)
            }
            None => Err(ParseDateError::UnterminatedYear),
        }
    }

    pub(crate) fn parse_day_in_year(&mut self) -> Result<DayInYear, ParseDateError> {
        if let Some(i) = self.data.find(['-', 'T']) {
            if &self.data[i..(i + 1)] == "T" {
                let ordinal = Self::parse_ordinal(&self.data[..i])?;
                self.data = &self.data[i..];
                Ok(ordinal)
            } else {
                let month_index = self.parse_02d()?;
                let month = Month::try_from(month_index)
                    .map_err(|_| ParseDateError::InvalidMonth { value: month_index })?;
                self.scan_char('-')?;
                let mday = self.parse_02d()?;
                Ok(DayInYear::Date { month, mday })
            }
        } else {
            let ordinal = Self::parse_ordinal(self.data)?;
            self.data = "";
            Ok(ordinal)
        }
    }

    pub(crate) fn parse_ordinal(s: &str) -> Result<DayInYear, ParseDateError> {
        if s.len() == 3 && s.chars().all(|c| c.is_ascii_digit()) {
            Ok(DayInYear::Ordinal(s.parse::<DaysT>()?))
        } else {
            Err(ParseDateError::InvalidOrdinal)
        }
    }

    pub(crate) fn parse_02d(&mut self) -> Result<u32, ParseDateError> {
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
            Some(got) => Err(ParseDateError::Invalid02dLength { got }),
            None => match self.data.chars().next() {
                Some(got) => Err(ParseDateError::Invalid02dStart { got }),
                None => Err(ParseDateError::Invalid02dSuddenEnd),
            },
        }
    }

    pub(crate) fn scan_char(&mut self, ch: char) -> Result<(), ParseDateError> {
        if let Some(s) = self.data.strip_prefix(ch) {
            self.data = s;
            Ok(())
        } else {
            match self.data.chars().next() {
                Some(c2) => Err(ParseDateError::UnexpectedChar {
                    expected: ch,
                    got: c2,
                }),
                None => Err(ParseDateError::UnexpectedEnd { expected: ch }),
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub(crate) enum DayInYear {
    Ordinal(DaysT),
    Date { month: Month, mday: u32 },
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) enum MonthShape {
    Solid {
        year: YearT,
        month: Month,
        range: RangeInclusive<u32>,
    },
    HasGap {
        year: YearT,
        month: Month,
        gap: Range<u32>,
        max_mday: u32,
    },
    Skipped {
        year: YearT,
        month: Month,
    },
}

impl MonthShape {
    pub(crate) fn len(&self) -> u32 {
        match self {
            MonthShape::Solid { range, .. } => range.end() - range.start() + 1,
            MonthShape::HasGap { gap, max_mday, .. } => max_mday - (gap.end - gap.start),
            MonthShape::Skipped { .. } => 0,
        }
    }

    pub(crate) fn has_mday(&self, mday: u32) -> bool {
        match self {
            MonthShape::Solid { range, .. } => range.contains(&mday),
            &MonthShape::HasGap {
                ref gap, max_mday, ..
            } => (1..=max_mday).contains(&mday) && !gap.contains(&mday),
            MonthShape::Skipped { .. } => false,
        }
    }

    // Returns a one-based ordinal
    pub(crate) fn get_mday_ordinal(&self, mday: u32) -> Result<u32, Error> {
        match *self {
            MonthShape::Solid {
                year,
                month,
                ref range,
            } => {
                if self.has_mday(mday) {
                    Ok(mday)
                } else if *range.start() == 1 {
                    Err(Error::MdayOutOfRange { year, month, mday })
                } else {
                    Err(Error::SkippedDate { year, month, mday })
                }
            }
            MonthShape::HasGap {
                year,
                month,
                ref gap,
                max_mday,
            } => {
                if mday == 0 {
                    Err(Error::MdayOutOfRange { year, month, mday })
                } else if mday < gap.start {
                    Ok(mday)
                } else if mday < gap.end {
                    Err(Error::SkippedDate { year, month, mday })
                } else if mday <= max_mday {
                    Ok(mday - u32::try_from(gap.len()).unwrap())
                } else {
                    Err(Error::MdayOutOfRange { year, month, mday })
                }
            }
            MonthShape::Skipped { year, month } => Err(Error::SkippedDate { year, month, mday }),
        }
    }

    // mday_ordinal is one-based
    pub(crate) fn get_mday_label(&self, mday_ordinal: u32) -> Option<u32> {
        match self {
            MonthShape::Solid { range, .. } => {
                let mday = mday_ordinal - 1 + range.start();
                (mday <= *range.end()).then_some(mday)
            }
            &MonthShape::HasGap {
                ref gap, max_mday, ..
            } => {
                if mday_ordinal < gap.start {
                    Some(mday_ordinal)
                } else {
                    let mday = mday_ordinal + (gap.end - gap.start);
                    (mday <= max_mday).then_some(mday)
                }
            }
            MonthShape::Skipped { .. } => None,
        }
    }
}

pub(crate) fn is_julian_leap_year(year: YearT) -> bool {
    year % JULIAN_LEAP_CYCLE_YEARS == 0
}

pub(crate) fn is_gregorian_leap_year(year: YearT) -> bool {
    year % JULIAN_LEAP_CYCLE_YEARS == 0 && (year % 100 != 0 || year % GREGORIAN_CYCLE_YEARS == 0)
}

// Convert a date in the proleptic Gregorian calendar to a Julian day
// Returns None on arithmetic underflow/overflow
// TODO: PROBLEM: This doesn't work for dates with negative JDs; address
// TODO: Try to rewrite to take ordinal instead of month & mday?
pub(crate) fn gregorian_ymd_to_jd(year: YearT, month: Month, mday: u32) -> Option<JulianDayT> {
    const MONTHS: JulianDayT = 12;
    let a = (month.number() as JulianDayT) - 14;
    add(
        sub(
            add(
                mul(JULIAN_LEAP_CYCLE_DAYS, add(add(year, 4800)?, a / MONTHS)?)?
                    / JULIAN_LEAP_CYCLE_YEARS,
                mul(
                    367,
                    month.number() as JulianDayT - 2 - mul(MONTHS, a / MONTHS)?,
                )? / MONTHS,
            )?,
            mul(3, (add(year, 4900)? + a / MONTHS) / 100)? / JULIAN_LEAP_CYCLE_YEARS,
        )?,
        sub(mday as JulianDayT, 32075)?,
    )
}

// Convert a date in the proleptic Julian calendar to a Julian day
// Returns None on arithmetic underflow/overflow
pub(crate) fn julian_yj_to_jd(year: YearT, ordinal: DaysT) -> Option<JulianDayT> {
    let idays = JulianDayT::try_from(ordinal - 1).unwrap();
    if year < JD0_YEAR {
        let rev_year = sub(JD0_YEAR, year)?;
        sub(
            idays,
            add(
                mul(rev_year, COMMON_YEAR_LENGTH)?,
                rev_year / JULIAN_LEAP_CYCLE_YEARS,
            )?,
        )
    } else {
        add(
            add(
                mul(sub(year, JD0_YEAR)?, COMMON_YEAR_LENGTH)?,
                add(year, -JD0_YEAR + JULIAN_LEAP_CYCLE_YEARS - 1)? / JULIAN_LEAP_CYCLE_YEARS,
            )?,
            idays,
        )
    }
}

// Returns None on arithmetic underflow/overflow
// TODO: PROBLEM: This doesn't work for dates with negative JDs; address
// TODO: Rewrite to return ordinal instead of month & mday?
pub(crate) fn jd_to_gregorian_ymd(jd: JulianDayT) -> Option<(YearT, Month, u32)> {
    let ell = add(jd, 68569)?;
    let n = mul(4, ell)? / GREGORIAN_CYCLE_DAYS;
    let ell = sub(ell, add(mul(GREGORIAN_CYCLE_DAYS, n)?, 3)? / 4)?;
    let i = mul(4000, add(ell, 1)?)? / 1461001;
    let ell = add(sub(ell, mul(JULIAN_LEAP_CYCLE_DAYS, i)? / 4)?, 31)?;
    let j = mul(80, ell)? / 2447;
    let d = sub(ell, mul(2447, j)? / 80)?;
    let ell = j / 11;
    let m = sub(add(j, 2)?, mul(12, ell)?)?;
    let y = add(add(mul(100, sub(n, 49)?)?, i)?, ell)?;
    Some((
        y,
        Month::try_from(u32::try_from(m).unwrap()).unwrap(),
        u32::try_from(d).unwrap(),
    ))
}

// Returns None on arithmetic underflow/overflow
pub(crate) fn jd_to_julian_yj(jd: JulianDayT) -> Option<(YearT, DaysT)> {
    if jd < 0 {
        let alt = sub(COMMON_YEAR_LENGTH, jd)?;
        let (alt_year, alt_ordinal) = jd_to_julian_yj(alt)?;
        let year = sub(JD0_YEAR, sub(alt_year, JD0_YEAR)?)?;
        let year_length = if is_julian_leap_year(year) {
            LEAP_YEAR_LENGTH as DaysT
        } else {
            COMMON_YEAR_LENGTH as DaysT
        };
        let ordinal = year_length - alt_ordinal + 1;
        Some((year, ordinal))
    } else {
        let mut year: YearT = jd / JULIAN_LEAP_CYCLE_DAYS * JULIAN_LEAP_CYCLE_YEARS;
        let mut ordinal: JulianDayT = jd % JULIAN_LEAP_CYCLE_DAYS;
        // Add a "virtual leap day" to the end of each common year so that
        // `ordinal` can be divided & modded by LEAP_YEAR_LENGTH evenly:
        if ordinal > COMMON_YEAR_LENGTH {
            ordinal += (ordinal - LEAP_YEAR_LENGTH) / COMMON_YEAR_LENGTH;
        }
        year += add(ordinal / LEAP_YEAR_LENGTH, JD0_YEAR)?;
        ordinal %= LEAP_YEAR_LENGTH;
        Some((year, DaysT::try_from(ordinal + 1).unwrap()))
    }
}

#[inline]
fn add(x: JulianDayT, y: JulianDayT) -> Option<JulianDayT> {
    x.checked_add(y)
}

#[inline]
fn sub(x: JulianDayT, y: JulianDayT) -> Option<JulianDayT> {
    x.checked_sub(y)
}

#[inline]
fn mul(x: JulianDayT, y: JulianDayT) -> Option<JulianDayT> {
    x.checked_mul(y)
}

// There is no need to check division, as it only fails with a divisor of zero
// or negative one, which we're not using.

pub(crate) fn cmp_range<T: Ord>(value: T, lower: T, upper: T) -> RangeOrdering {
    assert!(lower <= upper);
    match (value.cmp(&lower), value.cmp(&upper)) {
        (Ordering::Less, _) => RangeOrdering::Less,
        (Ordering::Equal, Ordering::Less) => RangeOrdering::EqLower,
        (Ordering::Equal, Ordering::Equal) => RangeOrdering::EqBoth,
        (Ordering::Equal, Ordering::Greater) => unreachable!(),
        (Ordering::Greater, Ordering::Less) => RangeOrdering::Between,
        (Ordering::Greater, Ordering::Equal) => RangeOrdering::EqUpper,
        (Ordering::Greater, Ordering::Greater) => RangeOrdering::Greater,
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub(crate) enum RangeOrdering {
    Less,
    EqLower,
    Between,
    EqBoth,
    EqUpper,
    Greater,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(-1, -4713, 365)]
    #[case(0, -4712, 1)]
    #[case(366, -4711, 1)]
    fn test_jd_to_julian_yj(#[case] jd: JulianDayT, #[case] year: YearT, #[case] ordinal: DaysT) {
        assert_eq!(jd_to_julian_yj(jd), Some((year, ordinal)));
    }
}
