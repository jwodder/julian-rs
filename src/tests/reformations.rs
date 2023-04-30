use crate::{
    inner, ncal, Calendar, DateError, Jdnum, Month, MonthKind, MonthShape, ReformingError,
    YearKind, GREGORIAN,
};
use rstest::rstest;

mod gregorian_reform {
    // A reformation with a month with a gap
    use super::*;

    #[test]
    fn init() {
        let cal = Calendar::reforming(GREGORIAN).unwrap();
        assert_eq!(cal, Calendar::GREGORIAN_REFORM);
        assert_eq!(cal.reformation(), Calendar::GREGORIAN_REFORM.reformation());
        assert_eq!(cal.reformation(), Some(GREGORIAN));
        let gap_fn = cal.gap().unwrap();
        let gap_const = Calendar::GREGORIAN_REFORM.gap().unwrap();
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
}

mod germany {
    // A reformation with a tailless month
    use super::*;

    #[test]
    fn init() {
        let cal = Calendar::reforming(ncal::GERMANY).unwrap();
        assert_eq!(cal.reformation(), Some(ncal::GERMANY));
        let gap = cal.gap().unwrap();
        assert_eq!(
            gap,
            inner::ReformGap {
                pre_reform: inner::Date {
                    year: 1700,
                    ordinal: 49,
                    month: Month::February,
                    day: 18
                },
                post_reform: inner::Date {
                    year: 1700,
                    ordinal: 50,
                    month: Month::March,
                    day: 1
                },
                gap_length: 11,
                kind: inner::GapKind::CrossMonth,
                ordinal_gap_start: 59,
                ordinal_gap: 10,
            }
        );
    }

    #[test]
    fn reformation_year() {
        let cal = Calendar::reforming(ncal::GERMANY).unwrap();
        assert_eq!(cal.year_kind(1700), YearKind::ReformCommon);
        assert_eq!(cal.year_length(1700), 355);
        let shape_feb = cal.month_shape(1700, Month::February).unwrap();
        assert_eq!(
            shape_feb,
            MonthShape {
                year: 1700,
                month: Month::February,
                inner: inner::MonthShape::Tailless {
                    max_day: 18,
                    natural_max_day: 29
                },
            }
        );
        let shape_mar = cal.month_shape(1700, Month::March).unwrap();
        assert_eq!(
            shape_mar,
            MonthShape {
                year: 1700,
                month: Month::March,
                inner: inner::MonthShape::Normal { max_day: 31 },
            }
        );
    }
}

mod russia {
    // A reformation with a headless month
    use super::*;

    #[test]
    fn init() {
        let cal = Calendar::reforming(ncal::RUSSIA).unwrap();
        assert_eq!(cal.reformation(), Some(ncal::RUSSIA));
        let gap = cal.gap().unwrap();
        assert_eq!(
            gap,
            inner::ReformGap {
                pre_reform: inner::Date {
                    year: 1918,
                    ordinal: 31,
                    month: Month::January,
                    day: 31
                },
                post_reform: inner::Date {
                    year: 1918,
                    ordinal: 32,
                    month: Month::February,
                    day: 14
                },
                gap_length: 13,
                kind: inner::GapKind::CrossMonth,
                ordinal_gap_start: 44,
                ordinal_gap: 13,
            }
        );
        assert_eq!(cal.year_kind(1918), YearKind::ReformCommon);
    }

    #[test]
    fn pre_reform_month() {
        let cal = Calendar::reforming(ncal::RUSSIA).unwrap();
        let shape = cal.month_shape(1918, Month::January).unwrap();
        assert_eq!(shape.year(), 1918);
        assert_eq!(shape.month(), Month::January);
        assert_eq!(shape.len(), 31);
        assert!(!shape.contains(0));
        assert!(shape.contains(1));
        assert!(shape.contains(31));
        assert!(!shape.contains(32));
        assert_eq!(shape.first_day(), 1);
        assert_eq!(shape.last_day(), 31);
        assert_eq!(shape.day_ordinal(0), None);
        assert_eq!(shape.day_ordinal(1), Some(1));
        assert_eq!(shape.day_ordinal(31), Some(31));
        assert_eq!(shape.day_ordinal(32), None);
        assert_eq!(shape.nth_day(0), None);
        assert_eq!(shape.nth_day(1), Some(1));
        assert_eq!(shape.nth_day(31), Some(31));
        assert_eq!(shape.nth_day(32), None);
        assert_eq!(shape.gap(), None);
        assert_eq!(shape.kind(), MonthKind::Normal);
    }

    #[test]
    fn post_reform_month() {
        let cal = Calendar::reforming(ncal::RUSSIA).unwrap();
        let shape = cal.month_shape(1918, Month::February).unwrap();
        assert_eq!(shape.year(), 1918);
        assert_eq!(shape.month(), Month::February);
        assert_eq!(shape.len(), 15);
        assert!(!shape.contains(0));
        assert!(!shape.contains(1));
        assert!(!shape.contains(13));
        assert!(shape.contains(14));
        assert!(shape.contains(28));
        assert!(!shape.contains(29));
        assert_eq!(shape.first_day(), 14);
        assert_eq!(shape.last_day(), 28);
        assert_eq!(shape.day_ordinal(0), None);
        assert_eq!(shape.day_ordinal(1), None);
        assert_eq!(shape.day_ordinal(13), None);
        assert_eq!(shape.day_ordinal(14), Some(1));
        assert_eq!(shape.day_ordinal(28), Some(15));
        assert_eq!(shape.day_ordinal(29), None);
        assert_eq!(shape.nth_day(0), None);
        assert_eq!(shape.nth_day(1), Some(14));
        assert_eq!(shape.nth_day(15), Some(28));
        assert_eq!(shape.nth_day(16), None);
        assert_eq!(shape.gap(), Some(1..=13));
        assert_eq!(shape.kind(), MonthKind::Headless);
    }
}

#[test]
fn skipped_month() {
    let cal = Calendar::reforming(3145930).unwrap();
    assert_eq!(cal.reformation(), Some(3145930));
    let gap = cal.gap().unwrap();
    assert_eq!(
        gap,
        inner::ReformGap {
            pre_reform: inner::Date {
                year: 3901,
                ordinal: 31,
                month: Month::January,
                day: 31
            },
            post_reform: inner::Date {
                year: 3901,
                ordinal: 32,
                month: Month::March,
                day: 1
            },
            gap_length: 28,
            kind: inner::GapKind::CrossMonth,
            ordinal_gap_start: 59,
            ordinal_gap: 28,
        }
    );
    assert_eq!(cal.year_kind(3901), YearKind::ReformCommon);
    assert_eq!(cal.month_shape(3901, Month::February), None);
    assert_eq!(
        cal.month_shape(3901, Month::January).unwrap().kind(),
        MonthKind::Normal
    );
    assert_eq!(
        cal.month_shape(3901, Month::March).unwrap().kind(),
        MonthKind::Normal
    );
}

#[test]
fn skipped_year() {
    let cal = Calendar::reforming(19582149).unwrap();
    assert_eq!(cal.reformation(), Some(19582149));
    let gap = cal.gap().unwrap();
    assert_eq!(
        gap,
        inner::ReformGap {
            pre_reform: inner::Date {
                year: 48900,
                ordinal: 366,
                month: Month::December,
                day: 31
            },
            post_reform: inner::Date {
                year: 48902,
                ordinal: 1,
                month: Month::January,
                day: 1
            },
            gap_length: 365,
            kind: inner::GapKind::MultiYear,
            ordinal_gap_start: 0,
            ordinal_gap: 0,
        }
    );
    assert_eq!(cal.year_kind(48900), YearKind::Leap);
    assert_eq!(cal.year_kind(48901), YearKind::Skipped);
    assert_eq!(cal.year_kind(48902), YearKind::Common);
    assert_eq!(
        cal.month_shape(48900, Month::December).unwrap().kind(),
        MonthKind::Normal
    );
    assert_eq!(cal.month_shape(48901, Month::January), None);
    assert_eq!(cal.month_shape(48901, Month::June), None);
    assert_eq!(cal.month_shape(48901, Month::December), None);
    assert_eq!(
        cal.month_shape(48902, Month::January).unwrap().kind(),
        MonthKind::Normal
    );
}

#[test]
fn jdnum_min_reformation() {
    let r = Calendar::reforming(Jdnum::MIN);
    assert_eq!(r, Err(ReformingError::InvalidReformation));
    assert_eq!(
        r.unwrap_err().to_string(),
        "reformation date would not cause calendar to advance"
    );
}

#[test]
fn pre_min_valid_reformation() {
    let r = Calendar::reforming(1830691);
    assert_eq!(r, Err(ReformingError::InvalidReformation));
    assert_eq!(
        r.unwrap_err().to_string(),
        "reformation date would not cause calendar to advance"
    );
}

#[test]
fn min_valid_reformation() {
    let cal = Calendar::reforming(1830692).unwrap();
    assert_eq!(cal.reformation(), Some(1830692));
    let gap = cal.gap().unwrap();
    assert_eq!(
        gap,
        inner::ReformGap {
            pre_reform: inner::Date {
                year: 300,
                ordinal: 59,
                month: Month::February,
                day: 28
            },
            post_reform: inner::Date {
                year: 300,
                ordinal: 60,
                month: Month::March,
                day: 1
            },
            gap_length: 1,
            kind: inner::GapKind::CrossMonth,
            ordinal_gap_start: 59,
            ordinal_gap: 0,
        }
    );
    assert_eq!(
        cal.month_shape(300, Month::February).unwrap().kind(),
        MonthKind::Tailless
    );
    assert_eq!(
        cal.at_ymd(300, Month::February, 29),
        Err(DateError::SkippedDate {
            year: 300,
            month: Month::February,
            day: 29
        })
    );
    assert_eq!(
        cal.month_shape(300, Month::March).unwrap().kind(),
        MonthKind::Normal
    );
}

#[test]
fn max_reformation() {
    assert!(Calendar::reforming(2147439588).is_ok());
}

#[test]
fn past_max_reformation() {
    let r = Calendar::reforming(2147439589);
    assert_eq!(r, Err(ReformingError::Arithmetic));
    assert_eq!(r.unwrap_err().to_string(), "arithmetic overflow/underflow");
}
