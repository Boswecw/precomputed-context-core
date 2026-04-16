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
cargo run --bin proof_imported_lineage_rehydrate
cargo run --bin proof_lineage_activation
cargo run --bin proof_active_lineage_attestation
cargo run --bin proof_consumer_handoff
cargo run --bin proof_consumer_acknowledgment
cargo run --bin proof_downstream_release

release_hash_1="$(sha256sum target/proof_artifacts/slice32_downstream_release/downstream_release_report.json | awk '{print $1}')"
cargo run --bin proof_downstream_release >/dev/null
release_hash_2="$(sha256sum target/proof_artifacts/slice32_downstream_release/downstream_release_report.json | awk '{print $1}')"
if [[ "$release_hash_1" != "$release_hash_2" ]]; then
  echo "downstream release report hash changed across repeated emission"
  exit 1
fi

test -f target/proof_artifacts/slice32_downstream_release/current/downstream_release_receipt.json
test -f target/proof_artifacts/slice32_downstream_release/current/release_package/consumer_acknowledgment_receipt.json
test -f target/proof_artifacts/slice32_downstream_release/current/release_package/return_channel_closure_receipt.json

echo "Slice 32 verification passed"
