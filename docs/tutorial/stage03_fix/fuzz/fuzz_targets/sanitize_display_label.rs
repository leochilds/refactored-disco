#![no_main]

use libfuzzer_sys::fuzz_target;
use secure_input::{sanitize_display_label, InputError};

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    let max_len = usize::from(data[0]).saturating_add(1);
    let candidate = &data[1..];

    if let Ok(label) = std::str::from_utf8(candidate) {
        match sanitize_display_label(label, max_len) {
            Ok(cleaned) => {
                if cleaned.trim().is_empty() {
                    panic!("sanitize_display_label accepted an invisible label: {label:?}");
                }
            }
            Err(InputError::TooLong { .. })
            | Err(InputError::Empty)
            | Err(InputError::InvalidCharacter(_)) => {}
            Err(InputError::NumericOverflow) => unreachable!("numeric overflow is unused"),
            Err(InputError::Io(_)) => unreachable!("IO errors cannot occur for string inputs"),
        }
    }
});
