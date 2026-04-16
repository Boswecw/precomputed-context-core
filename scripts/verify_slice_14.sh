#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

for cmd in cargo zip unzip sha256sum; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "missing required command: $cmd" >&2
    exit 1
  fi
done

cargo test
cargo run --bin proof_check -- --export-package
cargo run --bin proof_zip

ZIP_PATH="target/proof_artifacts/slice14_export.zip"
SHA_PATH="target/proof_artifacts/slice14_export.zip.sha256"

if [[ ! -f "$ZIP_PATH" ]]; then
  echo "missing zip artifact: $ZIP_PATH" >&2
  exit 1
fi

if [[ ! -f "$SHA_PATH" ]]; then
  echo "missing sha256 artifact: $SHA_PATH" >&2
  exit 1
fi

first_hash="$(sha256sum "$ZIP_PATH" | awk '{print $1}')"
cargo run --bin proof_zip
second_hash="$(sha256sum "$ZIP_PATH" | awk '{print $1}')"

if [[ "$first_hash" != "$second_hash" ]]; then
  echo "slice14 archive hash drift detected" >&2
  echo "first:  $first_hash" >&2
  echo "second: $second_hash" >&2
  exit 1
fi

mapfile -t zip_members < <(unzip -Z1 "$ZIP_PATH")
required_members=(
  "governed_flow_report.json"
  "replay_report.json"
  "replay_bundle_manifest.json"
  "package_index.json"
  "README.txt"
)

for member in "${required_members[@]}"; do
  if ! printf '%s\n' "${zip_members[@]}" | grep -Fx "$member" >/dev/null; then
    echo "missing required member in zip: $member" >&2
    exit 1
  fi
done

echo "slice14 verification passed"
echo "  zip:    $ZIP_PATH"
echo "  sha256: $SHA_PATH"
echo "  hash:   $second_hash"
