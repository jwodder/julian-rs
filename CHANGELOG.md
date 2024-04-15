v0.6.0 (in development)
-----------------------
- Increased MSRV to 1.67

v0.5.0 (2023-12-22)
-------------------
- **Breaking:** Moved error types to `errors` module
- **Breaking:** Moved iterator types to `iter` module
- Most non-trait methods are now `const`

v0.4.0 (2023-11-22)
-------------------
- Increased MSRV to 1.65
- `Days`, `Dates`, `Later`, `Earlier`, `AndLater`, and `AndEarlier` now
  implement `Clone`, `Debug`, `Eq`, and `PartialEq`

v0.3.1 (2023-05-15)
-------------------
- Lowered MSRV to 1.62

v0.3.0 (2023-05-14)
-------------------
- Added a `julian` binary for converting Julian day numbers to & from calendar
  dates
    - `julian` now has a `cli` feature enabled by default that adds the
      dependencies needed by the binary.  Library users are advised to disable
      default features so as not to pull these in.

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
