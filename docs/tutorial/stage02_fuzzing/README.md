# Stage 02: Fuzzing and Crash Analysis

With the ASCII-only trimming still in place we can unleash libFuzzer against
`sanitize_display_label`. The harness asserts that any accepted label must
remain visible after full Unicode trimming. Non-breaking spaces defeat the buggy
implementation, so the assertion fires almost immediately.

## Reproduce the buggy state

If you skipped Stage 01, restore the same buggy implementation before running
the fuzzer:

```bash
git checkout -- crates/secure_input/src/lib.rs
cp docs/tutorial/stage02_fuzzing/crates/secure_input/src/lib.rs \
   crates/secure_input/src/lib.rs
```

## Add the fuzz target

Copy the staged fuzz workspace into place to register the new harness:

```bash
cp docs/tutorial/stage02_fuzzing/fuzz/Cargo.toml fuzz/Cargo.toml
cp docs/tutorial/stage02_fuzzing/fuzz/fuzz_targets/sanitize_display_label.rs \
   fuzz/fuzz_targets/
```

The harness interprets the first byte as the maximum allowed label length and
treats the remaining bytes as UTF-8. Whenever the helper returns `Ok`, the
harness checks whether a full Unicode trim would collapse the label to nothing
and panics if so.

## Trigger the crash

A ready-made corpus file exercises the failure instantly:

```bash
cp docs/tutorial/stage02_fuzzing/fuzz/corpus/sanitize_display_label/nbsp-label \
   fuzz/corpus/sanitize_display_label/
cp docs/tutorial/stage02_fuzzing/fuzz/artifacts/sanitize_display_label/panic-nbsp \
   fuzz/artifacts/sanitize_display_label/
```

Inspect the bytes with a hex dump:

```bash
xxd -g 1 fuzz/corpus/sanitize_display_label/nbsp-label
```

The first byte (`0x05`) sets a generous length limit. The remaining four bytes
encode two Unicode non-breaking spaces (`0xC2 0xA0` each). Because the buggy
implementation only trims ASCII whitespace it accepts the label as-is. The
harness then applies `trim()` and observes that the cleaned label becomes empty,
causing the panic.

To replay the crash deterministically, point `cargo fuzz run` at the corpus or
artifact file:

```bash
cargo fuzz run sanitize_display_label \
  fuzz/corpus/sanitize_display_label/nbsp-label
```

Armed with a reproducer, advance to
[Stage 03](../stage03_fix/README.md) to harden the helper and confirm that the
fuzzer stops crashing.
