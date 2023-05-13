v0.3.0 (in development)
-----------------------

v0.2.0 (2023-05-13)
-------------------
- Added a `chrono` feature for enabling conversions to & from `chrono` types
- Gave `MonthShape` a `calendar()` method for obtaining the associated
  `Calendar`
- Gave `MonthShape` `nth_date()` and `dates()` methods for obtaining `Date`
  objects belonging to the month
- New `Date` methods:
    - `pred()` — returns the previous date
    - `succ()` — returns the next date
    - `later()` — returns an iterator over all later dates
    - `earlier()` — returns an iterator over all earlier dates
    - `and_later()` — returns an iterator that yields the receiver and all
      later dates
    - `and_earlier()` — returns an iterator that yields the receiver and all
      earlier dates
- Added a `Weekday` enum and a `Date::weekday()` method

v0.1.0 (2023-05-02)
-------------------
Initial release
