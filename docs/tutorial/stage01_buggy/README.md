# Stage 01: Buggy Baseline

This snapshot captures the original implementation of `read_sanitized_line`. It
simply slurps raw bytes from a `BufRead`, assumes that they form valid UTF-8,
and unwraps the conversion. The rest of the crate (including the unit tests)
still passes, so the latent bug is invisible until fuzzing exercises malformed
inputs.

## Apply this snapshot

From the repository root restore the stage contents into the working tree:

```bash
git checkout -- crates/secure_input/src/lib.rs
cp docs/tutorial/stage01_buggy/crates/secure_input/src/lib.rs \
   crates/secure_input/src/lib.rs
```

The first command ensures you start from a clean file before copying the staged
version over the top. Check `git status` afterwards and you should only see the
modified `lib.rs`.

## Prepare the fuzz workspace

If you have never created the `read_sanitized_line` harness before, generate it
now so the subsequent stages can reuse the target:

```bash
cargo fuzz add read_sanitized_line
```

The command is idempotent—running it again simply confirms that the fuzz target
exists under `fuzz/fuzz_targets/read_sanitized_line.rs`.

## Sanity check with the test suite

Even with the buggy implementation the regular tests still pass:

```bash
cargo test
```

This mirrors a realistic scenario: the crate ships unit coverage for the happy
path, but it never considered how malformed UTF-8 might reach the line reader.
Proceed to [Stage 02](../stage02_fuzzing/README.md) to let the fuzzer loose on
the implementation.
