use super::{
    DateError, DaysT, JulianDayT, Month, ParseDateError, COMMON_YEAR_LENGTH, LEAP_YEAR_LENGTH,
};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Range, RangeInclusive};

// Julian-calendar year in which Julian day number 0 occurs
const JDN0_YEAR: i32 = -4712;

const GREGORIAN_CYCLE_DAYS: JulianDayT = 146097;
const GREGORIAN_CYCLE_YEARS: i32 = 400;

const JULIAN_LEAP_CYCLE_DAYS: JulianDayT = 1461;
const JULIAN_LEAP_CYCLE_YEARS: i32 = 4;

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
    /// Last Julian date in the calendar
    pub(crate) pre_reform: Date,

    /// First Gregorian date in the calendar
    pub(crate) post_reform: Date,

    /// Number of dates *as reckoned in the Julian calendar* (i.e., counting
    /// any Julian-only leap days that may have been skipped) between the last
    /// Julian date and the first Gregorian date
    pub(crate) gap_length: u32,

    pub(crate) kind: GapKind,

    /// Given a year and day-of-year in the proleptic Gregorian calendar where
    /// the year equals `post_reform.year`, if the ordinal is greater than
    /// `ordinal_gap_start`, then `ordinal_gap` must be subtracted from it in
    /// order to get the day-of-year in the reforming calendar.
    pub(crate) ordinal_gap_start: u32,

    pub(crate) ordinal_gap: u32,
}

impl ReformGap {
    pub(crate) fn cmp_year(&self, year: i32) -> RangeOrdering {
        cmp_range(year, self.pre_reform.year, self.post_reform.year)
    }

    pub(crate) fn cmp_year_month(&self, year: i32, month: Month) -> RangeOrdering {
        cmp_range(
            (year, month),
            (self.pre_reform.year, self.pre_reform.month),
            (self.post_reform.year, self.post_reform.month),
        )
    }

    pub(crate) fn cmp_ymd(&self, year: i32, month: Month, day: u32) -> RangeOrdering {
        cmp_range(
            (year, month, day),
            (
                self.pre_reform.year,
                self.pre_reform.month,
                self.pre_reform.day,
            ),
            (
                self.post_reform.year,
                self.post_reform.month,
                self.post_reform.day,
            ),
        )
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub(crate) struct Date {
    pub(crate) year: i32,
    pub(crate) ordinal: DaysT,
    pub(crate) month: Month,
    pub(crate) day: u32,
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
        pre_year: i32,
        pre_month: Month,
        post_year: i32,
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

    pub(crate) fn parse_year(&mut self) -> Result<i32, ParseDateError> {
        match self.data.match_indices('-').find(|&(i, _)| i > 0) {
            Some((i, _)) => {
                let year = self.data[..i].parse::<i32>()?;
                self.data = &self.data[i + 1..];
                Ok(year)
            }
            None => Err(ParseDateError::UnterminatedYear),
        }
    }

    pub(crate) fn parse_day_in_year(&mut self) -> Result<DayInYear, ParseDateError> {
        let field1 = self.parse_uint()?;
        if self.data.is_empty() {
            Ok(DayInYear::Ordinal(field1))
        } else {
            let month = Month::try_from(field1)
                .map_err(|_| ParseDateError::InvalidMonth { value: field1 })?;
            self.scan_char('-')?;
            let day = self.parse_uint()?;
            Ok(DayInYear::Date { month, day })
        }
    }

    pub(crate) fn parse_uint(&mut self) -> Result<u32, ParseDateError> {
        match self
            .data
            .char_indices()
            .take_while(|&(_, ch)| ch.is_ascii_digit())
            .last()
            .map(|(i, _)| i + 1)
        {
            Some(i) => {
                let n = self.data[..i].parse::<u32>()?;
                self.data = &self.data[i..];
                Ok(n)
            }
            None => match self.data.chars().next() {
                Some(got) => Err(ParseDateError::InvalidIntStart { got }),
                None => Err(ParseDateError::EmptyInt),
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
    Date { month: Month, day: u32 },
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) enum MonthShape {
    Solid {
        year: i32,
        month: Month,
        range: RangeInclusive<u32>,
        natural_max_day: DaysT,
    },
    HasGap {
        year: i32,
        month: Month,
        gap: Range<u32>,
        max_day: u32,
    },
    Skipped {
        year: i32,
        month: Month,
    },
}

impl MonthShape {
    pub(crate) fn len(&self) -> u32 {
        match self {
            MonthShape::Solid { range, .. } => range.end() - range.start() + 1,
            MonthShape::HasGap { gap, max_day, .. } => max_day - (gap.end - gap.start),
            MonthShape::Skipped { .. } => 0,
        }
    }

    /*
    pub(crate) fn has_day(&self, day: u32) -> bool {
        match self {
            MonthShape::Solid { range, .. } => range.contains(&day),
            &MonthShape::HasGap {
                ref gap, max_day, ..
            } => (1..=max_day).contains(&day) && !gap.contains(&day),
            MonthShape::Skipped { .. } => false,
        }
    }
    */

    // Returns a one-based ordinal
    pub(crate) fn get_day_ordinal(&self, day: u32) -> Result<u32, DateError> {
        match *self {
            MonthShape::Solid {
                year,
                month,
                ref range,
                natural_max_day,
            } => {
                if range.contains(&day) {
                    Ok(day - *range.start() + 1)
                } else if day == 0 || day > natural_max_day {
                    Err(DateError::DayOutOfRange {
                        year,
                        month,
                        day,
                        min_day: *range.start(),
                        max_day: *range.end(),
                    })
                } else {
                    Err(DateError::SkippedDate { year, month, day })
                }
            }
            MonthShape::HasGap {
                year,
                month,
                ref gap,
                max_day,
            } => {
                if day == 0 {
                    Err(DateError::DayOutOfRange {
                        year,
                        month,
                        day,
                        min_day: 1,
                        max_day,
                    })
                } else if day < gap.start {
                    Ok(day)
                } else if day < gap.end {
                    Err(DateError::SkippedDate { year, month, day })
                } else if day <= max_day {
                    Ok(day - u32::try_from(gap.len()).unwrap())
                } else {
                    Err(DateError::DayOutOfRange {
                        year,
                        month,
                        day,
                        min_day: 1,
                        max_day,
                    })
                }
            }
            // TODO: Should Skipped months return DayOutOfRange for day == 0?
            // What should the max_day be then?
            MonthShape::Skipped { year, month } => Err(DateError::SkippedDate { year, month, day }),
        }
    }

    // day_ordinal is one-based
    pub(crate) fn get_day_label(&self, day_ordinal: u32) -> Option<u32> {
        // Return early on impossibly large ordinals in order to avoid numeric
        // overflow later
        if day_ordinal == 0 || day_ordinal > 31 {
            return None;
        }
        match self {
            MonthShape::Solid { range, .. } => {
                let day = day_ordinal - 1 + range.start();
                (day <= *range.end()).then_some(day)
            }
            &MonthShape::HasGap {
                ref gap, max_day, ..
            } => {
                if day_ordinal < gap.start {
                    Some(day_ordinal)
                } else {
                    let day = day_ordinal + (gap.end - gap.start);
                    (day <= max_day).then_some(day)
                }
            }
            MonthShape::Skipped { .. } => None,
        }
    }
}

pub(crate) fn is_julian_leap_year(year: i32) -> bool {
    year % JULIAN_LEAP_CYCLE_YEARS == 0
}

pub(crate) fn is_gregorian_leap_year(year: i32) -> bool {
    year % JULIAN_LEAP_CYCLE_YEARS == 0 && (year % 100 != 0 || year % GREGORIAN_CYCLE_YEARS == 0)
}

// Convert a date in the proleptic Gregorian calendar to a Julian day number
// Returns None on arithmetic underflow/overflow
// TODO: PROBLEM: This doesn't work for dates with negative JDNs; address
pub(crate) fn gregorian_yj_to_jd(year: i32, ordinal: DaysT) -> Option<JulianDayT> {
    // JDN 0 = day 328 of year -4713 in the proleptic Gregorian calendar
    if (year, ordinal) < (-4713, 328) {
        // Negative JDN
        todo!()
    } else if year == -4713 {
        Some(JulianDayT::try_from(ordinal).unwrap() - 328)
    } else {
        // Number of centennials from -4713 through `year`
        //   = ceil((year + 4700) / 100)  [real division]
        //   = (year + 4700 + 99) / 100   [Euclidean division]
        //   = (year + 4800 - 1) / 100
        //   = (year - 1) / 100 + 48
        let centennials = (year - 1).div_euclid(100) + 48;
        // Number of quadricentennials from -4713 through `year`
        let quads = centennials / 4;
        let ydiff = add(year, 4712)?;
        // Number of leap days from -4712 up to (but not including) `year`
        // Subtract one leap day for each non-leap centennial
        //   = subtract `centennials - quads`
        let leap_days = sub(
            // We can't use `compose_julian()` here, as that order of
            // operations (specifically, not subtracting `centennials - quads`
            // in the middle) would lead to overflow for some JDN less than
            // JulianDayT::MAX.
            add(ydiff, JULIAN_LEAP_CYCLE_YEARS - 1)? / JULIAN_LEAP_CYCLE_YEARS,
            centennials - quads,
        )?;
        let year_start = add(mul(ydiff, COMMON_YEAR_LENGTH)?, leap_days)?;
        let days = add(year_start, JulianDayT::try_from(ordinal - 1).unwrap())?;
        // Add 38 (JDN of -4712-01-01 N.S.)
        add(days, 38)
    }
}

// Convert a date in the proleptic Julian calendar to a Julian day number
// Returns None on arithmetic underflow/overflow
pub(crate) fn julian_yj_to_jd(year: i32, ordinal: DaysT) -> Option<JulianDayT> {
    if year < JDN0_YEAR {
        let rev_year = sub(JDN0_YEAR, year)?;
        sub(
            JulianDayT::try_from(ordinal - 1).unwrap(),
            add(
                mul(rev_year, COMMON_YEAR_LENGTH)?,
                rev_year / JULIAN_LEAP_CYCLE_YEARS,
            )?,
        )
    } else {
        compose_julian(sub(year, JDN0_YEAR)?, ordinal)
    }
}

// Returns None on arithmetic underflow/overflow
// TODO: PROBLEM: This doesn't work for dates with negative JDNs; address
pub(crate) fn jd_to_gregorian_yj(jd: JulianDayT) -> Option<(i32, DaysT)> {
    const COMMON_CENTURY_DAYS: JulianDayT = COMMON_YEAR_LENGTH * 100 + 24;
    if jd < 0 {
        todo!()
    } else {
        // 113993 = JDN of -4400-01-01 N.S.
        let jd = jd - 113993;
        // Number of 400-year cycles elapsed since -4400-01-01:
        let quads = jd.div_euclid(GREGORIAN_CYCLE_DAYS);
        // Zero-based day within current 400-year cycle:
        let mut quad_point = jd.rem_euclid(GREGORIAN_CYCLE_DAYS);
        // Add a "virtual leap day" to the end of each centennial year after
        // the zeroth so that `decompose_julian()` can be applied.
        if let Some(after_first_year) = sub(quad_point, LEAP_YEAR_LENGTH) {
            quad_point += after_first_year / COMMON_CENTURY_DAYS;
        }
        let (ys, ordinal) = decompose_julian(quad_point)?;
        let year = add(mul(quads, 400)?, ys - 4400)?;
        Some((year, ordinal))
    }
}

// Returns None on arithmetic underflow/overflow
pub(crate) fn jd_to_julian_yj(jd: JulianDayT) -> Option<(i32, DaysT)> {
    if jd < 0 {
        let alt = sub(COMMON_YEAR_LENGTH, jd)?;
        let (alt_year, alt_ordinal) = jd_to_julian_yj(alt)?;
        let year = sub(JDN0_YEAR, sub(alt_year, JDN0_YEAR)?)?;
        let year_length = if is_julian_leap_year(year) {
            LEAP_YEAR_LENGTH as DaysT
        } else {
            COMMON_YEAR_LENGTH as DaysT
        };
        let ordinal = year_length - alt_ordinal + 1;
        Some((year, ordinal))
    } else {
        let (year, ordinal) = decompose_julian(jd)?;
        Some((add(year, JDN0_YEAR)?, ordinal))
    }
}

/// Given a nonnegative number of days from the start of a period in which
/// every fourth year is a leap year, beginning with the initial zero year,
/// return the number of completed years and the one-based day of the last
/// year.
///
/// Returns `None` on arithmetic underflow/overflow.
fn decompose_julian(days: JulianDayT) -> Option<(i32, DaysT)> {
    debug_assert!(days >= 0);
    let mut year: i32 = days / JULIAN_LEAP_CYCLE_DAYS * JULIAN_LEAP_CYCLE_YEARS;
    let mut ordinal: JulianDayT = days % JULIAN_LEAP_CYCLE_DAYS;
    // Add a "virtual leap day" to the end of each common year so that
    // `ordinal` can be divided & modded by LEAP_YEAR_LENGTH evenly:
    if ordinal > COMMON_YEAR_LENGTH {
        ordinal += (ordinal - LEAP_YEAR_LENGTH) / COMMON_YEAR_LENGTH;
    }
    year = add(year, ordinal / LEAP_YEAR_LENGTH)?;
    ordinal %= LEAP_YEAR_LENGTH;
    Some((year, DaysT::try_from(ordinal + 1).unwrap()))
}

/// Given a nonnegative number of years and a one-based day of year in a period
/// in which every fourth year is a leap year, beginning with the initial zero
/// year, return the total number of days.
///
/// Returns `None` on arithmetic underflow/overflow.
fn compose_julian(years: i32, ordinal: DaysT) -> Option<JulianDayT> {
    debug_assert!(years >= 0);
    debug_assert!(ordinal > 0);
    let common_days = mul(years, COMMON_YEAR_LENGTH)?;
    let leap_days = add(years, JULIAN_LEAP_CYCLE_YEARS - 1)? / JULIAN_LEAP_CYCLE_YEARS;
    add(
        add(common_days, leap_days)?,
        JulianDayT::try_from(ordinal - 1).unwrap(),
    )
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
    use rstest_reuse::{apply, template};

    #[rstest]
    #[case(-1, -4713, 365)]
    #[case(0, -4712, 1)]
    #[case(1, -4712, 2)]
    #[case(365, -4712, 366)]
    #[case(366, -4711, 1)]
    #[case(730, -4711, 365)]
    #[case(731, -4710, 1)]
    #[case(1095, -4710, 365)]
    #[case(1096, -4709, 1)]
    #[case(1460, -4709, 365)]
    #[case(1461, -4708, 1)]
    #[case(1826, -4708, 366)]
    #[case(1827, -4707, 1)]
    fn test_jd_to_julian_yj(#[case] jd: JulianDayT, #[case] year: i32, #[case] ordinal: DaysT) {
        assert_eq!(jd_to_julian_yj(jd), Some((year, ordinal)));
    }

    #[template]
    #[rstest]
    #[case(0, 0, 1)]
    #[case(1, 0, 2)]
    #[case(365, 0, 366)]
    #[case(366, 1, 1)]
    #[case(730, 1, 365)]
    #[case(731, 2, 1)]
    #[case(1095, 2, 365)]
    #[case(1096, 3, 1)]
    #[case(1460, 3, 365)]
    #[case(1461, 4, 1)]
    #[case(1826, 4, 366)]
    #[case(1827, 5, 1)]
    fn year_days(#[case] days: JulianDayT, #[case] years: i32, #[case] ordinal: DaysT) {}

    #[apply(year_days)]
    fn test_decompose_julian(#[case] days: JulianDayT, #[case] years: i32, #[case] ordinal: DaysT) {
        assert_eq!(decompose_julian(days), Some((years, ordinal)));
    }

    #[apply(year_days)]
    fn test_compose_julian(#[case] days: JulianDayT, #[case] years: i32, #[case] ordinal: DaysT) {
        assert_eq!(compose_julian(years, ordinal), Some(days));
    }

    #[template]
    #[rstest]
    #[case(0, -4713, 328)]
    #[case(1, -4713, 329)]
    #[case(37, -4713, 365)]
    #[case(38, -4712, 1)]
    #[case(365, -4712, 328)]
    #[case(366, -4712, 329)]
    #[case(403, -4712, 366)]
    #[case(404, -4711, 1)]
    #[case(730, -4711, 327)]
    #[case(731, -4711, 328)]
    #[case(768, -4711, 365)]
    #[case(769, -4710, 1)]
    #[case(1095, -4710, 327)]
    #[case(1096, -4710, 328)]
    #[case(1133, -4710, 365)]
    #[case(1134, -4709, 1)]
    #[case(1460, -4709, 327)]
    #[case(1461, -4709, 328)]
    #[case(1498, -4709, 365)]
    #[case(1499, -4708, 1)]
    #[case(1826, -4708, 328)]
    #[case(1827, -4708, 329)]
    #[case(1864, -4708, 366)]
    #[case(1865, -4707, 1)]
    #[case(4420, -4701, 365)]
    #[case(4785, -4700, 365)]
    #[case(4786, -4699, 1)]
    #[case(40945, -4600, 1)]
    #[case(41309, -4600, 365)]
    #[case(77469, -4500, 1)]
    #[case(113993, -4400, 1)]
    #[case(114358, -4400, 366)]
    #[case(150518, -4300, 1)]
    #[case(2147483647, 5874898, 154)]
    fn jd_gregorian_yj(#[case] jd: JulianDayT, #[case] year: i32, #[case] ordinal: DaysT) {}

    #[apply(jd_gregorian_yj)]
    fn test_jd_to_gregorian_yj(#[case] jd: JulianDayT, #[case] year: i32, #[case] ordinal: DaysT) {
        assert_eq!(jd_to_gregorian_yj(jd), Some((year, ordinal)));
    }

    #[apply(jd_gregorian_yj)]
    fn test_gregorian_yj_to_jd(#[case] jd: JulianDayT, #[case] year: i32, #[case] ordinal: DaysT) {
        assert_eq!(gregorian_yj_to_jd(year, ordinal), Some(jd));
    }

    #[test]
    fn test_gregorian_yj_to_past_max_jd() {
        assert_eq!(gregorian_yj_to_jd(5874898, 155), None);
    }
}
