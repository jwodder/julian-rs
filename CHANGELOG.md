v0.2.0 (in development)
-----------------------
- Added a `chrono` feature for enabling conversions to & from `chrono` types
- Gave `MonthShape` a `calendar()` method for obtaining the associated
  `Calendar`
- Gave `MonthShape` `nth_date()` and `dates()` methods for obtaining `Date`
  objects belonging to the month
- Gave `Date` `pred()` and `succ()` methods for getting the previous & next
  dates
- Added a `Weekday` enum and a `Date::weekday()` method

v0.1.0 (2023-05-02)
-------------------
Initial release
