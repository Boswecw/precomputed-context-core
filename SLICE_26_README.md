# Slice 26 bundle — lineage bundle intake and external verification proof

This bundle adds controlled intake for the portable signed lineage bundle proven in Slice 25.

## What Slice 26 adds

- `src/lineage_bundle_intake.rs`
  - typed lineage bundle intake receipt contract
  - intake verification against the external Slice 25 lineage bundle
  - controlled intake workspace publication
  - deterministic intake receipt emission
- `src/bin/proof_import_lineage_bundle_intake.rs`
  - proves accepted intake from the external lineage bundle
  - proves stable repeated intake receipt emission
  - proves fail-closed rejection for:
    - missing envelope
    - extra bundle member
    - bundle member sha mismatch
  - proves no intake receipt publication on rejected paths
- `tests/slice26_lineage_bundle_intake.rs`
- `scripts/verify_slice_26.sh`

## New expected artifacts

- `target/proof_artifacts/slice26_lineage_bundle_intake/current/intake_receipt.json`
- `target/proof_artifacts/slice26_lineage_bundle_intake/current/bundle/lineage_bundle_manifest.json`
- `target/proof_artifacts/slice26_lineage_bundle_intake/current/bundle/lineage_bundle_envelope.json`
- `target/proof_artifacts/slice26_lineage_bundle_intake/lineage_bundle_intake_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod lineage_bundle_intake;
```

## Notes

This slice proves the signed lineage bundle is not only exportable, but also admissible as external evidence under a deterministic intake workflow.
