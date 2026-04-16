# Canvas 05 — Artifact and Packet Contract Plan V1

**Version:** V1  
**Date:** April 7, 2026  
**Time:** America/New_York

## Purpose

This canvas defines the narrowed contract posture for V1 of the BDS Precomputed Context Program.

It exists because the prior planning set described artifact classes and shared fields directionally, but did not lock a concrete first-wave contract family, did not clearly separate artifacts from packets, and did not include required integrity and sensitivity fields.

V1 corrects that by defining a mandatory first-wave schema family and an example-payload requirement.

---

## Core Judgment

The first proof slice must build to concrete contracts, not descriptive schema ideas.

The V1 contract family must be:

- small
- mandatory
- versioned
- integrity-aware
- sensitivity-aware
- separately defined for artifacts and packets

No “recommended” language is allowed in the proof-slice contract set.

---

## Contract Families in V1

### Family A — Shared Artifact Base Contract

This defines the required base fields for all stable artifacts in V1.

### Family B — First-Wave Artifact Class Contracts

V1 includes only three artifact classes:

1. Repo Navigation Map  
2. Key File Packet  
3. Validation Command Packet

### Family C — First-Wave Packet Contract

V1 includes only one packet class:

1. Bounded Consumer Packet — Repo Navigation Assist

### Family D — Supporting Governance Contracts

V1 also requires starter supporting contracts for:

- invalidation events
- remediation items
- authority resolution records
- override records

---

## Shared Artifact Base Fields

Every artifact in V1 must include the following fields.

### Identity and version

- `schema_version`
- `artifact_id`
- `artifact_class`
- `repo_id`

### Purpose and summary

- `title`
- `operational_purpose`
- `summary_block`

### Source support and integrity

- `source_refs[]`
- `source_ref_hashes[]`
- `authority_level`

### Governed state

- `lifecycle_state`
- `freshness_state`
- `critic_status`
- `admissibility_state`

### Lineage and related context

- `related_artifact_refs[]`
- `supersedes_artifact_id`
- `protocol_refs[]`

### Provenance and timing

- `created_at`
- `last_validated_at`
- `producer_identity`

### Governance and restriction

- `sensitivity_classification`

---

## Shared Artifact Base Field Intent

### `summary_block`

This must remain compact and operational.
It is not a narrative essay field.

### `source_refs[]`

These must link directly to the authority or support surfaces used for derivation.

### `source_ref_hashes[]`

Each source ref must have a recorded integrity hash at generation time.
This supports traceability and change-aware reevaluation.

### `authority_level`

Starter allowed values:

- `canonical`
- `strong_derived`
- `weak_derived`
- `provisional`

### `sensitivity_classification`

Starter allowed values:

- `internal_general`
- `internal_sensitive`
- `restricted_runtime`

Packets must not include artifacts above allowed consumer sensitivity.

---

## First-Wave Artifact Contracts

## Artifact Class 1 — Repo Navigation Map

### Purpose

Provide a compact structural map of where to look first in a repo.

### Required class-specific fields

- `primary_directories[]`
- `entry_points[]`
- `canonical_docs[]`
- `build_test_commands[]`

### Class rules

- must not attempt to capture deep tribal knowledge
- must remain structural and navigational
- must identify where authoritative truth usually begins for this repo

---

## Artifact Class 2 — Key File Packet

### Purpose

Identify a single key file or a tightly bounded file surface and explain why it matters.

### Required class-specific fields

- `file_path`
- `why_it_matters`
- `dependent_surfaces[]`
- `edit_cautions[]`
- `read_before_edit_refs[]`

### Class rules

- must stay bounded to a real file or tightly bounded file surface
- edit cautions must be source-supported or clearly marked as derived guidance
- must not become a hidden multi-file essay under one record

---

## Artifact Class 3 — Validation Command Packet

### Purpose

Provide the exact commands or checks required to validate the covered repo surface.

### Required class-specific fields

- `commands[]`
- `execution_order[]`
- `expected_pass_conditions[]`
- `environment_requirements[]`

### Class rules

- commands must be executable and source-supported
- pass conditions must be concrete enough to verify
- environment requirements must be explicit where relevant

---

## First-Wave Packet Contract

## Packet Class — Bounded Consumer Packet: Repo Navigation Assist

### Purpose

Provide a bounded, consumer-ready navigation packet assembled from approved first-wave artifacts.

### Required fields

- `schema_version`
- `packet_id`
- `packet_role`
- `repo_id`
- `included_artifact_ids[]`
- `included_artifact_hashes[]`
- `packet_constraints[]`
- `packet_budget_band`
- `lane_compatibility[]`
- `lifecycle_state`
- `admissibility_state`
- `created_at`
- `last_evaluated_at`

### Packet rules

- included artifacts must be queryable and traceable
- packet must be recomputed or reevaluated when required constituents degrade
- packet may not silently smooth over contradictory or blocked artifact posture
- packet must remain compact and role-specific

---

## Supporting Governance Contracts

### Invalidation Event Contract

Must include at minimum:

- `schema_version`
- `event_id`
- `event_type`
- `repo_id`
- `source_refs[]`
- `related_artifact_ids[]`
- `related_packet_ids[]`
- `causation_id`
- `correlation_id`
- `idempotency_key`
- `emitted_at`
- `event_payload`

### Remediation Item Contract

Must include at minimum:

- `schema_version`
- `remediation_id`
- `repo_id`
- `affected_object_type`
- `affected_object_ids[]`
- `issue_type`
- `severity`
- `blocking_status`
- `recommended_action`
- `created_at`
- `status`

### Authority Resolution Record Contract

Must include at minimum:

- `schema_version`
- `repo_id`
- `repo_archetype`
- `authority_order[]`
- `approved_source_families[]`
- `disallowed_source_families[]`
- `ambiguity_rules[]`
- `approved_derivation_scope[]`
- `created_at`
- `last_reviewed_at`

### Override Record Contract

Must include at minimum:

- `schema_version`
- `override_id`
- `actor_identity`
- `reason`
- `scope`
- `start_time`
- `expiry_time`
- `affected_object_type`
- `affected_object_ids[]`
- `review_required_by`

---

## Packet vs Artifact Boundary Rules

### Artifacts

Artifacts are stable derivative records.
They should not contain consumer-specific framing that would churn every time prompt or lane policy changes.

### Packets

Packets are consumer-facing and policy-sensitive.
They may reference lane compatibility, budget band, and inclusion constraints.
They may not redefine source truth.

This boundary must remain clean.

---

## Schema Evolution Rules

V1 requires:

- explicit `schema_version`
- locked enum values for the first-wave contracts
- migration-required posture for breaking changes
- invalid-payload tests for each class

No contract should be considered implementation-ready without a version field and compatibility stance.

---

## Example Payload Bundle Requirement

Before implementation begins, the program must produce:

- 1 valid Repo Navigation Map example
- 1 invalid Repo Navigation Map example
- 1 valid Key File Packet example
- 1 invalid Key File Packet example
- 1 valid Validation Command Packet example
- 1 invalid Validation Command Packet example
- 1 valid Repo Navigation Assist packet example
- 1 invalid Repo Navigation Assist packet example
- 1 valid remediation item example
- 1 valid invalidation event example

These examples are part of the proof substrate, not optional documentation.

---

## Anti-Garbage Rules

The following fields are high-risk for unbounded free text and must be constrained by validation and review:

- `summary_block`
- `why_it_matters`
- `edit_cautions[]`
- `recommended_action`

V1 must include formatting and boundedness rules so these fields do not become vague narrative sprawl.

---

## Final Judgment

V1 must build against a narrow, concrete contract set.

If contracts remain descriptive instead of locked, the implementation will drift before the proof slice is even complete.

