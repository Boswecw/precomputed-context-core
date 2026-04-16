# Slice 32 bundle — downstream publication eligibility gate and bounded release package proof

This bundle adds the downstream publication gate that requires intact consumer acknowledgment closure continuity before emitting a bounded release package.

## What Slice 32 adds

- `src/downstream_release.rs`
  - downstream publication eligibility receipt contract
  - closure continuity reconstruction and verification
  - bounded release package publication
- `src/bin/proof_downstream_release.rs`
  - proves deterministic repeated release receipt/report emission
  - proves exact bounded release package member set
  - proves fail-closed rejection for:
    - missing closure receipt
    - closure drift
    - handoff package member drift
  - proves no release publication on rejected paths
- `tests/slice32_downstream_release.rs`
- `scripts/verify_slice_32.sh`

## New expected artifacts

- `target/proof_artifacts/slice32_downstream_release/current/downstream_release_receipt.json`
- `target/proof_artifacts/slice32_downstream_release/current/release_package/activation_receipt.json`
- `target/proof_artifacts/slice32_downstream_release/current/release_package/attestation_receipt.json`
- `target/proof_artifacts/slice32_downstream_release/current/release_package/consumer_contract.json`
- `target/proof_artifacts/slice32_downstream_release/current/release_package/supersession_chain_receipt.json`
- `target/proof_artifacts/slice32_downstream_release/current/release_package/consumer_acknowledgment_receipt.json`
- `target/proof_artifacts/slice32_downstream_release/current/release_package/return_channel_closure_receipt.json`
- `target/proof_artifacts/slice32_downstream_release/downstream_release_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod downstream_release;
```
