pub mod ncal;
use super::JulianDayT;

pub const GREGORIAN: JulianDayT = 2299161; // noon on 1582-10-15
pub const UNITED_KINGDOM: JulianDayT = 2361222; // noon on 1752-09-14

/// The smallest Julian day number that can be passed to
/// [`Calendar::reforming()`] without getting an [`Error::InvalidReformation`]
/// error.
///
/// This Julian day number corresponds to the date 0300-03-01 N.S. (0300-02-29
/// O.S.).
pub const MIN_REFORM_JDN: JulianDayT = 1830692;
