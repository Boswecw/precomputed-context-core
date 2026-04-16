# Canvas 13 — Slice 12 Repo File Map and Work Order v1

Date: 2026-04-15
Time: 20:05 ET
Repo: `Boswecw/precomputed-context-core`
Scope: `Suggested file map and work order for Slice 12`

## Goal

Provide a concrete repo work order for implementing Slice 12 without letting the slice sprawl into Slice 13 or Slice 14 work.

## Existing files likely to change

### `src/lib.rs`
Add exports for the new durable evidence and replay modules.

### `src/bin/proof_check.rs`
Extend output to summarize durable evidence emission and replay equivalence.

### `src/proof/scenario.rs`
Keep the live governed-flow scenario, but add hooks so it can emit evidence records during the scenario.

### `src/events.rs`
Do not redefine event semantics here. Only add the minimal bridges needed to emit durable intake evidence.

### `src/invalidation_engine.rs`
Emit artifact and packet outcome evidence after the governed result is computed.

### `src/remediation_flow.rs`
Emit remediation evidence after the plan is generated.

## New files recommended

### `src/durable_evidence.rs`
Own record types and record validation.

### `src/evidence_store.rs`
Own append-only deterministic persistence.

### `src/evidence_bundle.rs`
Own bundle assembly for replay and proof artifact export.

### `src/replay.rs`
Own replay loading, replay execution, and final equivalence checks.

### `src/proof/replay_scenario.rs`
Own replay proof coverage.

## Suggested implementation sequence

### Step 1
Create `src/durable_evidence.rs`.

Deliverables:

- record structs
- enums for admission results
- validators
- serde support

### Step 2
Create `src/evidence_store.rs`.

Deliverables:

- append-only writer
- deterministic file naming
- deterministic serialization order
- proof-artifact root management

### Step 3
Wire event receipt emission from `src/events.rs`.

Deliverables:

- accepted receipt writes
- duplicate receipt writes
- poison receipt writes
- coalesced batch writes

### Step 4
Wire governed outcome emission from `src/invalidation_engine.rs` and `src/remediation_flow.rs`.

Deliverables:

- artifact invalidation record writes
- packet reevaluation record writes
- remediation record writes

### Step 5
Create `src/evidence_bundle.rs`.

Deliverables:

- replay bundle manifest builder
- stable bundle digest
- file export helpers

### Step 6
Create `src/replay.rs` and `src/proof/replay_scenario.rs`.

Deliverables:

- replay loader
- replay executor
- equivalence assertions
- tamper detection checks

### Step 7
Extend `src/bin/proof_check.rs`.

Deliverables:

- durable evidence summary
- replay equivalence summary
- nonzero exit on replay failure

## Guardrails

1. Do not put semantic truth into the evidence store.
2. Do not let replay mutate historical evidence.
3. Do not let the slice introduce consumer admission behavior yet.
4. Do not let repo discovery sneak in through proof helpers.
5. Do not let proof output depend on wall-clock randomness without deterministic control.

## Definition of done

The slice is done only when a fresh run can:

1. execute the governed flow
2. emit durable evidence
3. assemble a replay bundle
4. replay from stored evidence only
5. prove final-state equivalence
6. fail closed on tampered evidence
