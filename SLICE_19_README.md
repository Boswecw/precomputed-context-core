# Slice 19 bundle — policy surface and receipt-linked authorization evidence

This bundle is the next additive slice on top of your green Slice 18 state.

## What Slice 19 adds

- `src/import_policy.rs`
  - library-owned authorization policy contract
  - deterministic policy file emission
  - policy hashing for receipt/report proof
- `src/authorization_evidence.rs`
  - deterministic evidence link binding:
    - policy file
    - trust envelope
    - authorization receipt
    - import receipt when present
- upgraded `src/import_authorization.rs`
  - authorization receipt now carries `policy_sha256`
  - authorization receipt now carries `trust_envelope_sha256`
  - policy file loading surface added
- `src/bin/proof_policy_surface.rs`
  - emits deterministic policy file and policy surface report
- upgraded `src/bin/proof_import_authorize.rs`
  - authorizes using policy file
  - emits receipt-linked authorization evidence
  - proves fail-closed rejection for:
    - missing policy
    - signer removed by policy
    - policy scope mismatch
    - invalid signature
- `tests/slice19_policy_surface.rs`
- `scripts/verify_slice_19.sh`

## New expected artifacts

- `target/proof_artifacts/slice19_policy/import_authorization_policy.json`
- `target/proof_artifacts/slice19_policy/policy_surface_report.json`
- `target/proof_artifacts/slice19_policy/current/authorization_receipt.json`
- `target/proof_artifacts/slice19_policy/current/authorization_evidence_link.json`
- `target/proof_artifacts/slice19_policy/authorization_report.json`

## Required lib wiring

Add these exports to `src/lib.rs` if they are not already present:

```rust
pub mod import_policy;
pub mod authorization_evidence;
```

## Suggested next slice after Slice 19

Make `proof_import_rehydrate` fail closed unless a valid authorization receipt exists and matches the policy/evidence bundle for the current package.
