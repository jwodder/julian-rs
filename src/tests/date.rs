use crate::{ncal, Calendar, Jdnum, Month};

#[test]
fn display() {
    let date = Calendar::REFORM1582.at_ymd(2023, Month::April, 20).unwrap();
    assert_eq!(format!("{date}"), "2023-04-20");
}

#[test]
fn display_padded() {
    let date = Calendar::REFORM1582.at_ymd(4, Month::April, 2).unwrap();
    assert_eq!(format!("{date}"), "0004-04-02");
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

#[test]
fn pred_min_date() {
    let date = Calendar::GREGORIAN.at_jdn(Jdnum::MIN);
    assert_eq!(date.pred(), None);
}

#[test]
fn succ_min_date() {
    let date = Calendar::GREGORIAN.at_jdn(Jdnum::MIN);
    let succ = date.succ().unwrap();
    assert_eq!(succ.calendar(), Calendar::GREGORIAN);
    assert_eq!(succ.year(), -5884323);
    assert_eq!(succ.month(), Month::May);
    assert_eq!(succ.day(), 16);
    assert_eq!(succ.julian_day_number(), Jdnum::MIN + 1);
}

#[test]
fn pred_across_normal_year() {
    let date = Calendar::GREGORIAN.at_ymd(2023, Month::January, 1).unwrap();
    let pred = date.pred().unwrap();
    assert_eq!(pred.calendar(), Calendar::GREGORIAN);
    assert_eq!(pred.year(), 2022);
    assert_eq!(pred.month(), Month::December);
    assert_eq!(pred.day(), 31);
    assert_eq!(pred.ordinal(), 365);
}

#[test]
fn succ_across_normal_year() {
    let date = Calendar::GREGORIAN
        .at_ymd(2022, Month::December, 31)
        .unwrap();
    let succ = date.succ().unwrap();
    assert_eq!(succ.calendar(), Calendar::GREGORIAN);
    assert_eq!(succ.year(), 2023);
    assert_eq!(succ.month(), Month::January);
    assert_eq!(succ.day(), 1);
    assert_eq!(succ.ordinal(), 1);
}

#[test]
fn pred_max_date() {
    let date = Calendar::GREGORIAN.at_jdn(Jdnum::MAX);
    let succ = date.pred().unwrap();
    assert_eq!(succ.calendar(), Calendar::GREGORIAN);
    assert_eq!(succ.year(), 5874898);
    assert_eq!(succ.month(), Month::June);
    assert_eq!(succ.day(), 2);
    assert_eq!(succ.julian_day_number(), Jdnum::MAX - 1);
}

#[test]
fn succ_max_date() {
    let date = Calendar::GREGORIAN.at_jdn(Jdnum::MAX);
    assert_eq!(date.succ(), None);
}
