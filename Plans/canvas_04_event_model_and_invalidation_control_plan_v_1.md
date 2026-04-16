# Canvas 04 — Event Model and Invalidation Control Plan V1

**Version:** V1  
**Date:** April 7, 2026  
**Time:** America/New_York

## Purpose

This canvas defines the event-driven behavior of the BDS Precomputed Context Program.

It exists because the prior planning set described source change, invalidation, remediation, supersession, and packet reevaluation as if they would simply happen. That is not sufficient. The system is event-driven in practice, and if the event model is not explicit, invalidation storms, duplicate work, packet drift, and inconsistent operator truth will follow.

V1 makes the event model first-class.

---

## Core Judgment

The program is not operationally linear.
It is a governed event system.

At minimum, V1 must model:

- source changes
- authority changes
- artifact creation and approval
- invalidation and freshness downgrade
- remediation creation and resolution
- supersession
- packet composition and reevaluation
- consumer admission change

These events must be explicit, typed, traceable, deduplicated, and replay-safe.

---

## Event Families

### Family A — Source Events

These originate from canonical source movement or authority changes.

Starter event types:

- `source_changed`
- `source_moved`
- `source_deleted`
- `source_validation_changed`
- `authority_record_changed`

### Family B — Artifact Events

These originate from artifact derivation and governance transitions.

Starter event types:

- `artifact_candidate_created`
- `artifact_validation_passed`
- `artifact_validation_failed`
- `artifact_approved`
- `artifact_blocked`
- `artifact_freshness_changed`
- `artifact_invalidated`
- `artifact_superseded`

### Family C — Remediation Events

These originate from issue routing and repair lifecycle.

Starter event types:

- `remediation_created`
- `remediation_triaged`
- `remediation_repair_started`
- `remediation_resolved`
- `remediation_closed_without_repair`

### Family D — Packet Events

These originate from packet construction and packet trust changes.

Starter event types:

- `packet_candidate_created`
- `packet_composed`
- `packet_approved`
- `packet_invalidated`
- `packet_revalidated`
- `packet_admissibility_changed`

### Family E — Override and Governance Events

These originate from human control actions.

Starter event types:

- `operator_override_created`
- `operator_override_expired`
- `operator_override_revoked`
- `governance_rule_changed`

---

## Required Event Fields

Every event record must include:

- `event_id`
- `event_type`
- `schema_version`
- `emitted_at`
- `emitter_service`
- `repo_id`
- `related_artifact_ids[]`
- `related_packet_ids[]`
- `source_refs[]`
- `causation_id`
- `correlation_id`
- `idempotency_key`
- `event_payload`

### Field intent

- `causation_id` links the event to the immediate triggering event  
- `correlation_id` ties a broader chain of related events together  
- `idempotency_key` prevents duplicate processing across retries or repeated event emission  

These are mandatory in V1.

---

## Event Ownership

### Source events

Emitted by:
- source detection layer
- repo watcher layer
- authority resolution governance layer when authority records change

### Artifact governance events

Emitted by:
- artifact generation service
- validation service
- critic service
- governance/state service

### Remediation events

Emitted by:
- remediation workflow service

### Packet events

Emitted by:
- packet composition service
- admissibility evaluation service

### Override events

Emitted by:
- operator/governance action service

No UI surface should emit stateful system events directly.

---

## Invalidation Control Model

The program must not react to every source event with immediate uncontrolled fan-out.

### Rule 1 — Coalescing is mandatory

Source events must enter a coalescing window before broad dependency evaluation.

The exact timing and batching thresholds belong to operational policy, but the control principle is fixed:

- no one-event one-cascade default
- no uncontrolled storm behavior
- no repeated invalidation for the same underlying change set

### Rule 2 — Invalidation must be dependency-aware

Only directly or transitively affected artifacts may be downgraded.

The system must use a dependency model linking:

- source refs to artifacts
- artifacts to packets
- authority records to dependent artifacts

### Rule 3 — Downgrade should be staged where possible

The system must not jump straight to `invalidated` for every signal.

Starter downgrade pattern:

- low-confidence concern => `review_due`
- credible drift or aging concern => `stale`
- material structural or authority break => `invalidated`

### Rule 4 — Packet reevaluation is required on constituent degradation

If a required constituent artifact changes to any of the following:

- `stale`
- `invalidated`
- `blocked`
- `superseded`

then dependent packets must be reevaluated.

### Rule 5 — Repeated identical events must dedupe cleanly

The same effective change must not generate uncontrolled duplicate work.

---

## Dependency Graph Requirements

The event system requires a dependency representation, even if minimal in V1.

V1 must support at least:

- source_ref -> artifact links
- artifact -> packet constituent links
- authority_record -> artifact scope links
- remediation_item -> affected object links

Without this, invalidation becomes blind and noisy.

---

## Retry and Failure Handling

### Retry rules

Handlers must support retry for transient failures.
Retries must honor idempotency.

### Poison-event handling

If an event repeatedly fails processing:

1. quarantine it  
2. emit a failure or remediation signal  
3. do not let it endlessly retry in the main path  
4. surface the failure to operator trust surfaces if it affects readiness

### Replay rules

The event system must support safe replay for recovery and audit where feasible.
Replay must not create duplicate approvals, duplicate remediation items, or contradictory state transitions.

---

## Event Sequences

### Sequence A — Source Move

1. `source_moved` emitted  
2. event enters coalescing window  
3. dependency evaluation finds linked artifacts  
4. affected artifacts downgraded to `review_due`, `stale`, or `invalidated` as policy requires  
5. packets depending on those artifacts marked for reevaluation  
6. remediation items created if structural repair is required  
7. ForgeCommand read models update to reflect new trust posture

### Sequence B — Authority Record Change

1. `authority_record_changed` emitted  
2. affected artifact set identified  
3. artifacts relying on changed authority reevaluated  
4. blocked or downgraded artifacts produce packet reevaluation  
5. if authority ambiguity appears, remediation item created

### Sequence C — Artifact Repair and Supersession

1. remediation marked `in_repair`  
2. regenerated candidate artifact created  
3. validation and critic pass  
4. new artifact approved  
5. old artifact becomes `superseded`  
6. dependent packets reevaluated and reapproved as needed

---

## Event Storage and Audit

All events that materially affect artifact, packet, remediation, or override truth must be auditable.

V1 does not require a fully generalized event-sourced system, but it does require:

- durable event records for governed transitions
- queryable event history for affected artifacts and packets
- source-trigger traceability for operator inspection

---

## UI Traceability Requirement

ForgeCommand must be able to answer:

- what changed
- when it changed
- which source or authority record triggered the downgrade
- which dependent artifacts or packets were affected

This requires event traces that remain queryable from artifact detail and packet detail views.

---

## Verification Requirements

V1 must test:

- event schema validity
- coalescing behavior
- dedupe behavior
- idempotent retry behavior
- dependency-aware downgrade behavior
- poison-event quarantine path
- packet reevaluation after constituent change
- operator traceability of triggering events

---

## Final Judgment

The event model is not an implementation detail.
It is one of the main control surfaces of the system.

If this remains vague, the program will collapse under repo churn, duplicate work, and inconsistent trust behavior.

