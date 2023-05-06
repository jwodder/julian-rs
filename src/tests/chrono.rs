#![cfg(feature = "chrono")]
use crate::{Calendar, Date, Month, TryFromDateError, Weekday};
use chrono::{naive::NaiveDate, Datelike};

#[test]
fn convert_gregorian_to_chrono() {
    let date = Calendar::GREGORIAN.at_ymd(2023, Month::May, 2).unwrap();
    let chdate = NaiveDate::try_from(date).unwrap();
    assert_eq!(chdate.year(), 2023);
    assert_eq!(chdate.month(), 5);
    assert_eq!(chdate.day(), 2);
}

#[test]
fn convert_julian_to_chrono() {
    let date = Calendar::JULIAN.at_ymd(2023, Month::May, 2).unwrap();
    let chdate = NaiveDate::try_from(date).unwrap();
    assert_eq!(chdate.year(), 2023);
    assert_eq!(chdate.month(), 5);
    assert_eq!(chdate.day(), 15);
}

#[test]
fn convert_pre_reform_to_chrono() {
    let date = Calendar::REFORM1582.last_julian_date().unwrap();
    let chdate = NaiveDate::try_from(date).unwrap();
    assert_eq!(chdate.year(), 1582);
    assert_eq!(chdate.month(), 10);
    assert_eq!(chdate.day(), 14);
}

#[test]
fn convert_post_reform_to_chrono() {
    let date = Calendar::REFORM1582.first_gregorian_date().unwrap();
    let chdate = NaiveDate::try_from(date).unwrap();
    assert_eq!(chdate.year(), 1582);
    assert_eq!(chdate.month(), 10);
    assert_eq!(chdate.day(), 15);
}

#[test]
fn convert_to_min_chrono() {
    let date = Calendar::GREGORIAN
        .at_ymd(-262144, Month::January, 1)
        .unwrap();
    let chdate = NaiveDate::try_from(date).unwrap();
    assert_eq!(chdate.year(), -262144);
    assert_eq!(chdate.month(), 1);
    assert_eq!(chdate.day(), 1);
}

#[test]
fn convert_pre_min_chrono() {
    let date = Calendar::GREGORIAN
        .at_ymd(-262145, Month::December, 31)
        .unwrap();
    assert_eq!(NaiveDate::try_from(date), Err(TryFromDateError));
}

#[test]
fn convert_to_max_chrono() {
    let date = Calendar::GREGORIAN
        .at_ymd(262143, Month::December, 31)
        .unwrap();
    let chdate = NaiveDate::try_from(date).unwrap();
    assert_eq!(chdate.year(), 262143);
    assert_eq!(chdate.month(), 12);
    assert_eq!(chdate.day(), 31);
}

#[test]
fn convert_post_max_chrono() {
    let date = Calendar::GREGORIAN
        .at_ymd(262144, Month::January, 1)
        .unwrap();
    assert_eq!(NaiveDate::try_from(date), Err(TryFromDateError));
}

#[test]
fn convert_from_chrono() {
    let chdate = NaiveDate::from_ymd_opt(2023, 5, 2).unwrap();
    let date = Date::from(chdate);
    assert_eq!(date.calendar(), Calendar::GREGORIAN);
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::May);
    assert_eq!(date.day(), 2);
}

#[test]
fn convert_pre_gregorian_from_chrono() {
    let chdate = NaiveDate::from_ymd_opt(1582, 10, 4).unwrap();
    let date = Date::from(chdate);
    assert_eq!(date.calendar(), Calendar::GREGORIAN);
    assert_eq!(date.year(), 1582);
    assert_eq!(date.month(), Month::October);
    assert_eq!(date.day(), 4);
}

#[test]
fn convert_from_min_chrono() {
    let date = Date::from(NaiveDate::MIN);
    assert_eq!(date.calendar(), Calendar::GREGORIAN);
    assert_eq!(date.year(), -262144);
    assert_eq!(date.month(), Month::January);
    assert_eq!(date.day(), 1);
}

#[test]
fn convert_from_max_chrono() {
    let date = Date::from(NaiveDate::MAX);
    assert_eq!(date.calendar(), Calendar::GREGORIAN);
    assert_eq!(date.year(), 262143);
    assert_eq!(date.month(), Month::December);
    assert_eq!(date.day(), 31);
}

#[test]
fn convert_january() {
    assert_eq!(chrono::Month::from(Month::January), chrono::Month::January);
    assert_eq!(Month::from(chrono::Month::January), Month::January);
}

#[test]
fn convert_february() {
    assert_eq!(
        chrono::Month::from(Month::February),
        chrono::Month::February
    );
    assert_eq!(Month::from(chrono::Month::February), Month::February);
}

#[test]
fn convert_march() {
    assert_eq!(chrono::Month::from(Month::March), chrono::Month::March);
    assert_eq!(Month::from(chrono::Month::March), Month::March);
}

#[test]
fn convert_april() {
    assert_eq!(chrono::Month::from(Month::April), chrono::Month::April);
    assert_eq!(Month::from(chrono::Month::April), Month::April);
}

#[test]
fn convert_may() {
    assert_eq!(chrono::Month::from(Month::May), chrono::Month::May);
    assert_eq!(Month::from(chrono::Month::May), Month::May);
}

#[test]
fn convert_june() {
    assert_eq!(chrono::Month::from(Month::June), chrono::Month::June);
    assert_eq!(Month::from(chrono::Month::June), Month::June);
}

#[test]
fn convert_july() {
    assert_eq!(chrono::Month::from(Month::July), chrono::Month::July);
    assert_eq!(Month::from(chrono::Month::July), Month::July);
}

#[test]
fn convert_august() {
    assert_eq!(chrono::Month::from(Month::August), chrono::Month::August);
    assert_eq!(Month::from(chrono::Month::August), Month::August);
}

#[test]
fn convert_september() {
    assert_eq!(
        chrono::Month::from(Month::September),
        chrono::Month::September
    );
    assert_eq!(Month::from(chrono::Month::September), Month::September);
}

#[test]
fn convert_october() {
    assert_eq!(chrono::Month::from(Month::October), chrono::Month::October);
    assert_eq!(Month::from(chrono::Month::October), Month::October);
}

#[test]
fn convert_november() {
    assert_eq!(
        chrono::Month::from(Month::November),
        chrono::Month::November
    );
    assert_eq!(Month::from(chrono::Month::November), Month::November);
}

#[test]
fn convert_december() {
    assert_eq!(
        chrono::Month::from(Month::December),
        chrono::Month::December
    );
    assert_eq!(Month::from(chrono::Month::December), Month::December);
}

#[test]
fn convert_monday() {
    assert_eq!(chrono::Weekday::from(Weekday::Monday), chrono::Weekday::Mon);
    assert_eq!(Weekday::from(chrono::Weekday::Mon), Weekday::Monday);
}

#[test]
fn convert_tuesday() {
    assert_eq!(
        chrono::Weekday::from(Weekday::Tuesday),
        chrono::Weekday::Tue
    );
    assert_eq!(Weekday::from(chrono::Weekday::Tue), Weekday::Tuesday);
}

#[test]
fn convert_wednesday() {
    assert_eq!(
        chrono::Weekday::from(Weekday::Wednesday),
        chrono::Weekday::Wed
    );
    assert_eq!(Weekday::from(chrono::Weekday::Wed), Weekday::Wednesday);
}

#[test]
fn convert_thursday() {
    assert_eq!(
        chrono::Weekday::from(Weekday::Thursday),
        chrono::Weekday::Thu
    );
    assert_eq!(Weekday::from(chrono::Weekday::Thu), Weekday::Thursday);
}

#[test]
fn convert_friday() {
    assert_eq!(chrono::Weekday::from(Weekday::Friday), chrono::Weekday::Fri);
    assert_eq!(Weekday::from(chrono::Weekday::Fri), Weekday::Friday);
}

#[test]
fn convert_saturday() {
    assert_eq!(
        chrono::Weekday::from(Weekday::Saturday),
        chrono::Weekday::Sat
    );
    assert_eq!(Weekday::from(chrono::Weekday::Sat), Weekday::Saturday);
}

#[test]
fn convert_sunday() {
    assert_eq!(chrono::Weekday::from(Weekday::Sunday), chrono::Weekday::Sun);
    assert_eq!(Weekday::from(chrono::Weekday::Sun), Weekday::Sunday);
}
