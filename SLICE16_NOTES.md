# Slice 16 — Package import and replay rehydration

This bundle is additive.

What it adds:
- `src/bin/proof_import_rehydrate.rs`
  - verifies the package zip and companion sha256
  - enforces the exact expected member set
  - extracts the verified package into a controlled workspace
  - rehydrates replay/governed-flow/package-meta files into a deterministic import layout
  - emits a deterministic `import_receipt.json`
  - uses a stage directory so failed imports do not publish partial state
- `scripts/verify_slice_16.sh`
  - proves the happy path passes
  - proves `import_receipt.json` is stable across repeated runs
  - proves a missing replay manifest fails closed and does not publish a receipt

Expected command surface:
- `cargo run --bin proof_import_rehydrate`
- `bash scripts/verify_slice_16.sh`
