use crate::{ncal, Calendar, Month};

#[test]
fn display() {
    let date = Calendar::REFORM1582.at_ymd(2023, Month::April, 20).unwrap();
    assert_eq!(format!("{date}"), "2023-04-20");
}

#[test]
fn alternate_display() {
    let date = Calendar::REFORM1582.at_ymd(2023, Month::April, 20).unwrap();
    assert_eq!(format!("{date:#}"), "2023-110");
}

#[test]
fn alternate_display_padded() {
    let date = Calendar::REFORM1582.at_ymd(2023, Month::March, 15).unwrap();
    assert_eq!(format!("{date:#}"), "2023-074");
}

#[test]
fn ord() {
    use std::cmp::Ordering;
    let julian = Calendar::JULIAN;
    let r1582 = Calendar::REFORM1582;
    let r1752 = Calendar::reforming(ncal::UNITED_KINGDOM).unwrap();
    let gregorian = Calendar::GREGORIAN;
    let dates = [
        julian.at_jdn(-69105),
        r1582.at_jdn(-69105),
        r1752.at_jdn(-69105),
        gregorian.at_jdn(-69105),
        julian.at_jdn(0),
        r1582.at_jdn(0),
        r1752.at_jdn(0),
        gregorian.at_jdn(0),
        julian.at_jdn(69105),
        r1582.at_jdn(69105),
        r1752.at_jdn(69105),
        gregorian.at_jdn(69105),
    ];
    for (i, d1) in dates.iter().enumerate() {
        for (j, d2) in dates.iter().enumerate() {
            match i.cmp(&j) {
                Ordering::Less => assert!(d1 < d2),
                Ordering::Equal => assert!(d1 == d2),
                Ordering::Greater => assert!(d1 > d2),
            }
        }
    }
}

#[test]
fn convert_to() {
    let julian = Calendar::JULIAN;
    let gregorian = Calendar::GREGORIAN;
    let date = gregorian.at_ymd(2023, Month::April, 30).unwrap();
    let date2 = date.convert_to(julian);
    assert_eq!(date2.year(), 2023);
    assert_eq!(date2.month(), Month::April);
    assert_eq!(date2.day(), 17);
    assert_eq!(date.julian_day_number(), date2.julian_day_number());
    assert_eq!(date.julian_day_number(), 2460065);
    assert_eq!(date2, julian.at_ymd(2023, Month::April, 17).unwrap());
}
