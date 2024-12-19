#![cfg(feature = "time")]
use crate::{errors::TryFromDateError, Calendar, Date, Month, Weekday};

#[test]
fn convert_gregorian_to_time() {
    let date = Calendar::GREGORIAN.at_ymd(2023, Month::May, 2).unwrap();
    let chdate = time::Date::try_from(date).unwrap();
    assert_eq!(chdate.year(), 2023);
    assert_eq!(chdate.month(), time::Month::May);
    assert_eq!(chdate.day(), 2);
}

#[test]
fn convert_julian_to_time() {
    let date = Calendar::JULIAN.at_ymd(2023, Month::May, 2).unwrap();
    let chdate = time::Date::try_from(date).unwrap();
    assert_eq!(chdate.year(), 2023);
    assert_eq!(chdate.month(), time::Month::May);
    assert_eq!(chdate.day(), 15);
}

#[test]
fn convert_pre_reform_to_time() {
    let date = Calendar::REFORM1582.last_julian_date().unwrap();
    let chdate = time::Date::try_from(date).unwrap();
    assert_eq!(chdate.year(), 1582);
    assert_eq!(chdate.month(), time::Month::October);
    assert_eq!(chdate.day(), 14);
}

#[test]
fn convert_post_reform_to_time() {
    let date = Calendar::REFORM1582.first_gregorian_date().unwrap();
    let chdate = time::Date::try_from(date).unwrap();
    assert_eq!(chdate.year(), 1582);
    assert_eq!(chdate.month(), time::Month::October);
    assert_eq!(chdate.day(), 15);
}

#[test]
fn convert_to_min_time() {
    let date = Calendar::GREGORIAN
        .at_ymd(-9999, Month::January, 1)
        .unwrap();
    let chdate = time::Date::try_from(date).unwrap();
    assert_eq!(chdate, time::Date::MIN);
}

#[test]
fn convert_pre_min_time() {
    let date = Calendar::GREGORIAN
        .at_ymd(-10000, Month::December, 31)
        .unwrap();
    assert_eq!(time::Date::try_from(date), Err(TryFromDateError));
}

#[test]
fn convert_to_max_time() {
    let date = Calendar::GREGORIAN
        .at_ymd(9999, Month::December, 31)
        .unwrap();
    let chdate = time::Date::try_from(date).unwrap();
    assert_eq!(chdate, time::Date::MAX);
}

#[test]
fn convert_post_max_time() {
    let date = Calendar::GREGORIAN
        .at_ymd(10000, Month::January, 1)
        .unwrap();
    assert_eq!(time::Date::try_from(date), Err(TryFromDateError));
}

#[test]
fn convert_from_time() {
    let chdate = time::Date::from_calendar_date(2023, time::Month::May, 2).unwrap();
    let date = Date::from(chdate);
    assert_eq!(date.calendar(), Calendar::GREGORIAN);
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), Month::May);
    assert_eq!(date.day(), 2);
}

#[test]
fn convert_pre_gregorian_from_time() {
    let chdate = time::Date::from_calendar_date(1582, time::Month::October, 4).unwrap();
    let date = Date::from(chdate);
    assert_eq!(date.calendar(), Calendar::GREGORIAN);
    assert_eq!(date.year(), 1582);
    assert_eq!(date.month(), Month::October);
    assert_eq!(date.day(), 4);
}

#[test]
fn convert_from_min_time() {
    let date = Date::from(time::Date::MIN);
    assert_eq!(date.calendar(), Calendar::GREGORIAN);
    assert_eq!(date.year(), -9999);
    assert_eq!(date.month(), Month::January);
    assert_eq!(date.day(), 1);
}

#[test]
fn convert_from_max_time() {
    let date = Date::from(time::Date::MAX);
    assert_eq!(date.calendar(), Calendar::GREGORIAN);
    assert_eq!(date.year(), 9999);
    assert_eq!(date.month(), Month::December);
    assert_eq!(date.day(), 31);
}

#[test]
fn convert_january() {
    assert_eq!(time::Month::from(Month::January), time::Month::January);
    assert_eq!(Month::from(time::Month::January), Month::January);
}

#[test]
fn convert_february() {
    assert_eq!(time::Month::from(Month::February), time::Month::February);
    assert_eq!(Month::from(time::Month::February), Month::February);
}

#[test]
fn convert_march() {
    assert_eq!(time::Month::from(Month::March), time::Month::March);
    assert_eq!(Month::from(time::Month::March), Month::March);
}

#[test]
fn convert_april() {
    assert_eq!(time::Month::from(Month::April), time::Month::April);
    assert_eq!(Month::from(time::Month::April), Month::April);
}

#[test]
fn convert_may() {
    assert_eq!(time::Month::from(Month::May), time::Month::May);
    assert_eq!(Month::from(time::Month::May), Month::May);
}

#[test]
fn convert_june() {
    assert_eq!(time::Month::from(Month::June), time::Month::June);
    assert_eq!(Month::from(time::Month::June), Month::June);
}

#[test]
fn convert_july() {
    assert_eq!(time::Month::from(Month::July), time::Month::July);
    assert_eq!(Month::from(time::Month::July), Month::July);
}

#[test]
fn convert_august() {
    assert_eq!(time::Month::from(Month::August), time::Month::August);
    assert_eq!(Month::from(time::Month::August), Month::August);
}

#[test]
fn convert_september() {
    assert_eq!(time::Month::from(Month::September), time::Month::September);
    assert_eq!(Month::from(time::Month::September), Month::September);
}

#[test]
fn convert_october() {
    assert_eq!(time::Month::from(Month::October), time::Month::October);
    assert_eq!(Month::from(time::Month::October), Month::October);
}

#[test]
fn convert_november() {
    assert_eq!(time::Month::from(Month::November), time::Month::November);
    assert_eq!(Month::from(time::Month::November), Month::November);
}

#[test]
fn convert_december() {
    assert_eq!(time::Month::from(Month::December), time::Month::December);
    assert_eq!(Month::from(time::Month::December), Month::December);
}

#[test]
fn convert_monday() {
    assert_eq!(time::Weekday::from(Weekday::Monday), time::Weekday::Monday);
    assert_eq!(Weekday::from(time::Weekday::Monday), Weekday::Monday);
}

#[test]
fn convert_tuesday() {
    assert_eq!(
        time::Weekday::from(Weekday::Tuesday),
        time::Weekday::Tuesday
    );
    assert_eq!(Weekday::from(time::Weekday::Tuesday), Weekday::Tuesday);
}

#[test]
fn convert_wednesday() {
    assert_eq!(
        time::Weekday::from(Weekday::Wednesday),
        time::Weekday::Wednesday
    );
    assert_eq!(Weekday::from(time::Weekday::Wednesday), Weekday::Wednesday);
}

#[test]
fn convert_thursday() {
    assert_eq!(
        time::Weekday::from(Weekday::Thursday),
        time::Weekday::Thursday
    );
    assert_eq!(Weekday::from(time::Weekday::Thursday), Weekday::Thursday);
}

#[test]
fn convert_friday() {
    assert_eq!(time::Weekday::from(Weekday::Friday), time::Weekday::Friday);
    assert_eq!(Weekday::from(time::Weekday::Friday), Weekday::Friday);
}

#[test]
fn convert_saturday() {
    assert_eq!(
        time::Weekday::from(Weekday::Saturday),
        time::Weekday::Saturday
    );
    assert_eq!(Weekday::from(time::Weekday::Saturday), Weekday::Saturday);
}

#[test]
fn convert_sunday() {
    assert_eq!(time::Weekday::from(Weekday::Sunday), time::Weekday::Sunday);
    assert_eq!(Weekday::from(time::Weekday::Sunday), Weekday::Sunday);
}
