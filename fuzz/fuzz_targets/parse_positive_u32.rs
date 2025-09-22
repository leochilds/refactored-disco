#![no_main]

use libfuzzer_sys::fuzz_target;
use secure_input::parse_positive_u32;

fuzz_target!(|data: &[u8]| {
    let candidate = String::from_utf8_lossy(data);

    let _ = parse_positive_u32(&candidate);
});
