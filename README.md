# Fuzzing Toolkit Demo

This repository is a small fuzzing toolkit showcase. Everything centres on the
`secure_input` Rust crate, which doubles as the demo application: it accepts
untrusted input, cleans it up, and gives the fuzzing stack something realistic
to stress.

## Where the demo crate lives

The crate sits at the repository root in the standard Cargo layout:

- `src/lib.rs` exports the input-sanitising helpers that fuzz targets will poke
  at.
- `src/main.rs` wires those helpers into a simple command-line interface so you
  can see the behaviour by hand.

Fuzzing harnesses link straight against the library API. A target that calls
`parse_positive_u32`, for example, lets the fuzzer hammer the same entry points
that the binary uses.

## Phase roadmap

- **Phase A – Enable per-repo, developer-friendly fuzz targets.** Add
  [`cargo-fuzz`](https://rust-fuzz.github.io/book/cargo-fuzz.html) as a
  development dependency and create a `fuzz_targets/` directory with one to
  three harnesses per crate. Focus each harness on the parsing,
  deserialisation, or FFI edges of the public API so contributors can reproduce
  bugs locally. Provide a short template pull request that shows how to add a
  new target, seed an initial corpus, and hook the matching CI snippet.
- **Phase B – Short merge request checks.** Add a GitLab CI job that runs
  `cargo fuzz run <target> -- -max_total_time=60` (or `-runs=1000`) to catch
  obvious crashes on merge requests. Use the existing runners when they have
  the needed sanitizers and LLVM toolchain; otherwise supply a specialised
  executor.
- **Phase C – Long-running fuzzing and orchestration.** Bring
  [ClusterFuzzLite](https://google.github.io/clusterfuzzlite/) into the CI
  pipeline so code changes get incremental fuzzing and scheduled batch jobs grow
  the corpus. Back the effort with durable infrastructure—such as an EKS cluster
  provisioned through Terraform or a pool of autoscaling EC2 spot instances—that
  can run `libFuzzer`/`cargo-fuzz` workers for long stretches. Store corpora and
  crash artifacts in S3 so the data survives restarts.
- **Phase D – Crash workflow.** Route crashes into ClusterFuzz or your existing
  triage automation. When a crash is confirmed and minimised, open a GitLab
  issue for the owning team that contains the reproduction steps, sanitizer
  stack trace, and any suspected files or lines. Link the S3-hosted artifacts so
  engineers can pull them down quickly—ClusterFuzz can handle much of this flow
  when it runs end-to-end.

## Running the demo binary

```bash
cargo run
```

When prompted, enter a positive integer containing at most ten digits. Invalid
input (empty strings, control characters, non-digit characters, or numbers that
exceed `u32::MAX`) is rejected with an explanatory error message.


## Fuzzing

See [docs/fuzzing.md](docs/fuzzing.md) for instructions on installing `cargo-fuzz` and running the available harnesses.
For new harness contributions, start with the template in [docs/template_pr.md](docs/template_pr.md).
The guide walks through creating the target, seeding a corpus, and wiring the CI smoke test that reviewers expect in fuzzing pull requests.

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
