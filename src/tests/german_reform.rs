use crate::{inner, Calendar, Month, YearKind};
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
                    mday: 18
                },
                post_reform: inner::Date {
                    year: 1700,
                    ordinal: 50,
                    month: Month::March,
                    mday: 1
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
            range: 1..=18
        }
    );
    let shape_mar = cal.month_shape(1700, Month::March);
    assert_eq!(
        shape_mar,
        inner::MonthShape::Solid {
            year: 1700,
            month: Month::March,
            range: 1..=31
        }
    );
}
