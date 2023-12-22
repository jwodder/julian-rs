use super::{errors::ParseDateError, Jdnum, Month, COMMON_YEAR_LENGTH, LEAP_YEAR_LENGTH};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

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

    pub(crate) kind: GapKind,

    /// Given a year and day-of-year in the proleptic Gregorian calendar where
    /// the year equals `post_reform.year`, if the ordinal is greater than
    /// `ordinal_gap_start`, then `ordinal_gap` must be subtracted from it in
    /// order to get the day-of-year in the reforming calendar.
    pub(crate) ordinal_gap_start: u32,

    pub(crate) ordinal_gap: u32,
}

impl ReformGap {
    pub(crate) const fn cmp_year(&self, year: i32) -> RangeOrdering {
        cmp_int_range(year, self.pre_reform.year, self.post_reform.year)
    }

    pub(crate) const fn cmp_year_month(&self, year: i32, month: Month) -> RangeOrdering {
        cmp_ym_range(
            (year, month),
            (self.pre_reform.year, self.pre_reform.month),
            (self.post_reform.year, self.post_reform.month),
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

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub(crate) enum MonthShape {
    Normal {
        max_day: u32,
    },
    Headless {
        min_day: u32,
        max_day: u32,
    },
    Tailless {
        max_day: u32,
        natural_max_day: u32,
    },
    Gapped {
        gap_start: u32,
        gap_end: u32,
        max_day: u32,
    },
}

pub(crate) struct DateParser<'a> {
    data: &'a str,
}

impl<'a> DateParser<'a> {
    pub(crate) const fn new(data: &'a str) -> Self {
        DateParser { data }
    }

    pub(crate) const fn is_empty(&self) -> bool {
        self.data.is_empty()
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

    pub(crate) fn parse_int(&mut self) -> Result<i32, ParseDateError> {
        let mut first = true;
        match scan(self.data, |c| {
            (std::mem::replace(&mut first, false) && (c == '-' || c == '+')) || c.is_ascii_digit()
        }) {
            ("", _) => match self.data.chars().next() {
                Some(got) => Err(ParseDateError::InvalidIntStart { got }),
                None => Err(ParseDateError::EmptyInt),
            },
            (numstr, rest) => {
                let n = numstr.parse::<i32>()?;
                self.data = rest;
                Ok(n)
            }
        }
    }

    pub(crate) fn parse_uint(&mut self) -> Result<u32, ParseDateError> {
        match scan(self.data, |c| c.is_ascii_digit()) {
            ("", _) => match self.data.chars().next() {
                Some(got) => Err(ParseDateError::InvalidUIntStart { got }),
                None => Err(ParseDateError::EmptyInt),
            },
            (numstr, rest) => {
                let n = numstr.parse::<u32>()?;
                self.data = rest;
                Ok(n)
            }
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

pub(crate) fn scan<P: FnMut(char) -> bool>(s: &str, mut predicate: P) -> (&str, &str) {
    let boundary = s
        .char_indices()
        .find(move |&(_, ch)| !predicate(ch))
        .map_or_else(|| s.len(), |(i, _)| i);
    s.split_at(boundary)
}

pub(crate) const fn is_julian_leap_year(year: i32) -> bool {
    year % JULIAN_LEAP_CYCLE_YEARS == 0
}

pub(crate) const fn is_gregorian_leap_year(year: i32) -> bool {
    year % JULIAN_LEAP_CYCLE_YEARS == 0 && (year % 100 != 0 || year % GREGORIAN_CYCLE_YEARS == 0)
}

/// Converts a Julian day number to the corresponding year and day of year in
/// the proleptic Julian calendar.
///
/// Valid for all `Jdnum` values.
pub(crate) const fn jdn2julian(jd: Jdnum) -> (i32, u32) {
    let (year, ordinal) = decompose_julian(jd);
    (year + JDN0_YEAR, ordinal)
}

/// Converts a year and day of year in the proleptic Julian calendar to the
/// corresponding Julian day number.
///
/// Returns None on arithmetic underflow/overflow.
pub(crate) const fn julian2jdn(year: i32, ordinal: u32) -> Option<Jdnum> {
    // Use a match because `?` isn't allowed in const fn's.
    match year.checked_sub(JDN0_YEAR) {
        Some(years) => compose_julian(years, ordinal),
        None => None,
    }
}

/// Converts a Julian day number to the corresponding year and day of year in
/// the proleptic Gregorian calendar.
///
/// Valid for all `Jdnum` values.
pub(crate) const fn jdn2gregorian(jd: Jdnum) -> (i32, u32) {
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
    if let Some(after_first_year) = quad_point.checked_sub(LEAP_YEAR_LENGTH) {
        // This is the one point at which we need to use truncated division
        // rather than Euclidean division.
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
#[allow(clippy::cast_possible_wrap)]
pub(crate) const fn gregorian2jdn(year: i32, ordinal: u32) -> Option<Jdnum> {
    // Tuple comparison doesn't work in const contexts
    if year < -5884323
        || (year == -5884323 && ordinal < 135)
        || (year == 5874898 && ordinal > 154)
        || year > 5874898
    {
        // Would overflow/underflow
        return None;
    }
    // JDN 0 = day 328 of year -4713 in the proleptic Gregorian calendar
    // Number of centennials from -4713 through `year`
    //   = ceil((year + 4700) / 100)  [real division]
    //   = (year + 4700 + 99) / 100   [Euclidean division]
    //   = (year + 4800 - 1) / 100
    //   = (year - 1) / 100 + 48
    let centennials = (year - 1).div_euclid(100) + 48;
    // Number of quadricentennials from -4713 through `year`
    let quads = centennials.div_euclid(4);
    let ydiff = year + 4712;
    // Number of leap days from the start of -4712 up to (but not
    // including) the start of `year`
    // Subtract one leap day for each non-leap centennial
    //   = subtract `centennials - quads`
    // We can't use `compose_julian()` here, as that order of operations
    // (specifically, not subtracting `centennials - quads` in the middle)
    // would lead to overflow for some JDN less than Jdnum::MAX.
    let leap_days = (ydiff + (JULIAN_LEAP_CYCLE_YEARS - 1)).div_euclid(JULIAN_LEAP_CYCLE_YEARS)
        - (centennials - quads);
    let year_days = ydiff * COMMON_YEAR_LENGTH;
    // Add 38 (JDN of -4712-01-01 N.S.)
    let offset = ((ordinal - 1) as Jdnum) + 38;
    Some((year_days + offset) + leap_days)
}

/// Given a number of days from the start of a period in which every fourth
/// year is a leap year, beginning with the initial zero year, return the
/// number of completed years and the one-based day of the last year.
///
/// If the number of days is negative, the year will be negative, and the day
/// of the year will be positive.
///
/// Valid for all `Jdnum` values.
#[allow(clippy::cast_sign_loss)]
const fn decompose_julian(days: Jdnum) -> (i32, u32) {
    let mut year: i32 = days.div_euclid(JULIAN_LEAP_CYCLE_DAYS) * JULIAN_LEAP_CYCLE_YEARS;
    let mut ordinal: Jdnum = days.rem_euclid(JULIAN_LEAP_CYCLE_DAYS);
    // Add a "virtual leap day" to the end of each common year so that
    // `ordinal` can be divided & modded by LEAP_YEAR_LENGTH evenly:
    if ordinal > COMMON_YEAR_LENGTH {
        ordinal += (ordinal - LEAP_YEAR_LENGTH).div_euclid(COMMON_YEAR_LENGTH);
    }
    year += ordinal.div_euclid(LEAP_YEAR_LENGTH);
    ordinal %= LEAP_YEAR_LENGTH;
    (year, (ordinal + 1) as u32)
}

/// Given a (possibly negative) number of years and a one-based positive day of
/// year in a period in which every fourth year is a leap year, beginning with
/// the initial zero year, return the total number of days.
///
/// Returns `None` on arithmetic underflow/overflow.
#[allow(clippy::cast_possible_wrap)]
const fn compose_julian(years: i32, ordinal: u32) -> Option<Jdnum> {
    // Tuple comparison doesn't work in const contexts
    if years < -5879490
        || (years == -5879490 && ordinal < 75)
        || (years == 5879489 && ordinal > 290)
        || years > 5879489
    {
        // Would overflow/underflow
        return None;
    }
    debug_assert!(
        ordinal > 0,
        "compose_julian: ordinal must be greater than zero"
    );
    let common_days = years * COMMON_YEAR_LENGTH;
    let leap_days = (years + JULIAN_LEAP_CYCLE_YEARS - 1).div_euclid(JULIAN_LEAP_CYCLE_YEARS);
    // Add `leap_days + (ordinal - 1)` first to avoid underflow when
    // `common_days` and `leap_days` are both negative numbers of large
    // magnitude:
    Some(common_days + (leap_days + ((ordinal - 1) as Jdnum)))
}

const fn cmp_int_range(value: i32, lower: i32, upper: i32) -> RangeOrdering {
    debug_assert!(lower <= upper);
    if value < lower {
        RangeOrdering::Less
    } else if lower == value {
        if value < upper {
            RangeOrdering::EqLower
        } else {
            // debug_assert_eq! isn't const.
            debug_assert!(value == upper);
            RangeOrdering::EqBoth
        }
    } else {
        debug_assert!(lower < value);
        if value < upper {
            RangeOrdering::Between
        } else if value == upper {
            RangeOrdering::EqUpper
        } else {
            debug_assert!(upper < value);
            RangeOrdering::Greater
        }
    }
}

const fn cmp_ym_range(
    value: (i32, Month),
    lower: (i32, Month),
    upper: (i32, Month),
) -> RangeOrdering {
    let (year, month) = value;
    let (lower_year, lower_month) = lower;
    let (upper_year, upper_month) = upper;
    debug_assert!(
        lower_year < upper_year || (lower_year == upper_year && lower_month.le(upper_month))
    );
    if year < lower_year {
        RangeOrdering::Less
    } else if lower_year == year {
        if month.lt(lower_month) {
            RangeOrdering::Less
        } else if month.eq(lower_month) {
            if year < upper_year || (year == upper_year && month.lt(upper_month)) {
                RangeOrdering::EqLower
            } else {
                debug_assert!(year == upper_year && month.eq(upper_month));
                RangeOrdering::EqBoth
            }
        } else {
            debug_assert!(lower_month.lt(month));
            if year < upper_year {
                RangeOrdering::Between
            } else {
                debug_assert!(year == upper_year);
                if month.lt(upper_month) {
                    RangeOrdering::Between
                } else if month.eq(upper_month) {
                    RangeOrdering::EqUpper
                } else {
                    debug_assert!(upper_month.lt(month));
                    RangeOrdering::Greater
                }
            }
        }
    } else {
        debug_assert!(lower_year < year);
        if year < upper_year {
            RangeOrdering::Between
        } else if year == upper_year {
            if month.lt(upper_month) {
                RangeOrdering::Between
            } else if month.eq(upper_month) {
                RangeOrdering::EqUpper
            } else {
                debug_assert!(upper_month.lt(month));
                RangeOrdering::Greater
            }
        } else {
            debug_assert!(upper_year < year);
            RangeOrdering::Greater
        }
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
    #[case(-2147483647, -5879490, 76)]
    #[case(-2147483284, -5879489, 74)]
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
    #[case(2147483282, 5879488, 291)]
    #[case(2147483646, 5879489, 289)]
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

    #[rstest]
    #[case(-5879491, 76)]
    #[case(-5879490, 74)]
    #[case(5879489, 291)]
    #[case(5879490, 289)]
    fn test_compose_julian_out_of_bounds(#[case] years: i32, #[case] ordinal: u32) {
        assert_eq!(compose_julian(years, ordinal), None);
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
    fn julian_to_pre_min_jdn() {
        assert_eq!(julian2jdn(-5884202, 74), None);
    }

    #[test]
    fn julian_to_past_max_jdn() {
        assert_eq!(julian2jdn(5874777, 291), None);
    }

    #[template]
    #[rstest]
    #[case(-2147483648, -5884323, 135)]
    #[case(-2147483647, -5884323, 136)]
    #[case(-2147483284, -5884322, 134)]
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
    #[case(2147483283, 5874897, 155)]
    #[case(2147483646, 5874898, 153)]
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

    #[rstest]
    #[case(-5884324, 136)]
    #[case(-5884323, 134)]
    #[case(5874898, 155)]
    #[case(5874899, 153)]
    fn test_gregorian2jdn_out_of_bounds(#[case] year: i32, #[case] ordinal: u32) {
        assert_eq!(gregorian2jdn(year, ordinal), None);
    }

    #[test]
    fn cmp_nontrivial_int_range() {
        use RangeOrdering::*;
        assert_eq!(cmp_int_range(1, 5, 10), Less);
        assert_eq!(cmp_int_range(4, 5, 10), Less);
        assert_eq!(cmp_int_range(5, 5, 10), EqLower);
        assert_eq!(cmp_int_range(6, 5, 10), Between);
        assert_eq!(cmp_int_range(10, 5, 10), EqUpper);
        assert_eq!(cmp_int_range(11, 5, 10), Greater);
        assert_eq!(cmp_int_range(15, 5, 10), Greater);
    }

    #[test]
    fn cmp_trivial_int_range() {
        use RangeOrdering::*;
        assert_eq!(cmp_int_range(1, 7, 7), Less);
        assert_eq!(cmp_int_range(6, 7, 7), Less);
        assert_eq!(cmp_int_range(7, 7, 7), EqBoth);
        assert_eq!(cmp_int_range(8, 7, 7), Greater);
        assert_eq!(cmp_int_range(10, 7, 7), Greater);
    }

    #[rstest]
    #[case(1999, Month::January, RangeOrdering::Less)]
    #[case(1999, Month::April, RangeOrdering::Less)]
    #[case(1999, Month::June, RangeOrdering::Less)]
    #[case(1999, Month::August, RangeOrdering::Less)]
    #[case(1999, Month::December, RangeOrdering::Less)]
    #[case(2020, Month::January, RangeOrdering::Less)]
    #[case(2020, Month::April, RangeOrdering::Less)]
    #[case(2020, Month::July, RangeOrdering::Less)]
    #[case(2020, Month::August, RangeOrdering::EqLower)]
    #[case(2020, Month::September, RangeOrdering::Between)]
    #[case(2020, Month::December, RangeOrdering::Between)]
    #[case(2022, Month::January, RangeOrdering::Between)]
    #[case(2022, Month::April, RangeOrdering::Between)]
    #[case(2022, Month::June, RangeOrdering::Between)]
    #[case(2022, Month::August, RangeOrdering::Between)]
    #[case(2022, Month::December, RangeOrdering::Between)]
    #[case(2023, Month::March, RangeOrdering::Between)]
    #[case(2023, Month::April, RangeOrdering::EqUpper)]
    #[case(2023, Month::June, RangeOrdering::Greater)]
    #[case(2023, Month::July, RangeOrdering::Greater)]
    #[case(2023, Month::August, RangeOrdering::Greater)]
    #[case(2023, Month::September, RangeOrdering::Greater)]
    #[case(2023, Month::December, RangeOrdering::Greater)]
    #[case(2525, Month::January, RangeOrdering::Greater)]
    #[case(2525, Month::April, RangeOrdering::Greater)]
    #[case(2525, Month::June, RangeOrdering::Greater)]
    #[case(2525, Month::August, RangeOrdering::Greater)]
    #[case(2525, Month::December, RangeOrdering::Greater)]
    fn cmp_multi_year_ym_range(#[case] year: i32, #[case] month: Month, #[case] r: RangeOrdering) {
        assert_eq!(
            cmp_ym_range((year, month), (2020, Month::August), (2023, Month::April)),
            r
        );
    }

    #[rstest]
    #[case(2022, Month::February, RangeOrdering::Less)]
    #[case(2022, Month::April, RangeOrdering::Less)]
    #[case(2022, Month::June, RangeOrdering::Less)]
    #[case(2022, Month::August, RangeOrdering::Less)]
    #[case(2022, Month::December, RangeOrdering::Less)]
    #[case(2023, Month::January, RangeOrdering::Less)]
    #[case(2023, Month::March, RangeOrdering::Less)]
    #[case(2023, Month::April, RangeOrdering::EqLower)]
    #[case(2023, Month::May, RangeOrdering::Between)]
    #[case(2023, Month::July, RangeOrdering::Between)]
    #[case(2023, Month::August, RangeOrdering::EqUpper)]
    #[case(2023, Month::September, RangeOrdering::Greater)]
    #[case(2023, Month::December, RangeOrdering::Greater)]
    #[case(2024, Month::January, RangeOrdering::Greater)]
    #[case(2024, Month::April, RangeOrdering::Greater)]
    #[case(2024, Month::June, RangeOrdering::Greater)]
    #[case(2024, Month::August, RangeOrdering::Greater)]
    #[case(2024, Month::October, RangeOrdering::Greater)]
    fn cmp_single_year_ym_range(#[case] year: i32, #[case] month: Month, #[case] r: RangeOrdering) {
        assert_eq!(
            cmp_ym_range((year, month), (2023, Month::April), (2023, Month::August)),
            r
        );
    }

    #[rstest]
    #[case(2022, Month::January, RangeOrdering::Less)]
    #[case(2022, Month::June, RangeOrdering::Less)]
    #[case(2022, Month::July, RangeOrdering::Less)]
    #[case(2022, Month::August, RangeOrdering::Less)]
    #[case(2022, Month::December, RangeOrdering::Less)]
    #[case(2023, Month::January, RangeOrdering::Less)]
    #[case(2023, Month::June, RangeOrdering::Less)]
    #[case(2023, Month::July, RangeOrdering::EqBoth)]
    #[case(2023, Month::August, RangeOrdering::Greater)]
    #[case(2023, Month::October, RangeOrdering::Greater)]
    #[case(2024, Month::January, RangeOrdering::Greater)]
    #[case(2024, Month::June, RangeOrdering::Greater)]
    #[case(2024, Month::July, RangeOrdering::Greater)]
    #[case(2024, Month::August, RangeOrdering::Greater)]
    #[case(2024, Month::December, RangeOrdering::Greater)]
    fn cmp_trivial_ym_range(#[case] year: i32, #[case] month: Month, #[case] r: RangeOrdering) {
        assert_eq!(
            cmp_ym_range((year, month), (2023, Month::July), (2023, Month::July)),
            r
        );
    }

    #[test]
    fn julian_leap_year() {
        assert!(is_julian_leap_year(-400));
        assert!(is_julian_leap_year(-100));
        assert!(is_julian_leap_year(-4));
        assert!(!is_julian_leap_year(-2));
        assert!(!is_julian_leap_year(-1));
        assert!(is_julian_leap_year(0));
        assert!(!is_julian_leap_year(1));
        assert!(!is_julian_leap_year(2));
        assert!(is_julian_leap_year(4));
        assert!(!is_julian_leap_year(6));
        assert!(is_julian_leap_year(100));
        assert!(is_julian_leap_year(400));
        assert!(is_julian_leap_year(1000));
        assert!(is_julian_leap_year(2000));
    }

    #[test]
    fn gregorian_leap_year() {
        assert!(is_gregorian_leap_year(-400));
        assert!(!is_gregorian_leap_year(-100));
        assert!(is_gregorian_leap_year(-4));
        assert!(!is_gregorian_leap_year(-2));
        assert!(!is_gregorian_leap_year(-1));
        assert!(is_gregorian_leap_year(0));
        assert!(!is_gregorian_leap_year(1));
        assert!(!is_gregorian_leap_year(2));
        assert!(is_gregorian_leap_year(4));
        assert!(!is_gregorian_leap_year(6));
        assert!(!is_gregorian_leap_year(100));
        assert!(is_gregorian_leap_year(400));
        assert!(!is_gregorian_leap_year(1000));
        assert!(is_gregorian_leap_year(2000));
    }

    #[test]
    fn test_scan_half() {
        assert_eq!(scan("123abc", |c| c.is_ascii_digit()), ("123", "abc"));
    }

    #[test]
    fn test_scan_all() {
        assert_eq!(scan("123456", |c| c.is_ascii_digit()), ("123456", ""));
    }

    #[test]
    fn test_scan_none() {
        assert_eq!(scan("abc123", |c| c.is_ascii_digit()), ("", "abc123"));
    }
}
