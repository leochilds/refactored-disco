use secure_input::{parse_positive_u32, read_sanitized_line, sanitize_text, InputError};
use std::io::Cursor;

#[test]
fn read_line_accepts_valid_input() {
    let mut cursor = Cursor::new("  safe value  \n");
    let line = read_sanitized_line(&mut cursor, 16).expect("input should be accepted");
    assert_eq!(line, "safe value");
}

#[test]
fn read_line_rejects_control_characters() {
    let mut cursor = Cursor::new("\u{001B}bad\n");
    let err =
        read_sanitized_line(&mut cursor, 16).expect_err("control characters must be rejected");
    assert!(matches!(err, InputError::InvalidCharacter('\u{001B}')));
}

#[test]
fn read_line_rejects_empty_input() {
    let mut cursor = Cursor::new("\n");
    let err = read_sanitized_line(&mut cursor, 16).expect_err("blank lines must be rejected");
    assert!(matches!(err, InputError::Empty));
}

#[test]
fn sanitize_text_enforces_length_limit() {
    let long_input = "a".repeat(65);
    let err = sanitize_text(&long_input, 64).expect_err("overly long input must fail");
    assert!(matches!(
        err,
        InputError::TooLong {
            max: 64,
            actual: 65
        }
    ));
}

#[test]
fn parse_positive_u32_accepts_valid_numbers() {
    assert_eq!(parse_positive_u32("42").unwrap(), 42);
    assert_eq!(parse_positive_u32("  4294967295  ").unwrap(), 4_294_967_295);
}

#[test]
fn parse_positive_u32_rejects_invalid_numbers() {
    let err = parse_positive_u32("-1").expect_err("negative numbers must fail");
    assert!(matches!(err, InputError::InvalidCharacter('-')));

    let err = parse_positive_u32("99999999999").expect_err("overflow should fail");
    assert!(matches!(
        err,
        InputError::TooLong { .. } | InputError::NumericOverflow
    ));
}
