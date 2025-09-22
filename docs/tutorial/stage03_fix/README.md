# Stage 03: Harden the Helper and Re-Fuzz

The fix teaches `sanitize_display_label` how to trim full Unicode whitespace
before validating the label. The helper now mirrors the behaviour of `str::trim`
and continues to reject control characters and overlong inputs. New unit tests
cover non-breaking spaces to prevent regressions.

## Apply the patch

Restore the clean file and then copy the fixed implementation into place:

```bash
git checkout -- crates/secure_input/src/lib.rs
cp docs/tutorial/stage03_fix/crates/secure_input/src/lib.rs \
   crates/secure_input/src/lib.rs
```

At this point `git diff` should show the Unicode-aware trimming logic along with
extra tests for non-breaking spaces.

## Rebuild confidence with tests

The expanded unit test suite now covers multi-byte characters, truncation, and
the once-missing whitespace scenario:

```bash
cargo test
```

Everything should pass, confirming that the fix did not regress existing
functionality.

## Replay the fuzz artifact

Before launching a fresh fuzzing session, re-run the previously crashing input
against the fixed code:

```bash
cargo fuzz run sanitize_display_label \
  fuzz/artifacts/sanitize_display_label/panic-nbsp
```

Instead of panicking, the harness now sees `sanitize_display_label` return
`Err(InputError::Empty)` because the helper trims the non-breaking spaces.
libFuzzer treats that as handled input and exits cleanly.

## Fuzz without crashes

Finally, let libFuzzer loose again to ensure the issue truly disappeared:

```bash
cargo fuzz run sanitize_display_label
```

The run should stay alive, continuously mutating inputs. You can stop it after a
few minutes with `Ctrl+C`. Congratulations—you reproduced a fuzz-discovered
Unicode trimming bug, understood the artifact, and shipped a resilient fix!
