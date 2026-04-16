## 40. Validation and Proof

Validation is evidence-based and fail-closed.

### Minimum validation posture

- `cargo test` must pass
- slice verifier scripts must pass
- proof binaries must emit deterministic artifacts where repeatability is part of the contract
- tamper scenarios must reject cleanly
- failed validation paths must not publish success receipts or reports

### Current terminal verifier

The current proof chain culminates in `bash scripts/verify_slice_36.sh`, which exercises the full end-to-end chain through terminal consumer import and program capstone reporting.
