# Canvas 07 — Testing and Verification Plan V1

**Version:** V1  
**Date:** April 7, 2026  
**Time:** America/New_York

## Purpose

This canvas defines the revised first-wave testing and verification posture for the BDS Precomputed Context Program.

It exists because the prior planning set had the right lifecycle mindset but remained too category-level. V1 tightens the proof posture so the first slice must demonstrate contract reality, state discipline, event correctness, invalidation control, packet gating, and operator truth.

The goal is not to prove that artifacts can exist.
The goal is to prove that they can be governed safely.

---

## Core Judgment

The proof slice succeeds only if it demonstrates trust maintenance.

V1 verification must prove:

- contracts are real and reject invalid shapes
- state algebra is enforced
- authority ambiguity is handled safely
- event behavior is traceable and controlled
- invalidation does not thrash the system
- packets cannot bypass degraded constituent truth
- ForgeCommand reflects governed truth accurately
- security and override controls actually work

---

## Verification Principles

### Principle 1 — Contract-first proof

No first-wave class is considered real until valid and invalid payload tests pass.

### Principle 2 — Fail-closed over optimistic behavior

Where trust posture is unclear, tests must confirm restrictive behavior.

### Principle 3 — Event truth matters as much as state truth

The system must prove not only final records, but also the event chains that caused them.

### Principle 4 — Middle states must be tested directly

States such as `review_due`, `passed_with_concerns`, and `admissible_with_warning` are dangerous if left under-tested.

### Principle 5 — Operator-facing truth must match canonical records

Read models may simplify presentation, but they may not contradict stored governed truth.

---

## First-Wave Test Families

## Family 1 — Schema Validity Tests

### Purpose

Prove that the V1 contract family is concrete and enforceable.

### Required targets

- shared artifact base contract
- Repo Navigation Map
- Key File Packet
- Validation Command Packet
- Repo Navigation Assist packet
- remediation item contract
- invalidation event contract
- authority resolution record contract
- override record contract

### Required checks

- required field presence
- enum correctness
- invalid payload rejection
- type enforcement
- missing-field rejection
- illegal extra-shape rejection where policy requires

### Pass condition

All valid payload fixtures pass deterministically.
All invalid fixtures fail deterministically.

---

## Family 2 — State Algebra Tests

### Purpose

Prove legal and illegal state combinations and transitions are enforced.

### Required checks

- artifact legal lifecycle transitions
- freshness transitions
- packet reevaluation rules
- illegal combination rejection
- service ownership discipline
- override non-mutation behavior

### Example cases

- `candidate` artifact cannot be `admissible`
- `blocked` artifact cannot appear `fresh`
- `invalidated` artifact cannot remain packet-admissible through required constituents
- packet with missing reevaluation cannot stay approved for use

### Pass condition

Illegal combinations fail.
Legal transitions succeed only through allowed owners.

---

## Family 3 — Authority Resolution Tests

### Purpose

Prove that repo archetype and authority rules actually constrain derivation.

### Required checks

- repo archetype assignment required before derivation
- approved source-family enforcement
- disallowed source-family rejection
- ambiguity detection behavior
- blocked derivation when authority conflict affects required truth

### Pass condition

Artifacts do not promote when authority posture is unresolved in a way that policy forbids.

---

## Family 4 — Event Handling Tests

### Purpose

Prove that the event model is real and safe.

### Required checks

- event schema validity
- coalescing behavior
- duplicate-event dedupe by idempotency key
- retry behavior for transient failures
- poison-event quarantine behavior
- causation and correlation linkage

### Pass condition

Event handling remains stable, deduplicated, and traceable under repeat and failure conditions.

---

## Family 5 — Invalidation and Revalidation Tests

### Purpose

Prove that source and authority changes affect the right objects with the right downgrade posture.

### Required trigger simulations

- source move
- source delete
- validation command drift
- authority record change
- source ref hash mismatch

### Required checks

- dependency-aware downgrade
- staged downgrade where applicable
- remediation item creation
- packet reevaluation after constituent change
- restored posture after repair and revalidation

### Pass condition

Affected objects degrade correctly, unaffected objects do not thrash, and repaired objects recover through explicit workflow.

---

## Family 6 — Packet Admission Tests

### Purpose

Prove that consumers receive only policy-valid packets.

### Required checks

- fresh packet admitted
- review_due packet warning posture handled correctly
- stale packet restricted or blocked per rule
- invalidated constituent causes packet downgrade
- blocked artifact prevents packet normal admission
- sensitivity restriction enforced
- lane compatibility respected where applicable

### Pass condition

No packet bypasses constituent trust posture or sensitivity policy.

---

## Family 7 — ForgeCommand Read-Model Truth Tests

### Purpose

Prove that the operator surfaces reflect underlying governed truth.

### Required checks

- readiness overview correctness
- repo detail summary correctness
- artifact detail state history correctness
- remediation queue correctness
- packet admissibility view correctness
- trigger traceability visibility

### Pass condition

The operator-facing trust surfaces do not misrepresent degraded or blocked posture.

---

## Family 8 — Security and Override Tests

### Purpose

Prove that governance controls are real.

### Required checks

- RBAC enforcement on approval actions
- RBAC enforcement on override creation
- override expiry behavior
- audit trail presence for approval and override events
- fail-closed behavior under missing trust data
- sensitivity policy enforcement

### Pass condition

Unauthorized paths fail, overrides remain bounded, and auditability survives the workflow.

---

## Family 9 — Mutation Tests

### Purpose

Prove the system detects deliberate breakage.

### Required posture

At least one deliberate mutation must be introduced for each major family, such as:

- invalid schema field
- illegal state combination
- wrong authority source family
- duplicate event replay
- broken source hash
- packet referencing an invalidated artifact
- UI read model masking degraded truth

### Pass condition

The system detects and routes the mutation appropriately.

---

## Family 10 — End-to-End Proof Slice Tests

### Purpose

Prove the full first-wave lifecycle works coherently.

### Required flow

1. ForgeCommand repo onboarded with authority resolution record  
2. first-wave artifacts generated  
3. artifacts validated and approved  
4. repo navigation assist packet composed  
5. packet admitted under normal posture  
6. source or validation change event triggered  
7. coalesced invalidation downgrades affected artifact(s)  
8. packet reevaluated and downgraded  
9. remediation item created  
10. repair path executed  
11. new artifact approved and prior artifact superseded  
12. packet revalidated  
13. ForgeCommand surfaces reflect the entire change path truthfully

### Pass condition

The full lifecycle remains queryable, policy-valid, and operator-visible.

---

## Oracle Requirement

Every first-wave class must have:

- at least one golden valid example
- at least one golden invalid example
- a concrete verification oracle

No class may rely solely on vague “looks useful” judgment in the V1 proof slice.

---

## Critic Constraint in V1

The critic must be kept as deterministic as possible in the first wave.

V1 should minimize open-ended judgment behavior.
Where critic checks exist, they should prefer:

- structural validation
- compactness validation
- contradiction detection
- source support checks
- explicitly scripted concern rules

This keeps the first proof slice auditable and less fragile.

---

## Performance and Burden Checks

V1 should include starter burden-oriented checks for:

- event coalescing effectiveness
- duplicate invalidation suppression
- remediation queue growth under a simulated change burst
- packet reevaluation latency for the first-wave scope

These do not need to be hyperscale tests in V1, but they must exist enough to detect obvious operational fragility.

---

## Final Judgment

The V1 proof slice is not complete when artifacts exist.
It is complete only when the system proves that artifacts, packets, state, events, remediation, and operator truth remain coherent under change.

