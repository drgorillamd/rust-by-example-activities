use serial_test::serial;
use crate::*;

#[test]
#[serial]
fn test_display() {
    assert_stdout_eq!(chap2::chap2::display_matrix(), "( 1.1 1.2 )\n( 2.1 2.2 )\n");
}

#[test]
#[serial]
fn test_transpose() {
    assert_stdout_eq!(chap2::chap2::display_transpose(), "Matrix:\n( 1.1 1.2 )\n( 2.1 2.2 )\nTranspose:\n( 1.1 2.1 )\n( 1.2 2.2 )\n");
}