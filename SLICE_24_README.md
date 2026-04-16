# Slice 24 bundle — supersession chain proof for promotion lineage

This bundle adds an explicit supersession lineage proof layer on top of the green Slice 23 re-promotion state.

## What Slice 24 adds

- `src/supersession_chain.rs`
  - typed supersession chain receipt contract
  - explicit lineage links between:
    - Slice 21 promotion
    - Slice 22 rollback
    - Slice 23 re-promotion
  - validation that rollback is chained to the original promotion
  - validation that re-promotion is chained to the rollback and original promotion
- `src/bin/proof_import_supersession_chain.rs`
  - proves complete supersession lineage receipt emission
  - proves fail-closed rejection for:
    - missing rollback receipt
    - re-promotion rollback-hash mismatch
    - missing re-promotion receipt
  - proves no publication on rejected paths
- `tests/slice24_supersession_chain.rs`
- `scripts/verify_slice_24.sh`

## New expected artifacts

- `target/proof_artifacts/slice24_supersession/current/supersession_chain_receipt.json`
- `target/proof_artifacts/slice24_supersession/supersession_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod supersession_chain;
```

## Notes

This slice converts promotion, rollback, and re-promotion from adjacent proofs into an explicit lineage receipt with governed supersession links.
