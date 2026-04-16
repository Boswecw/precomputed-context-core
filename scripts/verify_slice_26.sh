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
cargo run --bin proof_import_supersession_chain
cargo run --bin proof_import_lineage_bundle
cargo run --bin proof_import_lineage_bundle_intake

intake_hash_1="$(sha256sum target/proof_artifacts/slice26_lineage_bundle_intake/lineage_bundle_intake_report.json | awk '{print $1}')"
cargo run --bin proof_import_lineage_bundle_intake >/dev/null
intake_hash_2="$(sha256sum target/proof_artifacts/slice26_lineage_bundle_intake/lineage_bundle_intake_report.json | awk '{print $1}')"
if [[ "$intake_hash_1" != "$intake_hash_2" ]]; then
  echo "lineage bundle intake report hash changed across repeated emission"
  exit 1
fi

test -f target/proof_artifacts/slice26_lineage_bundle_intake/current/intake_receipt.json
test -f target/proof_artifacts/slice26_lineage_bundle_intake/current/bundle/lineage_bundle_manifest.json
test -f target/proof_artifacts/slice26_lineage_bundle_intake/current/bundle/lineage_bundle_envelope.json

echo "Slice 26 verification passed"
