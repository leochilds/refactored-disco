# Fuzzing Guide

This project uses [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz) to
exercise the parsing and sanitisation routines exposed by `secure_input`.

Looking for a complete walkthrough? Follow the [hands-on fuzzing
tutorial](tutorial/README.md) to recreate the bug that libFuzzer uncovered in
`sanitize_display_label`, inspect the crash artifact, and apply the eventual
fix.

## Prerequisites

1. Install the `cargo-fuzz` subcommand (only required once per development
   environment):

   ```bash
   cargo install cargo-fuzz
   ```

2. Ensure the LLVM tooling required by `libFuzzer` is available. On
   Debian/Ubuntu systems the `llvm` and `clang` packages satisfy the requirement.
   Other platforms may ship the tools as part of Xcode (macOS) or via the system
   package manager.

## Available targets

Four fuzz targets live under `fuzz/fuzz_targets/`:

- `sanitize_text` feeds arbitrary strings and length limits into
  [`sanitize_text`](../crates/secure_input/src/lib.rs) to explore edge cases
  involving trimming and character validation.
- `sanitize_display_label` checks that
  [`sanitize_display_label`](../crates/secure_input/src/lib.rs) never accepts
  labels that collapse to empty once full Unicode trimming is applied.
- `read_sanitized_line` constructs buffered readers from arbitrary byte streams
  to stress the UTF-8 handling and truncation logic in
  [`read_sanitized_line`](../crates/secure_input/src/lib.rs).
- `parse_positive_u32` generates candidate numeric strings for
  [`parse_positive_u32`](../crates/secure_input/src/lib.rs), covering digit
  filtering and overflow detection.

List the targets by running:

```bash
cargo fuzz list
```

## Running a fuzzer

Each target can be started via `cargo fuzz run <target-name>`. For example, to
fuzz the `sanitize_display_label` interface:

```bash
cargo fuzz run sanitize_display_label
```

The command builds the fuzzing harness in release mode and then continuously
executes randomly generated inputs until interrupted (Ctrl+C). Corpus and crash
artifacts are stored under `fuzz/artifacts/<target-name>/`.

To resume from the saved corpus on subsequent runs, simply re-run the same
command; `cargo-fuzz` automatically loads the previously discovered inputs.

## Collecting coverage or reproducing crashes

If a crash is discovered, it will be saved as an individual file in the
artifacts directory. Reproduce it with:

```bash
cargo fuzz run <target-name> fuzz/artifacts/<target-name>/<file>
```

Generating coverage reports is supported via `cargo fuzz coverage <target-name>`
once LLVM tools are installed. Refer to the
[`cargo-fuzz` documentation](https://github.com/rust-fuzz/cargo-fuzz#quickstart)
for platform-specific notes and advanced options such as sanitiser integration.
