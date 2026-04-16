# Slice 20 bundle — authorization-gated rehydrate publication

This bundle adds a controlled rehydrate publication gate on top of the green Slice 19 state.

## What Slice 20 adds

- `src/import_gate.rs`
  - validates policy hash, trust-envelope hash, authorization receipt hash, evidence-link hash, and import-receipt hash together
  - emits a typed rehydrate gate receipt
  - publishes a gated import receipt only after authorization validation succeeds
- `src/bin/proof_import_rehydrate_authorized.rs`
  - proves the authorized publication path
  - proves fail-closed rejection for:
    - missing authorization receipt
    - evidence mismatch/tamper
    - missing import receipt
  - proves no gated import-receipt publication on rejected paths
- `tests/slice20_import_gate.rs`
- `scripts/verify_slice_20.sh`

## New expected artifacts

- `target/proof_artifacts/slice20_rehydrate_gate/current/gate_receipt.json`
- `target/proof_artifacts/slice20_rehydrate_gate/current/import_receipt.json`
- `target/proof_artifacts/slice20_rehydrate_gate/rehydrate_gate_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod import_gate;
```

## Notes

This slice gates the controlled publication surface for rehydrated import evidence. It does not replace the earlier Slice 16 import proof; it binds that proven import receipt to the Slice 19 authorization material and only republishes it into the Slice 20 gated workspace when the evidence chain validates.
