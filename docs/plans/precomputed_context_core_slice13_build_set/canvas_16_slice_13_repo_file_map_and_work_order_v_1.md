# Canvas 16 — Slice 13 Repo File Map and Work Order v1

**Date:** 2026-04-15  
**Time:** 9:35 PM ET

## Objective

Implement deterministic proof artifact export on top of the now-green Slice 12 replay foundation.

## File map

### New files

- `src/proof/export.rs`

### Files to update

- `src/proof/mod.rs`
- `src/lib.rs`
- `src/bin/proof_check.rs`

## Work order

### Step 1

Create `src/proof/export.rs`.

This file should:

- define the package structs
- define the export function
- write stable JSON outputs
- write `README.txt`
- include export tests

### Step 2

Update `src/proof/mod.rs`.

This file should:

- expose the export module
- re-export the export function and package types

### Step 3

Update `src/lib.rs`.

This file should:

- re-export proof export items from the crate root

### Step 4

Update `src/bin/proof_check.rs`.

This file should:

- keep the current default console behavior
- add `--export-package` support
- call the library export function instead of owning export logic directly

### Step 5

Run verification.

Commands:

- `cargo test`
- `cargo run --bin proof_check`
- `cargo run --bin proof_check -- --export-package`

### Step 6

Inspect emitted artifacts.

Recommended command:

`find target/proof_artifacts/slice13_export -maxdepth 1 -type f | sort`

## Acceptance standard

Do not move beyond Slice 13 until:

- tests are green
- export command is green
- package files are all present
- index file parses
- replay report shows equivalent replay
