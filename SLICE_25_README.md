# Slice 25 bundle — signed lineage bundle and export proof

This bundle adds a portable signed lineage bundle layer on top of the green Slice 24 supersession chain.

## What Slice 25 adds

- `src/lineage_bundle.rs`
  - typed lineage bundle manifest contract
  - typed lineage bundle envelope contract
  - deterministic bundle publication for:
    - Slice 21 promotion receipt
    - Slice 22 rollback receipt
    - Slice 23 re-promotion receipt
    - Slice 24 supersession chain receipt
  - deterministic envelope signature over the manifest hash
  - bundle verification enforcing exact member set, member sha integrity, manifest hash integrity, and signature integrity
- `src/bin/proof_import_lineage_bundle.rs`
  - proves portable signed lineage bundle publication
  - proves deterministic repeated manifest emission
  - proves fail-closed rejection for:
    - extra bundle member
    - envelope manifest-hash mismatch
    - bundle member sha mismatch
- `tests/slice25_lineage_bundle.rs`
- `scripts/verify_slice_25.sh`

## New expected artifacts

- `target/proof_artifacts/slice25_lineage_bundle/current/lineage_bundle_manifest.json`
- `target/proof_artifacts/slice25_lineage_bundle/current/lineage_bundle_envelope.json`
- `target/proof_artifacts/slice25_lineage_bundle/lineage_bundle_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod lineage_bundle;
```

## Notes

This slice makes the lineage proof portable. Promotion, rollback, re-promotion, and supersession can now travel as a signed, verifiable evidence bundle rather than only existing as local adjacent artifacts.
