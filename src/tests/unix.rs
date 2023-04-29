use crate::{jdn2unix, unix2jdn, ArithmeticError, Calendar, Jdnum, Month};
use rstest::rstest;

#[rstest]
#[case(-185753453990400, -5884202, Month::March, 16, 0)]
#[case(-2147483648, 1901, Month::December, 13, 74752)]
#[case(-1234567890, 1930, Month::November, 18, 1710)]
#[case(-1000000000, 1938, Month::April, 24, 80000)]
#[case(-100000000, 1966, Month::October, 31, 51200)]
#[case(-1, 1969, Month::December, 31, 86399)]
#[case(0, 1970, Month::January, 1, 0)]
#[case(1, 1970, Month::January, 1, 1)]
#[case(100000000, 1973, Month::March, 3, 35200)]
#[case(1000000000, 2001, Month::September, 9, 6400)]
#[case(1234567890, 2009, Month::February, 13, 84690)]
#[case(1682028168, 2023, Month::April, 20, 79368)]
#[case(2147483647, 2038, Month::January, 19, 11647)]
#[case(185331720383999, 5874898, Month::June, 3, 86399)]
fn at_unix_time(
    #[case] ts: i64,
    #[case] year: i32,
    #[case] month: Month,
    #[case] day: u32,
    #[case] seconds: u32,
) {
    let (date, s) = Calendar::GREGORIAN_REFORM.at_unix_time(ts).unwrap();
    assert_eq!(date.year(), year);
    assert_eq!(date.month(), month);
    assert_eq!(date.day(), day);
    assert_eq!(s, seconds);
}

#[test]
fn cal_pre_min_unix_time() {
    let r = Calendar::GREGORIAN_REFORM.at_unix_time(-185753453990401);
    assert_eq!(r, Err(ArithmeticError));
    assert_eq!(r.unwrap_err().to_string(), "arithmetic overflow/underflow");
}

#[test]
fn cal_past_max_unix_time() {
    let r = Calendar::GREGORIAN_REFORM.at_unix_time(185331720384000);
    assert_eq!(r, Err(ArithmeticError));
    assert_eq!(r.unwrap_err().to_string(), "arithmetic overflow/underflow");
}

#[rstest]
#[case(-185753453990400, -2147483648, 0)]
#[case(0, 2440588, 0)]
#[case(185331720383999, 2147483647, 86399)]
fn jdn_at_unix_time(#[case] ts: i64, #[case] jdn: Jdnum, #[case] seconds: u32) {
    let (j, s) = unix2jdn(ts).unwrap();
    assert_eq!(j, jdn);
    assert_eq!(s, seconds);
}

#[test]
fn jdn_pre_min_unix_time() {
    let r = unix2jdn(-185753453990401);
    assert_eq!(r, Err(ArithmeticError));
    assert_eq!(r.unwrap_err().to_string(), "arithmetic overflow/underflow");
}

#[test]
fn jdn_past_max_unix_time() {
    let r = unix2jdn(185331720384000);
    assert_eq!(r, Err(ArithmeticError));
    assert_eq!(r.unwrap_err().to_string(), "arithmetic overflow/underflow");
}

#[rstest]
#[case(-2147483648, -185753453990400)]
#[case(2440588, 0)]
#[case(2147483647, 185331720297600)]
fn jdn_to_unix_time(#[case] jdn: Jdnum, #[case] ts: i64) {
    let t = jdn2unix(jdn);
    assert_eq!(t, ts);
}
