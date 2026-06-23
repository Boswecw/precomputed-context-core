#!/usr/bin/env bash
set -euo pipefail

cargo test

cargo run --bin proof_context_assembly_continuity

report_path="target/proof_artifacts/context_assembly_continuity/continuity_manifest_report.json"

hash_1="$(sha256sum "$report_path" | awk '{print $1}')"
cargo run --bin proof_context_assembly_continuity >/dev/null
hash_2="$(sha256sum "$report_path" | awk '{print $1}')"
if [[ "$hash_1" != "$hash_2" ]]; then
  echo "continuity manifest report hash changed across repeated emission"
  exit 1
fi

test -f "$report_path"

echo "Continuity context-assembly verification passed"
