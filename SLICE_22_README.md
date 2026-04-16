# Slice 22 bundle — revocation and rollback proof for promoted import state

This bundle adds a revocation / rollback proof layer on top of the green Slice 21 promoted import state.

## What Slice 22 adds

- `src/promotion_revocation.rs`
  - typed revocation request contract
  - typed rollback receipt contract
  - validation that binds revocation to the Slice 21 promotion receipt and promoted import receipt hashes
  - controlled rollback publication that emits a rollback receipt and ensures no promoted import receipt remains published in the Slice 22 workspace
- `src/bin/proof_import_revocation.rs`
  - proves approved revocation and rollback
  - proves fail-closed rejection for:
    - missing revocation request
    - promotion receipt hash mismatch
    - missing promoted import receipt
  - proves no promoted publication after rollback
- `tests/slice22_promotion_revocation.rs`
- `scripts/verify_slice_22.sh`

## New expected artifacts

- `target/proof_artifacts/slice22_revocation/operator_revocation.json`
- `target/proof_artifacts/slice22_revocation/current/rollback_receipt.json`
- `target/proof_artifacts/slice22_revocation/revocation_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod promotion_revocation;
```

## Notes

This slice does not replace Slice 21. It adds an operator-authoritative rollback surface so promoted import state can be explicitly revoked and proven rolled back with deterministic evidence.
