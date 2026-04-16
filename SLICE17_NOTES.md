# Slice 17 — Library-owned import contract and round-trip proof

This bundle is additive.

What it adds:
- `src/import_contract.rs`
  - typed import receipt loading
  - library-owned workspace validation
  - byte-for-byte round-trip validation between package and rehydrated files
  - deterministic roundtrip report rendering
- `src/bin/proof_import_roundtrip.rs`
  - exercises the library-owned import contract against the Slice 16 workspace
  - writes `roundtrip_report.json`
- `scripts/apply_slice_17.sh`
  - idempotently exports `import_contract` from `src/lib.rs`
- `scripts/verify_slice_17.sh`
  - proves the happy path passes
  - proves the roundtrip report is stable across repeated runs
  - proves corrupted rehydrated content fails closed and does not publish a report

Expected command surface:
- `bash scripts/apply_slice_17.sh`
- `cargo run --bin proof_import_roundtrip`
- `bash scripts/verify_slice_17.sh`
