# Pull request template: add a fuzz target

This template outlines the minimum steps expected in a pull request that introduces a new `cargo-fuzz` harness to the repository. Copy the checklist into your PR description, expand the placeholders, and replace the example names with your target.

## ✅ Summary checklist

- [ ] Created the harness at `fuzz/fuzz_targets/<target>.rs` and verified it builds.
- [ ] Added a seed corpus under `fuzz/corpus/<target>/` with at least one input that exercises the happy path.
- [ ] Exercised the harness locally with a bounded run to ensure the corpus loads and the harness returns.
- [ ] Linked to the CI snippet (below) or updated the relevant workflow to execute the target in merge requests.

## 1. Add the harness

```bash
# From the repository root
cargo fuzz add <target>
```

The command scaffolds `fuzz/fuzz_targets/<target>.rs` and updates `fuzz/Cargo.toml`. Replace the generated body with a thin wrapper around the public API you want to stress. A typical harness looks like this:

```rust
// fuzz/fuzz_targets/<target>.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use secure_input::sanitize_text; // Replace with the API under test

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let _ = sanitize_text(input, 128);
    }
});
```

Run `cargo fmt` to keep the harness consistent with the rest of the repo.

## 2. Seed a minimal corpus

Create a directory for the target and populate it with at least one valid input. The corpus lives directly under `fuzz/corpus/<target>/` so `cargo-fuzz` will find it automatically.

```bash
mkdir -p fuzz/corpus/<target>
printf '42' > fuzz/corpus/<target>/seed-001
```

Tailor the seed to the behaviour you want to exercise—use structured formats, JSON snippets, or strings that demonstrate edge cases. Multiple files are welcome, but avoid oversized corpora that slow down CI.

## 3. Sanity-check the harness locally

Before opening the PR, run a short bounded fuzzing session to ensure the harness links correctly, can ingest the seed corpus, and does not crash immediately:

```bash
cargo fuzz run <target> -- -runs=1000
```

Feel free to increase the run count or replace `-runs` with `-max_total_time=<seconds>` when iterating on crash fixes. Record any other validation (e.g., `cargo test`) in your PR description.

## 4. Reference CI coverage

Include the following GitHub Actions snippet (or update the existing workflow) so reviewers know how the harness will run in automation. Swap `<target>` for the real name and add the job to `.github/workflows/ci.yml` (or the equivalent pipeline file used by your fork).

```yaml
jobs:
  fuzz-<target>:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Install cargo-fuzz
        run: cargo install cargo-fuzz
      - name: Run libFuzzer smoke test
        run: cargo fuzz run <target> -- -max_total_time=60
```

For GitLab CI, the job body can be re-used inside a stage with `cargo fuzz run <target> -- -runs=1000`. Either way, ensure the job is part of the merge request pipeline so crashes block the PR.

---

When you raise the pull request, paste this template into the description, fill in the details, and remove the instructional text. The reviewer will use it as a checklist to confirm the harness is production-ready.
