use crate::{Calendar, DateError, Month};

#[test]
fn day_0() {
    let r = Calendar::gregorian_reform().at_ymd(2023, Month::April, 0);
    assert_eq!(
        r,
        Err(DateError::DayOutOfRange {
            year: 2023,
            month: Month::April,
            day: 0,
            max_day: 30
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "day 0 is outside of valid range 1-30 for April 2023"
    );
}

#[test]
fn day_32() {
    let r = Calendar::gregorian_reform().at_ymd(2023, Month::April, 32);
    assert_eq!(
        r,
        Err(DateError::DayOutOfRange {
            year: 2023,
            month: Month::April,
            day: 32,
            max_day: 30,
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "day 32 is outside of valid range 1-30 for April 2023"
    );
}

#[test]
fn sep_31() {
    let r = Calendar::gregorian_reform().at_ymd(2023, Month::September, 31);
    assert_eq!(
        r,
        Err(DateError::DayOutOfRange {
            year: 2023,
            month: Month::September,
            day: 31,
            max_day: 30,
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "day 31 is outside of valid range 1-30 for September 2023"
    );
}

#[test]
fn invalid_leap_day() {
    let r = Calendar::gregorian_reform().at_ymd(2023, Month::February, 29);
    assert_eq!(
        r,
        Err(DateError::DayOutOfRange {
            year: 2023,
            month: Month::February,
            day: 29,
            max_day: 28,
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "day 29 is outside of valid range 1-28 for February 2023"
    );
}

#[test]
fn valid_leap_day() {
    let date = Calendar::gregorian_reform()
        .at_ymd(2024, Month::February, 29)
        .unwrap();
    assert_eq!(date.year(), 2024);
    assert_eq!(date.month(), Month::February);
    assert_eq!(date.day(), 29);
}

#[test]
fn skipped_date() {
    let r = Calendar::gregorian_reform().at_ymd(1582, Month::October, 10);
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
    let r = Calendar::gregorian_reform().at_ymd(1582, Month::October, 5);
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
    let r = Calendar::gregorian_reform().at_ymd(1582, Month::October, 14);
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
