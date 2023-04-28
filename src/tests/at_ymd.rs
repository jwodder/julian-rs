use crate::ncal;
use crate::{Calendar, DateError, Month};

#[test]
fn day_0() {
    let r = Calendar::GREGORIAN_REFORM.at_ymd(2023, Month::April, 0);
    assert_eq!(
        r,
        Err(DateError::DayOutOfRange {
            year: 2023,
            month: Month::April,
            day: 0,
            min_day: 1,
            max_day: 30
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "day 0 is outside of valid range 1-30 for 2023 April"
    );
}

#[test]
fn day_32() {
    let r = Calendar::GREGORIAN_REFORM.at_ymd(2023, Month::April, 32);
    assert_eq!(
        r,
        Err(DateError::DayOutOfRange {
            year: 2023,
            month: Month::April,
            day: 32,
            min_day: 1,
            max_day: 30,
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "day 32 is outside of valid range 1-30 for 2023 April"
    );
}

#[test]
fn sep_31() {
    let r = Calendar::GREGORIAN_REFORM.at_ymd(2023, Month::September, 31);
    assert_eq!(
        r,
        Err(DateError::DayOutOfRange {
            year: 2023,
            month: Month::September,
            day: 31,
            min_day: 1,
            max_day: 30,
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "day 31 is outside of valid range 1-30 for 2023 September"
    );
}

#[test]
fn invalid_leap_day() {
    let r = Calendar::GREGORIAN_REFORM.at_ymd(2023, Month::February, 29);
    assert_eq!(
        r,
        Err(DateError::DayOutOfRange {
            year: 2023,
            month: Month::February,
            day: 29,
            min_day: 1,
            max_day: 28,
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "day 29 is outside of valid range 1-28 for 2023 February"
    );
}

#[test]
fn valid_leap_day() {
    let date = Calendar::GREGORIAN_REFORM
        .at_ymd(2024, Month::February, 29)
        .unwrap();
    assert_eq!(date.year(), 2024);
    assert_eq!(date.month(), Month::February);
    assert_eq!(date.day(), 29);
}

#[test]
fn skipped_date() {
    let r = Calendar::GREGORIAN_REFORM.at_ymd(1582, Month::October, 10);
    assert_eq!(
        r,
        Err(DateError::SkippedDate {
            year: 1582,
            month: Month::October,
            day: 10
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "date 1582-10-10 was skipped by calendar reform"
    );
}

#[test]
fn first_skipped_date() {
    let r = Calendar::GREGORIAN_REFORM.at_ymd(1582, Month::October, 5);
    assert_eq!(
        r,
        Err(DateError::SkippedDate {
            year: 1582,
            month: Month::October,
            day: 5
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "date 1582-10-05 was skipped by calendar reform"
    );
}

#[test]
fn last_skipped_date() {
    let r = Calendar::GREGORIAN_REFORM.at_ymd(1582, Month::October, 14);
    assert_eq!(
        r,
        Err(DateError::SkippedDate {
            year: 1582,
            month: Month::October,
            day: 14
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "date 1582-10-14 was skipped by calendar reform"
    );
}

#[test]
fn skipped_month_start() {
    let cal = Calendar::reforming(ncal::RUSSIA).unwrap();
    let r = cal.at_ymd(1918, Month::February, 1);
    assert_eq!(
        r,
        Err(DateError::SkippedDate {
            year: 1918,
            month: Month::February,
            day: 1
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "date 1918-02-01 was skipped by calendar reform"
    );
}

#[test]
fn past_end_of_headless_month() {
    let cal = Calendar::reforming(ncal::RUSSIA).unwrap();
    let r = cal.at_ymd(1918, Month::February, 29);
    assert_eq!(
        r,
        Err(DateError::DayOutOfRange {
            year: 1918,
            month: Month::February,
            day: 29,
            min_day: 14,
            max_day: 28,
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "day 29 is outside of valid range 14-28 for 1918 February"
    );
}

#[test]
fn headless_month_day_zero() {
    let cal = Calendar::reforming(ncal::RUSSIA).unwrap();
    let r = cal.at_ymd(1918, Month::February, 0);
    assert_eq!(
        r,
        Err(DateError::DayOutOfRange {
            year: 1918,
            month: Month::February,
            day: 0,
            min_day: 14,
            max_day: 28,
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "day 0 is outside of valid range 14-28 for 1918 February"
    );
}

#[test]
fn headless_month_day_ordinal_1() {
    let cal = Calendar::reforming(ncal::RUSSIA).unwrap();
    let date = cal.at_ymd(1918, Month::February, 14).unwrap();
    assert_eq!(date.year(), 1918);
    assert_eq!(date.ordinal(), 32);
    assert_eq!(date.month(), Month::February);
    assert_eq!(date.day(), 14);
    assert_eq!(date.day_ordinal(), 1);
}

#[test]
fn skipped_month_end() {
    let cal = Calendar::reforming(ncal::DENMARK).unwrap();
    let r = cal.at_ymd(1700, Month::February, 19);
    assert_eq!(
        r,
        Err(DateError::SkippedDate {
            year: 1700,
            month: Month::February,
            day: 19
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "date 1700-02-19 was skipped by calendar reform"
    );
}

#[test]
fn past_natural_end_of_tailless_month() {
    let cal = Calendar::reforming(ncal::DENMARK).unwrap();
    let r = cal.at_ymd(1700, Month::February, 30);
    assert_eq!(
        r,
        Err(DateError::DayOutOfRange {
            year: 1700,
            month: Month::February,
            day: 30,
            min_day: 1,
            max_day: 18,
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "day 30 is outside of valid range 1-18 for 1700 February"
    );
}
