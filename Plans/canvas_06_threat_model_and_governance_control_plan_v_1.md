# Canvas 06 — Threat Model and Governance Control Plan V1

**Version:** V1  
**Date:** April 7, 2026  
**Time:** America/New_York

## Purpose

This canvas defines the first-wave security and governance control posture for the BDS Precomputed Context Program.

It exists because the prior planning set treated provenance, admissibility, and operator visibility as governance concerns, but did not fully state the security reality: this system shapes model-visible context. That makes it part of the control-plane security boundary.

V1 therefore treats artifact and packet governance as both an operational trust problem and a security problem.

---

## Core Judgment

The precomputed context layer is a model-input control surface.

That means:

- misleading derived guidance is a security-relevant failure
- stale packet admission is a security-relevant failure
- approval abuse is a security-relevant failure
- sensitive internal context leakage is a security-relevant failure
- tamper or spoof of governance records is a security-relevant failure

Security and governance controls must ship with the first proof slice, not after it.

---

## First-Wave Threat Areas

### Threat 1 — Misleading Derived Guidance

An artifact compresses source truth inaccurately, overstates certainty, or hides ambiguity while still appearing approved.

Potential consequences:

- unsafe consumer behavior
- incorrect repo modification guidance
- false operator trust
- propagation of wrong operational instructions into packets

### Threat 2 — Source Poisoning

A malicious or low-trust source surface influences derivation in a way that produces misleading artifacts.

Potential consequences:

- incorrect authority mapping
- poisoned validation commands
- manipulated key-file guidance
- model-visible steering through derived artifacts

### Threat 3 — Approval Abuse

An unauthorized or weakly controlled actor approves an artifact or packet that should remain blocked or restricted.

Potential consequences:

- bypassed governance
- silent admission of weak or stale artifacts
- operator trust collapse

### Threat 4 — Stale Packet Use

A packet remains available or consumable after one or more constituents have degraded.

Potential consequences:

- consumers act on stale repo structure
- packets outlive source truth silently
- divergence between stored truth and runtime use

### Threat 5 — Sensitive Context Leakage

An artifact or packet contains operational detail, validation commands, or internal conventions that exceed the allowed sensitivity for a given consumer path.

Potential consequences:

- unnecessary exposure of internal sensitive knowledge
- over-broad model visibility
- trust-boundary failure across systems or lanes

### Threat 6 — Tamper or Audit Evasion

Governed records, overrides, or approval history are altered, removed, or made non-traceable.

Potential consequences:

- loss of provenance trust
- unverifiable operator actions
- inability to reconstruct why a packet was admitted or blocked

---

## Protected Assets

V1 must treat the following as protected governance assets:

- authority resolution records
- artifact records
- packet records
- source reference hashes
- remediation items
- approval and override records
- event history affecting trust posture
- consumer admission decisions

---

## Governance Roles

The V1 system must define explicit roles.

### Artifact Producer

May generate or regenerate candidate artifacts.
May not approve final admission by default.

### Critic Reviewer

May set critic outcomes within the critic service boundary.
May not directly alter final admissibility or override blocked posture.

### Operator Approver

May approve artifacts or packets where policy allows.
Must be subject to RBAC and audit.

### Remediation Triager

May classify and route remediation items.
Must not falsify underlying artifact state.

### Governance Owner

Owns protocol-level rules, authority controls, and change control decisions.

### Consumer System Identity

Represents the consuming service or lane identity for packet policy checks.

---

## Required V1 Controls

### Control 1 — Role-Based Access Control

All approval, override, and remediation actions must be subject to explicit RBAC.
No shared informal control posture.

### Control 2 — Immutable Audit Trail for Governed Actions

Approval, override, remediation closure, and key state changes must be durably recorded.

### Control 3 — Source Reference Integrity Hashing

Each source ref must have a recorded integrity hash at generation time.
This is required for traceability and change detection.

### Control 4 — Sensitivity Classification

Every artifact and packet must carry a sensitivity classification.
Consumer policy must enforce that packets do not exceed allowed sensitivity.

### Control 5 — Fail-Closed on Unknown Trust Posture

If required trust data is missing, unresolved, or contradictory, the system must default toward non-admission rather than convenience.

### Control 6 — Packet Reevaluation on Constituent Change

A packet may not remain quietly consumable after required constituent degradation.

### Control 7 — Override Recording with Expiry

Any override must be time-bounded, reason-recorded, and reviewable.

---

## Override Model

Overrides do not erase the truth of the underlying governed state.

Required override record fields:

- `override_id`
- `actor_identity`
- `reason`
- `scope`
- `start_time`
- `expiry_time`
- `affected_object_type`
- `affected_object_ids[]`
- `review_required_by`
- `created_at`

### Override rules

- overrides must be explicit
- overrides must expire
- overrides must be visible in operator surfaces
- overrides must not mutate source state history into a false healthy posture

---

## Sensitivity Model

Starter values:

- `internal_general`
- `internal_sensitive`
- `restricted_runtime`

### Rules

- consumers must declare their allowed sensitivity band
- packets may only include artifacts within allowed sensitivity policy
- sensitivity classification must survive into packet policy evaluation
- no packet may down-classify constituent sensitivity silently

---

## Approval and Admission Rules

### Approval

Approval should be distinct from generation.
A generated candidate is not a trusted governed artifact until it passes the required gates.

### Admission

Admission is separate from approval.
An approved artifact may still be non-admissible for a given consumer path depending on freshness, sensitivity, lane restrictions, or packet policy.

This distinction is mandatory.

---

## Threat Responses

### Response to misleading derived guidance

- critic checks for overstatement and ambiguity masking
- authority ambiguity blocks promotion where required
- operator inspection shows support and trigger traceability

### Response to source poisoning

- approved source family lists per repo
- authority resolution records
- weak or disallowed source families cannot silently act as primary authority

### Response to approval abuse

- RBAC
- immutable audit trail
- explicit operator identity on governed actions
- override visibility

### Response to stale packet use

- required packet reevaluation on constituent degradation
- fail-closed if reevaluation is pending or blocked

### Response to sensitive leakage

- sensitivity classification at artifact and packet level
- consumer sensitivity policy enforcement

### Response to tamper or audit evasion

- durable event history
- immutable or append-only audit posture for governed actions where feasible
- no silent deletion of approval rationale or override history

---

## Consumer Verification Requirement

Consumers must not trust packets purely by presence.
They must receive or verify:

- packet identity
- admissibility state
- evaluation time
- included artifact identity and integrity linkage where policy requires
- applicable restrictions or warnings

Consumer integration must not become a bypass channel.

---

## V1 Verification Requirements

The first-wave verification suite must include:

- RBAC tests
- override expiry tests
- sensitivity restriction tests
- fail-closed admission tests
- packet reevaluation tests after constituent downgrade
- audit trail presence tests
- source-ref hash presence tests

---

## Deferred But Not Ignored

The following may be expanded after V1 but must be acknowledged now:

- cryptographic signing beyond source-ref hashing
- stronger tamper-evident storage strategies
- more granular sensitivity taxonomies
- broader actor separation and dual-control patterns

These are future hardening areas, but V1 must already establish the correct control direction.

---

## Final Judgment

This program is not merely about cleaner context.
It is about controlled context.

If governance and security controls are deferred, the program risks becoming a polished but unsafe model-steering layer.

