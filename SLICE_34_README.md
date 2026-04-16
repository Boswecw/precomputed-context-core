# Slice 34 bundle — release attestation and handoff-boundary export package proof

This bundle adds the handoff-boundary attestation proof over the Slice 32 downstream release package and the Slice 33 operator release-readiness artifacts.

## What Slice 34 adds

- `src/release_attestation.rs`
  - release attestation receipt contract
  - continuity validation between downstream release receipt and release-readiness receipt
  - continuity validation for packaged acknowledgment and closure receipts
  - handoff-boundary package publication
- `src/bin/proof_release_attestation.rs`
  - proves deterministic repeated attestation receipt/report emission
  - proves fail-closed rejection for:
    - missing operator release summary
    - readiness receipt continuity tamper
    - missing closure receipt in the downstream release package
  - proves no publication on rejected paths
- `tests/slice34_release_attestation.rs`
- `scripts/verify_slice_34.sh`

## New expected artifacts

- `target/proof_artifacts/slice34_release_attestation/current/release_attestation_receipt.json`
- `target/proof_artifacts/slice34_release_attestation/current/handoff_boundary_package/downstream_release_receipt.json`
- `target/proof_artifacts/slice34_release_attestation/current/handoff_boundary_package/release_readiness_receipt.json`
- `target/proof_artifacts/slice34_release_attestation/current/handoff_boundary_package/operator_release_summary.json`
- `target/proof_artifacts/slice34_release_attestation/current/handoff_boundary_package/consumer_acknowledgment_receipt.json`
- `target/proof_artifacts/slice34_release_attestation/current/handoff_boundary_package/return_channel_closure_receipt.json`
- `target/proof_artifacts/slice34_release_attestation/release_attestation_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod release_attestation;
```
