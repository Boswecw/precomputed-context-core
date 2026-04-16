#!/usr/bin/env bash
set -euo pipefail

cargo test
cargo run --bin proof_check -- --export-package
cargo run --bin proof_zip
cargo run --bin proof_trust_envelope
cargo run --bin proof_policy_surface
cargo run --bin proof_intake_check
cargo run --bin proof_import_rehydrate
cargo run --bin proof_import_authorize
cargo run --bin proof_import_roundtrip

policy_hash_1="$(sha256sum target/proof_artifacts/slice19_policy/policy_surface_report.json | awk '{print $1}')"
cargo run --bin proof_policy_surface >/dev/null
policy_hash_2="$(sha256sum target/proof_artifacts/slice19_policy/policy_surface_report.json | awk '{print $1}')"
if [[ "$policy_hash_1" != "$policy_hash_2" ]]; then
  echo "policy surface report hash changed across repeated emission"
  exit 1
fi

auth_hash_1="$(sha256sum target/proof_artifacts/slice19_policy/authorization_report.json | awk '{print $1}')"
cargo run --bin proof_import_authorize >/dev/null
auth_hash_2="$(sha256sum target/proof_artifacts/slice19_policy/authorization_report.json | awk '{print $1}')"
if [[ "$auth_hash_1" != "$auth_hash_2" ]]; then
  echo "authorization report hash changed across repeated emission"
  exit 1
fi

test -f target/proof_artifacts/slice19_policy/import_authorization_policy.json
test -f target/proof_artifacts/slice19_policy/current/authorization_receipt.json
test -f target/proof_artifacts/slice19_policy/current/authorization_evidence_link.json

echo "Slice 19 verification passed"
