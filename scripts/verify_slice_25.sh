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

bundle_hash_1="$(sha256sum target/proof_artifacts/slice25_lineage_bundle/lineage_bundle_report.json | awk '{print $1}')"
cargo run --bin proof_import_lineage_bundle >/dev/null
bundle_hash_2="$(sha256sum target/proof_artifacts/slice25_lineage_bundle/lineage_bundle_report.json | awk '{print $1}')"
if [[ "$bundle_hash_1" != "$bundle_hash_2" ]]; then
  echo "lineage bundle report hash changed across repeated emission"
  exit 1
fi

test -f target/proof_artifacts/slice25_lineage_bundle/current/lineage_bundle_manifest.json
test -f target/proof_artifacts/slice25_lineage_bundle/current/lineage_bundle_envelope.json

echo "Slice 25 verification passed"
