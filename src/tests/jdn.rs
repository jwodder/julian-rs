use crate::{Calendar, DateError, Jdnum, Month};
use rstest::rstest;
use rstest_reuse::{apply, template};

#[template]
#[rstest]
#[case(-2147483648, -5884202, Month::March, 16)]
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
#[case(2147483647, 5874898, Month::June, 3)]
fn julian_reform(#[case] days: Jdnum, #[case] year: i32, #[case] month: Month, #[case] day: u32) {}

#[apply(julian_reform)]
fn jdn_to_gregorian_reform(
    #[case] days: Jdnum,
    #[case] year: i32,
    #[case] month: Month,
    #[case] day: u32,
) {
    let date = Calendar::GREGORIAN_REFORM.at_jdn(days);
    assert_eq!(date.year(), year);
    assert_eq!(date.month(), month);
    assert_eq!(date.day(), day);
}

#[apply(julian_reform)]
fn gregorian_reform_to_jdn(
    #[case] days: Jdnum,
    #[case] year: i32,
    #[case] month: Month,
    #[case] day: u32,
) {
    let date = Calendar::GREGORIAN_REFORM.at_ymd(year, month, day).unwrap();
    assert_eq!(date.julian_day_number(), days);
}

#[test]
fn gregorian_ymd_to_pre_min_jdn() {
    let r = Calendar::gregorian().at_ymd(-5884323, Month::May, 14);
    assert_eq!(r, Err(DateError::Arithmetic));
    assert_eq!(r.unwrap_err().to_string(), "arithmetic overflow/underflow");
}

#[test]
fn gregorian_ymd_to_past_max_jdn() {
    let r = Calendar::gregorian().at_ymd(5874898, Month::June, 4);
    assert_eq!(r, Err(DateError::Arithmetic));
    assert_eq!(r.unwrap_err().to_string(), "arithmetic overflow/underflow");
}

#[test]
fn julian_ymd_to_pre_min_jdn() {
    let r = Calendar::julian().at_ymd(-5884202, Month::March, 15);
    assert_eq!(r, Err(DateError::Arithmetic));
    assert_eq!(r.unwrap_err().to_string(), "arithmetic overflow/underflow");
}

#[test]
fn julian_ymd_to_past_max_jdn() {
    let r = Calendar::julian().at_ymd(5874777, Month::October, 18);
    assert_eq!(r, Err(DateError::Arithmetic));
    assert_eq!(r.unwrap_err().to_string(), "arithmetic overflow/underflow");
}
