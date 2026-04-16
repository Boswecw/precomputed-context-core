# Slice 30 bundle — execution-readiness gate and bounded consumer handoff proof

This bundle adds the readiness gate that reconstructs and verifies downstream attestation before permitting a bounded consumer handoff package.

## What Slice 30 adds

- `src/consumer_handoff.rs`
  - execution-readiness validation over admitted activation + consumer contract + attestation continuity
  - bounded consumer handoff receipt contract
  - bounded handoff package publication with exact member set
- `src/bin/proof_consumer_handoff.rs`
  - proves readiness-approved handoff receipt emission
  - proves stable repeated handoff receipt emission
  - proves bounded package member set
  - proves fail-closed rejection for:
    - missing attestation receipt
    - attestation mismatch/tamper
    - missing activation receipt
  - proves no handoff receipt publication on rejected paths
- `tests/slice30_consumer_handoff.rs`
- `scripts/verify_slice_30.sh`

## New expected artifacts

- `target/proof_artifacts/slice30_consumer_handoff/current/handoff_receipt.json`
- `target/proof_artifacts/slice30_consumer_handoff/current/handoff_package/consumer_contract.json`
- `target/proof_artifacts/slice30_consumer_handoff/current/handoff_package/attestation_receipt.json`
- `target/proof_artifacts/slice30_consumer_handoff/current/handoff_package/activation_receipt.json`
- `target/proof_artifacts/slice30_consumer_handoff/current/handoff_package/supersession_chain_receipt.json`
- `target/proof_artifacts/slice30_consumer_handoff/consumer_handoff_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod consumer_handoff;
```
