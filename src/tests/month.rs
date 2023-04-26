use crate::{Month, TryIntoMonthError};

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
