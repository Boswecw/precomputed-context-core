# Canvas 08 — ForgeCommand Trust Surface Plan V1

**Version:** V1  
**Date:** April 7, 2026  
**Time:** America/New_York

## Purpose

This canvas defines the minimal but sufficient ForgeCommand operator surfaces required for the V1 proof slice of the BDS Precomputed Context Program.

It exists because the earlier surface plan was directionally strong but risked too many badge families, too much state burden on the operator, and too broad an initial surface set.

V1 narrows the UI to the surfaces needed to answer core operator trust questions quickly.

---

## Core Judgment

The first UI job is not to show everything.
It is to support safe operator decisions quickly.

The operator must not be forced to mentally combine multiple overlapping statuses every time they inspect a repo or packet. The UI may retain detailed state families underneath, but the primary presentation must reduce cognitive load.

---

## First-Wave Operator Questions

ForgeCommand V1 must help the operator answer these questions clearly:

1. Is this repo context-ready right now?  
2. Can this packet be consumed safely right now?  
3. What changed?  
4. Why was this artifact or packet downgraded?  
5. What action is needed next?

If a surface does not help answer one of these questions, it is not required for the V1 slice.

---

## First-Wave Surface Set

## Surface A — Readiness Overview

### Purpose

Provide a compact ecosystem or first-wave repo summary showing trust posture at a glance.

### Required fields

- repo name
- trust band
- critical issue count
- stale or invalidated artifact count
- packet readiness count
- last validation time

### UI intent

This is the operator’s top-level trust surface.
It should answer whether the repo appears healthy enough to rely on and whether action is immediately required.

### Rules

- must not present a healthy-looking summary when critical required artifacts or packets are degraded
- trust band must be derived from governed read-model logic, not ad hoc styling

---

## Surface B — Repo Detail

### Purpose

Provide repo-specific trust posture and context coverage detail.

### Required fields

- repo trust summary
- artifact class coverage summary
- open remediation item count
- packet admissibility summary
- recent invalidation events
- recent supersession events where relevant

### UI intent

This is the operator’s main drill-down view for the first-wave repo.

### Rules

- coverage must include missing required class visibility
- invalidation must be tied to recent triggering events
- packet posture must be visible without requiring a separate hunting workflow

---

## Surface C — Artifact Detail

### Purpose

Provide inspectable trust truth for a single artifact.

### Required fields

- artifact title
- artifact class
- trust card
- source refs
- source-ref integrity or linkage info
- authority level
- lifecycle, freshness, critic, and admissibility details in drill-down form
- triggering event traceability
- related remediation items
- lineage and supersession info where applicable

### UI intent

This surface must answer:

- can I trust this artifact right now?
- what caused its current posture?
- what does it depend on?

### Rules

- trigger traceability is mandatory for downgraded artifacts
- operator should not need to cross-reference multiple pages just to see why this artifact degraded

---

## Surface D — Remediation Queue

### Purpose

Provide actionable visibility into trust-relevant issues requiring repair or review.

### Required fields

- severity
- issue type
- affected object
- blocking status
- recommended next action
- created time
- current remediation status

### UI intent

This is not just a list of failures.
It is the action queue for clearing trust debt.

### Rules

- the queue must prioritize blocking and critical items clearly
- recommended next action must be visible without opening every record
- remediation items must remain tied to the affected repo, artifact, or packet

---

## Surface E — Packet Admissibility View

### Purpose

Show whether a consumer-ready packet is safe to use now.

### Required fields

- packet role
- trust card
- included artifacts
- admissibility posture
- reason summary
- applicable restrictions or warnings
- last evaluation time

### UI intent

This surface must answer:

- may NeuroForge or NeuronForge consume this packet right now?
- if not, what is blocking it?

### Rules

- packet posture must reflect constituent truth
- included artifact identity must be visible
- warnings or restrictions must be intelligible, not buried

---

## Trust Card Model

The V1 surfaces must reduce badge sprawl by using a primary **trust card** pattern.

### Trust card contents

- overall trust band
- top 3 reasons driving posture
- next required action

### Example interpretation

A repo or packet may still carry detailed state families, but the operator should see first:

- Ready / Partial / Degraded / Blocked
- why
- what to do next

This avoids making the operator compute meaning from five separate badge families on every surface.

---

## Detailed State Presentation Rule

Detailed state families remain important, but they should be placed in drill-down or secondary sections.

Examples:

- lifecycle state
- freshness state
- critic status
- admissibility state
- sensitivity classification

These must not dominate the primary overview layer unless a specific state is the main cause of degradation.

---

## Trigger Traceability Requirement

Every downgraded artifact or packet must expose:

- triggering event type
- triggering source or authority ref
- trigger time
- affected dependencies where available

This is mandatory in V1.
Without this, the operator cannot answer “why am I seeing this?” and trust in the surface collapses.

---

## Read-Model Requirements

ForgeCommand must receive read models designed for operator clarity.

Required read-model families:

- readiness overview row
- repo detail summary
- artifact detail record
- remediation queue row
- packet admissibility row
- trigger trace record

These should not merely mirror raw storage tables.
They should be purpose-built for operator decisions.

---

## V1 Interaction Set

The initial interaction set should remain narrow.

### Required first-wave actions

- inspect repo detail
- inspect artifact detail
- inspect packet admissibility
- inspect remediation item
- inspect trigger traceability

### Optional first-wave actions if ready

- request revalidation
- acknowledge or review remediation item

### Deferred actions

The following may wait until after the proof slice:

- full approval workflow inside UI
- broad override authoring in UI
- advanced timeline analytics
- large lineage exploration surfaces

These remain future candidates, not V1 requirements.

---

## Performance Expectation

The first-wave surfaces should support fast operator comprehension.

Practical rule:

a surface should make its primary answer understandable in a few seconds, not after deep state decoding.

---

## Anti-False-Confidence Rules

V1 surfaces must explicitly avoid:

- healthy-looking trust cards when required packets are blocked
- hiding missing required artifact classes
- presenting a packet as safe when constituent reevaluation is pending
- collapsing all degraded states into vague warning language without reasons

---

## Final Judgment

ForgeCommand V1 should behave as a trust surface, not a metrics board.

If the UI makes the operator perform the integration work mentally, the surface has failed even if the backend state is technically correct.

