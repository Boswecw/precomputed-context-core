# Slice 28 bundle — lineage admission gate and activation proof

This bundle adds the admission and activation proof layer on top of the rehydrated lineage state from Slice 27.

## What Slice 28 adds

- `src/lineage_activation.rs`
  - typed lineage activation receipt contract
  - rehydrate-receipt continuity binding
  - canonical supersession-chain reconstruction check over rehydrated lineage state
  - controlled active-lineage publication
- `src/bin/proof_lineage_activation.rs`
  - proves accepted lineage admission and activation
  - proves stable repeated activation receipt emission
  - proves fail-closed rejection for:
    - missing rehydrate receipt
    - corrupted supersession chain receipt
    - missing re-promotion receipt
  - proves no activation receipt publication on rejected paths
- `tests/slice28_lineage_activation.rs`
- `scripts/verify_slice_28.sh`

## New expected artifacts

- `target/proof_artifacts/slice28_lineage_activation/current/activation_receipt.json`
- `target/proof_artifacts/slice28_lineage_activation/current/active_lineage/promotion_receipt.json`
- `target/proof_artifacts/slice28_lineage_activation/current/active_lineage/rollback_receipt.json`
- `target/proof_artifacts/slice28_lineage_activation/current/active_lineage/re_promotion_receipt.json`
- `target/proof_artifacts/slice28_lineage_activation/current/active_lineage/supersession_chain_receipt.json`
- `target/proof_artifacts/slice28_lineage_activation/lineage_activation_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod lineage_activation;
```

## Notes

This slice proves the imported lineage can be admitted into an active governed state for downstream use, not merely reconstructed as evidence.
