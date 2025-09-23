# Hands-On Fuzzing Tutorial

This tutorial walks through the exact workflow that uncovered and fixed a real
bug in `sanitize_display_label`. Each stage provides a code snapshot along with
the commands required to reproduce the issue, inspect the crash artifact, and
verify the fix.

1. [Stage 01 – Buggy Baseline](stage01_buggy/README.md): restore the original
   implementation and confirm that unit tests pass despite the latent bug.
2. [Stage 02 – Fuzzing and Crash Analysis](stage02_fuzzing/README.md): add a new
   fuzz harness, trigger the crash, and dissect the saved corpus entry.
3. [Stage 03 – Harden the Helper and Re-Fuzz](stage03_fix/README.md): apply the
   Unicode-aware implementation, re-run tests, and verify that fuzzing no longer
   crashes.

Each stage directory also contains the relevant source files so you can copy
them directly into the workspace if you want to follow along step-by-step.
