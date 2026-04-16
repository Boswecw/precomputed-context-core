# Slice 31 bundle — consumer acknowledgment receipt and return-channel closure proof

This bundle adds downstream consumer acknowledgment over the bounded handoff package and a closure receipt that proves the return channel closes only when acknowledgment continuity matches the exact published handoff receipt.

## What Slice 31 adds

- `src/consumer_acknowledgment.rs`
  - bounded handoff package validation
  - consumer acknowledgment receipt contract
  - return-channel closure receipt contract
- `src/bin/proof_consumer_acknowledgment.rs`
  - proves deterministic repeated acknowledgment and closure receipt emission
  - proves fail-closed rejection for:
    - missing handoff receipt
    - bounded package tamper
    - acknowledgment continuity tamper
  - proves no acknowledgment/closure publication on rejected paths
- `tests/slice31_consumer_acknowledgment.rs`
- `scripts/verify_slice_31.sh`

## New expected artifacts

- `target/proof_artifacts/slice31_consumer_acknowledgment/current/consumer_acknowledgment_receipt.json`
- `target/proof_artifacts/slice31_consumer_acknowledgment/current/return_channel_closure_receipt.json`
- `target/proof_artifacts/slice31_consumer_acknowledgment/consumer_acknowledgment_report.json`

## Required lib wiring

Add this export to `src/lib.rs` if it is not already present:

```rust
pub mod consumer_acknowledgment;
```
