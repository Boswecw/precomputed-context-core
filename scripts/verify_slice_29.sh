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

attestation_hash_1="$(sha256sum target/proof_artifacts/slice29_lineage_consumption/lineage_attestation_report.json | awk '{print $1}')"
cargo run --bin proof_active_lineage_attestation >/dev/null
attestation_hash_2="$(sha256sum target/proof_artifacts/slice29_lineage_consumption/lineage_attestation_report.json | awk '{print $1}')"
if [[ "$attestation_hash_1" != "$attestation_hash_2" ]]; then
  echo "lineage attestation report hash changed across repeated emission"
  exit 1
fi

test -f target/proof_artifacts/slice29_lineage_consumption/current/consumer_contract.json
test -f target/proof_artifacts/slice29_lineage_consumption/current/attestation_receipt.json

echo "Slice 29 verification passed"
