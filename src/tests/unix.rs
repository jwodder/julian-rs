use crate::{Calendar, Month, YearT};
use rstest::rstest;

#[rstest]
#[case(-2147483647, 1901, Month::December, 13, 74753)]
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
fn at_unix_time(
    #[case] ts: i64,
    #[case] year: YearT,
    #[case] month: Month,
    #[case] day: u32,
    #[case] seconds: u32,
) {
    let (date, s) = Calendar::gregorian_reform().at_unix_time(ts).unwrap();
    assert_eq!(date.year(), year);
    assert_eq!(date.month(), month);
    assert_eq!(date.day(), day);
    assert_eq!(s, seconds);
}
