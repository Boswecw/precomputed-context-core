# Canvas 10 — Immediate Next Actions V1

**Version:** V1  
**Date:** April 7, 2026  
**Time:** America/New_York

## Purpose

This canvas turns the V1 revision set into an explicit immediate action list.

It exists to prevent the next step from slipping back into broad planning language or premature implementation. The immediate next actions are designed to lock the missing control surfaces first, then allow the proof slice to begin on safe footing.

---

## Core Judgment

The next move is not “start building the system.”
The next move is “lock the governing control files that make safe implementation possible.”

These actions are ordered.
They should be completed in sequence unless an explicit dependency exception is recorded.

---

## Immediate Action 1 — Write the Authority Resolution Spec

### Why this is first

No derivation can be trusted until repo archetype and authority precedence are explicit.

### Required output

A formal spec that defines:

- starter repo archetypes
- authority precedence stacks
- source family model
- ambiguity rules
- fail-closed conditions
- required onboarding record

### Completion test

ForgeCommand can be formally classified and given an approved authority resolution record.

---

## Immediate Action 2 — Write the Lifecycle State Machine Spec

### Why this is next

The system cannot store or surface trustworthy posture until lifecycle, freshness, critic, admissibility, and remediation relationships are locked.

### Required output

A formal state spec that defines:

- allowed values for each primary state family
- service ownership of each state family
- legal transitions
- illegal combinations
- packet-specific state rules
- override non-mutation rules

### Completion test

Illegal combination tests can be written directly from the spec.

---

## Immediate Action 3 — Write the Event Model Spec

### Why this matters now

The proof slice will immediately depend on source change, invalidation, packet reevaluation, and remediation events.

### Required output

A formal event spec that defines:

- event families
- required event fields
- event ownership
- coalescing rule
- dedupe rule
- retry rule
- poison-event handling
- dependency-aware invalidation rule
- packet reevaluation rule

### Completion test

A source move and authority change can be expressed as valid event sequences with explicit downstream effects.

---

## Immediate Action 4 — Produce Locked JSON Schemas for the First-Wave Contracts

### Why this matters now

Implementation must build to concrete payloads, not descriptive field lists.

### Required output

Concrete schemas for:

- shared artifact base
- Repo Navigation Map
- Key File Packet
- Validation Command Packet
- Repo Navigation Assist packet
- remediation item
- invalidation event
- authority resolution record
- override record

### Completion test

Each schema validates at least one valid fixture and rejects at least one invalid fixture.

---

## Immediate Action 5 — Produce the Example Payload Bundle

### Why this matters now

The proof slice needs golden examples and verification anchors.

### Required output

- one valid and one invalid example for each first-wave artifact class
- one valid and one invalid packet example
- one valid invalidation event example
- one valid remediation item example
- one valid authority resolution record example
- one valid override record example

### Completion test

The test harness can consume these as first-wave fixtures.

---

## Immediate Action 6 — Write the Threat Model

### Why this matters now

The system is a model-input control surface and must not treat security as a later hardening concern.

### Required output

A V1 threat model covering:

- misleading derived guidance
- source poisoning
- approval abuse
- stale packet use
- sensitive context leakage
- tamper or audit evasion
- required first-wave controls

### Completion test

RBAC, override, sensitivity, and fail-closed tests can be derived from the document.

---

## Immediate Action 7 — Classify ForgeCommand Under the Repo Archetype Model

### Why this matters now

The first proof-slice repo must be onboarded through the same governance model the program intends to enforce.

### Required output

A ForgeCommand-specific onboarding record including:

- repo archetype
- authority precedence order
- approved source families
- disallowed source families
- known authority gaps
- approved derivation scope

### Completion test

The first-wave extraction rules for ForgeCommand can be written without unresolved authority ambiguity.

---

## Immediate Action 8 — Lock the First-Wave Proof-Slice Acceptance Tests

### Why this matters now

Without locked proof criteria, implementation drifts toward artifact count instead of trust proof.

### Required output

A proof-slice acceptance document covering:

- required artifact generation outcomes
- required packet admission outcome
- required invalidation scenario
- required remediation creation outcome
- required supersession outcome
- required ForgeCommand trust-surface outcomes

### Completion test

The team can say exactly what must happen before V1 is considered proven.

---

## Immediate Action 9 — Begin Implementation Only After the Above Are Locked

### Why this is explicit

This action exists to prevent premature repo work from starting while foundational meaning is still unstable.

### Required output

A recorded implementation start decision citing completion of Actions 1 through 8.

### Completion test

Implementation begins from a stable control baseline rather than from evolving planning notes.

---

## Starter Work Packet Suggestion

The immediate work can be grouped into three short control packages.

### Package A — Governance Lock

- authority resolution spec
- lifecycle state machine spec
- threat model

### Package B — Execution Lock

- event model spec
- first-wave schemas
- example payload bundle

### Package C — Proof-Slice Admission Lock

- ForgeCommand onboarding record
- proof-slice acceptance tests
- implementation start decision

This grouping helps keep the revision work structured without collapsing dependencies.

---

## Anti-Pattern Warnings

Do not do the following next:

- do not start broad repo discovery before ForgeCommand onboarding is complete
- do not generate Non-Obvious Pattern artifacts in V1
- do not let UI work outrun state and event model lock
- do not create packets before packet admission rules are written
- do not let approval or override surfaces appear before RBAC and audit posture are defined

These are predictable failure paths and should be avoided explicitly.

---

## Final Judgment

The immediate next actions are now clear.

Lock the control files.  
Lock the first-wave contracts.  
Lock the proof conditions.  
Then start the ForgeCommand proof slice.

That is the correct V1 posture.

