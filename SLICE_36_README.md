# Slice 36 bundle — terminal consumer import validation and program capstone proof

This bundle adds the final terminal-consumer validation layer over the Slice 35 sealed release bundle.

## What Slice 36 adds

- `src/terminal_consumer_import.rs`
  - terminal consumer import receipt contract
  - exact validated terminal bundle member-set enforcement
  - terminal boundary manifest hash verification
  - release attestation continuity verification at final consumer boundary
- `src/bin/proof_terminal_consumer_import.rs`
  - proves deterministic repeated terminal consumer receipt emission
  - proves fail-closed rejection for:
    - missing manifest
    - terminal bundle member drift
    - manifest hash tamper
  - proves no publication on rejected paths
  - emits final `program_capstone_report.json`
- `tests/slice36_terminal_consumer_import.rs`
- `scripts/verify_slice_36.sh`

## New expected artifacts

- `target/proof_artifacts/slice36_terminal_consumer/current/terminal_consumer_import_receipt.json`
- `target/proof_artifacts/slice36_terminal_consumer/current/validated_terminal_bundle/release_attestation_receipt.json`
- `target/proof_artifacts/slice36_terminal_consumer/current/validated_terminal_bundle/downstream_release_receipt.json`
- `target/proof_artifacts/slice36_terminal_consumer/current/validated_terminal_bundle/release_readiness_receipt.json`
- `target/proof_artifacts/slice36_terminal_consumer/current/validated_terminal_bundle/operator_release_summary.json`
- `target/proof_artifacts/slice36_terminal_consumer/current/validated_terminal_bundle/consumer_acknowledgment_receipt.json`
- `target/proof_artifacts/slice36_terminal_consumer/current/validated_terminal_bundle/return_channel_closure_receipt.json`
- `target/proof_artifacts/slice36_terminal_consumer/program_capstone_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod terminal_consumer_import;
```
