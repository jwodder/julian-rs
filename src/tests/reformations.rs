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
        assert_eq!(cal.year_kind(1582), YearKind::ReformCommon);
        assert_eq!(cal.year_length(1582), 355);
    }

    #[test]
    fn reformation_month() {
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
        assert_eq!(shape.month(), October);
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
    // Also, a reformation that skips a Julian-only leap day
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
        assert_eq!(cal.year_kind(1700), YearKind::ReformCommon);
        assert_eq!(cal.year_length(1700), 355);
    }

    #[test]
    fn pre_reform_month() {
        let cal = Calendar::reforming(ncal::GERMANY).unwrap();
        let shape = cal.month_shape(1700, Month::February).unwrap();
        assert_eq!(
            shape,
            MonthShape {
                year: 1700,
                month: Month::February,
                inner: inner::MonthShape::Tailless {
                    max_day: 18,
                    natural_max_day: 29,
                },
            }
        );
        assert_eq!(shape.year(), 1700);
        assert_eq!(shape.month(), Month::February);
        assert_eq!(shape.len(), 18);
        assert!(!shape.contains(0));
        assert!(shape.contains(1));
        assert!(shape.contains(18));
        assert!(!shape.contains(19));
        assert_eq!(shape.first_day(), 1);
        assert_eq!(shape.last_day(), 18);
        assert_eq!(shape.day_ordinal(0), None);
        assert_eq!(shape.day_ordinal(1), Some(1));
        assert_eq!(shape.day_ordinal(18), Some(18));
        assert_eq!(shape.day_ordinal(19), None);
        assert_eq!(shape.nth_day(0), None);
        assert_eq!(shape.nth_day(1), Some(1));
        assert_eq!(shape.nth_day(18), Some(18));
        assert_eq!(shape.nth_day(19), None);
        assert_eq!(shape.gap(), Some(19..=29));
        assert_eq!(shape.kind(), MonthKind::Tailless);
        assert_eq!(
            cal.at_ymd(1700, Month::February, 29),
            Err(DateError::SkippedDate {
                year: 1700,
                month: Month::February,
                day: 29
            })
        );
    }

    #[test]
    fn post_reform_month() {
        let cal = Calendar::reforming(ncal::GERMANY).unwrap();
        let shape = cal.month_shape(1700, Month::March).unwrap();
        assert_eq!(
            shape,
            MonthShape {
                year: 1700,
                month: Month::March,
                inner: inner::MonthShape::Normal { max_day: 31 },
            }
        );
        assert_eq!(shape.year(), 1700);
        assert_eq!(shape.month(), Month::March);
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
        assert_eq!(cal.year_length(1918), 352);
    }

    #[test]
    fn pre_reform_month() {
        let cal = Calendar::reforming(ncal::RUSSIA).unwrap();
        let shape = cal.month_shape(1918, Month::January).unwrap();
        assert_eq!(
            shape,
            MonthShape {
                year: 1918,
                month: Month::January,
                inner: inner::MonthShape::Normal { max_day: 31 },
            }
        );
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
        assert_eq!(
            shape,
            MonthShape {
                year: 1918,
                month: Month::February,
                inner: inner::MonthShape::Headless {
                    min_day: 14,
                    max_day: 28,
                },
            }
        );
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

mod prussia {
    // A reformation with a tailless month followed by a headless month
    // Also, reformation year contains a pre-reformation leap day
    use super::*;
    const PRUSSIA: Jdnum = 2310076;

    #[test]
    fn init() {
        let cal = Calendar::reforming(PRUSSIA).unwrap();
        assert_eq!(cal.reformation(), Some(PRUSSIA));
        let gap = cal.gap().unwrap();
        assert_eq!(
            gap,
            inner::ReformGap {
                pre_reform: inner::Date {
                    year: 1612,
                    ordinal: 235,
                    month: Month::August,
                    day: 22
                },
                post_reform: inner::Date {
                    year: 1612,
                    ordinal: 236,
                    month: Month::September,
                    day: 2
                },
                gap_length: 10,
                kind: inner::GapKind::CrossMonth,
                ordinal_gap_start: 245,
                ordinal_gap: 10,
            }
        );
        assert_eq!(cal.year_kind(1612), YearKind::ReformLeap);
        assert_eq!(cal.year_length(1612), 356);
    }

    #[test]
    fn pre_reform_month() {
        let cal = Calendar::reforming(PRUSSIA).unwrap();
        let shape = cal.month_shape(1612, Month::August).unwrap();
        assert_eq!(
            shape,
            MonthShape {
                year: 1612,
                month: Month::August,
                inner: inner::MonthShape::Tailless {
                    max_day: 22,
                    natural_max_day: 31
                },
            }
        );
        assert_eq!(shape.year(), 1612);
        assert_eq!(shape.month(), Month::August);
        assert_eq!(shape.len(), 22);
        assert!(!shape.contains(0));
        assert!(shape.contains(1));
        assert!(shape.contains(22));
        assert!(!shape.contains(23));
        assert_eq!(shape.first_day(), 1);
        assert_eq!(shape.last_day(), 22);
        assert_eq!(shape.day_ordinal(0), None);
        assert_eq!(shape.day_ordinal(1), Some(1));
        assert_eq!(shape.day_ordinal(22), Some(22));
        assert_eq!(shape.day_ordinal(23), None);
        assert_eq!(shape.nth_day(0), None);
        assert_eq!(shape.nth_day(1), Some(1));
        assert_eq!(shape.nth_day(22), Some(22));
        assert_eq!(shape.nth_day(23), None);
        assert_eq!(shape.gap(), Some(23..=31));
        assert_eq!(shape.kind(), MonthKind::Tailless);
    }

    #[test]
    fn post_reform_month() {
        let cal = Calendar::reforming(PRUSSIA).unwrap();
        let shape = cal.month_shape(1612, Month::September).unwrap();
        assert_eq!(
            shape,
            MonthShape {
                year: 1612,
                month: Month::September,
                inner: inner::MonthShape::Headless {
                    min_day: 2,
                    max_day: 30,
                },
            }
        );
        assert_eq!(shape.year(), 1612);
        assert_eq!(shape.month(), Month::September);
        assert_eq!(shape.len(), 29);
        assert!(!shape.contains(0));
        assert!(!shape.contains(1));
        assert!(shape.contains(2));
        assert!(shape.contains(30));
        assert!(!shape.contains(31));
        assert_eq!(shape.first_day(), 2);
        assert_eq!(shape.last_day(), 30);
        assert_eq!(shape.day_ordinal(0), None);
        assert_eq!(shape.day_ordinal(1), None);
        assert_eq!(shape.day_ordinal(2), Some(1));
        assert_eq!(shape.day_ordinal(30), Some(29));
        assert_eq!(shape.day_ordinal(31), None);
        assert_eq!(shape.nth_day(0), None);
        assert_eq!(shape.nth_day(1), Some(2));
        assert_eq!(shape.nth_day(29), Some(30));
        assert_eq!(shape.nth_day(30), None);
        assert_eq!(shape.gap(), Some(1..=1));
        assert_eq!(shape.kind(), MonthKind::Headless);
    }
}

mod china {
    // A cross-year reformation
    // Also, a "tailless year"
    use super::*;

    #[test]
    fn init() {
        let cal = Calendar::reforming(ncal::CHINA).unwrap();
        assert_eq!(cal.reformation(), Some(ncal::CHINA));
        let gap = cal.gap().unwrap();
        assert_eq!(
            gap,
            inner::ReformGap {
                pre_reform: inner::Date {
                    year: 1911,
                    ordinal: 352,
                    month: Month::December,
                    day: 18
                },
                post_reform: inner::Date {
                    year: 1912,
                    ordinal: 1,
                    month: Month::January,
                    day: 1
                },
                gap_length: 13,
                kind: inner::GapKind::CrossYear,
                ordinal_gap_start: 0,
                ordinal_gap: 0,
            }
        );
        assert_eq!(cal.year_kind(1911), YearKind::ReformCommon);
        assert_eq!(cal.year_length(1911), 352);
        assert_eq!(cal.year_kind(1912), YearKind::Leap);
        assert_eq!(cal.year_length(1912), 366);
    }

    #[test]
    fn pre_reform_month() {
        let cal = Calendar::reforming(ncal::CHINA).unwrap();
        let shape = cal.month_shape(1911, Month::December).unwrap();
        assert_eq!(
            shape,
            MonthShape {
                year: 1911,
                month: Month::December,
                inner: inner::MonthShape::Tailless {
                    max_day: 18,
                    natural_max_day: 31
                },
            }
        );
        assert_eq!(shape.year(), 1911);
        assert_eq!(shape.month(), Month::December);
        assert_eq!(shape.len(), 18);
        assert!(!shape.contains(0));
        assert!(shape.contains(1));
        assert!(shape.contains(18));
        assert!(!shape.contains(19));
        assert_eq!(shape.first_day(), 1);
        assert_eq!(shape.last_day(), 18);
        assert_eq!(shape.day_ordinal(0), None);
        assert_eq!(shape.day_ordinal(1), Some(1));
        assert_eq!(shape.day_ordinal(18), Some(18));
        assert_eq!(shape.day_ordinal(19), None);
        assert_eq!(shape.nth_day(0), None);
        assert_eq!(shape.nth_day(1), Some(1));
        assert_eq!(shape.nth_day(18), Some(18));
        assert_eq!(shape.nth_day(19), None);
        assert_eq!(shape.gap(), Some(19..=31));
        assert_eq!(shape.kind(), MonthKind::Tailless);
    }

    #[test]
    fn post_reform_month() {
        let cal = Calendar::reforming(ncal::CHINA).unwrap();
        let shape = cal.month_shape(1912, Month::January).unwrap();
        assert_eq!(
            shape,
            MonthShape {
                year: 1912,
                month: Month::January,
                inner: inner::MonthShape::Normal { max_day: 31 },
            }
        );
        assert_eq!(shape.year(), 1912);
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
}

#[test]
fn iceland() {
    // Reformation year contains a pre-reformation Julian-only leap day
    let cal = Calendar::reforming(ncal::ICELAND).unwrap();
    assert_eq!(cal.reformation(), Some(ncal::ICELAND));
    let gap = cal.gap().unwrap();
    assert_eq!(
        gap,
        inner::ReformGap {
            pre_reform: inner::Date {
                year: 1700,
                ordinal: 321,
                month: Month::November,
                day: 16
            },
            post_reform: inner::Date {
                year: 1700,
                ordinal: 322,
                month: Month::November,
                day: 28
            },
            gap_length: 11,
            kind: inner::GapKind::IntraMonth,
            ordinal_gap_start: 331,
            ordinal_gap: 10,
        }
    );
    assert_eq!(cal.year_kind(1700), YearKind::ReformLeap);
    assert_eq!(cal.year_length(1700), 355);
    assert!(cal.at_ymd(1700, Month::February, 29).is_ok());
}

#[test]
fn czechia() {
    // Reformation year contains a post-reformation leap day
    let cal = Calendar::reforming(ncal::CZECH_REPUBLIC).unwrap();
    assert_eq!(cal.reformation(), Some(ncal::CZECH_REPUBLIC));
    let gap = cal.gap().unwrap();
    assert_eq!(
        gap,
        inner::ReformGap {
            pre_reform: inner::Date {
                year: 1584,
                ordinal: 6,
                month: Month::January,
                day: 6
            },
            post_reform: inner::Date {
                year: 1584,
                ordinal: 7,
                month: Month::January,
                day: 17
            },
            gap_length: 10,
            kind: inner::GapKind::IntraMonth,
            ordinal_gap_start: 16,
            ordinal_gap: 10,
        }
    );
    assert_eq!(cal.year_kind(1584), YearKind::ReformLeap);
    assert_eq!(cal.year_length(1584), 356);
    assert!(cal.at_ymd(1584, Month::February, 29).is_ok());
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
    assert_eq!(
        cal.month_shape(3901, Month::January).unwrap().kind(),
        MonthKind::Normal
    );
    assert_eq!(cal.month_shape(3901, Month::February), None);
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

mod minreform {
    use super::*;

    #[test]
    fn init() {
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
        assert_eq!(cal.year_kind(300), YearKind::ReformCommon);
        assert_eq!(cal.year_length(300), 365);
    }

    #[test]
    fn pre_reform_month() {
        let cal = Calendar::reforming(1830692).unwrap();
        let shape = cal.month_shape(300, Month::February).unwrap();
        assert_eq!(
            shape,
            MonthShape {
                year: 300,
                month: Month::February,
                inner: inner::MonthShape::Tailless {
                    max_day: 28,
                    natural_max_day: 29,
                },
            }
        );
        assert_eq!(shape.year(), 300);
        assert_eq!(shape.month(), Month::February);
        assert_eq!(shape.len(), 28);
        assert!(!shape.contains(0));
        assert!(shape.contains(1));
        assert!(shape.contains(28));
        assert!(!shape.contains(29));
        assert_eq!(shape.first_day(), 1);
        assert_eq!(shape.last_day(), 28);
        assert_eq!(shape.day_ordinal(0), None);
        assert_eq!(shape.day_ordinal(1), Some(1));
        assert_eq!(shape.day_ordinal(28), Some(28));
        assert_eq!(shape.day_ordinal(29), None);
        assert_eq!(shape.nth_day(0), None);
        assert_eq!(shape.nth_day(1), Some(1));
        assert_eq!(shape.nth_day(28), Some(28));
        assert_eq!(shape.nth_day(29), None);
        assert_eq!(shape.gap(), Some(29..=29));
        assert_eq!(shape.kind(), MonthKind::Tailless);
        assert_eq!(
            cal.at_ymd(300, Month::February, 29),
            Err(DateError::SkippedDate {
                year: 300,
                month: Month::February,
                day: 29
            })
        );
    }

    #[test]
    fn post_reform_month() {
        let cal = Calendar::reforming(1830692).unwrap();
        let shape = cal.month_shape(300, Month::March).unwrap();
        assert_eq!(
            shape,
            MonthShape {
                year: 300,
                month: Month::March,
                inner: inner::MonthShape::Normal { max_day: 31 },
            }
        );
        assert_eq!(shape.year(), 300);
        assert_eq!(shape.month(), Month::March);
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
