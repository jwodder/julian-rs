use crate::{ParseWeekdayError, TryIntoWeekdayError, Weekday};
use rstest::rstest;

#[test]
fn monday() {
    let wd = Weekday::Monday;
    assert_eq!(wd.name(), "Monday");
    assert_eq!(wd.short_name(), "Mon");
    assert_eq!(wd.to_string(), "Monday");
    assert_eq!(format!("{wd:#}"), "Mon");
    assert_eq!(wd.number(), 1);
    assert_eq!(wd.number0(), 0);
    assert_eq!(wd.pred(), None);
    assert_eq!(wd.succ(), Some(Weekday::Tuesday));
    assert_eq!("Monday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("Mon".parse::<Weekday>().unwrap(), wd);
    assert_eq!("monday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("mon".parse::<Weekday>().unwrap(), wd);
    assert_eq!("MONDAY".parse::<Weekday>().unwrap(), wd);
    assert_eq!("MON".parse::<Weekday>().unwrap(), wd);
    assert_eq!(Weekday::try_from(1i8).unwrap(), wd);
    assert_eq!(Weekday::try_from(1u8).unwrap(), wd);
    assert_eq!(Weekday::try_from(1i16).unwrap(), wd);
    assert_eq!(Weekday::try_from(1u16).unwrap(), wd);
    assert_eq!(Weekday::try_from(1i32).unwrap(), wd);
    assert_eq!(Weekday::try_from(1u32).unwrap(), wd);
    assert_eq!(Weekday::try_from(1i64).unwrap(), wd);
    assert_eq!(Weekday::try_from(1u64).unwrap(), wd);
    assert_eq!(Weekday::try_from(1i128).unwrap(), wd);
    assert_eq!(Weekday::try_from(1u128).unwrap(), wd);
    assert_eq!(Weekday::try_from(1isize).unwrap(), wd);
    assert_eq!(Weekday::try_from(1usize).unwrap(), wd);
}

#[test]
fn tuesday() {
    let wd = Weekday::Tuesday;
    assert_eq!(wd.name(), "Tuesday");
    assert_eq!(wd.short_name(), "Tue");
    assert_eq!(wd.to_string(), "Tuesday");
    assert_eq!(format!("{wd:#}"), "Tue");
    assert_eq!(wd.number(), 2);
    assert_eq!(wd.number0(), 1);
    assert_eq!(wd.pred(), Some(Weekday::Monday));
    assert_eq!(wd.succ(), Some(Weekday::Wednesday));
    assert_eq!("Tuesday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("Tue".parse::<Weekday>().unwrap(), wd);
    assert_eq!("tuesday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("tue".parse::<Weekday>().unwrap(), wd);
    assert_eq!("TUESDAY".parse::<Weekday>().unwrap(), wd);
    assert_eq!("TUE".parse::<Weekday>().unwrap(), wd);
    assert_eq!(Weekday::try_from(2i8).unwrap(), wd);
    assert_eq!(Weekday::try_from(2u8).unwrap(), wd);
    assert_eq!(Weekday::try_from(2i16).unwrap(), wd);
    assert_eq!(Weekday::try_from(2u16).unwrap(), wd);
    assert_eq!(Weekday::try_from(2i32).unwrap(), wd);
    assert_eq!(Weekday::try_from(2u32).unwrap(), wd);
    assert_eq!(Weekday::try_from(2i64).unwrap(), wd);
    assert_eq!(Weekday::try_from(2u64).unwrap(), wd);
    assert_eq!(Weekday::try_from(2i128).unwrap(), wd);
    assert_eq!(Weekday::try_from(2u128).unwrap(), wd);
    assert_eq!(Weekday::try_from(2isize).unwrap(), wd);
    assert_eq!(Weekday::try_from(2usize).unwrap(), wd);
}

#[test]
fn wednesday() {
    let wd = Weekday::Wednesday;
    assert_eq!(wd.name(), "Wednesday");
    assert_eq!(wd.short_name(), "Wed");
    assert_eq!(wd.to_string(), "Wednesday");
    assert_eq!(format!("{wd:#}"), "Wed");
    assert_eq!(wd.number(), 3);
    assert_eq!(wd.number0(), 2);
    assert_eq!(wd.pred(), Some(Weekday::Tuesday));
    assert_eq!(wd.succ(), Some(Weekday::Thursday));
    assert_eq!("Wednesday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("Wed".parse::<Weekday>().unwrap(), wd);
    assert_eq!("wednesday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("wed".parse::<Weekday>().unwrap(), wd);
    assert_eq!("WEDNESDAY".parse::<Weekday>().unwrap(), wd);
    assert_eq!("WED".parse::<Weekday>().unwrap(), wd);
    assert_eq!(Weekday::try_from(3i8).unwrap(), wd);
    assert_eq!(Weekday::try_from(3u8).unwrap(), wd);
    assert_eq!(Weekday::try_from(3i16).unwrap(), wd);
    assert_eq!(Weekday::try_from(3u16).unwrap(), wd);
    assert_eq!(Weekday::try_from(3i32).unwrap(), wd);
    assert_eq!(Weekday::try_from(3u32).unwrap(), wd);
    assert_eq!(Weekday::try_from(3i64).unwrap(), wd);
    assert_eq!(Weekday::try_from(3u64).unwrap(), wd);
    assert_eq!(Weekday::try_from(3i128).unwrap(), wd);
    assert_eq!(Weekday::try_from(3u128).unwrap(), wd);
    assert_eq!(Weekday::try_from(3isize).unwrap(), wd);
    assert_eq!(Weekday::try_from(3usize).unwrap(), wd);
}

#[test]
fn thursday() {
    let wd = Weekday::Thursday;
    assert_eq!(wd.name(), "Thursday");
    assert_eq!(wd.short_name(), "Thu");
    assert_eq!(wd.to_string(), "Thursday");
    assert_eq!(format!("{wd:#}"), "Thu");
    assert_eq!(wd.number(), 4);
    assert_eq!(wd.number0(), 3);
    assert_eq!(wd.pred(), Some(Weekday::Wednesday));
    assert_eq!(wd.succ(), Some(Weekday::Friday));
    assert_eq!("Thursday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("Thu".parse::<Weekday>().unwrap(), wd);
    assert_eq!("thursday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("thu".parse::<Weekday>().unwrap(), wd);
    assert_eq!("THURSDAY".parse::<Weekday>().unwrap(), wd);
    assert_eq!("THU".parse::<Weekday>().unwrap(), wd);
    assert_eq!(Weekday::try_from(4i8).unwrap(), wd);
    assert_eq!(Weekday::try_from(4u8).unwrap(), wd);
    assert_eq!(Weekday::try_from(4i16).unwrap(), wd);
    assert_eq!(Weekday::try_from(4u16).unwrap(), wd);
    assert_eq!(Weekday::try_from(4i32).unwrap(), wd);
    assert_eq!(Weekday::try_from(4u32).unwrap(), wd);
    assert_eq!(Weekday::try_from(4i64).unwrap(), wd);
    assert_eq!(Weekday::try_from(4u64).unwrap(), wd);
    assert_eq!(Weekday::try_from(4i128).unwrap(), wd);
    assert_eq!(Weekday::try_from(4u128).unwrap(), wd);
    assert_eq!(Weekday::try_from(4isize).unwrap(), wd);
    assert_eq!(Weekday::try_from(4usize).unwrap(), wd);
}

#[test]
fn friday() {
    let wd = Weekday::Friday;
    assert_eq!(wd.name(), "Friday");
    assert_eq!(wd.short_name(), "Fri");
    assert_eq!(wd.to_string(), "Friday");
    assert_eq!(format!("{wd:#}"), "Fri");
    assert_eq!(wd.number(), 5);
    assert_eq!(wd.number0(), 4);
    assert_eq!(wd.pred(), Some(Weekday::Thursday));
    assert_eq!(wd.succ(), Some(Weekday::Saturday));
    assert_eq!("Friday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("Fri".parse::<Weekday>().unwrap(), wd);
    assert_eq!("friday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("fri".parse::<Weekday>().unwrap(), wd);
    assert_eq!("FRIDAY".parse::<Weekday>().unwrap(), wd);
    assert_eq!("FRI".parse::<Weekday>().unwrap(), wd);
    assert_eq!(Weekday::try_from(5i8).unwrap(), wd);
    assert_eq!(Weekday::try_from(5u8).unwrap(), wd);
    assert_eq!(Weekday::try_from(5i16).unwrap(), wd);
    assert_eq!(Weekday::try_from(5u16).unwrap(), wd);
    assert_eq!(Weekday::try_from(5i32).unwrap(), wd);
    assert_eq!(Weekday::try_from(5u32).unwrap(), wd);
    assert_eq!(Weekday::try_from(5i64).unwrap(), wd);
    assert_eq!(Weekday::try_from(5u64).unwrap(), wd);
    assert_eq!(Weekday::try_from(5i128).unwrap(), wd);
    assert_eq!(Weekday::try_from(5u128).unwrap(), wd);
    assert_eq!(Weekday::try_from(5isize).unwrap(), wd);
    assert_eq!(Weekday::try_from(5usize).unwrap(), wd);
}

#[test]
fn saturday() {
    let wd = Weekday::Saturday;
    assert_eq!(wd.name(), "Saturday");
    assert_eq!(wd.short_name(), "Sat");
    assert_eq!(wd.to_string(), "Saturday");
    assert_eq!(format!("{wd:#}"), "Sat");
    assert_eq!(wd.number(), 6);
    assert_eq!(wd.number0(), 5);
    assert_eq!(wd.pred(), Some(Weekday::Friday));
    assert_eq!(wd.succ(), Some(Weekday::Sunday));
    assert_eq!("Saturday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("Sat".parse::<Weekday>().unwrap(), wd);
    assert_eq!("saturday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("sat".parse::<Weekday>().unwrap(), wd);
    assert_eq!("SATURDAY".parse::<Weekday>().unwrap(), wd);
    assert_eq!("SAT".parse::<Weekday>().unwrap(), wd);
    assert_eq!(Weekday::try_from(6i8).unwrap(), wd);
    assert_eq!(Weekday::try_from(6u8).unwrap(), wd);
    assert_eq!(Weekday::try_from(6i16).unwrap(), wd);
    assert_eq!(Weekday::try_from(6u16).unwrap(), wd);
    assert_eq!(Weekday::try_from(6i32).unwrap(), wd);
    assert_eq!(Weekday::try_from(6u32).unwrap(), wd);
    assert_eq!(Weekday::try_from(6i64).unwrap(), wd);
    assert_eq!(Weekday::try_from(6u64).unwrap(), wd);
    assert_eq!(Weekday::try_from(6i128).unwrap(), wd);
    assert_eq!(Weekday::try_from(6u128).unwrap(), wd);
    assert_eq!(Weekday::try_from(6isize).unwrap(), wd);
    assert_eq!(Weekday::try_from(6usize).unwrap(), wd);
}

#[test]
fn sunday() {
    let wd = Weekday::Sunday;
    assert_eq!(wd.name(), "Sunday");
    assert_eq!(wd.short_name(), "Sun");
    assert_eq!(wd.to_string(), "Sunday");
    assert_eq!(format!("{wd:#}"), "Sun");
    assert_eq!(wd.number(), 7);
    assert_eq!(wd.number0(), 6);
    assert_eq!(wd.pred(), Some(Weekday::Saturday));
    assert_eq!(wd.succ(), None);
    assert_eq!("Sunday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("Sun".parse::<Weekday>().unwrap(), wd);
    assert_eq!("sunday".parse::<Weekday>().unwrap(), wd);
    assert_eq!("sun".parse::<Weekday>().unwrap(), wd);
    assert_eq!("SUNDAY".parse::<Weekday>().unwrap(), wd);
    assert_eq!("SUN".parse::<Weekday>().unwrap(), wd);
    assert_eq!(Weekday::try_from(7i8).unwrap(), wd);
    assert_eq!(Weekday::try_from(7u8).unwrap(), wd);
    assert_eq!(Weekday::try_from(7i16).unwrap(), wd);
    assert_eq!(Weekday::try_from(7u16).unwrap(), wd);
    assert_eq!(Weekday::try_from(7i32).unwrap(), wd);
    assert_eq!(Weekday::try_from(7u32).unwrap(), wd);
    assert_eq!(Weekday::try_from(7i64).unwrap(), wd);
    assert_eq!(Weekday::try_from(7u64).unwrap(), wd);
    assert_eq!(Weekday::try_from(7i128).unwrap(), wd);
    assert_eq!(Weekday::try_from(7u128).unwrap(), wd);
    assert_eq!(Weekday::try_from(7isize).unwrap(), wd);
    assert_eq!(Weekday::try_from(7usize).unwrap(), wd);
}

#[rstest]
#[case("Mon.")]
#[case("Tues")]
#[case("Freitag")]
#[case("1")]
#[case(" Mon")]
#[case(" Mon ")]
#[case("Mon ")]
fn parse_error(#[case] s: &str) {
    let r = s.parse::<Weekday>();
    assert_eq!(r, Err(ParseWeekdayError));
    assert_eq!(r.unwrap_err().to_string(), "invalid weekday name");
}

#[test]
fn try_from_zero() {
    let r = Weekday::try_from(0);
    assert_eq!(r, Err(TryIntoWeekdayError));
    assert_eq!(
        r.unwrap_err().to_string(),
        "value out of range for weekday number; must be from 1 through 7"
    );
}

#[test]
fn try_from_eight() {
    let r = Weekday::try_from(8);
    assert_eq!(r, Err(TryIntoWeekdayError));
    assert_eq!(
        r.unwrap_err().to_string(),
        "value out of range for weekday number; must be from 1 through 7"
    );
}
