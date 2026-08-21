//! Unit tests for the contract version and its bind rule.

use super::{CONTRACT_VERSION, is_compatible};

#[test]
fn the_contract_binds_to_itself() {
    assert!(is_compatible(CONTRACT_VERSION));
}

#[test]
fn a_newer_minor_on_the_module_side_binds() {
    let (major, minor) = CONTRACT_VERSION;
    assert!(is_compatible((major, minor + 1)));
}

#[test]
fn an_older_minor_on_the_module_side_is_rejected() {
    let (major, minor) = CONTRACT_VERSION;
    assert!(!is_compatible((major, minor.saturating_sub(1) )) || minor == 0);
}

#[test]
fn a_different_major_is_rejected() {
    let (major, minor) = CONTRACT_VERSION;
    assert!(!is_compatible((major + 1, minor)));
    assert!(!is_compatible((major.saturating_sub(1), minor)) || major == 0);
}

#[test]
fn the_shipped_contract_version_is_pinned() {
    assert_eq!(CONTRACT_VERSION, (1, 0));
}
