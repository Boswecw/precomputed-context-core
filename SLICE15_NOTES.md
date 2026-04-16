# Slice 15 — Package intake verification

This bundle is additive.

What it adds:
- `src/bin/proof_intake_check.rs`
  - verifies `slice14_export.zip`
  - verifies the companion sha256 file
  - enforces the exact expected archive member set
  - verifies `package_index.json` references the expected payload members
- `scripts/verify_slice_15.sh`
  - proves the happy path passes
  - proves fail-closed behavior for:
    - extra member tampering
    - missing member tampering
    - sha256 mismatch

Expected command surface:
- `cargo run --bin proof_intake_check`
- `bash scripts/verify_slice_15.sh`
