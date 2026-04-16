# Slice 29 bundle — active-lineage consumption contract and downstream attestation proof

This bundle adds the typed downstream-consumption contract and attestation proof layer on top of the active lineage state from Slice 28.

## What Slice 29 adds

- `src/lineage_consumption.rs`
  - typed active-lineage consumer contract
  - consumer contract validation
  - activation-receipt continuity binding
  - canonical supersession-chain reconstruction check over active lineage state
  - deterministic active-lineage digest computation
  - downstream attestation receipt contract
- `src/bin/proof_active_lineage_attestation.rs`
  - proves accepted downstream attestation over admitted active lineage
  - proves stable repeated attestation receipt emission
  - proves fail-closed rejection for:
    - missing activation receipt
    - contract that does not require admitted lineage
    - corrupted supersession chain receipt
  - proves no attestation receipt publication on rejected paths
- `tests/slice29_lineage_consumption.rs`
- `scripts/verify_slice_29.sh`

## New expected artifacts

- `target/proof_artifacts/slice29_lineage_consumption/current/consumer_contract.json`
- `target/proof_artifacts/slice29_lineage_consumption/current/attestation_receipt.json`
- `target/proof_artifacts/slice29_lineage_consumption/lineage_attestation_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod lineage_consumption;
```

## Notes

This slice proves downstream consumers cannot legitimately consume lineage state without a typed contract and a deterministic attestation bound to the active lineage activation receipt.
