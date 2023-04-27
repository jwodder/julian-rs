use crate::{Calendar, DateError, DaysT, Month, ParseDateError, YearT};
use assert_matches::assert_matches;
use rstest::rstest;

#[test]
fn ymd() {
    let date = Calendar::gregorian_reform()
        .parse_date("2023-04-20")
        .unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 20);
    assert_eq!(date.ordinal(), 110);
}

#[test]
fn ordinal_date() {
    let date = Calendar::gregorian_reform().parse_date("2023-110").unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 20);
    assert_eq!(date.ordinal(), 110);
}

#[test]
fn ordinal_date_padded() {
    let date = Calendar::gregorian_reform().parse_date("2023-006").unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::January);
    assert_eq!(date.day(), 6);
    assert_eq!(date.ordinal(), 6);
}

#[test]
fn negative_ymd() {
    let date = Calendar::gregorian_reform()
        .parse_date("-2023-04-20")
        .unwrap();
    assert_eq!(date.year(), -2023);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 20);
    assert_eq!(date.ordinal(), 110);
}

#[test]
fn plus_ymd() {
    let date = Calendar::gregorian_reform()
        .parse_date("+2023-04-20")
        .unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 20);
    assert_eq!(date.ordinal(), 110);
}

#[test]
fn ymd_short_year() {
    let date = Calendar::gregorian_reform().parse_date("20-04-20").unwrap();
    assert_eq!(date.year(), 20);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 20);
    assert_eq!(date.ordinal(), 111);
}

#[test]
fn short_ordinal() {
    let date = Calendar::gregorian_reform().parse_date("1234-56").unwrap();
    assert_eq!(date.year(), 1234);
    assert_eq!(date.ordinal(), 56);
}

#[test]
fn long_ordinal() {
    let date = Calendar::gregorian_reform()
        .parse_date("1234-0078")
        .unwrap();
    assert_eq!(date.year(), 1234);
    assert_eq!(date.ordinal(), 78);
}

#[test]
fn big_ordinal() {
    let r = Calendar::gregorian_reform().parse_date("1234-5678");
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::OrdinalOutOfRange {
            year: 1234,
            ordinal: 5678
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "invalid calendar date: day-of-year ordinal 5678 is outside of valid range for year 1234",
    );
}

#[test]
fn ymd_short_month() {
    let date = Calendar::gregorian_reform()
        .parse_date("2023-4-20")
        .unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 20);
}

#[test]
fn ymd_long_month() {
    let date = Calendar::gregorian_reform()
        .parse_date("2023-012-20")
        .unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::December);
    assert_eq!(date.day(), 20);
}

#[test]
fn ymd_short_day() {
    let date = Calendar::gregorian_reform()
        .parse_date("2023-04-2")
        .unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 2);
}

#[test]
fn year_hyphen() {
    let r = Calendar::gregorian_reform().parse_date("2023-");
    assert_eq!(r, Err(ParseDateError::EmptyInt));
    assert_eq!(
        r.unwrap_err().to_string(),
        "expected one or more digits, reached end of input",
    );
}

#[test]
fn year_month_hyphen() {
    let r = Calendar::gregorian_reform().parse_date("2023-04-");
    assert_eq!(r, Err(ParseDateError::EmptyInt));
    assert_eq!(
        r.unwrap_err().to_string(),
        "expected one or more digits, reached end of input"
    );
}

#[test]
fn year_hyphen_hyphen_md() {
    let r = Calendar::gregorian_reform().parse_date("2023--04-20");
    assert_eq!(r, Err(ParseDateError::InvalidIntStart { got: '-' }));
    assert_eq!(
        r.unwrap_err().to_string(),
        "expected one or more digits, got non-digit '-'"
    );
}

#[test]
fn zero_month() {
    let r = Calendar::gregorian_reform().parse_date("2023-00-13");
    assert_eq!(r, Err(ParseDateError::InvalidMonth { value: 0 }));
    assert_eq!(r.unwrap_err().to_string(), "invalid month number: 0");
}

#[test]
fn smarch() {
    let r = Calendar::gregorian_reform().parse_date("2023-13-13");
    assert_eq!(r, Err(ParseDateError::InvalidMonth { value: 13 }));
    assert_eq!(r.unwrap_err().to_string(), "invalid month number: 13");
}

#[test]
fn day_0() {
    let r = Calendar::gregorian_reform().parse_date("2023-04-00");
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::DayOutOfRange {
            year: 2023,
            month: Month::April,
            day: 0
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "invalid calendar date: day 0 is outside of valid range for April 2023"
    );
}

#[test]
fn day_32() {
    let r = Calendar::gregorian_reform().parse_date("2023-04-32");
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::DayOutOfRange {
            year: 2023,
            month: Month::April,
            day: 32
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "invalid calendar date: day 32 is outside of valid range for April 2023"
    );
}

#[test]
fn sep_31() {
    let r = Calendar::gregorian_reform().parse_date("2023-09-31");
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::DayOutOfRange {
            year: 2023,
            month: Month::September,
            day: 31
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "invalid calendar date: day 31 is outside of valid range for September 2023"
    );
}

#[test]
fn invalid_leap_day() {
    let r = Calendar::gregorian_reform().parse_date("2023-02-29");
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::DayOutOfRange {
            year: 2023,
            month: Month::February,
            day: 29
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "invalid calendar date: day 29 is outside of valid range for February 2023"
    );
}

#[test]
fn valid_leap_day() {
    let date = Calendar::gregorian_reform()
        .parse_date("2024-02-29")
        .unwrap();
    assert_eq!(date.year(), 2024);
    assert_eq!(date.month(), Month::February);
    assert_eq!(date.day(), 29);
}

#[test]
fn skipped_date() {
    let r = Calendar::gregorian_reform().parse_date("1582-10-10");
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::SkippedDate {
            year: 1582,
            month: Month::October,
            day: 10
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "invalid calendar date: date 1582-10-10 was skipped by calendar reform"
    );
}

#[test]
fn first_skipped_date() {
    let r = Calendar::gregorian_reform().parse_date("1582-10-05");
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::SkippedDate {
            year: 1582,
            month: Month::October,
            day: 5
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "invalid calendar date: date 1582-10-05 was skipped by calendar reform"
    );
}

#[test]
fn last_skipped_date() {
    let r = Calendar::gregorian_reform().parse_date("1582-10-14");
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::SkippedDate {
            year: 1582,
            month: Month::October,
            day: 14
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "invalid calendar date: date 1582-10-14 was skipped by calendar reform"
    );
}

#[rstest]
#[case(2023, 366)]
#[case(2023, 1000)]
#[case(2024, 367)]
#[case(2024, 1000)]
#[case(1582, 356)]
#[case(1582, 1000)]
fn invalid_ordinal_date(#[case] year: YearT, #[case] ordinal: DaysT) {
    let r = Calendar::gregorian_reform().parse_date(&format!("{year:04}-{ordinal:03}"));
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::OrdinalOutOfRange {
            year,
            ordinal
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        format!("invalid calendar date: day-of-year ordinal {ordinal} is outside of valid range for year {year}")
    );
}

#[test]
fn bad_month_day_sep() {
    let r = Calendar::gregorian_reform().parse_date("2023-04:20");
    assert_eq!(
        r,
        Err(ParseDateError::UnexpectedChar {
            expected: '-',
            got: ':'
        })
    );
    assert_eq!(r.unwrap_err().to_string(), "expected '-', got ':'");
}

#[test]
fn just_year() {
    let r = Calendar::gregorian_reform().parse_date("2023");
    assert_eq!(r, Err(ParseDateError::UnterminatedYear));
    assert_eq!(r.unwrap_err().to_string(), "year not terminated by '-'");
}

#[test]
fn nonint_year() {
    use std::num::IntErrorKind::InvalidDigit;
    let r = Calendar::gregorian_reform().parse_date("202e-04-20");
    assert_matches!(r, Err(ParseDateError::ParseInt(ref e)) if e.kind() == &InvalidDigit);
    assert!(r
        .unwrap_err()
        .to_string()
        .starts_with("invalid calendar date: numeric parse error: "));
}

#[test]
fn empty() {
    let r = Calendar::gregorian_reform().parse_date("");
    assert_eq!(r, Err(ParseDateError::UnterminatedYear));
    assert_eq!(r.unwrap_err().to_string(), "year not terminated by '-'");
}
