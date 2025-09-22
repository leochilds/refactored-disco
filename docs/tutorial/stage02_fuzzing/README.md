# Stage 02: Fuzzing and Crash Analysis

With the naive UTF-8 handling still in place we can unleash libFuzzer against
`read_sanitized_line`. The target already lives under
`fuzz/fuzz_targets/read_sanitized_line.rs`, but the command below shows how the
harness would be generated from scratch in a real project.

## Reproduce the buggy state

If you skipped Stage 01, restore the same buggy implementation before running
the fuzzer:

```bash
git checkout -- crates/secure_input/src/lib.rs
cp docs/tutorial/stage02_fuzzing/crates/secure_input/src/lib.rs \
   crates/secure_input/src/lib.rs
```

## Ensure the fuzz target exists

`cargo fuzz add` scaffolds the harness and updates the fuzz workspace. This
repository already contains the generated files, but running the command is a
good reminder of the underlying workflow:

```bash
cargo fuzz add read_sanitized_line
```

You can inspect the harness at
[`fuzz/fuzz_targets/read_sanitized_line.rs`](../../fuzzing.md).

## Trigger the crash

Launch the fuzzer and wait for it to discover an input that triggers the panic
inside `String::from_utf8`:

```bash
cargo fuzz run read_sanitized_line
```

The run ends quickly with a stack trace similar to:

```
panicked at 'called `Result::unwrap()` on an `Err` value: FromUtf8Error { .. }'
```

`cargo-fuzz` stores the crashing input under
`fuzz/artifacts/read_sanitized_line/`. The repository includes a representative
artifact in case you want to skip the fuzzing time on repeat runs:

```bash
cp docs/tutorial/stage02_fuzzing/fuzz/artifacts/read_sanitized_line/panic-utf8 \
   fuzz/artifacts/read_sanitized_line/
```

Inspect the bytes with a hex dump:

```bash
xxd -g 1 fuzz/artifacts/read_sanitized_line/panic-utf8
```

The first byte (`0x0A`) is interpreted by the harness as the maximum length.
The remaining single byte (`0x80`) is not valid standalone UTF-8, so
`String::from_utf8` returns an error that we promptly unwrap, causing the panic.

To replay the failure deterministically, point `cargo fuzz run` at the artifact
file:

```bash
cargo fuzz run read_sanitized_line \
  fuzz/artifacts/read_sanitized_line/panic-utf8
```

Armed with a reproducer, advance to
[Stage 03](../stage03_fix/README.md) to harden the implementation and confirm
that fuzzing no longer crashes.
