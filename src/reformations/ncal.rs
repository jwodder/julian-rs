//! Constants for per-country reformation dates as per `ncal(1)`
//!
//! The dates in this module come from [the Debian version of `ncal.c` as of
//! 2023-04-26][src], so blame Debian for any historical inaccuracies.
//!
//! [src]: https://salsa.debian.org/meskes/bsdmainutils/-/blob/70ff77b0f084de4a14d79bed935e1958020f43dc/usr.bin/ncal/ncal.c
use crate::JulianDayT;

/// The Julian day of the date at which Albania first used the Gregorian
/// calendar (1912-12-14, following 1912-11-30 O.S.)
pub const ALBANIA: JulianDayT = 2419751;

/// The Julian day of the date at which Australia first used the Gregorian
/// calendar (1752-09-14, following 1752-09-02 O.S.)
pub const AUSTRALIA: JulianDayT = 2361222;

/// The Julian day of the date at which Austria first used the Gregorian
/// calendar (1583-10-16, following 1583-10-05 O.S.)
pub const AUSTRIA: JulianDayT = 2299527;

/// The Julian day of the date at which Belgium first used the Gregorian
/// calendar (1582-12-25, following 1582-12-14 O.S.)
pub const BELGIUM: JulianDayT = 2299232;

/// The Julian day of the date at which Bulgaria first used the Gregorian
/// calendar (1916-04-15, following 1916-03-31 O.S.)
pub const BULGARIA: JulianDayT = 2420969;

/// The Julian day of the date at which Canada first used the Gregorian
/// calendar (1752-09-14, following 1752-09-02 O.S.)
pub const CANADA: JulianDayT = 2361222;

/// The Julian day of the date at which China first used the Gregorian calendar
/// (1912-01-01, following 1911-12-18 O.S.)
pub const CHINA: JulianDayT = 2419403;

/// The Julian day of the date at which the Czech Republic first used the
/// Gregorian calendar (1584-01-19, following 1584-01-06 O.S.)
pub const CZECH_REPUBLIC: JulianDayT = 2299622;

/// The Julian day of the date at which Denmark first used the Gregorian
/// calendar (1700-03-01, following 1700-02-18 O.S.)
pub const DENMARK: JulianDayT = 2342032;

/// The Julian day of the date at which Finland first used the Gregorian
/// calendar (1753-03-01, following 1753-02-17 O.S.)
pub const FINLAND: JulianDayT = 2361390;

/// The Julian day of the date at which France first used the Gregorian
/// calendar (1582-12-20, following 1582-12-09 O.S.)
pub const FRANCE: JulianDayT = 2299227;

/// The Julian day of the date at which Germany first used the Gregorian
/// calendar (1700-03-01, following 1700-02-18 O.S.)
pub const GERMANY: JulianDayT = 2342032;

/// The Julian day of the date at which Greece first used the Gregorian
/// calendar (1924-03-24, following 1924-03-09 O.S.)
pub const GREECE: JulianDayT = 2423869;

/// The Julian day of the date at which Hungary first used the Gregorian
/// calendar (1587-11-01, following 1587-10-21 O.S.)
pub const HUNGARY: JulianDayT = 2301004;

/// The Julian day of the date at which Iceland first used the Gregorian
/// calendar (1700-11-28, following 1700-11-16 O.S.)
pub const ICELAND: JulianDayT = 2342304;

/// The Julian day of the date at which Italy first used the Gregorian calendar
/// (1582-10-15, following 1582-10-04 O.S.)
pub const ITALY: JulianDayT = 2299161;

/// The Julian day of the date at which Japan first used the Gregorian calendar
/// (1919-01-01, following 1918-12-18 O.S.)
pub const JAPAN: JulianDayT = 2421960;

/// The Julian day of the date at which Latvia first used the Gregorian
/// calendar (1918-02-15, following 1918-02-01 O.S.)
pub const LATVIA: JulianDayT = 2421640;

/// The Julian day of the date at which Lithuania first used the Gregorian
/// calendar (1918-02-15, following 1918-02-01 O.S.)
pub const LITHUANIA: JulianDayT = 2421640;

/// The Julian day of the date at which Luxembourg first used the Gregorian
/// calendar (1582-12-25, following 1582-12-14 O.S.)
pub const LUXEMBOURG: JulianDayT = 2299232;

/// The Julian day of the date at which Netherlands first used the Gregorian
/// calendar (1582-12-25, following 1582-12-14 O.S.)
pub const NETHERLANDS: JulianDayT = 2299232;

/// The Julian day of the date at which Norway first used the Gregorian
/// calendar (1700-03-01, following 1700-02-18 O.S.)
pub const NORWAY: JulianDayT = 2342032;

/// The Julian day of the date at which Poland first used the Gregorian
/// calendar (1582-10-15, following 1582-10-04 O.S.)
pub const POLAND: JulianDayT = 2299161;

/// The Julian day of the date at which Portugal first used the Gregorian
/// calendar (1582-10-15, following 1582-10-04 O.S.)
pub const PORTUGAL: JulianDayT = 2299161;

/// The Julian day of the date at which Romania first used the Gregorian
/// calendar (1919-04-16, following 1919-03-31 O.S.)
pub const ROMANIA: JulianDayT = 2422065;

/// The Julian day of the date at which Russia first used the Gregorian
/// calendar (1918-02-16, following 1918-01-31 O.S.)
pub const RUSSIA: JulianDayT = 2421641;

/// The Julian day of the date at which Slovenia first used the Gregorian
/// calendar (1919-03-20, following 1919-03-04 O.S.)
pub const SLOVENIA: JulianDayT = 2422038;

/// The Julian day of the date at which Spain first used the Gregorian calendar
/// (1582-10-15, following 1582-10-04 O.S.)
pub const SPAIN: JulianDayT = 2299161;

/// The Julian day of the date at which Sweden first used the Gregorian
/// calendar (1753-03-01, following 1753-02-17 O.S.)
pub const SWEDEN: JulianDayT = 2361390;

/// The Julian day of the date at which Switzerland first used the Gregorian
/// calendar (1655-03-11, following 1655-02-28 O.S.)
pub const SWITZERLAND: JulianDayT = 2325606;

/// The Julian day of the date at which Turkey first used the Gregorian
/// calendar (1927-01-01, following 1926-12-18 O.S.)
pub const TURKEY: JulianDayT = 2424882;

/// The Julian day of the date at which the United Kingdom first used the
/// Gregorian calendar (1752-09-14, following 1752-09-02 O.S.)
pub const UNITED_KINGDOM: JulianDayT = 2361222;

/// The Julian day of the date at which the United States first used the
/// Gregorian calendar (1752-09-14, following 1752-09-02 O.S.)
pub const UNITED_STATES: JulianDayT = 2361222;

/// The Julian day of the date at which Yugoslavia first used the Gregorian
/// calendar (1919-03-20, following 1919-03-04 O.S.)
pub const YUGOSLAVIA: JulianDayT = 2422038;
