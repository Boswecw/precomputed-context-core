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

rehydrate_hash_1="$(sha256sum target/proof_artifacts/slice27_lineage_rehydrate/lineage_rehydrate_report.json | awk '{print $1}')"
cargo run --bin proof_imported_lineage_rehydrate >/dev/null
rehydrate_hash_2="$(sha256sum target/proof_artifacts/slice27_lineage_rehydrate/lineage_rehydrate_report.json | awk '{print $1}')"
if [[ "$rehydrate_hash_1" != "$rehydrate_hash_2" ]]; then
  echo "lineage rehydrate report hash changed across repeated emission"
  exit 1
fi

test -f target/proof_artifacts/slice27_lineage_rehydrate/current/rehydrate_receipt.json
test -f target/proof_artifacts/slice27_lineage_rehydrate/current/lineage_state/promotion_receipt.json
test -f target/proof_artifacts/slice27_lineage_rehydrate/current/lineage_state/rollback_receipt.json
test -f target/proof_artifacts/slice27_lineage_rehydrate/current/lineage_state/re_promotion_receipt.json
test -f target/proof_artifacts/slice27_lineage_rehydrate/current/lineage_state/supersession_chain_receipt.json

echo "Slice 27 verification passed"
