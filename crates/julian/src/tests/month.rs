use crate::{
    errors::{ParseMonthError, TryIntoMonthError},
    iter::MonthIter,
    Month,
};
use rstest::rstest;

#[test]
fn january() {
    let m = Month::January;
    assert_eq!(m.name(), "January");
    assert_eq!(m.short_name(), "Jan");
    assert_eq!(m.to_string(), "January");
    assert_eq!(format!("{m:#}"), "Jan");
    assert_eq!(m.number(), 1);
    assert_eq!(m.number0(), 0);
    assert_eq!(m.pred(), None);
    assert_eq!(m.succ(), Some(Month::February));
    assert_eq!("January".parse::<Month>().unwrap(), m);
    assert_eq!("Jan".parse::<Month>().unwrap(), m);
    assert_eq!("january".parse::<Month>().unwrap(), m);
    assert_eq!("jan".parse::<Month>().unwrap(), m);
    assert_eq!("JANUARY".parse::<Month>().unwrap(), m);
    assert_eq!("JAN".parse::<Month>().unwrap(), m);
    assert_eq!(Month::try_from(1i8).unwrap(), m);
    assert_eq!(Month::try_from(1u8).unwrap(), m);
    assert_eq!(Month::try_from(1i16).unwrap(), m);
    assert_eq!(Month::try_from(1u16).unwrap(), m);
    assert_eq!(Month::try_from(1i32).unwrap(), m);
    assert_eq!(Month::try_from(1u32).unwrap(), m);
    assert_eq!(Month::try_from(1i64).unwrap(), m);
    assert_eq!(Month::try_from(1u64).unwrap(), m);
    assert_eq!(Month::try_from(1i128).unwrap(), m);
    assert_eq!(Month::try_from(1u128).unwrap(), m);
    assert_eq!(Month::try_from(1isize).unwrap(), m);
    assert_eq!(Month::try_from(1usize).unwrap(), m);
}

#[test]
fn february() {
    let m = Month::February;
    assert_eq!(m.name(), "February");
    assert_eq!(m.short_name(), "Feb");
    assert_eq!(m.to_string(), "February");
    assert_eq!(format!("{m:#}"), "Feb");
    assert_eq!(m.number(), 2);
    assert_eq!(m.number0(), 1);
    assert_eq!(m.pred(), Some(Month::January));
    assert_eq!(m.succ(), Some(Month::March));
    assert_eq!("February".parse::<Month>().unwrap(), m);
    assert_eq!("Feb".parse::<Month>().unwrap(), m);
    assert_eq!("february".parse::<Month>().unwrap(), m);
    assert_eq!("feb".parse::<Month>().unwrap(), m);
    assert_eq!("FEBRUARY".parse::<Month>().unwrap(), m);
    assert_eq!("FEB".parse::<Month>().unwrap(), m);
    assert_eq!(Month::try_from(2i8).unwrap(), m);
    assert_eq!(Month::try_from(2u8).unwrap(), m);
    assert_eq!(Month::try_from(2i16).unwrap(), m);
    assert_eq!(Month::try_from(2u16).unwrap(), m);
    assert_eq!(Month::try_from(2i32).unwrap(), m);
    assert_eq!(Month::try_from(2u32).unwrap(), m);
    assert_eq!(Month::try_from(2i64).unwrap(), m);
    assert_eq!(Month::try_from(2u64).unwrap(), m);
    assert_eq!(Month::try_from(2i128).unwrap(), m);
    assert_eq!(Month::try_from(2u128).unwrap(), m);
    assert_eq!(Month::try_from(2isize).unwrap(), m);
    assert_eq!(Month::try_from(2usize).unwrap(), m);
}

#[test]
fn march() {
    let m = Month::March;
    assert_eq!(m.name(), "March");
    assert_eq!(m.short_name(), "Mar");
    assert_eq!(m.to_string(), "March");
    assert_eq!(format!("{m:#}"), "Mar");
    assert_eq!(m.number(), 3);
    assert_eq!(m.number0(), 2);
    assert_eq!(m.pred(), Some(Month::February));
    assert_eq!(m.succ(), Some(Month::April));
    assert_eq!("March".parse::<Month>().unwrap(), m);
    assert_eq!("Mar".parse::<Month>().unwrap(), m);
    assert_eq!("march".parse::<Month>().unwrap(), m);
    assert_eq!("mar".parse::<Month>().unwrap(), m);
    assert_eq!("MARCH".parse::<Month>().unwrap(), m);
    assert_eq!("MAR".parse::<Month>().unwrap(), m);
    assert_eq!(Month::try_from(3i8).unwrap(), m);
    assert_eq!(Month::try_from(3u8).unwrap(), m);
    assert_eq!(Month::try_from(3i16).unwrap(), m);
    assert_eq!(Month::try_from(3u16).unwrap(), m);
    assert_eq!(Month::try_from(3i32).unwrap(), m);
    assert_eq!(Month::try_from(3u32).unwrap(), m);
    assert_eq!(Month::try_from(3i64).unwrap(), m);
    assert_eq!(Month::try_from(3u64).unwrap(), m);
    assert_eq!(Month::try_from(3i128).unwrap(), m);
    assert_eq!(Month::try_from(3u128).unwrap(), m);
    assert_eq!(Month::try_from(3isize).unwrap(), m);
    assert_eq!(Month::try_from(3usize).unwrap(), m);
}

#[test]
fn april() {
    let m = Month::April;
    assert_eq!(m.name(), "April");
    assert_eq!(m.short_name(), "Apr");
    assert_eq!(m.to_string(), "April");
    assert_eq!(format!("{m:#}"), "Apr");
    assert_eq!(m.number(), 4);
    assert_eq!(m.number0(), 3);
    assert_eq!(m.pred(), Some(Month::March));
    assert_eq!(m.succ(), Some(Month::May));
    assert_eq!("April".parse::<Month>().unwrap(), m);
    assert_eq!("Apr".parse::<Month>().unwrap(), m);
    assert_eq!("april".parse::<Month>().unwrap(), m);
    assert_eq!("apr".parse::<Month>().unwrap(), m);
    assert_eq!("APRIL".parse::<Month>().unwrap(), m);
    assert_eq!("APR".parse::<Month>().unwrap(), m);
    assert_eq!(Month::try_from(4i8).unwrap(), m);
    assert_eq!(Month::try_from(4u8).unwrap(), m);
    assert_eq!(Month::try_from(4i16).unwrap(), m);
    assert_eq!(Month::try_from(4u16).unwrap(), m);
    assert_eq!(Month::try_from(4i32).unwrap(), m);
    assert_eq!(Month::try_from(4u32).unwrap(), m);
    assert_eq!(Month::try_from(4i64).unwrap(), m);
    assert_eq!(Month::try_from(4u64).unwrap(), m);
    assert_eq!(Month::try_from(4i128).unwrap(), m);
    assert_eq!(Month::try_from(4u128).unwrap(), m);
    assert_eq!(Month::try_from(4isize).unwrap(), m);
    assert_eq!(Month::try_from(4usize).unwrap(), m);
}

#[test]
fn may() {
    let m = Month::May;
    assert_eq!(m.name(), "May");
    assert_eq!(m.short_name(), "May");
    assert_eq!(m.to_string(), "May");
    assert_eq!(format!("{m:#}"), "May");
    assert_eq!(m.number(), 5);
    assert_eq!(m.number0(), 4);
    assert_eq!(m.pred(), Some(Month::April));
    assert_eq!(m.succ(), Some(Month::June));
    assert_eq!("May".parse::<Month>().unwrap(), m);
    assert_eq!("may".parse::<Month>().unwrap(), m);
    assert_eq!("MAY".parse::<Month>().unwrap(), m);
    assert_eq!(Month::try_from(5i8).unwrap(), m);
    assert_eq!(Month::try_from(5u8).unwrap(), m);
    assert_eq!(Month::try_from(5i16).unwrap(), m);
    assert_eq!(Month::try_from(5u16).unwrap(), m);
    assert_eq!(Month::try_from(5i32).unwrap(), m);
    assert_eq!(Month::try_from(5u32).unwrap(), m);
    assert_eq!(Month::try_from(5i64).unwrap(), m);
    assert_eq!(Month::try_from(5u64).unwrap(), m);
    assert_eq!(Month::try_from(5i128).unwrap(), m);
    assert_eq!(Month::try_from(5u128).unwrap(), m);
    assert_eq!(Month::try_from(5isize).unwrap(), m);
    assert_eq!(Month::try_from(5usize).unwrap(), m);
}

#[test]
fn june() {
    let m = Month::June;
    assert_eq!(m.name(), "June");
    assert_eq!(m.short_name(), "Jun");
    assert_eq!(m.to_string(), "June");
    assert_eq!(format!("{m:#}"), "Jun");
    assert_eq!(m.number(), 6);
    assert_eq!(m.number0(), 5);
    assert_eq!(m.pred(), Some(Month::May));
    assert_eq!(m.succ(), Some(Month::July));
    assert_eq!("June".parse::<Month>().unwrap(), m);
    assert_eq!("Jun".parse::<Month>().unwrap(), m);
    assert_eq!("june".parse::<Month>().unwrap(), m);
    assert_eq!("jun".parse::<Month>().unwrap(), m);
    assert_eq!("JUNE".parse::<Month>().unwrap(), m);
    assert_eq!("JUN".parse::<Month>().unwrap(), m);
    assert_eq!(Month::try_from(6i8).unwrap(), m);
    assert_eq!(Month::try_from(6u8).unwrap(), m);
    assert_eq!(Month::try_from(6i16).unwrap(), m);
    assert_eq!(Month::try_from(6u16).unwrap(), m);
    assert_eq!(Month::try_from(6i32).unwrap(), m);
    assert_eq!(Month::try_from(6u32).unwrap(), m);
    assert_eq!(Month::try_from(6i64).unwrap(), m);
    assert_eq!(Month::try_from(6u64).unwrap(), m);
    assert_eq!(Month::try_from(6i128).unwrap(), m);
    assert_eq!(Month::try_from(6u128).unwrap(), m);
    assert_eq!(Month::try_from(6isize).unwrap(), m);
    assert_eq!(Month::try_from(6usize).unwrap(), m);
}

#[test]
fn july() {
    let m = Month::July;
    assert_eq!(m.name(), "July");
    assert_eq!(m.short_name(), "Jul");
    assert_eq!(m.to_string(), "July");
    assert_eq!(format!("{m:#}"), "Jul");
    assert_eq!(m.number(), 7);
    assert_eq!(m.number0(), 6);
    assert_eq!(m.pred(), Some(Month::June));
    assert_eq!(m.succ(), Some(Month::August));
    assert_eq!("July".parse::<Month>().unwrap(), m);
    assert_eq!("Jul".parse::<Month>().unwrap(), m);
    assert_eq!("july".parse::<Month>().unwrap(), m);
    assert_eq!("jul".parse::<Month>().unwrap(), m);
    assert_eq!("JULY".parse::<Month>().unwrap(), m);
    assert_eq!("JUL".parse::<Month>().unwrap(), m);
    assert_eq!(Month::try_from(7i8).unwrap(), m);
    assert_eq!(Month::try_from(7u8).unwrap(), m);
    assert_eq!(Month::try_from(7i16).unwrap(), m);
    assert_eq!(Month::try_from(7u16).unwrap(), m);
    assert_eq!(Month::try_from(7i32).unwrap(), m);
    assert_eq!(Month::try_from(7u32).unwrap(), m);
    assert_eq!(Month::try_from(7i64).unwrap(), m);
    assert_eq!(Month::try_from(7u64).unwrap(), m);
    assert_eq!(Month::try_from(7i128).unwrap(), m);
    assert_eq!(Month::try_from(7u128).unwrap(), m);
    assert_eq!(Month::try_from(7isize).unwrap(), m);
    assert_eq!(Month::try_from(7usize).unwrap(), m);
}

#[test]
fn august() {
    let m = Month::August;
    assert_eq!(m.name(), "August");
    assert_eq!(m.short_name(), "Aug");
    assert_eq!(m.to_string(), "August");
    assert_eq!(format!("{m:#}"), "Aug");
    assert_eq!(m.number(), 8);
    assert_eq!(m.number0(), 7);
    assert_eq!(m.pred(), Some(Month::July));
    assert_eq!(m.succ(), Some(Month::September));
    assert_eq!("August".parse::<Month>().unwrap(), m);
    assert_eq!("Aug".parse::<Month>().unwrap(), m);
    assert_eq!("august".parse::<Month>().unwrap(), m);
    assert_eq!("aug".parse::<Month>().unwrap(), m);
    assert_eq!("AUGUST".parse::<Month>().unwrap(), m);
    assert_eq!("AUG".parse::<Month>().unwrap(), m);
    assert_eq!(Month::try_from(8i8).unwrap(), m);
    assert_eq!(Month::try_from(8u8).unwrap(), m);
    assert_eq!(Month::try_from(8i16).unwrap(), m);
    assert_eq!(Month::try_from(8u16).unwrap(), m);
    assert_eq!(Month::try_from(8i32).unwrap(), m);
    assert_eq!(Month::try_from(8u32).unwrap(), m);
    assert_eq!(Month::try_from(8i64).unwrap(), m);
    assert_eq!(Month::try_from(8u64).unwrap(), m);
    assert_eq!(Month::try_from(8i128).unwrap(), m);
    assert_eq!(Month::try_from(8u128).unwrap(), m);
    assert_eq!(Month::try_from(8isize).unwrap(), m);
    assert_eq!(Month::try_from(8usize).unwrap(), m);
}

#[test]
fn september() {
    let m = Month::September;
    assert_eq!(m.name(), "September");
    assert_eq!(m.short_name(), "Sep");
    assert_eq!(m.to_string(), "September");
    assert_eq!(format!("{m:#}"), "Sep");
    assert_eq!(m.number(), 9);
    assert_eq!(m.number0(), 8);
    assert_eq!(m.pred(), Some(Month::August));
    assert_eq!(m.succ(), Some(Month::October));
    assert_eq!("September".parse::<Month>().unwrap(), m);
    assert_eq!("Sep".parse::<Month>().unwrap(), m);
    assert_eq!("september".parse::<Month>().unwrap(), m);
    assert_eq!("sep".parse::<Month>().unwrap(), m);
    assert_eq!("SEPTEMBER".parse::<Month>().unwrap(), m);
    assert_eq!("SEP".parse::<Month>().unwrap(), m);
    assert_eq!(Month::try_from(9i8).unwrap(), m);
    assert_eq!(Month::try_from(9u8).unwrap(), m);
    assert_eq!(Month::try_from(9i16).unwrap(), m);
    assert_eq!(Month::try_from(9u16).unwrap(), m);
    assert_eq!(Month::try_from(9i32).unwrap(), m);
    assert_eq!(Month::try_from(9u32).unwrap(), m);
    assert_eq!(Month::try_from(9i64).unwrap(), m);
    assert_eq!(Month::try_from(9u64).unwrap(), m);
    assert_eq!(Month::try_from(9i128).unwrap(), m);
    assert_eq!(Month::try_from(9u128).unwrap(), m);
    assert_eq!(Month::try_from(9isize).unwrap(), m);
    assert_eq!(Month::try_from(9usize).unwrap(), m);
}

#[test]
fn october() {
    let m = Month::October;
    assert_eq!(m.name(), "October");
    assert_eq!(m.short_name(), "Oct");
    assert_eq!(m.to_string(), "October");
    assert_eq!(format!("{m:#}"), "Oct");
    assert_eq!(m.number(), 10);
    assert_eq!(m.number0(), 9);
    assert_eq!(m.pred(), Some(Month::September));
    assert_eq!(m.succ(), Some(Month::November));
    assert_eq!("October".parse::<Month>().unwrap(), m);
    assert_eq!("Oct".parse::<Month>().unwrap(), m);
    assert_eq!("october".parse::<Month>().unwrap(), m);
    assert_eq!("oct".parse::<Month>().unwrap(), m);
    assert_eq!("OCTOBER".parse::<Month>().unwrap(), m);
    assert_eq!("OCT".parse::<Month>().unwrap(), m);
    assert_eq!(Month::try_from(10i8).unwrap(), m);
    assert_eq!(Month::try_from(10u8).unwrap(), m);
    assert_eq!(Month::try_from(10i16).unwrap(), m);
    assert_eq!(Month::try_from(10u16).unwrap(), m);
    assert_eq!(Month::try_from(10i32).unwrap(), m);
    assert_eq!(Month::try_from(10u32).unwrap(), m);
    assert_eq!(Month::try_from(10i64).unwrap(), m);
    assert_eq!(Month::try_from(10u64).unwrap(), m);
    assert_eq!(Month::try_from(10i128).unwrap(), m);
    assert_eq!(Month::try_from(10u128).unwrap(), m);
    assert_eq!(Month::try_from(10isize).unwrap(), m);
    assert_eq!(Month::try_from(10usize).unwrap(), m);
}

#[test]
fn november() {
    let m = Month::November;
    assert_eq!(m.name(), "November");
    assert_eq!(m.short_name(), "Nov");
    assert_eq!(m.to_string(), "November");
    assert_eq!(format!("{m:#}"), "Nov");
    assert_eq!(m.number(), 11);
    assert_eq!(m.number0(), 10);
    assert_eq!(m.pred(), Some(Month::October));
    assert_eq!(m.succ(), Some(Month::December));
    assert_eq!("November".parse::<Month>().unwrap(), m);
    assert_eq!("Nov".parse::<Month>().unwrap(), m);
    assert_eq!("november".parse::<Month>().unwrap(), m);
    assert_eq!("nov".parse::<Month>().unwrap(), m);
    assert_eq!("NOVEMBER".parse::<Month>().unwrap(), m);
    assert_eq!("NOV".parse::<Month>().unwrap(), m);
    assert_eq!(Month::try_from(11).unwrap(), m);
    assert_eq!(Month::try_from(11i8).unwrap(), m);
    assert_eq!(Month::try_from(11u8).unwrap(), m);
    assert_eq!(Month::try_from(11i16).unwrap(), m);
    assert_eq!(Month::try_from(11u16).unwrap(), m);
    assert_eq!(Month::try_from(11i32).unwrap(), m);
    assert_eq!(Month::try_from(11u32).unwrap(), m);
    assert_eq!(Month::try_from(11i64).unwrap(), m);
    assert_eq!(Month::try_from(11u64).unwrap(), m);
    assert_eq!(Month::try_from(11i128).unwrap(), m);
    assert_eq!(Month::try_from(11u128).unwrap(), m);
    assert_eq!(Month::try_from(11isize).unwrap(), m);
    assert_eq!(Month::try_from(11usize).unwrap(), m);
}

#[test]
fn december() {
    let m = Month::December;
    assert_eq!(m.name(), "December");
    assert_eq!(m.short_name(), "Dec");
    assert_eq!(m.to_string(), "December");
    assert_eq!(format!("{m:#}"), "Dec");
    assert_eq!(m.number(), 12);
    assert_eq!(m.number0(), 11);
    assert_eq!(m.pred(), Some(Month::November));
    assert_eq!(m.succ(), None);
    assert_eq!("December".parse::<Month>().unwrap(), m);
    assert_eq!("Dec".parse::<Month>().unwrap(), m);
    assert_eq!("december".parse::<Month>().unwrap(), m);
    assert_eq!("dec".parse::<Month>().unwrap(), m);
    assert_eq!("DECEMBER".parse::<Month>().unwrap(), m);
    assert_eq!("DEC".parse::<Month>().unwrap(), m);
    assert_eq!(Month::try_from(12i8).unwrap(), m);
    assert_eq!(Month::try_from(12u8).unwrap(), m);
    assert_eq!(Month::try_from(12i16).unwrap(), m);
    assert_eq!(Month::try_from(12u16).unwrap(), m);
    assert_eq!(Month::try_from(12i32).unwrap(), m);
    assert_eq!(Month::try_from(12u32).unwrap(), m);
    assert_eq!(Month::try_from(12i64).unwrap(), m);
    assert_eq!(Month::try_from(12u64).unwrap(), m);
    assert_eq!(Month::try_from(12i128).unwrap(), m);
    assert_eq!(Month::try_from(12u128).unwrap(), m);
    assert_eq!(Month::try_from(12isize).unwrap(), m);
    assert_eq!(Month::try_from(12usize).unwrap(), m);
}

#[rstest]
#[case("Jan.")]
#[case("Sept")]
#[case("Smarch")]
#[case("1")]
#[case(" Jan")]
#[case(" Jan ")]
#[case("Jan ")]
fn parse_error(#[case] s: &str) {
    let r = s.parse::<Month>();
    assert_eq!(r, Err(ParseMonthError));
    assert_eq!(r.unwrap_err().to_string(), "invalid month name");
}

#[test]
fn try_from_zero() {
    let r = Month::try_from(0);
    assert_eq!(r, Err(TryIntoMonthError));
    assert_eq!(
        r.unwrap_err().to_string(),
        "value out of range for month number; must be from 1 through 12"
    );
}

#[test]
fn try_from_thirteen() {
    let r = Month::try_from(13);
    assert_eq!(r, Err(TryIntoMonthError));
    assert_eq!(
        r.unwrap_err().to_string(),
        "value out of range for month number; must be from 1 through 12"
    );
}

#[test]
fn month_iter() {
    let mut iter = MonthIter::new();
    assert_eq!(iter.size_hint(), (12, Some(12)));
    assert_eq!(iter.next(), Some(Month::January));
    assert_eq!(iter.size_hint(), (11, Some(11)));
    assert_eq!(iter.next(), Some(Month::February));
    assert_eq!(iter.size_hint(), (10, Some(10)));
    assert_eq!(iter.next(), Some(Month::March));
    assert_eq!(iter.size_hint(), (9, Some(9)));
    assert_eq!(iter.next(), Some(Month::April));
    assert_eq!(iter.size_hint(), (8, Some(8)));
    assert_eq!(iter.next(), Some(Month::May));
    assert_eq!(iter.size_hint(), (7, Some(7)));
    assert_eq!(iter.next(), Some(Month::June));
    assert_eq!(iter.size_hint(), (6, Some(6)));
    assert_eq!(iter.next(), Some(Month::July));
    assert_eq!(iter.size_hint(), (5, Some(5)));
    assert_eq!(iter.next(), Some(Month::August));
    assert_eq!(iter.size_hint(), (4, Some(4)));
    assert_eq!(iter.next(), Some(Month::September));
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some(Month::October));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some(Month::November));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some(Month::December));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
}

#[test]
fn month_iter_rev() {
    let mut iter = MonthIter::new().rev();
    assert_eq!(iter.size_hint(), (12, Some(12)));
    assert_eq!(iter.next(), Some(Month::December));
    assert_eq!(iter.size_hint(), (11, Some(11)));
    assert_eq!(iter.next(), Some(Month::November));
    assert_eq!(iter.size_hint(), (10, Some(10)));
    assert_eq!(iter.next(), Some(Month::October));
    assert_eq!(iter.size_hint(), (9, Some(9)));
    assert_eq!(iter.next(), Some(Month::September));
    assert_eq!(iter.size_hint(), (8, Some(8)));
    assert_eq!(iter.next(), Some(Month::August));
    assert_eq!(iter.size_hint(), (7, Some(7)));
    assert_eq!(iter.next(), Some(Month::July));
    assert_eq!(iter.size_hint(), (6, Some(6)));
    assert_eq!(iter.next(), Some(Month::June));
    assert_eq!(iter.size_hint(), (5, Some(5)));
    assert_eq!(iter.next(), Some(Month::May));
    assert_eq!(iter.size_hint(), (4, Some(4)));
    assert_eq!(iter.next(), Some(Month::April));
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some(Month::March));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some(Month::February));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some(Month::January));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
}

#[test]
fn pad_display() {
    let m = Month::April;
    assert_eq!(format!("{m:.^10}"), "..April...");
    assert_eq!(format!("{m:.4}"), "Apri");
}
