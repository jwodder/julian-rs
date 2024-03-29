use crate::{
    errors::{DateError, ParseDateError},
    Calendar, Month,
};
use assert_matches::assert_matches;
use rstest::rstest;

#[test]
fn ymd() {
    let date = Calendar::REFORM1582.parse_date("2023-04-20").unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 20);
    assert_eq!(date.ordinal(), 110);
}

#[test]
fn ordinal_date() {
    let date = Calendar::REFORM1582.parse_date("2023-110").unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 20);
    assert_eq!(date.ordinal(), 110);
}

#[test]
fn ordinal_date_padded() {
    let date = Calendar::REFORM1582.parse_date("2023-006").unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::January);
    assert_eq!(date.day(), 6);
    assert_eq!(date.ordinal(), 6);
}

#[test]
fn negative_ymd() {
    let date = Calendar::REFORM1582.parse_date("-2023-04-20").unwrap();
    assert_eq!(date.year(), -2023);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 20);
    assert_eq!(date.ordinal(), 110);
}

#[test]
fn plus_ymd() {
    let date = Calendar::REFORM1582.parse_date("+2023-04-20").unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 20);
    assert_eq!(date.ordinal(), 110);
}

#[test]
fn ymd_short_year() {
    let date = Calendar::REFORM1582.parse_date("20-04-20").unwrap();
    assert_eq!(date.year(), 20);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 20);
    assert_eq!(date.ordinal(), 111);
}

#[test]
fn short_ordinal() {
    let date = Calendar::REFORM1582.parse_date("1234-56").unwrap();
    assert_eq!(date.year(), 1234);
    assert_eq!(date.ordinal(), 56);
}

#[test]
fn long_ordinal() {
    let date = Calendar::REFORM1582.parse_date("1234-0078").unwrap();
    assert_eq!(date.year(), 1234);
    assert_eq!(date.ordinal(), 78);
}

#[test]
fn big_ordinal() {
    let r = Calendar::REFORM1582.parse_date("1234-5678");
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::OrdinalOutOfRange {
            year: 1234,
            ordinal: 5678,
            max_ordinal: 365,
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "invalid calendar date: day-of-year ordinal 5678 is outside of valid range 1-365 for year 1234",
    );
}

#[test]
fn ymd_short_month() {
    let date = Calendar::REFORM1582.parse_date("2023-4-20").unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 20);
}

#[test]
fn ymd_long_month() {
    let date = Calendar::REFORM1582.parse_date("2023-012-20").unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::December);
    assert_eq!(date.day(), 20);
}

#[test]
fn ymd_short_day() {
    let date = Calendar::REFORM1582.parse_date("2023-04-2").unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::April);
    assert_eq!(date.day(), 2);
}

#[test]
fn year_hyphen() {
    let r = Calendar::REFORM1582.parse_date("2023-");
    assert_eq!(r, Err(ParseDateError::EmptyInt));
    assert_eq!(
        r.unwrap_err().to_string(),
        "expected integer, got end of input",
    );
}

#[test]
fn year_month_hyphen() {
    let r = Calendar::REFORM1582.parse_date("2023-04-");
    assert_eq!(r, Err(ParseDateError::EmptyInt));
    assert_eq!(
        r.unwrap_err().to_string(),
        "expected integer, got end of input"
    );
}

#[test]
fn year_hyphen_hyphen_md() {
    let r = Calendar::REFORM1582.parse_date("2023--04-20");
    assert_eq!(r, Err(ParseDateError::InvalidUIntStart { got: '-' }));
    assert_eq!(
        r.unwrap_err().to_string(),
        "expected unsigned integer, got '-'"
    );
}

#[test]
fn zero_month() {
    let r = Calendar::REFORM1582.parse_date("2023-00-13");
    assert_eq!(r, Err(ParseDateError::InvalidMonth { value: 0 }));
    assert_eq!(r.unwrap_err().to_string(), "invalid month number: 0");
}

#[test]
fn smarch() {
    let r = Calendar::REFORM1582.parse_date("2023-13-13");
    assert_eq!(r, Err(ParseDateError::InvalidMonth { value: 13 }));
    assert_eq!(r.unwrap_err().to_string(), "invalid month number: 13");
}

#[test]
fn day_0() {
    let r = Calendar::REFORM1582.parse_date("2023-04-00");
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::DayOutOfRange {
            year: 2023,
            month: Month::April,
            day: 0,
            min_day: 1,
            max_day: 30,
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "invalid calendar date: day 0 is outside of valid range 1-30 for 2023 April"
    );
}

#[test]
fn day_32() {
    let r = Calendar::REFORM1582.parse_date("2023-04-32");
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::DayOutOfRange {
            year: 2023,
            month: Month::April,
            day: 32,
            min_day: 1,
            max_day: 30
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "invalid calendar date: day 32 is outside of valid range 1-30 for 2023 April"
    );
}

#[test]
fn sep_31() {
    let r = Calendar::REFORM1582.parse_date("2023-09-31");
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::DayOutOfRange {
            year: 2023,
            month: Month::September,
            day: 31,
            min_day: 1,
            max_day: 30,
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "invalid calendar date: day 31 is outside of valid range 1-30 for 2023 September"
    );
}

#[test]
fn invalid_leap_day() {
    let r = Calendar::REFORM1582.parse_date("2023-02-29");
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::DayOutOfRange {
            year: 2023,
            month: Month::February,
            day: 29,
            min_day: 1,
            max_day: 28,
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        "invalid calendar date: day 29 is outside of valid range 1-28 for 2023 February"
    );
}

#[test]
fn valid_leap_day() {
    let date = Calendar::REFORM1582.parse_date("2024-02-29").unwrap();
    assert_eq!(date.year(), 2024);
    assert_eq!(date.month(), Month::February);
    assert_eq!(date.day(), 29);
}

#[test]
fn skipped_date() {
    let r = Calendar::REFORM1582.parse_date("1582-10-10");
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
    let r = Calendar::REFORM1582.parse_date("1582-10-05");
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
    let r = Calendar::REFORM1582.parse_date("1582-10-14");
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
#[case(2023, 0, 365)]
#[case(2023, 366, 365)]
#[case(2023, 1000, 365)]
#[case(2024, 0, 366)]
#[case(2024, 367, 366)]
#[case(2024, 1000, 366)]
#[case(1582, 0, 355)]
#[case(1582, 356, 355)]
#[case(1582, 1000, 355)]
fn invalid_ordinal_date(#[case] year: i32, #[case] ordinal: u32, #[case] max_ordinal: u32) {
    let r = Calendar::REFORM1582.parse_date(&format!("{year:04}-{ordinal:03}"));
    assert_eq!(
        r,
        Err(ParseDateError::InvalidDate(DateError::OrdinalOutOfRange {
            year,
            ordinal,
            max_ordinal,
        }))
    );
    assert_eq!(
        r.unwrap_err().to_string(),
        format!("invalid calendar date: day-of-year ordinal {ordinal} is outside of valid range 1-{max_ordinal} for year {year}")
    );
}

#[test]
fn bad_month_day_sep() {
    let r = Calendar::REFORM1582.parse_date("2023-04:20");
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
fn bad_year_month_sep() {
    let r = Calendar::REFORM1582.parse_date("2023:04-20");
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
fn bad_year_ordinal_sep() {
    let r = Calendar::REFORM1582.parse_date("2023:110");
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
    let r = Calendar::REFORM1582.parse_date("2023");
    assert_eq!(r, Err(ParseDateError::UnexpectedEnd { expected: '-' }));
    assert_eq!(r.unwrap_err().to_string(), "expected '-', got end of input");
}

#[test]
fn nonint_year() {
    let r = Calendar::REFORM1582.parse_date("202e-04-20");
    assert_eq!(
        r,
        Err(ParseDateError::UnexpectedChar {
            expected: '-',
            got: 'e'
        })
    );
    assert_eq!(r.unwrap_err().to_string(), "expected '-', got 'e'");
}

#[test]
fn colon_ymd() {
    let r = Calendar::REFORM1582.parse_date(":2023-04-20");
    assert_eq!(r, Err(ParseDateError::InvalidIntStart { got: ':' }));
    assert_eq!(
        r.unwrap_err().to_string(),
        "expected signed integer, got ':'"
    );
}

#[test]
fn empty() {
    let r = Calendar::REFORM1582.parse_date("");
    assert_eq!(r, Err(ParseDateError::EmptyInt));
    assert_eq!(
        r.unwrap_err().to_string(),
        "expected integer, got end of input"
    );
}

#[test]
fn hyphen_hyphen_start() {
    use std::num::IntErrorKind::InvalidDigit;
    let r = Calendar::REFORM1582.parse_date("--12-23-56");
    assert_matches!(r, Err(ParseDateError::ParseInt(ref e)) => {
        assert_eq!(e.kind(), &InvalidDigit);
    });
}

#[test]
fn too_large_year() {
    use std::num::IntErrorKind::PosOverflow;
    let r = Calendar::REFORM1582.parse_date("999999999999-01-01");
    assert_matches!(r, Err(ParseDateError::ParseInt(ref e)) => {
        assert_eq!(e.kind(), &PosOverflow);
    });
}

#[test]
fn too_small_year() {
    use std::num::IntErrorKind::NegOverflow;
    let r = Calendar::REFORM1582.parse_date("-999999999999-01-01");
    assert_matches!(r, Err(ParseDateError::ParseInt(ref e)) => {
        assert_eq!(e.kind(), &NegOverflow);
    });
}

#[test]
fn too_large_month() {
    use std::num::IntErrorKind::PosOverflow;
    let r = Calendar::REFORM1582.parse_date("2023-999999999999-01");
    assert_matches!(r, Err(ParseDateError::ParseInt(ref e)) => {
        assert_eq!(e.kind(), &PosOverflow);
    });
}

#[test]
fn too_large_day() {
    use std::num::IntErrorKind::PosOverflow;
    let r = Calendar::REFORM1582.parse_date("2023-04-999999999999");
    assert_matches!(r, Err(ParseDateError::ParseInt(ref e)) => {
        assert_eq!(e.kind(), &PosOverflow);
    });
}

#[test]
fn too_large_ordinal() {
    use std::num::IntErrorKind::PosOverflow;
    let r = Calendar::REFORM1582.parse_date("2023-999999999999");
    assert_matches!(r, Err(ParseDateError::ParseInt(ref e)) => {
        assert_eq!(e.kind(), &PosOverflow);
    });
}

#[test]
fn trailing() {
    let r = Calendar::REFORM1582.parse_date("2023-04-20.");
    assert_eq!(r, Err(ParseDateError::Trailing));
    assert_eq!(r.unwrap_err().to_string(), "trailing characters after date");
}
