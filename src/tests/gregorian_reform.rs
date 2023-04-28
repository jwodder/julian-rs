use crate::{inner, Calendar, DateError, Month, YearT};
use assert_matches::assert_matches;
use rstest::rstest;

#[test]
fn init_gregorian_reform() {
    let cal = Calendar::gregorian_reform();
    // Use assert_matches! instead of assert_eq! because Calendar's Eq
    // implementation ignores `gap`
    assert_matches!(
        cal.0,
        inner::Calendar::Reforming {
            reformation: 2299161,
            gap: inner::ReformGap {
                pre_reform: inner::Date {
                    year: 1582,
                    ordinal: 277,
                    month: Month::October,
                    day: 4
                },
                post_reform: inner::Date {
                    year: 1582,
                    ordinal: 278,
                    month: Month::October,
                    day: 15
                },
                gap_length: 10,
                kind: inner::GapKind::IntraMonth
            }
        }
    );
}

#[test]
fn reformation_month_shape() {
    use Month::October;
    let cal = Calendar::gregorian_reform();
    let shape = cal.month_shape(1582, October);
    assert_eq!(
        shape,
        inner::MonthShape::HasGap {
            year: 1582,
            month: October,
            gap: 5..15,
            max_day: 31
        }
    );
    assert_eq!(shape.len(), 21);
    //assert!(!shape.has_day(0));
    //assert!(shape.has_day(1));
    //assert!(shape.has_day(4));
    //assert!(!shape.has_day(5));
    //assert!(!shape.has_day(14));
    //assert!(shape.has_day(15));
    //assert!(shape.has_day(31));
    //assert!(!shape.has_day(32));
    assert_eq!(
        shape.get_day_ordinal(0),
        Err(DateError::DayOutOfRange {
            year: 1582,
            month: October,
            day: 0,
            min_day: 1,
            max_day: 31,
        })
    );
    assert_eq!(shape.get_day_ordinal(1), Ok(1));
    assert_eq!(shape.get_day_ordinal(4), Ok(4));
    assert_eq!(
        shape.get_day_ordinal(5),
        Err(DateError::SkippedDate {
            year: 1582,
            month: October,
            day: 5
        })
    );
    assert_eq!(
        shape.get_day_ordinal(14),
        Err(DateError::SkippedDate {
            year: 1582,
            month: October,
            day: 14
        })
    );
    assert_eq!(shape.get_day_ordinal(15), Ok(5));
    assert_eq!(shape.get_day_ordinal(31), Ok(21));
    assert_eq!(
        shape.get_day_ordinal(32),
        Err(DateError::DayOutOfRange {
            year: 1582,
            month: October,
            day: 32,
            min_day: 1,
            max_day: 31,
        })
    );
    assert_eq!(shape.get_day_label(1), Some(1));
    assert_eq!(shape.get_day_label(2), Some(2));
    assert_eq!(shape.get_day_label(4), Some(4));
    assert_eq!(shape.get_day_label(5), Some(15));
    assert_eq!(shape.get_day_label(21), Some(31));
    assert_eq!(shape.get_day_label(22), None);
}

#[rstest]
#[case(2023, Month::January, 31)]
#[case(2023, Month::February, 28)]
#[case(2023, Month::March, 31)]
#[case(2023, Month::April, 30)]
#[case(2023, Month::May, 31)]
#[case(2023, Month::June, 30)]
#[case(2023, Month::July, 31)]
#[case(2023, Month::August, 31)]
#[case(2023, Month::September, 30)]
#[case(2023, Month::October, 31)]
#[case(2023, Month::November, 30)]
#[case(2023, Month::December, 31)]
#[case(2024, Month::January, 31)]
#[case(2024, Month::February, 29)]
#[case(2024, Month::March, 31)]
#[case(2024, Month::April, 30)]
#[case(2024, Month::May, 31)]
#[case(2024, Month::June, 30)]
#[case(2024, Month::July, 31)]
#[case(2024, Month::August, 31)]
#[case(2024, Month::September, 30)]
#[case(2024, Month::October, 31)]
#[case(2024, Month::November, 30)]
#[case(2024, Month::December, 31)]
#[case(1582, Month::January, 31)]
#[case(1582, Month::February, 28)]
#[case(1582, Month::March, 31)]
#[case(1582, Month::April, 30)]
#[case(1582, Month::May, 31)]
#[case(1582, Month::June, 30)]
#[case(1582, Month::July, 31)]
#[case(1582, Month::August, 31)]
#[case(1582, Month::September, 30)]
#[case(1582, Month::October, 21)]
#[case(1582, Month::November, 30)]
#[case(1582, Month::December, 31)]
fn month_length(#[case] year: YearT, #[case] month: Month, #[case] length: u32) {
    let cal = Calendar::gregorian_reform();
    assert_eq!(cal.month_length(year, month), length);
}
