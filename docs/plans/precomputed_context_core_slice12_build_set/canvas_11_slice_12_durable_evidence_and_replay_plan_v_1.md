# Canvas 11 — Slice 12 Durable Evidence and Replay Plan v1

Date: 2026-04-15
Time: 20:05 ET
Repo: `Boswecw/precomputed-context-core`
Scope: `Slice 12 — Durable evidence and replay substrate`

## Purpose

This canvas defines the next governed implementation slice for `precomputed-context-core` after the currently proven authority, lifecycle, contract, event, invalidation, remediation, and governed-flow proof substrate.

The purpose of Slice 12 is to convert the current in-memory proof-capable control core into a replayable evidence-bearing core without changing the meaning of authority, admissibility, invalidation, or override behavior.

## Why Slice 12 is next

The crate already proves:

- authority validation
- state algebra and transition legality
- contract validation and schema export
- fixture shape validation
- event dedupe and coalescing
- invalidation decisions
- remediation planning
- governed end-to-end proof execution

The remaining major weakness is that event and decision history is not yet durable. The current proof is correct, but the system cannot yet:

- replay historical decisions from stored evidence
- prove deterministic equivalence between live and replay execution
- preserve an operator-grade audit trail of accepted events and resulting posture changes
- bind packet reevaluation and remediation outcomes to durable evidence records

This slice closes that gap first, before consumer admission, repo onboarding, or any ForgeCommand trust surface work.

## Locked objectives

1. Introduce append-only durable evidence records for core decisions.
2. Preserve fail-closed behavior.
3. Preserve override non-mutation discipline.
4. Preserve current semantic truth for authority, artifact admissibility, packet admissibility, invalidation, and remediation.
5. Add deterministic replay proof from stored evidence.
6. Keep the slice crate-local and contract-first.

## Explicit non-goals

- no ForgeCommand UI work
- no repo discovery implementation
- no consumer packet admission API yet
- no RBAC layer
- no orchestration worker layer
- no cloud sync substrate
- no storage engine abstraction beyond what this slice minimally requires for deterministic evidence persistence

## Core design rule

Durable evidence must record decisions already made by governed core logic. Durable evidence must not become a second semantic authority.

The existing core remains the authority for:

- authority validation
- state legality
- event admissibility
- invalidation rules
- remediation planning

The durable layer only records:

- what was accepted
- what was rejected
- what was coalesced
- what changed
- what remediation was required
- enough structured detail to deterministically replay and verify the result

## Required evidence objects

Slice 12 should add durable record shapes for the following:

### 1. Event receipt record

Purpose:
Record every event intake attempt.

Required fields:

- `receipt_id`
- `event_id`
- `idempotency_key`
- `event_kind`
- `repo_id`
- `correlated_scope`
- `received_at`
- `admission_result` (`accepted | duplicate | poisoned | rejected`)
- `admission_reason`
- `coalesced_batch_id` optional
- `source_digest`

### 2. Coalesced batch record

Purpose:
Record the exact membership and outcome of a coalesced event batch.

Required fields:

- `batch_id`
- `repo_id`
- `correlated_scope`
- `member_event_ids[]`
- `opened_at`
- `closed_at`
- `batch_outcome`
- `poisoned` boolean

### 3. Artifact invalidation evidence record

Purpose:
Persist every governed artifact posture change caused by accepted events.

Required fields:

- `invalidation_record_id`
- `artifact_id`
- `prior_freshness`
- `next_freshness`
- `prior_admissibility`
- `next_admissibility`
- `cause_event_ids[]`
- `cause_summary`
- `changed_at`

### 4. Packet reevaluation evidence record

Purpose:
Persist every packet posture reevaluation caused by constituent artifact changes.

Required fields:

- `packet_reevaluation_id`
- `packet_id`
- `prior_admissibility`
- `next_admissibility`
- `constituent_artifact_ids[]`
- `trigger_invalidation_record_ids[]`
- `changed_at`
- `change_summary`

### 5. Remediation evidence record

Purpose:
Persist the generated remediation plan in a durable, replayable shape.

Required fields:

- `remediation_record_id`
- `target_kind` (`artifact | packet`)
- `target_id`
- `blocking` boolean
- `trigger_summary`
- `recommended_actions[]`
- `generated_at`

### 6. Replay bundle manifest

Purpose:
Bind a sequence of event receipts and outcome records into a deterministic replay unit.

Required fields:

- `replay_bundle_id`
- `created_at`
- `repo_id`
- `event_receipt_ids[]`
- `artifact_invalidation_record_ids[]`
- `packet_reevaluation_record_ids[]`
- `remediation_record_ids[]`
- `final_summary`
- `proof_digest`

## Required module additions

Recommended new files under `src/`:

- `durable_evidence.rs`
- `replay.rs`
- `evidence_bundle.rs`
- `evidence_store.rs`
- `proof/replay_scenario.rs`

Recommended responsibilities:

### `durable_evidence.rs`

Own the record structs and their validation.

### `evidence_store.rs`

Provide append-only file-backed persistence for Slice 12.

Initial implementation may use JSONL or deterministic JSON bundle files under a controlled proof directory.

Requirements:

- append-only semantics
- deterministic serialization
- no silent mutation
- stable ordering rules

### `evidence_bundle.rs`

Build replayable proof bundles from persisted records.

### `replay.rs`

Load stored evidence and replay the governed sequence to confirm the same final posture.

### `proof/replay_scenario.rs`

Add a second proof surface after the current governed-flow scenario:

- live execution path
- persisted evidence path
- replay path
- equivalence assertion

## File-backed persistence rule

Use a simple deterministic proof substrate first.

Preferred first implementation:

- `proof_artifacts/` output directory
- append-only JSONL or one-record-per-file deterministic JSON
- stable lexicographic naming using record ids

Do not introduce a database yet unless the crate already requires it elsewhere. The goal of this slice is evidence truth and replay determinism, not storage sophistication.

## Invariants

Slice 12 must preserve these invariants:

1. Duplicate events never create duplicate accepted outcome records.
2. Poison events are durably recorded as poison decisions.
3. Replay from durable evidence yields the same final artifact and packet posture as live execution.
4. Override records never rewrite historical governed truth.
5. Durable evidence remains append-only.
6. Replay never depends on ambient runtime state.
7. Evidence serialization is deterministic.

## Verification plan

### Unit proof targets

1. Event receipt record validation passes for valid accepted, duplicate, and poison records.
2. Artifact invalidation evidence record validation fails closed on missing causal linkage.
3. Packet reevaluation evidence record validation fails closed on empty trigger set.
4. Remediation evidence record fails closed if blocking is true but actions are empty.
5. Replay bundle manifest rejects dangling record references.

### Integration proof targets

1. Live governed scenario emits durable evidence records.
2. Event dedupe results in one accepted receipt plus duplicate receipts for repeated idempotency keys.
3. Coalesced batch membership is durably recorded.
4. Artifact invalidation evidence is emitted when source overlap or authority events degrade freshness/admissibility.
5. Packet reevaluation evidence is emitted when constituent artifacts degrade.
6. Remediation evidence is emitted when blocking conditions are reached.

### Replay proof targets

1. Stored evidence replays to the same final artifact posture.
2. Stored evidence replays to the same final packet posture.
3. Replay detects tampered evidence and fails closed.
4. Replay ordering is deterministic.

### Proof entrypoints

Preserve existing:

- `cargo test`
- `cargo run --bin proof_check`

Add:

- replay proof coverage inside `cargo test`
- `proof_check` output extended with durable evidence and replay equivalence summary

## Acceptance criteria

Slice 12 is complete when all of the following are true:

- durable evidence records exist for accepted event intake and resulting governed outcomes
- durable records are deterministic and append-only
- replay is possible from stored evidence only
- replay proves equivalence with live execution
- duplicate and poison behavior remain fail-closed
- override non-mutation remains intact
- `cargo test` passes
- `cargo run --bin proof_check` passes and reports replay equivalence

## Recommended implementation order

1. Add durable record types and validators.
2. Add append-only file-backed evidence store.
3. Extend governed flow to emit durable evidence records.
4. Add replay loader and deterministic replay engine.
5. Add replay proof scenario.
6. Extend `proof_check` summary.
7. Add tamper and duplicate-path tests.

## Out-of-scope reminder for next slice

Do not begin consumer admission or repo onboarding inside Slice 12.

Those should remain Slice 13 and Slice 14 work so that this slice stays narrowly focused on evidence durability and replay proof.
