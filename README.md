# Secure Input Handling

This project demonstrates a small Rust binary crate focused on reading and
sanitising untrusted user input. The core logic lives in `src/lib.rs` where
functions validate free-form text and convert it into safe, structured values.
The binary in `src/main.rs` exposes a minimal command-line interface that asks
the user for a positive integer and reports the sanitised result.

## Library API

The library exposes three key helpers:

- `sanitize_text` trims whitespace, rejects control characters, and enforces a
  caller-provided length limit.
- `read_sanitized_line` reads a single line from any `BufRead` implementation
  (such as standard input) and applies the same validation rules.
- `parse_positive_u32` builds upon `sanitize_text` to ensure the input contains
  nothing but digits before parsing it into a `u32`.

All helpers return a custom `InputError` type so callers can handle failure
conditions precisely without panicking.

## Running the binary

```bash
cargo run
```

When prompted, enter a positive integer containing at most ten digits. Invalid
input (empty strings, control characters, non-digit characters, or numbers that
exceed `u32::MAX`) is rejected with an explanatory error message.

## Tests and linting

Execute the full suite of checks with:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

The integration tests in `tests/input_handling.rs` exercise the API with valid
and invalid input to ensure sanitisation and error reporting behave as
expected.
