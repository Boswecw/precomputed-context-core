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
cargo run --bin proof_import_repromotion

repromotion_hash_1="$(sha256sum target/proof_artifacts/slice23_repromotion/repromotion_report.json | awk '{print $1}')"
cargo run --bin proof_import_repromotion >/dev/null
repromotion_hash_2="$(sha256sum target/proof_artifacts/slice23_repromotion/repromotion_report.json | awk '{print $1}')"
if [[ "$repromotion_hash_1" != "$repromotion_hash_2" ]]; then
  echo "re-promotion report hash changed across repeated emission"
  exit 1
fi

test -f target/proof_artifacts/slice23_repromotion/current/re_promotion_receipt.json
test -f target/proof_artifacts/slice23_repromotion/current/import_receipt.json

echo "Slice 23 verification passed"
