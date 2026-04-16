# Slice 23 bundle — controlled re-promotion proof after rollback

This bundle adds a controlled re-promotion proof layer on top of the green Slice 22 rollback state.

## What Slice 23 adds

- `src/re_promotion.rs`
  - typed re-approval contract
  - typed re-promotion receipt contract
  - validation that binds re-approval to the Slice 22 rollback receipt and the Slice 21 source promotion artifacts
  - controlled publication of a re-promoted import receipt only after re-approval validation succeeds
- `src/bin/proof_import_repromotion.rs`
  - proves approved re-promotion after rollback
  - proves fail-closed rejection for:
    - missing re-approval
    - rollback receipt hash mismatch
    - missing source promoted import receipt
  - proves no publication on rejected paths
- `tests/slice23_repromotion.rs`
- `scripts/verify_slice_23.sh`

## New expected artifacts

- `target/proof_artifacts/slice23_repromotion/operator_reapproval.json`
- `target/proof_artifacts/slice23_repromotion/current/re_promotion_receipt.json`
- `target/proof_artifacts/slice23_repromotion/current/import_receipt.json`
- `target/proof_artifacts/slice23_repromotion/repromotion_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod re_promotion;
```

## Notes

This slice does not replace Slice 22. It adds an operator-authoritative re-promotion surface so a previously rolled-back promoted import state can be explicitly re-approved and re-published with deterministic evidence.
