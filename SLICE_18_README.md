# Slice 18 bundle — signed trust envelope and import authorization proof

This bundle is intentionally **additive-first** because I could not inspect the live repo files from here. It gives you the new slice surface without overwriting your proven Slice 12–17 files.

## What this bundle adds

- `src/trust_envelope.rs`
  - deterministic companion trust-envelope generation for `slice14_export.zip`
  - controlled signer profiles
  - signature verification
  - zip-to-envelope digest matching
- `src/import_authorization.rs`
  - library-owned import authorization policy
  - authorization receipt typing
  - authorization decision helper
- `src/bin/proof_trust_envelope.rs`
  - emits `target/proof_artifacts/slice14_export.zip.trust_envelope.json`
- `src/bin/proof_import_authorize.rs`
  - proves trusted import authorization succeeds
  - proves missing envelope, rogue signer, invalid signature, and zip-sha mismatch fail closed
  - emits:
    - `target/proof_artifacts/slice18_import_authorization/current/authorization_receipt.json`
    - `target/proof_artifacts/slice18_import_authorization/authorization_report.json`
- `tests/slice18_trust_envelope.rs`
- `scripts/verify_slice_18.sh`

## Intentional constraint

The signing surface in this slice is a **deterministic proof signer model** built from signer-specific secrets and SHA-256, not a full asymmetric key system. That keeps the slice bounded and deterministic while still proving:

- signed companion envelope exists
- signer identity is checked
- signer authorization is policy-controlled
- zip bytes must match the envelope digest
- authorization fails closed on tamper or unauthorized signer

## Required one-time wiring

Add these two exports to `src/lib.rs` if they are not already present:

```rust
pub mod trust_envelope;
pub mod import_authorization;
```

## Expected new artifacts after verify

- `target/proof_artifacts/slice14_export.zip.trust_envelope.json`
- `target/proof_artifacts/slice18_import_authorization/current/authorization_receipt.json`
- `target/proof_artifacts/slice18_import_authorization/authorization_report.json`

## Suggested next slice after this bundle is green

Gate `proof_import_rehydrate` behind successful authorization receipt publication so authorization becomes part of the live import path rather than only the proof path.
