#![no_main]

use libfuzzer_sys::fuzz_target;
use secure_input::sanitize_text;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    let (len_byte, rest) = data.split_first().unwrap();
    let max_len = (*len_byte as usize) % 128;
    let candidate = String::from_utf8_lossy(rest);

    let _ = sanitize_text(&candidate, max_len);
});
