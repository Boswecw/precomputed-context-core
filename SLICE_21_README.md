# Slice 21 bundle — operator approval and promotion surface

This bundle adds an operator approval / promotion proof layer on top of the green Slice 20 gated rehydrate publication state.

## What Slice 21 adds

- `src/promotion_gate.rs`
  - typed operator approval contract
  - typed promotion receipt contract
  - validation that binds approval to the Slice 20 gate receipt and gated import receipt hashes
  - controlled publication of a promoted import receipt only after approval validation succeeds
- `src/bin/proof_import_promotion.rs`
  - proves approved promotion
  - proves fail-closed rejection for:
    - missing approval
    - gate hash mismatch
    - missing gated import receipt
  - proves no promoted publication on rejected paths
- `tests/slice21_promotion_gate.rs`
- `scripts/verify_slice_21.sh`

## New expected artifacts

- `target/proof_artifacts/slice21_promotion/operator_approval.json`
- `target/proof_artifacts/slice21_promotion/current/promotion_receipt.json`
- `target/proof_artifacts/slice21_promotion/current/import_receipt.json`
- `target/proof_artifacts/slice21_promotion/promotion_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod promotion_gate;
```

## Notes

This slice does not replace Slice 20. It adds an operator-authoritative promotion surface on top of the gated rehydrate publication chain so a promoted import receipt only appears in the Slice 21 workspace after approval validation succeeds.
