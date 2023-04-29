//! Constants for per-country reformation dates as per `ncal(1)`
//!
//! The dates in this module come from [the Debian version of `ncal.c` as of
//! 2023-04-26][src], so blame Debian for any historical inaccuracies.
//!
//! [src]: https://salsa.debian.org/meskes/bsdmainutils/-/blob/70ff77b0f084de4a14d79bed935e1958020f43dc/usr.bin/ncal/ncal.c
use crate::Jdnum;

/// The Julian day number of the date at which Albania first used the Gregorian
/// calendar (1912-12-14, following 1912-11-30 O.S.)
pub const ALBANIA: Jdnum = 2419751;

/// The Julian day number of the date at which Australia first used the
/// Gregorian calendar (1752-09-14, following 1752-09-02 O.S.)
pub const AUSTRALIA: Jdnum = 2361222;

/// The Julian day number of the date at which Austria first used the Gregorian
/// calendar (1583-10-16, following 1583-10-05 O.S.)
pub const AUSTRIA: Jdnum = 2299527;

/// The Julian day number of the date at which Belgium first used the Gregorian
/// calendar (1582-12-25, following 1582-12-14 O.S.)
pub const BELGIUM: Jdnum = 2299232;

/// The Julian day number of the date at which Bulgaria first used the
/// Gregorian calendar (1916-04-14, following 1916-03-31 O.S.)
pub const BULGARIA: Jdnum = 2420968;

/// The Julian day number of the date at which Canada first used the Gregorian
/// calendar (1752-09-14, following 1752-09-02 O.S.)
pub const CANADA: Jdnum = 2361222;

/// The Julian day number of the date at which China first used the Gregorian
/// calendar (1912-01-01, following 1911-12-18 O.S.)
pub const CHINA: Jdnum = 2419403;

/// The Julian day number of the date at which Czech Republic first used the
/// Gregorian calendar (1584-01-17, following 1584-01-06 O.S.)
pub const CZECH_REPUBLIC: Jdnum = 2299620;

/// The Julian day number of the date at which Denmark first used the Gregorian
/// calendar (1700-03-01, following 1700-02-18 O.S.)
pub const DENMARK: Jdnum = 2342032;

/// The Julian day number of the date at which Finland first used the Gregorian
/// calendar (1753-03-01, following 1753-02-17 O.S.)
pub const FINLAND: Jdnum = 2361390;

/// The Julian day number of the date at which France first used the Gregorian
/// calendar (1582-12-20, following 1582-12-09 O.S.)
pub const FRANCE: Jdnum = 2299227;

/// The Julian day number of the date at which Germany first used the Gregorian
/// calendar (1700-03-01, following 1700-02-18 O.S.)
pub const GERMANY: Jdnum = 2342032;

/// The Julian day number of the date at which Greece first used the Gregorian
/// calendar (1924-03-23, following 1924-03-09 O.S.)
pub const GREECE: Jdnum = 2423868;

/// The Julian day number of the date at which Hungary first used the Gregorian
/// calendar (1587-11-01, following 1587-10-21 O.S.)
pub const HUNGARY: Jdnum = 2301004;

/// The Julian day number of the date at which Iceland first used the Gregorian
/// calendar (1700-11-28, following 1700-11-16 O.S.)
pub const ICELAND: Jdnum = 2342304;

/// The Julian day number of the date at which Italy first used the Gregorian
/// calendar (1582-10-15, following 1582-10-04 O.S.)
pub const ITALY: Jdnum = 2299161;

/// The Julian day number of the date at which Japan first used the Gregorian
/// calendar (1919-01-01, following 1918-12-18 O.S.)
pub const JAPAN: Jdnum = 2421960;

/// The Julian day number of the date at which Latvia first used the Gregorian
/// calendar (1918-02-15, following 1918-02-01 O.S.)
pub const LATVIA: Jdnum = 2421640;

/// The Julian day number of the date at which Lithuania first used the
/// Gregorian calendar (1918-02-15, following 1918-02-01 O.S.)
pub const LITHUANIA: Jdnum = 2421640;

/// The Julian day number of the date at which Luxembourg first used the
/// Gregorian calendar (1582-12-25, following 1582-12-14 O.S.)
pub const LUXEMBOURG: Jdnum = 2299232;

/// The Julian day number of the date at which Netherlands first used the
/// Gregorian calendar (1582-12-25, following 1582-12-14 O.S.)
pub const NETHERLANDS: Jdnum = 2299232;

/// The Julian day number of the date at which Norway first used the Gregorian
/// calendar (1700-03-01, following 1700-02-18 O.S.)
pub const NORWAY: Jdnum = 2342032;

/// The Julian day number of the date at which Poland first used the Gregorian
/// calendar (1582-10-15, following 1582-10-04 O.S.)
pub const POLAND: Jdnum = 2299161;

/// The Julian day number of the date at which Portugal first used the
/// Gregorian calendar (1582-10-15, following 1582-10-04 O.S.)
pub const PORTUGAL: Jdnum = 2299161;

/// The Julian day number of the date at which Romania first used the Gregorian
/// calendar (1919-04-14, following 1919-03-31 O.S.)
pub const ROMANIA: Jdnum = 2422063;

/// The Julian day number of the date at which Russia first used the Gregorian
/// calendar (1918-02-14, following 1918-01-31 O.S.)
pub const RUSSIA: Jdnum = 2421639;

/// The Julian day number of the date at which Slovenia first used the
/// Gregorian calendar (1919-03-18, following 1919-03-04 O.S.)
pub const SLOVENIA: Jdnum = 2422036;

/// The Julian day number of the date at which Spain first used the Gregorian
/// calendar (1582-10-15, following 1582-10-04 O.S.)
pub const SPAIN: Jdnum = 2299161;

/// The Julian day number of the date at which Sweden first used the Gregorian
/// calendar (1753-03-01, following 1753-02-17 O.S.)
pub const SWEDEN: Jdnum = 2361390;

/// The Julian day number of the date at which Switzerland first used the
/// Gregorian calendar (1655-03-11, following 1655-02-28 O.S.)
pub const SWITZERLAND: Jdnum = 2325606;

/// The Julian day number of the date at which Turkey first used the Gregorian
/// calendar (1927-01-01, following 1926-12-18 O.S.)
pub const TURKEY: Jdnum = 2424882;

/// The Julian day number of the date at which United Kingdom first used the
/// Gregorian calendar (1752-09-14, following 1752-09-02 O.S.)
pub const UNITED_KINGDOM: Jdnum = 2361222;

/// The Julian day number of the date at which United States first used the
/// Gregorian calendar (1752-09-14, following 1752-09-02 O.S.)
pub const UNITED_STATES: Jdnum = 2361222;

/// The Julian day number of the date at which Yugoslavia first used the
/// Gregorian calendar (1919-03-18, following 1919-03-04 O.S.)
pub const YUGOSLAVIA: Jdnum = 2422036;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Calendar, Month};

    #[test]
    fn albania() {
        let cal = Calendar::reforming(ALBANIA).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1912);
        assert_eq!(last_julian.month(), Month::November);
        assert_eq!(last_julian.day(), 30);
        assert_eq!(last_julian.ordinal(), 335);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1912);
        assert_eq!(first_greg.month(), Month::December);
        assert_eq!(first_greg.day(), 14);
        assert_eq!(first_greg.ordinal(), 336);
    }

    #[test]
    fn australia() {
        let cal = Calendar::reforming(AUSTRALIA).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1752);
        assert_eq!(last_julian.month(), Month::September);
        assert_eq!(last_julian.day(), 2);
        assert_eq!(last_julian.ordinal(), 246);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1752);
        assert_eq!(first_greg.month(), Month::September);
        assert_eq!(first_greg.day(), 14);
        assert_eq!(first_greg.ordinal(), 247);
    }

    #[test]
    fn austria() {
        let cal = Calendar::reforming(AUSTRIA).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1583);
        assert_eq!(last_julian.month(), Month::October);
        assert_eq!(last_julian.day(), 5);
        assert_eq!(last_julian.ordinal(), 278);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1583);
        assert_eq!(first_greg.month(), Month::October);
        assert_eq!(first_greg.day(), 16);
        assert_eq!(first_greg.ordinal(), 279);
    }

    #[test]
    fn belgium() {
        let cal = Calendar::reforming(BELGIUM).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1582);
        assert_eq!(last_julian.month(), Month::December);
        assert_eq!(last_julian.day(), 14);
        assert_eq!(last_julian.ordinal(), 348);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1582);
        assert_eq!(first_greg.month(), Month::December);
        assert_eq!(first_greg.day(), 25);
        assert_eq!(first_greg.ordinal(), 349);
    }

    #[test]
    fn bulgaria() {
        let cal = Calendar::reforming(BULGARIA).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1916);
        assert_eq!(last_julian.month(), Month::March);
        assert_eq!(last_julian.day(), 31);
        assert_eq!(last_julian.ordinal(), 91);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1916);
        assert_eq!(first_greg.month(), Month::April);
        assert_eq!(first_greg.day(), 14);
        assert_eq!(first_greg.ordinal(), 92);
    }

    #[test]
    fn canada() {
        let cal = Calendar::reforming(CANADA).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1752);
        assert_eq!(last_julian.month(), Month::September);
        assert_eq!(last_julian.day(), 2);
        assert_eq!(last_julian.ordinal(), 246);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1752);
        assert_eq!(first_greg.month(), Month::September);
        assert_eq!(first_greg.day(), 14);
        assert_eq!(first_greg.ordinal(), 247);
    }

    #[test]
    fn china() {
        let cal = Calendar::reforming(CHINA).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1911);
        assert_eq!(last_julian.month(), Month::December);
        assert_eq!(last_julian.day(), 18);
        assert_eq!(last_julian.ordinal(), 352);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1912);
        assert_eq!(first_greg.month(), Month::January);
        assert_eq!(first_greg.day(), 1);
        assert_eq!(first_greg.ordinal(), 1);
    }

    #[test]
    fn czech_republic() {
        let cal = Calendar::reforming(CZECH_REPUBLIC).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1584);
        assert_eq!(last_julian.month(), Month::January);
        assert_eq!(last_julian.day(), 6);
        assert_eq!(last_julian.ordinal(), 6);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1584);
        assert_eq!(first_greg.month(), Month::January);
        assert_eq!(first_greg.day(), 17);
        assert_eq!(first_greg.ordinal(), 7);
    }

    #[test]
    fn denmark() {
        let cal = Calendar::reforming(DENMARK).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1700);
        assert_eq!(last_julian.month(), Month::February);
        assert_eq!(last_julian.day(), 18);
        assert_eq!(last_julian.ordinal(), 49);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1700);
        assert_eq!(first_greg.month(), Month::March);
        assert_eq!(first_greg.day(), 1);
        assert_eq!(first_greg.ordinal(), 50);
    }

    #[test]
    fn finland() {
        let cal = Calendar::reforming(FINLAND).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1753);
        assert_eq!(last_julian.month(), Month::February);
        assert_eq!(last_julian.day(), 17);
        assert_eq!(last_julian.ordinal(), 48);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1753);
        assert_eq!(first_greg.month(), Month::March);
        assert_eq!(first_greg.day(), 1);
        assert_eq!(first_greg.ordinal(), 49);
    }

    #[test]
    fn france() {
        let cal = Calendar::reforming(FRANCE).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1582);
        assert_eq!(last_julian.month(), Month::December);
        assert_eq!(last_julian.day(), 9);
        assert_eq!(last_julian.ordinal(), 343);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1582);
        assert_eq!(first_greg.month(), Month::December);
        assert_eq!(first_greg.day(), 20);
        assert_eq!(first_greg.ordinal(), 344);
    }

    #[test]
    fn germany() {
        let cal = Calendar::reforming(GERMANY).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1700);
        assert_eq!(last_julian.month(), Month::February);
        assert_eq!(last_julian.day(), 18);
        assert_eq!(last_julian.ordinal(), 49);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1700);
        assert_eq!(first_greg.month(), Month::March);
        assert_eq!(first_greg.day(), 1);
        assert_eq!(first_greg.ordinal(), 50);
    }

    #[test]
    fn greece() {
        let cal = Calendar::reforming(GREECE).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1924);
        assert_eq!(last_julian.month(), Month::March);
        assert_eq!(last_julian.day(), 9);
        assert_eq!(last_julian.ordinal(), 69);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1924);
        assert_eq!(first_greg.month(), Month::March);
        assert_eq!(first_greg.day(), 23);
        assert_eq!(first_greg.ordinal(), 70);
    }

    #[test]
    fn hungary() {
        let cal = Calendar::reforming(HUNGARY).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1587);
        assert_eq!(last_julian.month(), Month::October);
        assert_eq!(last_julian.day(), 21);
        assert_eq!(last_julian.ordinal(), 294);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1587);
        assert_eq!(first_greg.month(), Month::November);
        assert_eq!(first_greg.day(), 1);
        assert_eq!(first_greg.ordinal(), 295);
    }

    #[test]
    fn iceland() {
        let cal = Calendar::reforming(ICELAND).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1700);
        assert_eq!(last_julian.month(), Month::November);
        assert_eq!(last_julian.day(), 16);
        assert_eq!(last_julian.ordinal(), 321);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1700);
        assert_eq!(first_greg.month(), Month::November);
        assert_eq!(first_greg.day(), 28);
        assert_eq!(first_greg.ordinal(), 322);
    }

    #[test]
    fn italy() {
        let cal = Calendar::reforming(ITALY).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1582);
        assert_eq!(last_julian.month(), Month::October);
        assert_eq!(last_julian.day(), 4);
        assert_eq!(last_julian.ordinal(), 277);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1582);
        assert_eq!(first_greg.month(), Month::October);
        assert_eq!(first_greg.day(), 15);
        assert_eq!(first_greg.ordinal(), 278);
    }

    #[test]
    fn japan() {
        let cal = Calendar::reforming(JAPAN).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1918);
        assert_eq!(last_julian.month(), Month::December);
        assert_eq!(last_julian.day(), 18);
        assert_eq!(last_julian.ordinal(), 352);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1919);
        assert_eq!(first_greg.month(), Month::January);
        assert_eq!(first_greg.day(), 1);
        assert_eq!(first_greg.ordinal(), 1);
    }

    #[test]
    fn latvia() {
        let cal = Calendar::reforming(LATVIA).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1918);
        assert_eq!(last_julian.month(), Month::February);
        assert_eq!(last_julian.day(), 1);
        assert_eq!(last_julian.ordinal(), 32);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1918);
        assert_eq!(first_greg.month(), Month::February);
        assert_eq!(first_greg.day(), 15);
        assert_eq!(first_greg.ordinal(), 33);
    }

    #[test]
    fn lithuania() {
        let cal = Calendar::reforming(LITHUANIA).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1918);
        assert_eq!(last_julian.month(), Month::February);
        assert_eq!(last_julian.day(), 1);
        assert_eq!(last_julian.ordinal(), 32);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1918);
        assert_eq!(first_greg.month(), Month::February);
        assert_eq!(first_greg.day(), 15);
        assert_eq!(first_greg.ordinal(), 33);
    }

    #[test]
    fn luxembourg() {
        let cal = Calendar::reforming(LUXEMBOURG).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1582);
        assert_eq!(last_julian.month(), Month::December);
        assert_eq!(last_julian.day(), 14);
        assert_eq!(last_julian.ordinal(), 348);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1582);
        assert_eq!(first_greg.month(), Month::December);
        assert_eq!(first_greg.day(), 25);
        assert_eq!(first_greg.ordinal(), 349);
    }

    #[test]
    fn netherlands() {
        let cal = Calendar::reforming(NETHERLANDS).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1582);
        assert_eq!(last_julian.month(), Month::December);
        assert_eq!(last_julian.day(), 14);
        assert_eq!(last_julian.ordinal(), 348);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1582);
        assert_eq!(first_greg.month(), Month::December);
        assert_eq!(first_greg.day(), 25);
        assert_eq!(first_greg.ordinal(), 349);
    }

    #[test]
    fn norway() {
        let cal = Calendar::reforming(NORWAY).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1700);
        assert_eq!(last_julian.month(), Month::February);
        assert_eq!(last_julian.day(), 18);
        assert_eq!(last_julian.ordinal(), 49);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1700);
        assert_eq!(first_greg.month(), Month::March);
        assert_eq!(first_greg.day(), 1);
        assert_eq!(first_greg.ordinal(), 50);
    }

    #[test]
    fn poland() {
        let cal = Calendar::reforming(POLAND).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1582);
        assert_eq!(last_julian.month(), Month::October);
        assert_eq!(last_julian.day(), 4);
        assert_eq!(last_julian.ordinal(), 277);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1582);
        assert_eq!(first_greg.month(), Month::October);
        assert_eq!(first_greg.day(), 15);
        assert_eq!(first_greg.ordinal(), 278);
    }

    #[test]
    fn portugal() {
        let cal = Calendar::reforming(PORTUGAL).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1582);
        assert_eq!(last_julian.month(), Month::October);
        assert_eq!(last_julian.day(), 4);
        assert_eq!(last_julian.ordinal(), 277);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1582);
        assert_eq!(first_greg.month(), Month::October);
        assert_eq!(first_greg.day(), 15);
        assert_eq!(first_greg.ordinal(), 278);
    }

    #[test]
    fn romania() {
        let cal = Calendar::reforming(ROMANIA).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1919);
        assert_eq!(last_julian.month(), Month::March);
        assert_eq!(last_julian.day(), 31);
        assert_eq!(last_julian.ordinal(), 90);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1919);
        assert_eq!(first_greg.month(), Month::April);
        assert_eq!(first_greg.day(), 14);
        assert_eq!(first_greg.ordinal(), 91);
    }

    #[test]
    fn russia() {
        let cal = Calendar::reforming(RUSSIA).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1918);
        assert_eq!(last_julian.month(), Month::January);
        assert_eq!(last_julian.day(), 31);
        assert_eq!(last_julian.ordinal(), 31);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1918);
        assert_eq!(first_greg.month(), Month::February);
        assert_eq!(first_greg.day(), 14);
        assert_eq!(first_greg.ordinal(), 32);
    }

    #[test]
    fn slovenia() {
        let cal = Calendar::reforming(SLOVENIA).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1919);
        assert_eq!(last_julian.month(), Month::March);
        assert_eq!(last_julian.day(), 4);
        assert_eq!(last_julian.ordinal(), 63);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1919);
        assert_eq!(first_greg.month(), Month::March);
        assert_eq!(first_greg.day(), 18);
        assert_eq!(first_greg.ordinal(), 64);
    }

    #[test]
    fn spain() {
        let cal = Calendar::reforming(SPAIN).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1582);
        assert_eq!(last_julian.month(), Month::October);
        assert_eq!(last_julian.day(), 4);
        assert_eq!(last_julian.ordinal(), 277);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1582);
        assert_eq!(first_greg.month(), Month::October);
        assert_eq!(first_greg.day(), 15);
        assert_eq!(first_greg.ordinal(), 278);
    }

    #[test]
    fn sweden() {
        let cal = Calendar::reforming(SWEDEN).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1753);
        assert_eq!(last_julian.month(), Month::February);
        assert_eq!(last_julian.day(), 17);
        assert_eq!(last_julian.ordinal(), 48);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1753);
        assert_eq!(first_greg.month(), Month::March);
        assert_eq!(first_greg.day(), 1);
        assert_eq!(first_greg.ordinal(), 49);
    }

    #[test]
    fn switzerland() {
        let cal = Calendar::reforming(SWITZERLAND).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1655);
        assert_eq!(last_julian.month(), Month::February);
        assert_eq!(last_julian.day(), 28);
        assert_eq!(last_julian.ordinal(), 59);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1655);
        assert_eq!(first_greg.month(), Month::March);
        assert_eq!(first_greg.day(), 11);
        assert_eq!(first_greg.ordinal(), 60);
    }

    #[test]
    fn turkey() {
        let cal = Calendar::reforming(TURKEY).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1926);
        assert_eq!(last_julian.month(), Month::December);
        assert_eq!(last_julian.day(), 18);
        assert_eq!(last_julian.ordinal(), 352);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1927);
        assert_eq!(first_greg.month(), Month::January);
        assert_eq!(first_greg.day(), 1);
        assert_eq!(first_greg.ordinal(), 1);
    }

    #[test]
    fn united_kingdom() {
        let cal = Calendar::reforming(UNITED_KINGDOM).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1752);
        assert_eq!(last_julian.month(), Month::September);
        assert_eq!(last_julian.day(), 2);
        assert_eq!(last_julian.ordinal(), 246);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1752);
        assert_eq!(first_greg.month(), Month::September);
        assert_eq!(first_greg.day(), 14);
        assert_eq!(first_greg.ordinal(), 247);
    }

    #[test]
    fn united_states() {
        let cal = Calendar::reforming(UNITED_STATES).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1752);
        assert_eq!(last_julian.month(), Month::September);
        assert_eq!(last_julian.day(), 2);
        assert_eq!(last_julian.ordinal(), 246);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1752);
        assert_eq!(first_greg.month(), Month::September);
        assert_eq!(first_greg.day(), 14);
        assert_eq!(first_greg.ordinal(), 247);
    }

    #[test]
    fn yugoslavia() {
        let cal = Calendar::reforming(YUGOSLAVIA).unwrap();
        let last_julian = cal.last_julian_date().unwrap();
        assert_eq!(last_julian.year(), 1919);
        assert_eq!(last_julian.month(), Month::March);
        assert_eq!(last_julian.day(), 4);
        assert_eq!(last_julian.ordinal(), 63);
        let first_greg = cal.first_gregorian_date().unwrap();
        assert_eq!(first_greg.year(), 1919);
        assert_eq!(first_greg.month(), Month::March);
        assert_eq!(first_greg.day(), 18);
        assert_eq!(first_greg.ordinal(), 64);
    }
}
