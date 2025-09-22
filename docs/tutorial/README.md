# Hands-On Fuzzing Tutorial

This tutorial walks through the exact workflow that uncovered and fixed a real
bug in `read_sanitized_line`. Each stage provides a code snapshot along with the
commands required to reproduce the issue, inspect the crash artifact, and verify
the fix.

1. [Stage 01 – Buggy Baseline](stage01_buggy/README.md): restore the original
   implementation, regenerate the fuzz harness, and confirm that unit tests pass.
2. [Stage 02 – Fuzzing and Crash Analysis](stage02_fuzzing/README.md): run
   `cargo fuzz`, capture the panic, and dissect the saved artifact.
3. [Stage 03 – Harden the Reader and Re-Fuzz](stage03_fix/README.md): apply the
   defensive implementation, re-run tests, and verify that fuzzing no longer
   crashes.

Each stage directory also contains the relevant source files so you can copy
them directly into the workspace if you want to follow along step-by-step.
