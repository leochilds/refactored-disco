# Stage 01: Buggy Baseline

Stage 01 introduces a new helper, `sanitize_display_label`, that is meant to
trim user-provided labels before rendering them in a UI. The implementation is
naive: it only strips ASCII whitespace, so labels that consist purely of
non-breaking spaces (or other Unicode whitespace) slip through validation. The
unit tests never cover this edge case, leaving the bug dormant.

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

## Sanity check with the test suite

Even with the buggy implementation the regular tests still pass:

```bash
cargo test
```

That mirrors a realistic scenario: the helper is new, lightly tested, and the
edge case never appears in day-to-day usage. Proceed to
[Stage 02](../stage02_fuzzing/README.md) to build a fuzz harness around the
helper and surface the issue.
