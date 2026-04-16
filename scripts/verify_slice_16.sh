#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

for cmd in cargo zip unzip sha256sum mktemp; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "missing required command: $cmd" >&2
    exit 1
  fi
done

cargo test
cargo run --bin proof_check -- --export-package
cargo run --bin proof_zip
cargo run --bin proof_intake_check
cargo run --bin proof_import_rehydrate

WORKSPACE="target/proof_artifacts/slice16_import/current"
RECEIPT="$WORKSPACE/import_receipt.json"

for path in \
  "$RECEIPT" \
  "$WORKSPACE/package/README.txt" \
  "$WORKSPACE/package/package_index.json" \
  "$WORKSPACE/package/governed_flow_report.json" \
  "$WORKSPACE/package/replay_bundle_manifest.json" \
  "$WORKSPACE/package/replay_report.json" \
  "$WORKSPACE/rehydrated/package_meta/README.txt" \
  "$WORKSPACE/rehydrated/package_meta/package_index.json" \
  "$WORKSPACE/rehydrated/governed_flow/governed_flow_report.json" \
  "$WORKSPACE/rehydrated/replay/replay_bundle_manifest.json" \
  "$WORKSPACE/rehydrated/replay/replay_report.json"
  do
  if [[ ! -f "$path" ]]; then
    echo "missing expected slice16 output: $path" >&2
    exit 1
  fi
done

first_receipt_hash="$(sha256sum "$RECEIPT" | awk '{print $1}')"
cargo run --bin proof_import_rehydrate
second_receipt_hash="$(sha256sum "$RECEIPT" | awk '{print $1}')"
if [[ "$first_receipt_hash" != "$second_receipt_hash" ]]; then
  echo "slice16 receipt drift detected" >&2
  echo "first:  $first_receipt_hash" >&2
  echo "second: $second_receipt_hash" >&2
  exit 1
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

mkdir -p "$TMP_DIR/missing"
unzip -q target/proof_artifacts/slice14_export.zip -d "$TMP_DIR/missing"
rm -f "$TMP_DIR/missing/replay_bundle_manifest.json"
(
  cd "$TMP_DIR/missing"
  zip -q -X "$TMP_DIR/missing.zip" README.txt governed_flow_report.json package_index.json replay_report.json
)
sha256sum "$TMP_DIR/missing.zip" | awk '{print $1 "  missing.zip"}' > "$TMP_DIR/missing.zip.sha256"
if cargo run --bin proof_import_rehydrate -- --zip "$TMP_DIR/missing.zip" --sha "$TMP_DIR/missing.zip.sha256" --workspace "$TMP_DIR/import_missing"; then
  echo "missing-member import adversarial check unexpectedly passed" >&2
  exit 1
fi
if [[ -e "$TMP_DIR/import_missing/import_receipt.json" ]]; then
  echo "failed import left behind a receipt" >&2
  exit 1
fi

echo "slice16 verification passed"
echo "  workspace: $WORKSPACE"
echo "  receipt: $RECEIPT"
echo "  receipt_hash: $second_receipt_hash"
