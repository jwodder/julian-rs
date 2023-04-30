use crate::{ncal, Calendar, Month};

#[test]
fn display_date() {
    let date = Calendar::GREGORIAN_REFORM
        .at_ymd(2023, Month::April, 20)
        .unwrap();
    assert_eq!(format!("{date}"), "2023-04-20");
}

#[test]
fn alternate_display_date() {
    let date = Calendar::GREGORIAN_REFORM
        .at_ymd(2023, Month::April, 20)
        .unwrap();
    assert_eq!(format!("{date:#}"), "2023-110");
}

#[test]
fn alternate_display_date_padded() {
    let date = Calendar::GREGORIAN_REFORM
        .at_ymd(2023, Month::March, 15)
        .unwrap();
    assert_eq!(format!("{date:#}"), "2023-074");
}

#[test]
fn ord() {
    use std::cmp::Ordering;
    let julian = Calendar::julian();
    let r1582 = Calendar::GREGORIAN_REFORM;
    let r1752 = Calendar::reforming(ncal::UNITED_KINGDOM).unwrap();
    let gregorian = Calendar::gregorian();
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
