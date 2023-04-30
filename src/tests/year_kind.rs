use crate::YearKind;

#[test]
fn common() {
    let yk = YearKind::Common;
    assert!(yk.is_common());
    assert!(!yk.is_leap());
    assert!(!yk.is_reform());
    assert!(!yk.is_skipped());
}

#[test]
fn leap() {
    let yk = YearKind::Leap;
    assert!(!yk.is_common());
    assert!(yk.is_leap());
    assert!(!yk.is_reform());
    assert!(!yk.is_skipped());
}

#[test]
fn reform_common() {
    let yk = YearKind::ReformCommon;
    assert!(yk.is_common());
    assert!(!yk.is_leap());
    assert!(yk.is_reform());
    assert!(!yk.is_skipped());
}

#[test]
fn reform_leap() {
    let yk = YearKind::ReformLeap;
    assert!(!yk.is_common());
    assert!(yk.is_leap());
    assert!(yk.is_reform());
    assert!(!yk.is_skipped());
}

#[test]
fn skipped() {
    let yk = YearKind::Skipped;
    assert!(!yk.is_common());
    assert!(!yk.is_leap());
    assert!(!yk.is_reform());
    assert!(yk.is_skipped());
}
