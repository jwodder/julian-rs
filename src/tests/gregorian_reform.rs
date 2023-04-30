use crate::{inner, Calendar, DateError, Month, MonthKind, MonthShape, GREGORIAN};
use rstest::rstest;

#[test]
fn gregorian_reform() {
    let cal = Calendar::reforming(GREGORIAN).unwrap();
    assert_eq!(cal, Calendar::GREGORIAN_REFORM);
    let Calendar(inner::Calendar::Reforming {gap: gap_fn, ..}) = cal else {
        panic!("Reforming calendar is not reforming!")
    };
    let Calendar(inner::Calendar::Reforming {gap: gap_const, ..}) = Calendar::GREGORIAN_REFORM else {
        panic!("GREGORIAN_REFORM is not reforming!")
    };
    assert_eq!(gap_fn, gap_const);
    assert_eq!(
        gap_fn,
        inner::ReformGap {
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
            kind: inner::GapKind::IntraMonth,
            ordinal_gap_start: 287,
            ordinal_gap: 10,
        }
    );
}

#[test]
fn reformation_month_shape() {
    use Month::October;
    let cal = Calendar::GREGORIAN_REFORM;
    let shape = cal.month_shape(1582, October).unwrap();
    assert_eq!(
        shape,
        MonthShape {
            year: 1582,
            month: October,
            inner: inner::MonthShape::Gapped {
                gap_start: 5,
                gap_end: 14,
                max_day: 31,
            },
        }
    );
    assert_eq!(shape.year(), 1582);
    assert_eq!(shape.month(), Month::October);
    assert_eq!(shape.len(), 21);
    assert!(!shape.contains(0));
    assert!(shape.contains(1));
    assert!(shape.contains(4));
    assert!(!shape.contains(5));
    assert!(!shape.contains(14));
    assert!(shape.contains(15));
    assert!(shape.contains(31));
    assert!(!shape.contains(32));
    assert_eq!(shape.first_day(), 1);
    assert_eq!(shape.last_day(), 31);
    assert_eq!(
        shape.day_ordinal_err(0),
        Err(DateError::DayOutOfRange {
            year: 1582,
            month: October,
            day: 0,
            min_day: 1,
            max_day: 31,
        })
    );
    assert_eq!(shape.day_ordinal(1), Some(1));
    assert_eq!(shape.day_ordinal(4), Some(4));
    assert_eq!(
        shape.day_ordinal_err(5),
        Err(DateError::SkippedDate {
            year: 1582,
            month: October,
            day: 5
        })
    );
    assert_eq!(
        shape.day_ordinal_err(14),
        Err(DateError::SkippedDate {
            year: 1582,
            month: October,
            day: 14
        })
    );
    assert_eq!(shape.day_ordinal(15), Some(5));
    assert_eq!(shape.day_ordinal(31), Some(21));
    assert_eq!(
        shape.day_ordinal_err(32),
        Err(DateError::DayOutOfRange {
            year: 1582,
            month: October,
            day: 32,
            min_day: 1,
            max_day: 31,
        })
    );
    assert_eq!(shape.nth_day(0), None);
    assert_eq!(shape.nth_day(1), Some(1));
    assert_eq!(shape.nth_day(2), Some(2));
    assert_eq!(shape.nth_day(4), Some(4));
    assert_eq!(shape.nth_day(5), Some(15));
    assert_eq!(shape.nth_day(21), Some(31));
    assert_eq!(shape.nth_day(22), None);
    assert_eq!(shape.gap(), Some(5..=14));
    assert_eq!(shape.kind(), MonthKind::Gapped);
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
fn month_length(#[case] year: i32, #[case] month: Month, #[case] length: u32) {
    let cal = Calendar::GREGORIAN_REFORM;
    assert_eq!(cal.month_shape(year, month).unwrap().len(), length);
}
