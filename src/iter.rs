//! Iterator types
use crate::{Date, Month, MonthShape};
use std::iter::FusedIterator;
use std::ops::RangeInclusive;

/// An iterator over the days of a month.
///
/// A `Days` instance can be acquired by calling
/// [`MonthShape::days()`][crate::MonthShape::days].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Days {
    month_shape: MonthShape,
    inner: RangeInclusive<u32>,
}

impl Days {
    pub(crate) const fn new(month_shape: MonthShape) -> Self {
        Days {
            month_shape,
            inner: 1..=(month_shape.len()),
        }
    }
}

impl Iterator for Days {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.month_shape.nth_day(self.inner.next()?)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl FusedIterator for Days {}

impl ExactSizeIterator for Days {}

impl DoubleEndedIterator for Days {
    fn next_back(&mut self) -> Option<u32> {
        self.month_shape.nth_day(self.inner.next_back()?)
    }
}

/// An iterator over the [`Date`s][crate::Date] within a month.
///
/// A `Dates` instance can be acquired by calling
/// [`MonthShape::dates()`][MonthShape::dates].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dates {
    month_shape: MonthShape,
    inner: RangeInclusive<u32>,
}

impl Dates {
    pub(crate) const fn new(month_shape: MonthShape) -> Self {
        Dates {
            month_shape,
            inner: 1..=(month_shape.len()),
        }
    }
}

impl Iterator for Dates {
    type Item = Date;

    fn next(&mut self) -> Option<Date> {
        self.month_shape.nth_date(self.inner.next()?)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl FusedIterator for Dates {}

impl ExactSizeIterator for Dates {}

impl DoubleEndedIterator for Dates {
    fn next_back(&mut self) -> Option<Date> {
        self.month_shape.nth_date(self.inner.next_back()?)
    }
}

/// Iterator over calendar dates later than a given date.
///
/// A `Later` instance is acquired by calling
/// [`Date::later()`][crate::Date::later].
///
/// A `Later` iterator will stop yielding after it reaches 5874898-06-03 N.S.
/// (5874777-10-17 O.S.).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Later {
    date: Option<Date>,
}

impl Later {
    pub(crate) const fn new(date: Date) -> Later {
        Later { date: Some(date) }
    }
}

impl Iterator for Later {
    type Item = Date;

    fn next(&mut self) -> Option<Date> {
        self.date = self.date.and_then(|d| d.succ());
        self.date
    }
}

impl FusedIterator for Later {}

/// Iterator over calendar dates earlier than a given date.
///
/// An `Earlier` instance is acquired by calling
/// [`Date::earlier()`][crate::Date::earlier].
///
/// An `Earlier` iterator will stop yielding after it reaches -5884323-05-15
/// N.S. (-5884202-03-16 O.S.).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Earlier {
    date: Option<Date>,
}

impl Earlier {
    pub(crate) const fn new(date: Date) -> Earlier {
        Earlier { date: Some(date) }
    }
}

impl Iterator for Earlier {
    type Item = Date;

    fn next(&mut self) -> Option<Date> {
        self.date = self.date.and_then(|d| d.pred());
        self.date
    }
}

impl FusedIterator for Earlier {}

/// Iterator over calendar dates equal to or later than a given date.
///
/// An `AndLater` instance is acquired by calling
/// [`Date::and_later()`][crate::Date::and_later].
///
/// An `AndLater` iterator will stop yielding after it reaches 5874898-06-03
/// N.S. (5874777-10-17 O.S.).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AndLater {
    date: Option<Date>,
}

impl AndLater {
    pub(crate) const fn new(date: Date) -> AndLater {
        AndLater { date: Some(date) }
    }
}

impl Iterator for AndLater {
    type Item = Date;

    fn next(&mut self) -> Option<Date> {
        let date = self.date?;
        self.date = date.succ();
        Some(date)
    }
}

impl FusedIterator for AndLater {}

/// Iterator over calendar dates equal to or earlier than a given date.
///
/// An `AndEarlier` instance is acquired by calling
/// [`Date::and_earlier()`][crate::Date::and_earlier].
///
/// An `AndEarlier` iterator will stop yielding after it reaches -5884323-05-15
/// N.S. (-5884202-03-16 O.S.).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AndEarlier {
    date: Option<Date>,
}

impl AndEarlier {
    pub(crate) const fn new(date: Date) -> AndEarlier {
        AndEarlier { date: Some(date) }
    }
}

impl Iterator for AndEarlier {
    type Item = Date;

    fn next(&mut self) -> Option<Date> {
        let date = self.date?;
        self.date = date.pred();
        Some(date)
    }
}

impl FusedIterator for AndEarlier {}

/// Iterator over the months of the year in order.
///
/// # Example
///
/// ```
/// use julian::{Month, iter::MonthIter};
///
/// let mut iter = MonthIter::new();
/// assert_eq!(iter.next(), Some(Month::January));
/// assert_eq!(iter.next(), Some(Month::February));
/// assert_eq!(iter.next(), Some(Month::March));
/// assert_eq!(iter.next(), Some(Month::April));
/// assert_eq!(iter.next(), Some(Month::May));
/// assert_eq!(iter.next(), Some(Month::June));
/// assert_eq!(iter.next(), Some(Month::July));
/// assert_eq!(iter.next(), Some(Month::August));
/// assert_eq!(iter.next(), Some(Month::September));
/// assert_eq!(iter.next(), Some(Month::October));
/// assert_eq!(iter.next(), Some(Month::November));
/// assert_eq!(iter.next(), Some(Month::December));
/// assert_eq!(iter.next(), None);
/// ```
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct MonthIter(RangeInclusive<u16>);

impl MonthIter {
    /// Construct a new `MonthIter`
    pub const fn new() -> MonthIter {
        MonthIter(1..=12)
    }
}

impl Default for MonthIter {
    fn default() -> MonthIter {
        MonthIter::new()
    }
}

impl Iterator for MonthIter {
    type Item = Month;

    fn next(&mut self) -> Option<Month> {
        Some(
            u32::from(self.0.next()?)
                .try_into()
                .expect("inner iterator item should be valid month number"),
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl FusedIterator for MonthIter {}

impl ExactSizeIterator for MonthIter {}

impl DoubleEndedIterator for MonthIter {
    fn next_back(&mut self) -> Option<Month> {
        Some(
            u32::from(self.0.next_back()?)
                .try_into()
                .expect("inner iterator item should be valid month number"),
        )
    }
}
