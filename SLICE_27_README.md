# Slice 27 bundle — imported lineage rehydrate proof

This bundle adds rehydration proof for the externally admitted lineage bundle from Slice 26.

## What Slice 27 adds

- `src/lineage_bundle_rehydrate.rs`
  - typed lineage rehydrate receipt contract
  - intake-receipt hash binding
  - manifest and envelope hash continuity enforcement
  - canonical supersession-chain reconstruction check
  - controlled rehydrated lineage-state publication
- `src/bin/proof_imported_lineage_rehydrate.rs`
  - proves accepted imported lineage rehydration
  - proves stable repeated rehydrate receipt emission
  - proves fail-closed rejection for:
    - missing intake receipt
    - intake manifest hash mismatch
    - corrupted supersession chain receipt
  - proves no rehydrate receipt publication on rejected paths
- `tests/slice27_lineage_rehydrate.rs`
- `scripts/verify_slice_27.sh`

## New expected artifacts

- `target/proof_artifacts/slice27_lineage_rehydrate/current/rehydrate_receipt.json`
- `target/proof_artifacts/slice27_lineage_rehydrate/current/lineage_state/promotion_receipt.json`
- `target/proof_artifacts/slice27_lineage_rehydrate/current/lineage_state/rollback_receipt.json`
- `target/proof_artifacts/slice27_lineage_rehydrate/current/lineage_state/re_promotion_receipt.json`
- `target/proof_artifacts/slice27_lineage_rehydrate/current/lineage_state/supersession_chain_receipt.json`
- `target/proof_artifacts/slice27_lineage_rehydrate/lineage_rehydrate_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod lineage_bundle_rehydrate;
```

## Notes

This slice proves the imported lineage bundle can reconstruct a working, verified lineage state rather than only being stored as admitted evidence.
