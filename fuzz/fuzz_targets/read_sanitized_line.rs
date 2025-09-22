#![no_main]

use libfuzzer_sys::fuzz_target;
use secure_input::read_sanitized_line;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    let (len_byte, rest) = data.split_first().unwrap();
    let max_len = (*len_byte as usize) % 128;
    let mut cursor = Cursor::new(rest.to_vec());

    let _ = read_sanitized_line(&mut cursor, max_len);
});
