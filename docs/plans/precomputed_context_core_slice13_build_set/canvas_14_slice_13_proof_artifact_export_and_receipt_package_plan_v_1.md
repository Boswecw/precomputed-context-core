# Canvas 14 — Slice 13 Proof Artifact Export and Receipt Package Plan v1

**Date:** 2026-04-15  
**Time:** 9:35 PM ET

## Purpose

Slice 13 formalizes the operator-consumable export surface for the proof program now that Slice 12 replay is green. The next governed step is to move from “proof exists and replays correctly” to “proof emits a deterministic receipt package that an operator can inspect, archive, and compare between runs.”

## Current verified starting point

The crate already proves:

- typed governed contracts
- event dedupe and coalescing substrate
- invalidation and remediation flow proof
- durable evidence records
- append-only evidence store
- replay bundle assembly
- replay verification
- library-owned replay proof
- binary proof surface

## Slice 13 objective

Add a deterministic export package for proof runs so `proof_check` can emit a governed artifact set under a stable folder with a stable index and stable summaries.

## In scope

- proof artifact package root under `target/proof_artifacts/`
- deterministic run folder naming for exported proof packages
- JSON summary emission for governed flow proof
- JSON summary emission for replay proof
- package index file that lists emitted artifacts and paths
- one command path for operator export from `proof_check`
- fail-closed behavior when export prerequisites are invalid
- tests for export structure and emitted file contents

## Not in scope

- remote upload
- ForgeCommand UI integration
- durable database persistence
- RBAC
- long-term retention policy
- cross-repo aggregation

## Target package shape

Recommended deterministic package root:

`target/proof_artifacts/slice13_export/`

Recommended emitted shape:

- `governed_flow_report.json`
- `replay_report.json`
- `replay_bundle_manifest.json`
- `package_index.json`
- `README.txt`

## Determinism rules

- file names must be fixed and predictable
- index entries must use stable ordering
- replay bundle id must be copied exactly from the replay proof
- proof package must be rebuilt from current truth, not mutated copies
- export should overwrite only the package root selected for that run

## Required implementation additions

1. Add a proof export module under `src/proof/`.
2. Define package summary structs with stable serialized field order.
3. Add export helpers that create the package root and write JSON files.
4. Update `proof_check` to support a default console proof path and an export path.
5. Add tests that verify emitted files exist and parse.

## Suggested module additions

- `src/proof/export.rs`
- tests inside `src/proof/export.rs`
- `src/bin/proof_check.rs` upgrade for package export

## Proof command target

Primary command remains:

`cargo run --bin proof_check`

Slice 13 adds an export mode such as:

`cargo run --bin proof_check -- --export-package`

## Done condition

Slice 13 is done when the crate can emit a deterministic proof package that contains both governed flow and replay proof artifacts, and tests prove the package structure and contents are stable.
