# Stage 03: Harden the Reader and Re-Fuzz

The fix teaches `read_sanitized_line` how to cope with malformed UTF-8 and
excessively long inputs without panicking. The implementation mirrors the one in
`main`: it reads a bounded number of bytes, converts via `String::from_utf8`, and
falls back to truncating or bubbling up an `InputError` instead of unwrapping.

## Apply the patch

Restore the clean file and then copy the fixed implementation into place:

```bash
git checkout -- crates/secure_input/src/lib.rs
cp docs/tutorial/stage03_fix/crates/secure_input/src/lib.rs \
   crates/secure_input/src/lib.rs
```

At this point `git diff` should show the larger, defensive implementation.

## Rebuild confidence with tests

The expanded unit test suite now covers multi-byte characters, truncation, and
overflow behaviour:

```bash
cargo test
```

Everything should pass, confirming that the fix did not regress existing
functionality.

## Replay the fuzz artifact

Before launching a fresh fuzzing session, re-run the previously crashing input
against the fixed code:

```bash
cargo fuzz run read_sanitized_line \
  fuzz/artifacts/read_sanitized_line/panic-utf8
```

Instead of panicking, the harness now returns an `Err(InputError::Io(_))` because
it recognises the invalid UTF-8. libFuzzer treats that as a handled error and
exits cleanly.

## Fuzz without crashes

Finally, let libFuzzer loose again to ensure the issue truly disappeared:

```bash
cargo fuzz run read_sanitized_line
```

The run should stay alive, continuously mutating inputs. You can stop it after a
few minutes with `Ctrl+C`. Congratulations—you reproduced a real fuzz bug,
understood the artifact, and shipped a resilient fix!
