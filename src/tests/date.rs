use crate::{Calendar, Month};

#[test]
fn display_date() {
    let date = Calendar::gregorian_reform()
        .at_ymd(2023, Month::April, 20)
        .unwrap();
    assert_eq!(format!("{date}"), "2023-04-20");
}

#[test]
fn alternate_display_date() {
    let date = Calendar::gregorian_reform()
        .at_ymd(2023, Month::April, 20)
        .unwrap();
    assert_eq!(format!("{date:#}"), "2023-110");
}

#[test]
fn alternate_display_date_padded() {
    let date = Calendar::gregorian_reform()
        .at_ymd(2023, Month::March, 15)
        .unwrap();
    assert_eq!(format!("{date:#}"), "2023-074");
}
