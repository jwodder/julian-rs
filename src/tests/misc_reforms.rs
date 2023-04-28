use crate::reformations::MIN_REFORM_JDN;
use crate::{inner, Calendar, Month, ReformingError, YearKind};
use assert_matches::assert_matches;

#[test]
fn init_german_reformation() {
    let cal = Calendar::reforming(2342032).unwrap();
    // Use assert_matches! instead of assert_eq! because Calendar's Eq
    // implementation ignores `gap`
    assert_matches!(
        cal.0,
        inner::Calendar::Reforming {
            reformation: 2342032,
            gap: inner::ReformGap {
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
                kind: inner::GapKind::CrossMonth
            }
        }
    );
}

#[test]
fn german_reformation_year() {
    let cal = Calendar::reforming(2342032).unwrap();
    assert_eq!(cal.year_kind(1700), YearKind::ReformCommon);
    assert_eq!(cal.year_length(1700), 355);
    let shape_feb = cal.month_shape(1700, Month::February);
    assert_eq!(
        shape_feb,
        inner::MonthShape::Solid {
            year: 1700,
            month: Month::February,
            range: 1..=18,
            natural_max_day: 28,
        }
    );
    let shape_mar = cal.month_shape(1700, Month::March);
    assert_eq!(
        shape_mar,
        inner::MonthShape::Solid {
            year: 1700,
            month: Month::March,
            range: 1..=31,
            natural_max_day: 31,
        }
    );
}

#[test]
fn max_invalid_reformation() {
    let r = Calendar::reforming(MIN_REFORM_JDN - 1);
    assert_eq!(r, Err(ReformingError::InvalidReformation));
}

#[test]
fn min_valid_reformation() {
    let cal = Calendar::reforming(MIN_REFORM_JDN).unwrap();
    // Use assert_matches! instead of assert_eq! because Calendar's Eq
    // implementation ignores `gap`
    assert_matches!(
        cal.0,
        inner::Calendar::Reforming {
            reformation: 1830692,
            gap: inner::ReformGap {
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
                kind: inner::GapKind::CrossMonth
            }
        }
    );
}

#[test]
fn test_first_skipped_year() {
    let cal = Calendar::reforming(19582149).unwrap();
    // Use assert_matches! instead of assert_eq! because Calendar's Eq
    // implementation ignores `gap`
    assert_matches!(
        cal.0,
        inner::Calendar::Reforming {
            reformation: 19582149,
            gap: inner::ReformGap {
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
                kind: inner::GapKind::MultiYear
            }
        }
    );
    assert_eq!(cal.year_kind(48900), YearKind::Leap);
    assert_eq!(cal.year_kind(48901), YearKind::Skipped);
    assert_eq!(cal.year_kind(48902), YearKind::Common);
}
