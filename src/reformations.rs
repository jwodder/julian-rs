// TODO: Docs
pub mod ncal;
use super::JulianDayT;

/// The Julian day number of the date at which the Gregorian Reformation was
/// first put into effect (1582-10-15, following 1582-10-04 O.S.)
pub const GREGORIAN: JulianDayT = 2299161;

/// The Julian day number of the date at which the United Kingdom and its
/// colonies first used the Gregorian Calendar (1752-09-14, following
/// 1752-09-02 O.S.)
pub const UNITED_KINGDOM: JulianDayT = 2361222;

/// The smallest Julian day number that can be passed to
/// [`Calendar::reforming()`][crate::Calendar::reforming] without getting a
/// [`ReformingError`][crate::ReformingError] error.
///
/// This Julian day number corresponds to the date 0300-03-01 N.S. (0300-02-29
/// O.S.).
pub const MIN_REFORM_JDN: JulianDayT = 1830692;
