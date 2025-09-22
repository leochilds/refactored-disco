# Secure Input Fuzzing Toolkit

This repository exists to showcase how a focused fuzzing workflow looks for a
small Rust component. The libFuzzer targets are the star of the demo: they drive
untrusted data through a deliberately non-trivial input handling crate so you
can observe failure modes, reproduction steps, and coverage collection end to
end.

The `secure_input` crate still ships as part of the repo so the fuzzers have a
realistic API surface to attack, but think of it as supporting infrastructure.
Most contributors will spend their time extending the harnesses, growing the
corpora, and wiring automation.

## Repository layout

```
.
├── fuzz/                    # `cargo-fuzz` workspace with harnesses and corpora
├── crates/
│   └── secure_input/        # Library + demo CLI that the fuzzers exercise
├── docs/                    # Fuzzing playbooks and PR templates
└── README.md                # You are here
```

The workspace root is package-free; invoke binaries and tests through
`-p secure_input` or by using the default workspace member. For example:

```bash
cargo test                 # runs in crates/secure_input by default
cargo run -p secure_input  # builds the demo CLI without the fuzzing toolchain
```

## Fuzzing first steps

1. Install [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz):

   ```bash
   cargo install cargo-fuzz
   ```

2. Explore the existing harnesses or add a new one under
   [`fuzz/fuzz_targets/`](fuzz/fuzz_targets/). Each target documents the entry
   point it hammers and how it derives structured input from the raw byte
   stream.

3. Kick off a session:

   ```bash
   cargo fuzz run read_sanitized_line
   ```

   `cargo-fuzz` will resume from any saved corpus under
   `fuzz/corpus/<target>/` and store crashes in `fuzz/artifacts/<target>/`.

Full instructions, including coverage generation and CI expectations, live in
[docs/fuzzing.md](docs/fuzzing.md).

Prefer a guided exercise? The [hands-on fuzzing tutorial](docs/tutorial/README.md)
walks through the actual crash that first motivated the hardened
`read_sanitized_line` implementation.

## About the `secure_input` crate

The supporting crate exposes a few sanitisation helpers and a command-line demo
binary. The public API is intentionally small so the fuzzing harnesses can reach
interesting branches quickly:

- [`sanitize_text`](crates/secure_input/src/lib.rs) trims whitespace, rejects
  control characters, and enforces configurable length limits.
- [`read_sanitized_line`](crates/secure_input/src/lib.rs) reads from a buffered
  source while detecting truncation and invalid UTF-8.
- [`parse_positive_u32`](crates/secure_input/src/lib.rs) parses positive integers
  after sanitisation.

The binary in [`src/main.rs`](crates/secure_input/src/main.rs) simply wires the
helpers into a minimal REPL so you can experiment manually.

## Tests and linting

From the repository root run:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Integration tests live under
[`crates/secure_input/tests`](crates/secure_input/tests) and exercise both valid
and invalid input flows.
