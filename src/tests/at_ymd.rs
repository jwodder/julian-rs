use crate::{Calendar, Error, Month};

#[test]
fn mday_0() {
    let r = Calendar::gregorian_reform().at_ymd(2023, Month::April, 0);
    assert_eq!(
        r,
        Err(Error::MdayOutOfRange {
            year: 2023,
            month: Month::April,
            mday: 0
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "mday 0 is outside of valid range for April 2023"
    );
}

#[test]
fn mday_32() {
    let r = Calendar::gregorian_reform().at_ymd(2023, Month::April, 32);
    assert_eq!(
        r,
        Err(Error::MdayOutOfRange {
            year: 2023,
            month: Month::April,
            mday: 32
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "mday 32 is outside of valid range for April 2023"
    );
}

#[test]
fn sep_31() {
    let r = Calendar::gregorian_reform().at_ymd(2023, Month::September, 31);
    assert_eq!(
        r,
        Err(Error::MdayOutOfRange {
            year: 2023,
            month: Month::September,
            mday: 31
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "mday 31 is outside of valid range for September 2023"
    );
}

#[test]
fn invalid_leap_day() {
    let r = Calendar::gregorian_reform().at_ymd(2023, Month::February, 29);
    assert_eq!(
        r,
        Err(Error::MdayOutOfRange {
            year: 2023,
            month: Month::February,
            mday: 29
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "mday 29 is outside of valid range for February 2023"
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
        Err(Error::SkippedDate {
            year: 1582,
            month: Month::October,
            mday: 10
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
        Err(Error::SkippedDate {
            year: 1582,
            month: Month::October,
            mday: 5
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
        Err(Error::SkippedDate {
            year: 1582,
            month: Month::October,
            mday: 14
        })
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "date 1582-10-14 was skipped by calendar reform"
    );
}
