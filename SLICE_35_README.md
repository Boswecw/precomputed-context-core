# Slice 35 bundle — sealed release bundle and terminal boundary proof

This bundle adds the final sealed release bundle proof over the Slice 34 handoff-boundary package.

## What Slice 35 adds

- `src/sealed_release_bundle.rs`
  - terminal boundary manifest contract
  - sealed release receipt contract
  - exact handoff-boundary member-set validation
  - final sealed bundle publication for terminal consumer boundary
- `src/bin/proof_sealed_release_bundle.rs`
  - proves deterministic repeated sealed receipt emission
  - proves deterministic repeated terminal manifest emission
  - proves fail-closed rejection for:
    - missing attestation receipt
    - handoff-boundary member drift
    - attestation export-member-count tamper
  - proves no publication on rejected paths
- `tests/slice35_sealed_release_bundle.rs`
- `scripts/verify_slice_35.sh`

## New expected artifacts

- `target/proof_artifacts/slice35_sealed_release/current/sealed_release_receipt.json`
- `target/proof_artifacts/slice35_sealed_release/current/terminal_boundary_manifest.json`
- `target/proof_artifacts/slice35_sealed_release/current/sealed_release_bundle/release_attestation_receipt.json`
- `target/proof_artifacts/slice35_sealed_release/current/sealed_release_bundle/downstream_release_receipt.json`
- `target/proof_artifacts/slice35_sealed_release/current/sealed_release_bundle/release_readiness_receipt.json`
- `target/proof_artifacts/slice35_sealed_release/current/sealed_release_bundle/operator_release_summary.json`
- `target/proof_artifacts/slice35_sealed_release/current/sealed_release_bundle/consumer_acknowledgment_receipt.json`
- `target/proof_artifacts/slice35_sealed_release/current/sealed_release_bundle/return_channel_closure_receipt.json`
- `target/proof_artifacts/slice35_sealed_release/sealed_release_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod sealed_release_bundle;
```
