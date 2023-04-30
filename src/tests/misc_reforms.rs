use crate::{inner, Calendar, Month, MonthShape, ReformingError, YearKind};
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
                kind: inner::GapKind::CrossMonth,
                ordinal_gap_start: 59,
                ordinal_gap: 10,
            }
        }
    );
}

#[test]
fn german_reformation_year() {
    let cal = Calendar::reforming(2342032).unwrap();
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
                natural_max_day: 28
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

#[test]
fn pre_min_invalid_reformation() {
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
                kind: inner::GapKind::CrossMonth,
                ordinal_gap_start: 59,
                ordinal_gap: 0,
            }
        }
    );
}

#[test]
fn first_skipped_year() {
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
                kind: inner::GapKind::MultiYear,
                ordinal_gap_start: 0,
                ordinal_gap: 0,
            }
        }
    );
    assert_eq!(cal.year_kind(48900), YearKind::Leap);
    assert_eq!(cal.year_kind(48901), YearKind::Skipped);
    assert_eq!(cal.year_kind(48902), YearKind::Common);
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
