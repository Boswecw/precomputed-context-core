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

gate_hash_1="$(sha256sum target/proof_artifacts/slice20_rehydrate_gate/rehydrate_gate_report.json | awk '{print $1}')"
cargo run --bin proof_import_rehydrate_authorized >/dev/null
gate_hash_2="$(sha256sum target/proof_artifacts/slice20_rehydrate_gate/rehydrate_gate_report.json | awk '{print $1}')"
if [[ "$gate_hash_1" != "$gate_hash_2" ]]; then
  echo "rehydrate gate report hash changed across repeated emission"
  exit 1
fi

test -f target/proof_artifacts/slice20_rehydrate_gate/current/gate_receipt.json
test -f target/proof_artifacts/slice20_rehydrate_gate/current/import_receipt.json

echo "Slice 20 verification passed"
