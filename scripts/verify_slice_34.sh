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
cargo run --bin proof_release_readiness
cargo run --bin proof_release_attestation

attestation_hash_1="$(sha256sum target/proof_artifacts/slice34_release_attestation/release_attestation_report.json | awk '{print $1}')"
cargo run --bin proof_release_attestation >/dev/null
attestation_hash_2="$(sha256sum target/proof_artifacts/slice34_release_attestation/release_attestation_report.json | awk '{print $1}')"
if [[ "$attestation_hash_1" != "$attestation_hash_2" ]]; then
  echo "release attestation report hash changed across repeated emission"
  exit 1
fi

test -f target/proof_artifacts/slice34_release_attestation/current/release_attestation_receipt.json
test -f target/proof_artifacts/slice34_release_attestation/current/handoff_boundary_package/downstream_release_receipt.json
test -f target/proof_artifacts/slice34_release_attestation/current/handoff_boundary_package/release_readiness_receipt.json
test -f target/proof_artifacts/slice34_release_attestation/current/handoff_boundary_package/operator_release_summary.json
test -f target/proof_artifacts/slice34_release_attestation/current/handoff_boundary_package/consumer_acknowledgment_receipt.json
test -f target/proof_artifacts/slice34_release_attestation/current/handoff_boundary_package/return_channel_closure_receipt.json

echo "Slice 34 verification passed"
