use super::{DateError, Jdnum, Month, ParseDateError, COMMON_YEAR_LENGTH, LEAP_YEAR_LENGTH};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Range, RangeInclusive};

// Julian-calendar year in which Julian day number 0 occurs
const JDN0_YEAR: i32 = -4712;

const GREGORIAN_CYCLE_DAYS: Jdnum = 146097;
const GREGORIAN_CYCLE_YEARS: i32 = 400;

const JULIAN_LEAP_CYCLE_DAYS: Jdnum = 1461;
const JULIAN_LEAP_CYCLE_YEARS: i32 = 4;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Calendar {
    Julian,
    Gregorian,
    Reforming {
        reformation: Jdnum,
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

    /// Number of calendar dates *as reckoned in the Julian calendar* (i.e.,
    /// counting any Julian-only leap days that may have been skipped) from the
    /// reformation date in the Julian calendar up to (but not including) the
    /// reformation date in the Gregorian calendar.
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
    pub(crate) ordinal: u32,
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
    Ordinal(u32),
    Date { month: Month, day: u32 },
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) enum MonthShape {
    Solid {
        year: i32,
        month: Month,
        range: RangeInclusive<u32>,
        natural_max_day: u32,
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

/// Converts a Julian day number to the corresponding year and day of year in
/// the proleptic Julian calendar.
///
/// Valid for all `Jdnum` values.
pub(crate) fn jdn2julian(jd: Jdnum) -> (i32, u32) {
    let (year, ordinal) = decompose_julian(jd);
    (year + JDN0_YEAR, ordinal)
}

/// Converts a year and day of year in the proleptic Julian calendar to the
/// corresponding Julian day number.
///
/// Returns None on arithmetic underflow/overflow.
pub(crate) fn julian2jdn(year: i32, ordinal: u32) -> Option<Jdnum> {
    compose_julian(sub(year, JDN0_YEAR)?, ordinal)
}

/// Converts a Julian day number to the corresponding year and day of year in
/// the proleptic Gregorian calendar.
///
/// Valid for all `Jdnum` values.
pub(crate) fn jdn2gregorian(jd: Jdnum) -> (i32, u32) {
    const COMMON_CENTURY_DAYS: Jdnum = COMMON_YEAR_LENGTH * 100 + 24;
    // Calculate relative to a nearby quadricentennial.  Shift towards zero in
    // order to avoid overflow/underflow.
    let (offset, year_offset) = if jd < 0 {
        // JDN -32104 = -4800-01-01 N.S.
        (-32104, -4800)
    } else {
        // JDN 113993 = -4400-01-01 N.S.
        (113993, -4400)
    };
    let jd = jd - offset;
    // Number of 400-year cycles until Jan 1 of `year_offset`
    let quads = jd.div_euclid(GREGORIAN_CYCLE_DAYS);
    // Zero-based day within current 400-year cycle, counting from the
    // beginning of time:
    let mut quad_point = jd.rem_euclid(GREGORIAN_CYCLE_DAYS);
    // Add a "virtual leap day" to the end of each centennial year after the
    // zeroth so that `decompose_julian()` can be applied.
    if let Some(after_first_year) = sub(quad_point, LEAP_YEAR_LENGTH) {
        quad_point += after_first_year / COMMON_CENTURY_DAYS;
    }
    let (ys, ordinal) = decompose_julian(quad_point);
    let year = quads * 400 + ys + year_offset;
    (year, ordinal)
}

/// Converts a year and day of year in the proleptic Gregorian calendar to the
/// corresponding Julian day number.
///
/// Returns None on arithmetic underflow/overflow.
pub(crate) fn gregorian2jdn(year: i32, ordinal: u32) -> Option<Jdnum> {
    // JDN 0 = day 328 of year -4713 in the proleptic Gregorian calendar
    // Number of centennials from -4713 through `year`
    //   = ceil((year + 4700) / 100)  [real division]
    //   = (year + 4700 + 99) / 100   [Euclidean division]
    //   = (year + 4800 - 1) / 100
    //   = (year - 1) / 100 + 48
    let centennials = sub(year, 1)?.div_euclid(100) + 48;
    // Number of quadricentennials from -4713 through `year`
    let quads = centennials.div_euclid(4);
    let ydiff = add(year, 4712)?;
    // Number of leap days from the start of -4712 up to (but not
    // including) the start of `year`
    // Subtract one leap day for each non-leap centennial
    //   = subtract `centennials - quads`
    let leap_days = sub(
        // We can't use `compose_julian()` here, as that order of
        // operations (specifically, not subtracting `centennials - quads`
        // in the middle) would lead to overflow for some JDN less than
        // Jdnum::MAX.
        add(ydiff, JULIAN_LEAP_CYCLE_YEARS - 1)?.div_euclid(JULIAN_LEAP_CYCLE_YEARS),
        centennials - quads,
    )?;
    let year_days = mul(ydiff, COMMON_YEAR_LENGTH)?;
    // Add 38 (JDN of -4712-01-01 N.S.)
    let offset = Jdnum::try_from(ordinal - 1).unwrap() + 38;
    add(add(year_days, offset)?, leap_days)
}

/// Given a number of days from the start of a period in which every fourth
/// year is a leap year, beginning with the initial zero year, return the
/// number of completed years and the one-based day of the last year.
///
/// If the number of days is negative, the year will be negative, and the day
/// of the year will be positive.
///
/// Valid for all `Jdnum` values.
fn decompose_julian(days: Jdnum) -> (i32, u32) {
    let mut year: i32 = days.div_euclid(JULIAN_LEAP_CYCLE_DAYS) * JULIAN_LEAP_CYCLE_YEARS;
    let mut ordinal: Jdnum = days.rem_euclid(JULIAN_LEAP_CYCLE_DAYS);
    // Add a "virtual leap day" to the end of each common year so that
    // `ordinal` can be divided & modded by LEAP_YEAR_LENGTH evenly:
    if ordinal > COMMON_YEAR_LENGTH {
        ordinal += (ordinal - LEAP_YEAR_LENGTH) / COMMON_YEAR_LENGTH;
    }
    year += ordinal / LEAP_YEAR_LENGTH;
    ordinal %= LEAP_YEAR_LENGTH;
    (year, u32::try_from(ordinal + 1).unwrap())
}

/// Given a (possibly negative) number of years and a one-based positive day of
/// year in a period in which every fourth year is a leap year, beginning with
/// the initial zero year, return the total number of days.
///
/// Returns `None` on arithmetic underflow/overflow.
fn compose_julian(years: i32, ordinal: u32) -> Option<Jdnum> {
    debug_assert!(ordinal > 0);
    let common_days = mul(years, COMMON_YEAR_LENGTH)?;
    let leap_days = add(years, JULIAN_LEAP_CYCLE_YEARS - 1)?.div_euclid(JULIAN_LEAP_CYCLE_YEARS);
    add(
        common_days,
        add(leap_days, Jdnum::try_from(ordinal - 1).unwrap())?,
    )
}

#[inline]
fn add(x: Jdnum, y: Jdnum) -> Option<Jdnum> {
    x.checked_add(y)
}

#[inline]
fn sub(x: Jdnum, y: Jdnum) -> Option<Jdnum> {
    x.checked_sub(y)
}

#[inline]
fn mul(x: Jdnum, y: Jdnum) -> Option<Jdnum> {
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

    #[template]
    #[rstest]
    #[case(-2147483648, -5879490, 75)]
    #[case(-1462, -5, 365)]
    #[case(-1461, -4, 1)]
    #[case(-1096, -4, 366)]
    #[case(-1095, -3, 1)]
    #[case(-731, -3, 365)]
    #[case(-730, -2, 1)]
    #[case(-366, -2, 365)]
    #[case(-365, -1, 1)]
    #[case(-2, -1, 364)]
    #[case(-1, -1, 365)]
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
    #[case(2147483647, 5879489, 290)]
    fn year_days(#[case] days: Jdnum, #[case] years: i32, #[case] ordinal: u32) {}

    #[apply(year_days)]
    fn test_decompose_julian(#[case] days: Jdnum, #[case] years: i32, #[case] ordinal: u32) {
        assert_eq!(decompose_julian(days), (years, ordinal));
    }

    #[apply(year_days)]
    fn test_compose_julian(#[case] days: Jdnum, #[case] years: i32, #[case] ordinal: u32) {
        assert_eq!(compose_julian(years, ordinal), Some(days));
    }

    #[template]
    #[rstest]
    #[case(-2147483648, -5884202, 75)]
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
    #[case(2147483647, 5874777, 290)]
    fn jd_julian_yj(#[case] jd: Jdnum, #[case] year: i32, #[case] ordinal: u32) {}

    #[apply(jd_julian_yj)]
    fn test_jdn2julian(#[case] jd: Jdnum, #[case] year: i32, #[case] ordinal: u32) {
        assert_eq!(jdn2julian(jd), (year, ordinal));
    }

    #[apply(jd_julian_yj)]
    fn test_julian2jdn(#[case] jd: Jdnum, #[case] year: i32, #[case] ordinal: u32) {
        assert_eq!(julian2jdn(year, ordinal), Some(jd));
    }

    #[test]
    fn test_julian_to_pre_min_jdn() {
        assert_eq!(julian2jdn(-5884202, 74), None);
    }

    #[test]
    fn test_julian_to_past_max_jdn() {
        assert_eq!(julian2jdn(5874777, 291), None);
    }

    #[template]
    #[rstest]
    #[case(-2147483648, -5884323, 135)]
    #[case(-214725, -5300, 1)]
    #[case(-178201, -5200, 1)]
    #[case(-141676, -5100, 1)]
    #[case(-105152, -5000, 1)]
    #[case(-68628, -4900, 1)]
    #[case(-32469, -4801, 1)]
    #[case(-32142, -4801, 328)]
    #[case(-32104, -4800, 1)]
    #[case(-31738, -4799, 1)]
    #[case(-327, -4713, 1)]
    #[case(-1, -4713, 327)]
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
    fn jd_gregorian_yj(#[case] jd: Jdnum, #[case] year: i32, #[case] ordinal: u32) {}

    #[apply(jd_gregorian_yj)]
    fn test_jdn2gregorian(#[case] jd: Jdnum, #[case] year: i32, #[case] ordinal: u32) {
        assert_eq!(jdn2gregorian(jd), (year, ordinal));
    }

    #[apply(jd_gregorian_yj)]
    fn test_gregorian2jdn(#[case] jd: Jdnum, #[case] year: i32, #[case] ordinal: u32) {
        assert_eq!(gregorian2jdn(year, ordinal), Some(jd));
    }

    #[test]
    fn test_gregorian_to_pre_min_jdn() {
        assert_eq!(gregorian2jdn(-5884323, 134), None);
    }

    #[test]
    fn test_gregorian_to_past_max_jdn() {
        assert_eq!(gregorian2jdn(5874898, 155), None);
    }
}
