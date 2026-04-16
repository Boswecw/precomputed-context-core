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

promotion_hash_1="$(sha256sum target/proof_artifacts/slice21_promotion/promotion_report.json | awk '{print $1}')"
cargo run --bin proof_import_promotion >/dev/null
promotion_hash_2="$(sha256sum target/proof_artifacts/slice21_promotion/promotion_report.json | awk '{print $1}')"
if [[ "$promotion_hash_1" != "$promotion_hash_2" ]]; then
  echo "promotion report hash changed across repeated emission"
  exit 1
fi

test -f target/proof_artifacts/slice21_promotion/current/promotion_receipt.json
test -f target/proof_artifacts/slice21_promotion/current/import_receipt.json

echo "Slice 21 verification passed"
