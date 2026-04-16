# Canvas 03 — Lifecycle State Algebra and Ownership Plan V1

**Version:** V1  
**Date:** April 7, 2026  
**Time:** America/New_York

## Purpose

This canvas defines the canonical state model for artifacts, packets, and remediation records in the BDS Precomputed Context Program.

This exists because the earlier planning set allowed too much semantic overlap between lifecycle, freshness, critic posture, confidence posture, admissibility, and remediation. That overlap would create contradictory records, inconsistent read models, and ambiguous operator truth.

V1 corrects that by defining explicit state families, service ownership, legal combinations, and fail-closed behavior.

---

## Core Judgment

State must be algebraic, not descriptive.

The system must not rely on human interpretation of loosely related fields.
Every primary state family must have:

- a locked allowed-value set
- a single owning service or computation surface
- legal transition rules
- illegal combinations that fail validation
- downstream consequences for consumers and operator truth

---

## State Family Model

The V1 system uses five primary state families.

### 1. Artifact Lifecycle State

This describes the governing lifecycle posture of an artifact record.

Allowed values:

- `candidate`
- `approved`
- `superseded`
- `blocked`
- `retired`

### 2. Freshness State

This describes the artifact’s temporal and structural trust posture relative to source drift and validation.

Allowed values:

- `fresh`
- `review_due`
- `stale`
- `invalidated`

### 3. Critic Status

This describes the critic outcome for the artifact.

Allowed values:

- `not_reviewed`
- `passed`
- `passed_with_concerns`
- `failed`
- `remediation_required`

### 4. Admissibility State

This describes whether the artifact or packet may be consumed in normal operation.

Allowed values:

- `admissible`
- `admissible_with_warning`
- `restricted`
- `not_admissible`

### 5. Remediation Status

This describes the active lifecycle of a remediation item.

Allowed values:

- `none`
- `open`
- `triaged`
- `in_repair`
- `resolved`
- `closed_without_repair`

---

## Optional Supporting Fields

The following may exist, but they are not primary state families in V1:

- confidence_posture
- sensitivity_classification
- operator_override_present
- risk_band

These fields may influence policy or UI, but they must not replace the primary state families.

---

## Ownership Rules

Every primary state family must have one owner.

### Artifact Lifecycle State

Written by:
- governance/state service only

No other service may directly write or mutate artifact lifecycle state.

### Freshness State

Written by:
- validation/invalidation service only

### Critic Status

Written by:
- critic service only

### Admissibility State

Computed by:
- admissibility evaluation service only

This state is not a freeform writable field.
It must be recomputed from allowed inputs.

### Remediation Status

Written by:
- remediation workflow service only

---

## Packet State Rule

Packets have their own lifecycle evaluation.
They do not inherit approval automatically from constituent artifacts.

Starter packet lifecycle values:

- `candidate`
- `approved`
- `blocked`
- `retired`

Starter packet admissibility values follow the same admissibility enum used by artifacts.

Packets must be evaluated separately whenever any required constituent artifact changes in a way that affects trust posture.

---

## Artifact Approval Rules

An artifact may become `approved` only if all of the following are true:

- schema validation passed
- required source refs resolve
- authority resolution does not block derivation
- critic status is not `failed`
- critic status is not `remediation_required` unless explicit policy allows controlled approval with recorded concerns
- lifecycle state transition is permitted by policy

Default V1 rule: artifacts under unresolved ambiguity or failed validation must not become approved.

---

## Freshness Interpretation Rules

### `fresh`

Meaning:
- latest required validation passed
- no active invalidation trigger blocks use
- no policy rule forces downgrade

### `review_due`

Meaning:
- artifact is aging or weakly signaled for reevaluation
- not yet materially broken
- may still be consumable with warning if all other conditions allow it

### `stale`

Meaning:
- trust posture has materially degraded
- artifact is no longer normal-ready
- admissibility must be restricted or blocked by class rule

### `invalidated`

Meaning:
- structural or authoritative support is materially broken or no longer trustworthy
- normal use is prohibited

---

## Default Admissibility Rules

### Artifact default rules

- `approved` + `fresh` + `passed` => `admissible`
- `approved` + `review_due` + `passed` => `admissible_with_warning`
- `approved` + `review_due` + `passed_with_concerns` => `restricted` or `admissible_with_warning` by class rule
- `approved` + `stale` => `restricted` or `not_admissible`
- any `invalidated` artifact => `not_admissible`
- any `blocked` artifact => `not_admissible`
- any `superseded` artifact => `not_admissible`
- any `candidate` artifact => `not_admissible`
- critic `failed` => `not_admissible`

### Packet default rules

A packet may be `admissible` only if:

- all required constituent artifacts remain eligible under packet rules
- packet schema is valid
- packet-specific policy evaluation passes
- no constituent change requires reevaluation that has not yet occurred

If any required constituent artifact becomes `invalidated`, `blocked`, or `superseded`, the packet must be reevaluated and normally downgraded to `not_admissible` until reapproved.

---

## Illegal Combinations

The following are starter illegal combinations and must fail validation.

### Artifact illegal combinations

- lifecycle `candidate` + admissibility `admissible`
- lifecycle `candidate` + freshness `fresh`
- lifecycle `blocked` + freshness `fresh`
- lifecycle `blocked` + admissibility `admissible_with_warning`
- lifecycle `superseded` + admissibility `restricted`
- lifecycle `retired` + admissibility anything except `not_admissible`
- critic `failed` + lifecycle `approved`
- critic `failed` + admissibility anything except `not_admissible`
- freshness `invalidated` + admissibility anything except `not_admissible`

### Packet illegal combinations

- packet lifecycle `candidate` + admissibility `admissible`
- packet lifecycle `approved` + required constituent missing
- packet lifecycle `approved` + unresolved reevaluation required
- packet lifecycle `approved` + required constituent `invalidated`

---

## Transition Rules

### Lifecycle transitions for artifacts

Allowed starter transitions:

- `candidate` -> `approved`
- `candidate` -> `blocked`
- `approved` -> `superseded`
- `approved` -> `blocked`
- `blocked` -> `candidate` only through controlled regeneration or repair path
- `superseded` -> `retired`

Not allowed:

- `superseded` -> `approved`
- `retired` -> any active state
- `blocked` -> `approved` without revalidation path

### Freshness transitions

Allowed starter transitions:

- `fresh` -> `review_due`
- `review_due` -> `stale`
- `fresh` -> `invalidated`
- `review_due` -> `invalidated`
- `stale` -> `invalidated`
- `stale` -> `fresh` only after revalidation
- `invalidated` -> `fresh` only after repair or regeneration and full revalidation

---

## Service Write Discipline

No service may patch multiple primary state families ad hoc.

Example:

- critic service may write critic outcome but may not directly set final admissibility
- validation service may set freshness posture but may not set lifecycle approval
- UI surfaces may never write primary states directly

This discipline is mandatory to avoid state drift.

---

## Override Behavior

An operator override does not erase primary state truth.

If override logic exists, it must be represented through a separate override record and related derived behavior, not by falsifying the underlying lifecycle or freshness state.

Example:

- artifact remains `review_due`
- override allows temporary controlled packet admission
- override record captures scope, reason, expiry, and reviewer obligations

---

## Read-Model Rules

ForgeCommand may simplify presentation for operator clarity, but read-model simplification must never contradict underlying governed state.

The UI may produce an aggregated trust band, but the source state families remain canonical.

---

## Verification Requirements

V1 must include:

- valid combination tests
- illegal combination tests
- transition tests
- owner-discipline tests
- packet reevaluation tests
- override non-mutation tests

---

## Final Judgment

The state model must be treated as a hard contract surface.

If state remains informal, the entire program will drift into contradictory records, unreliable operator truth, and unsafe consumer behavior.

