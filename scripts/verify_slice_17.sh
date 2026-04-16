#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

for cmd in cargo unzip sha256sum mktemp python3; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "missing required command: $cmd" >&2
    exit 1
  fi
done

bash scripts/apply_slice_17.sh
cargo test
cargo run --bin proof_check -- --export-package
cargo run --bin proof_zip
cargo run --bin proof_intake_check
cargo run --bin proof_import_rehydrate
cargo run --bin proof_import_roundtrip

WORKSPACE="target/proof_artifacts/slice16_import/current"
REPORT="$WORKSPACE/roundtrip_report.json"

if [[ ! -f "$REPORT" ]]; then
  echo "missing roundtrip report: $REPORT" >&2
  exit 1
fi

first_hash="$(sha256sum "$REPORT" | awk '{print $1}')"
cargo run --bin proof_import_roundtrip
second_hash="$(sha256sum "$REPORT" | awk '{print $1}')"
if [[ "$first_hash" != "$second_hash" ]]; then
  echo "slice17 roundtrip report drift detected" >&2
  echo "first:  $first_hash" >&2
  echo "second: $second_hash" >&2
  exit 1
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
cp -a "$WORKSPACE" "$TMP_DIR/bad_workspace"
rm -f "$TMP_DIR/bad_workspace/roundtrip_report.json"
printf '\nrogue\n' >> "$TMP_DIR/bad_workspace/rehydrated/replay/replay_report.json"
if cargo run --bin proof_import_roundtrip -- --workspace "$TMP_DIR/bad_workspace"; then
  echo "corrupted rehydrated workspace unexpectedly passed" >&2
  exit 1
fi
if [[ -e "$TMP_DIR/bad_workspace/roundtrip_report.json" ]]; then
  echo "failed roundtrip validation left behind a report" >&2
  exit 1
fi

echo "slice17 verification passed"
echo "  workspace: $WORKSPACE"
echo "  report: $REPORT"
echo "  report_hash: $second_hash"
