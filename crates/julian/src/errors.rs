//! Error types
use crate::Month;
use core::num::ParseIntError;
use thiserror::Error;

#[cfg(any(feature = "chrono", feature = "time"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "chrono", feature = "time"))))]
/// Error returned when converting a [`Date`][crate::Date] to a
/// [`chrono::naive::NaiveDate`] or [`time::Date`] fails due to the source date
/// being outside the range of the target type.
#[derive(Clone, Copy, Debug, Default, Error, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[error("date out of range for foreign date type")]
pub struct TryFromDateError;

/// Error returned when parsing a month fails
#[derive(Clone, Copy, Debug, Default, Error, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[error("invalid month name")]
pub struct ParseMonthError;

/// Error returned when converting a number to a month fails
#[derive(Clone, Copy, Debug, Default, Error, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[error("value out of range for month number; must be from 1 through 12")]
pub struct TryIntoMonthError;

/// Error returned when parsing a weekday fails
#[derive(Clone, Copy, Debug, Default, Error, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[error("invalid weekday name")]
pub struct ParseWeekdayError;

/// Error returned when converting a number to a weekday fails
#[derive(Clone, Copy, Debug, Default, Error, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[error("value out of range for weekday number; must be from 1 through 7")]
pub struct TryIntoWeekdayError;

/// Error returned by [`Calendar::reforming()`][crate::Calendar::reforming]
/// when given an invalid reformation date
#[derive(Copy, Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum ReformingError {
    /// Returned if the reformation date would not cause the calendar to skip
    /// forwards
    #[error("reformation date would not cause calendar to advance")]
    InvalidReformation,

    /// Returned if an internal arithmetic operation encounters numeric
    /// overflow or underflow
    #[error("arithmetic overflow/underflow")]
    Arithmetic,
}

/// Error returned by various date-construction methods on invalid input
#[derive(Copy, Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum DateError {
    /// Returned if an internal arithmetic operation encounters numeric
    /// overflow or underflow
    #[error("arithmetic overflow/underflow")]
    Arithmetic,

    /// Returned by [`Calendar::at_ymd()`][crate::Calendar::at_ymd] if the
    /// given day of month value was zero or greater than the last day of the
    /// given month for the given year
    #[error("day {day} is outside of valid range {min_day}-{max_day} for {year:04} {month}")]
    DayOutOfRange {
        /// The year value supplied
        year: i32,
        /// The month value supplied
        month: Month,
        /// The invalid day of month supplied
        day: u32,
        /// The first valid day of the month
        min_day: u32,
        /// The last valid day of the month
        max_day: u32,
    },

    /// Returned by
    /// [`Calendar::at_ordinal_date()`][crate::Calendar::at_ordinal_date] if
    /// the given day of year value was zero or greater than the length of the
    /// given year
    #[error("day-of-year ordinal {ordinal} is outside of valid range 1-{max_ordinal} for year {year:04}")]
    OrdinalOutOfRange {
        /// The year value supplied
        year: i32,
        /// The invalid day of year value supplied
        ordinal: u32,
        /// The maximum valid day of year value
        max_ordinal: u32,
    },

    /// Returned by [`Calendar::at_ymd()`][crate::Calendar::at_ymd] if the
    /// given date was skipped by a calendar reformation
    #[error("date {year:04}-{:02}-{day:02} was skipped by calendar reform", month.number())]
    SkippedDate { year: i32, month: Month, day: u32 },
}

/// Error returned when an internal arithmetic operation encounters numeric
/// overflow or underflow
#[derive(Clone, Copy, Debug, Default, Error, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[error("arithmetic overflow/underflow")]
pub struct ArithmeticError;

impl From<ArithmeticError> for ReformingError {
    fn from(_: ArithmeticError) -> ReformingError {
        ReformingError::Arithmetic
    }
}

impl From<ArithmeticError> for DateError {
    fn from(_: ArithmeticError) -> DateError {
        DateError::Arithmetic
    }
}

/// Error returned by [`Calendar::parse_date()`][crate::Calendar::parse_date]
/// on an invalid input date string
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParseDateError {
    /// Returned if the date specified by the date string does not occur in the
    /// calendar
    #[error("invalid calendar date: {0}")]
    InvalidDate(#[from] DateError),

    /// Returned if the month component of the date string had an invalid
    /// numeric value (i.e., zero or greater than twelve)
    #[error("invalid month number: {value}")]
    InvalidMonth {
        /// The invalid month number
        value: u32,
    },

    /// Returned if the date string had extra trailing characters
    #[error("trailing characters after date")]
    Trailing,

    /// Returned if a non-digit, non-sign character was encountered in the date
    /// string while expecting a signed integer
    #[error("expected signed integer, got {got:?}")]
    InvalidIntStart {
        /// The character encountered
        got: char,
    },

    /// Returned if a non-digit was encountered in the date string while
    /// expecting an unsigned integer
    #[error("expected unsigned integer, got {got:?}")]
    InvalidUIntStart {
        /// The non-digit encountered
        got: char,
    },

    /// Returned if the end of the date string was reached while expecting an
    /// integer
    #[error("expected integer, got end of input")]
    EmptyInt,

    /// Returned if a specific character was expected but a different one was
    /// encountered instead
    #[error("expected {expected:?}, got {got:?}")]
    UnexpectedChar {
        /// The expected character
        expected: char,

        /// The character encountered
        got: char,
    },

    /// Returned if a specific character was expected but the end of the date
    /// string was reached instead
    #[error("expected {expected:?}, got end of input")]
    UnexpectedEnd {
        /// The expected character
        expected: char,
    },

    /// Returned if a numeric component of the date string could not be parsed
    /// as an integer
    #[error("numeric parse error: {0}")]
    ParseInt(#[from] ParseIntError),
}
