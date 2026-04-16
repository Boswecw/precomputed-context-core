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

ZIP_PATH="target/proof_artifacts/slice14_export.zip"
SHA_PATH="target/proof_artifacts/slice14_export.zip.sha256"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

cp "$ZIP_PATH" "$TMP_DIR/extra.zip"
(
  cd "$TMP_DIR"
  printf 'rogue\n' > rogue.txt
  zip -q extra.zip rogue.txt
)
sha256sum "$TMP_DIR/extra.zip" | awk '{print $1 "  extra.zip"}' > "$TMP_DIR/extra.zip.sha256"
if cargo run --bin proof_intake_check -- --zip "$TMP_DIR/extra.zip" --sha "$TMP_DIR/extra.zip.sha256"; then
  echo "extra-member adversarial check unexpectedly passed" >&2
  exit 1
fi

mkdir -p "$TMP_DIR/missing"
unzip -q "$ZIP_PATH" -d "$TMP_DIR/missing"
rm -f "$TMP_DIR/missing/README.txt"
(
  cd "$TMP_DIR/missing"
  zip -q -X "$TMP_DIR/missing.zip" governed_flow_report.json package_index.json replay_bundle_manifest.json replay_report.json
)
sha256sum "$TMP_DIR/missing.zip" | awk '{print $1 "  missing.zip"}' > "$TMP_DIR/missing.zip.sha256"
if cargo run --bin proof_intake_check -- --zip "$TMP_DIR/missing.zip" --sha "$TMP_DIR/missing.zip.sha256"; then
  echo "missing-member adversarial check unexpectedly passed" >&2
  exit 1
fi

printf '0000000000000000000000000000000000000000000000000000000000000000  slice14_export.zip\n' > "$TMP_DIR/wrong.sha256"
if cargo run --bin proof_intake_check -- --zip "$ZIP_PATH" --sha "$TMP_DIR/wrong.sha256"; then
  echo "sha-mismatch adversarial check unexpectedly passed" >&2
  exit 1
fi

echo "slice15 verification passed"
echo "  zip: $ZIP_PATH"
echo "  sha256: $SHA_PATH"
