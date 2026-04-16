# Canvas 15 — Slice 13 Verification Matrix v1

**Date:** 2026-04-15  
**Time:** 9:35 PM ET

## Verification goal

Prove that Slice 13 exports a deterministic operator-consumable proof package without weakening the current proof surface.

## Required checks

### 1. Existing proof regression

Command:

`cargo test`

Must prove:

- all existing contract tests remain green
- replay scenario proof remains green
- no regressions from export work

### 2. Default proof binary still works

Command:

`cargo run --bin proof_check`

Must prove:

- governed flow proof still prints
- Slice 12 replay proof still prints
- process exits zero

### 3. Export mode builds package

Command:

`cargo run --bin proof_check -- --export-package`

Must prove:

- package root is created
- governed flow report file exists
- replay report file exists
- replay bundle manifest file exists
- package index file exists
- README file exists

### 4. Package index parses and matches files

Must prove:

- index JSON parses
- listed files exist
- paths are stable relative paths
- package index order is deterministic

### 5. Governed flow report parses

Must prove:

- JSON parses into expected struct
- scenario id is present
- step list is present
- affected and unaffected ids are present

### 6. Replay report parses

Must prove:

- JSON parses into expected struct
- replay bundle id is present
- replay equivalence is true
- mismatch count is zero for golden path

### 7. Replay manifest copy parses

Must prove:

- manifest JSON parses
- replay bundle id matches replay report
- proof digest is present

### 8. Export rerun remains deterministic

Must prove:

- emitted file names remain unchanged
- index structure remains unchanged
- JSON keys remain stable

## Failure posture

Fail closed when:

- export root cannot be created
- required report cannot be serialized
- replay proof fails
- package index would reference a missing file

## Recommended test names

- `proof_export_writes_package_root`
- `proof_export_writes_index`
- `proof_export_writes_governed_flow_report`
- `proof_export_writes_replay_report`
- `proof_export_writes_manifest_copy`
- `proof_export_is_deterministic`
