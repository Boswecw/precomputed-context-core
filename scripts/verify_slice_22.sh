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
cargo run --bin proof_import_rehydrate_authorized
cargo run --bin proof_import_roundtrip
cargo run --bin proof_import_promotion
cargo run --bin proof_import_revocation

revocation_hash_1="$(sha256sum target/proof_artifacts/slice22_revocation/revocation_report.json | awk '{print $1}')"
cargo run --bin proof_import_revocation >/dev/null
revocation_hash_2="$(sha256sum target/proof_artifacts/slice22_revocation/revocation_report.json | awk '{print $1}')"
if [[ "$revocation_hash_1" != "$revocation_hash_2" ]]; then
  echo "revocation report hash changed across repeated emission"
  exit 1
fi

test -f target/proof_artifacts/slice22_revocation/current/rollback_receipt.json
if [[ -f target/proof_artifacts/slice22_revocation/current/import_receipt.json ]]; then
  echo "promoted import receipt should not exist after rollback"
  exit 1
fi

echo "Slice 22 verification passed"
