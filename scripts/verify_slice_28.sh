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

activation_hash_1="$(sha256sum target/proof_artifacts/slice28_lineage_activation/lineage_activation_report.json | awk '{print $1}')"
cargo run --bin proof_lineage_activation >/dev/null
activation_hash_2="$(sha256sum target/proof_artifacts/slice28_lineage_activation/lineage_activation_report.json | awk '{print $1}')"
if [[ "$activation_hash_1" != "$activation_hash_2" ]]; then
  echo "lineage activation report hash changed across repeated emission"
  exit 1
fi

test -f target/proof_artifacts/slice28_lineage_activation/current/activation_receipt.json
test -f target/proof_artifacts/slice28_lineage_activation/current/active_lineage/promotion_receipt.json
test -f target/proof_artifacts/slice28_lineage_activation/current/active_lineage/rollback_receipt.json
test -f target/proof_artifacts/slice28_lineage_activation/current/active_lineage/re_promotion_receipt.json
test -f target/proof_artifacts/slice28_lineage_activation/current/active_lineage/supersession_chain_receipt.json

echo "Slice 28 verification passed"
