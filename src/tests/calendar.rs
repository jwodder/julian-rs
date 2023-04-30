use crate::{ncal, Calendar, Month, MonthIter, MonthKind, YearKind, GREGORIAN};

#[test]
fn julian_properties() {
    let cal = Calendar::julian();
    assert!(cal.is_julian());
    assert!(!cal.is_gregorian());
    assert!(!cal.is_reforming());
    assert!(cal.is_proleptic());
    assert_eq!(cal.reformation(), None);
    assert_eq!(cal.last_julian_date(), None);
    assert_eq!(cal.first_gregorian_date(), None);
}

#[test]
fn gregorian_properties() {
    let cal = Calendar::gregorian();
    assert!(!cal.is_julian());
    assert!(cal.is_gregorian());
    assert!(!cal.is_reforming());
    assert!(cal.is_proleptic());
    assert_eq!(cal.reformation(), None);
    assert_eq!(cal.last_julian_date(), None);
    assert_eq!(cal.first_gregorian_date(), None);
}

#[test]
fn gregorian_reform_properties() {
    let cal = Calendar::GREGORIAN_REFORM;
    assert!(!cal.is_julian());
    assert!(!cal.is_gregorian());
    assert!(cal.is_reforming());
    assert!(!cal.is_proleptic());
    assert_eq!(cal.reformation(), Some(GREGORIAN));
    let last_julian = cal.last_julian_date().unwrap();
    assert_eq!(last_julian.calendar(), &cal);
    assert_eq!(last_julian.year(), 1582);
    assert_eq!(last_julian.month(), Month::October);
    assert_eq!(last_julian.day(), 4);
    assert_eq!(last_julian.ordinal(), 277);
    assert_eq!(last_julian.day_ordinal(), 4);
    let first_gregorian = cal.first_gregorian_date().unwrap();
    assert_eq!(first_gregorian.calendar(), &cal);
    assert_eq!(first_gregorian.year(), 1582);
    assert_eq!(first_gregorian.month(), Month::October);
    assert_eq!(first_gregorian.day(), 15);
    assert_eq!(first_gregorian.ordinal(), 278);
    assert_eq!(first_gregorian.day_ordinal(), 5);
}

#[test]
fn min_reform_properties() {
    let cal = Calendar::reforming(1830692).unwrap();
    assert!(!cal.is_julian());
    assert!(!cal.is_gregorian());
    assert!(cal.is_reforming());
    assert!(!cal.is_proleptic());
    assert_eq!(cal.reformation(), Some(1830692));
    let last_julian = cal.last_julian_date().unwrap();
    assert_eq!(last_julian.calendar(), &cal);
    assert_eq!(last_julian.year(), 300);
    assert_eq!(last_julian.month(), Month::February);
    assert_eq!(last_julian.day(), 28);
    assert_eq!(last_julian.ordinal(), 59);
    assert_eq!(last_julian.day_ordinal(), 28);
    let first_gregorian = cal.first_gregorian_date().unwrap();
    assert_eq!(first_gregorian.calendar(), &cal);
    assert_eq!(first_gregorian.year(), 300);
    assert_eq!(first_gregorian.month(), Month::March);
    assert_eq!(first_gregorian.day(), 1);
    assert_eq!(first_gregorian.ordinal(), 60);
    assert_eq!(first_gregorian.day_ordinal(), 1);
}

#[test]
fn ord() {
    use std::cmp::Ordering;
    let calendars = [
        Calendar::julian(),
        Calendar::reforming(1830692).unwrap(),
        Calendar::GREGORIAN_REFORM,
        Calendar::reforming(ncal::UNITED_KINGDOM).unwrap(),
        Calendar::reforming(ncal::RUSSIA).unwrap(),
        Calendar::reforming(2147439588).unwrap(),
        Calendar::gregorian(),
    ];
    for (i, cal1) in calendars.iter().enumerate() {
        for (j, cal2) in calendars.iter().enumerate() {
            match i.cmp(&j) {
                Ordering::Less => assert!(cal1 < cal2),
                Ordering::Equal => assert!(cal1 == cal2),
                Ordering::Greater => assert!(cal1 > cal2),
            }
        }
    }
}

#[test]
fn gregorian_leap_year() {
    let cal = Calendar::gregorian();
    assert_eq!(cal.year_kind(2000), YearKind::Leap);
    assert_eq!(cal.year_length(2000), 366);
    for (m, days) in MonthIter::new().zip([31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]) {
        let shape = cal.month_shape(2000, m).unwrap();
        assert_eq!(shape.len(), days);
        assert_eq!(shape.kind(), MonthKind::Normal);
    }
}

#[test]
fn gregorian_common_year() {
    let cal = Calendar::gregorian();
    assert_eq!(cal.year_kind(1900), YearKind::Common);
    assert_eq!(cal.year_length(1900), 365);
    for (m, days) in MonthIter::new().zip([31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]) {
        let shape = cal.month_shape(1900, m).unwrap();
        assert_eq!(shape.len(), days);
        assert_eq!(shape.kind(), MonthKind::Normal);
    }
}

#[test]
fn julian_leap_year() {
    let cal = Calendar::julian();
    assert_eq!(cal.year_kind(1900), YearKind::Leap);
    assert_eq!(cal.year_length(1900), 366);
    for (m, days) in MonthIter::new().zip([31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]) {
        let shape = cal.month_shape(1900, m).unwrap();
        assert_eq!(shape.len(), days);
        assert_eq!(shape.kind(), MonthKind::Normal);
    }
}

#[test]
fn julian_common_year() {
    let cal = Calendar::gregorian();
    assert_eq!(cal.year_kind(1901), YearKind::Common);
    assert_eq!(cal.year_length(1901), 365);
    for (m, days) in MonthIter::new().zip([31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]) {
        let shape = cal.month_shape(1901, m).unwrap();
        assert_eq!(shape.len(), days);
        assert_eq!(shape.kind(), MonthKind::Normal);
    }
}
