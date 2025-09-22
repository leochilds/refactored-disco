use std::fmt;
use std::io::{self, BufRead, Read};

/// Errors that can occur when validating or parsing external input.
#[derive(Debug)]
pub enum InputError {
    /// The provided input was empty after trimming whitespace.
    Empty,
    /// The provided input exceeded the allowed length.
    TooLong { max: usize, actual: usize },
    /// The provided input contained a disallowed character.
    InvalidCharacter(char),
    /// The provided input could not be represented as the requested number.
    NumericOverflow,
    /// Wrapper for I/O failures while reading from the input source.
    Io(io::Error),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::Empty => write!(f, "input was empty"),
            InputError::TooLong { max, actual } => {
                write!(f, "input was too long: {actual} characters (max {max})")
            }
            InputError::InvalidCharacter(ch) => {
                if ch.is_control() {
                    write!(f, "input contained control character U+{:04X}", *ch as u32)
                } else {
                    write!(f, "input contained invalid character '{ch}'")
                }
            }
            InputError::NumericOverflow => write!(f, "number was larger than supported range"),
            InputError::Io(err) => write!(f, "failed to read input: {err}"),
        }
    }
}

impl std::error::Error for InputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InputError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for InputError {
    fn from(value: io::Error) -> Self {
        InputError::Io(value)
    }
}

/// Sanitises a raw string by trimming whitespace, rejecting control characters
/// and enforcing a maximum length.
///
/// This helper is primarily intended for processing untrusted user input such
/// as data read from standard input. Leading and trailing whitespace is
/// discarded to minimise the risk of invisible characters being preserved.
/// The resulting string is guaranteed to be non-empty, contain no control
/// characters, and respect the provided maximum length.
pub fn sanitize_text(raw: &str, max_len: usize) -> Result<String, InputError> {
    let without_newline = raw.trim_end_matches(['\r', '\n']);
    let trimmed = without_newline.trim();

    if trimmed.is_empty() {
        return Err(InputError::Empty);
    }

    let length = trimmed.chars().count();
    if length > max_len {
        return Err(InputError::TooLong {
            max: max_len,
            actual: length,
        });
    }

    for ch in trimmed.chars() {
        if ch.is_control() {
            return Err(InputError::InvalidCharacter(ch));
        }
    }

    Ok(trimmed.to_string())
}

/// Reads a single line from the provided buffered reader and sanitises it.
///
/// The line must not exceed `max_len` characters after trimming whitespace and
/// may not contain control characters. End-of-file without any bytes read is
/// treated as empty input and results in an [`InputError::Empty`] error.
pub fn read_sanitized_line<R: BufRead>(
    reader: &mut R,
    max_len: usize,
) -> Result<String, InputError> {
    let mut buffer = String::new();
    let (bytes_read, truncated) = {
        let mut limited = (&mut *reader).take(max_len.saturating_add(1) as u64);
        let read = limited.read_line(&mut buffer)?;
        (read, limited.limit() == 0 && !buffer.ends_with('\n'))
    };

    if bytes_read == 0 {
        return Err(InputError::Empty);
    }

    if truncated {
        loop {
            let available = reader.fill_buf()?;
            if available.is_empty() {
                break;
            }

            if let Some(newline_idx) = available.iter().position(|&byte| byte == b'\n') {
                reader.consume(newline_idx + 1);
                break;
            }

            let len = available.len();
            reader.consume(len);
        }
    }

    sanitize_text(&buffer, max_len)
}

/// Parses a positive 32-bit unsigned integer from user-provided input.
///
/// The function trims surrounding whitespace, validates that the remaining
/// characters are all ASCII digits, and rejects values that exceed the range of
/// `u32`.
pub fn parse_positive_u32(input: &str) -> Result<u32, InputError> {
    let sanitized = sanitize_text(input, 10)?; // u32::MAX has 10 digits.

    if let Some(invalid) = sanitized.chars().find(|ch| !ch.is_ascii_digit()) {
        return Err(InputError::InvalidCharacter(invalid));
    }

    sanitized
        .parse::<u32>()
        .map_err(|_| InputError::NumericOverflow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn sanitize_text_rejects_empty() {
        assert!(matches!(sanitize_text("   \n", 5), Err(InputError::Empty)));
    }

    #[test]
    fn sanitize_text_trims_whitespace() {
        let result = sanitize_text("  Hello World  \r\n", 32).unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn read_sanitized_line_rejects_long_input() {
        let mut cursor = Cursor::new("six chars\n");
        let err = read_sanitized_line(&mut cursor, 5).unwrap_err();
        assert!(matches!(err, InputError::TooLong { .. }));
    }

    #[test]
    fn read_sanitized_line_discards_overlong_line() {
        let mut cursor = Cursor::new("abcdefg\nok\n");
        let err = read_sanitized_line(&mut cursor, 5).unwrap_err();
        assert!(matches!(err, InputError::TooLong { .. }));

        let second = read_sanitized_line(&mut cursor, 5).unwrap();
        assert_eq!(second, "ok");
    }

    #[test]
    fn parse_positive_u32_rejects_letters() {
        let err = parse_positive_u32("12ab").unwrap_err();
        assert!(matches!(err, InputError::InvalidCharacter('a')));
    }

    #[test]
    fn parse_positive_u32_rejects_overflow() {
        let err = parse_positive_u32("42949672960").unwrap_err();
        assert!(matches!(
            err,
            InputError::TooLong { .. } | InputError::NumericOverflow
        ));
    }
}
