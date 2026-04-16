# Canvas 02 — Authority Resolution and Repo Archetype Plan V1

**Version:** V1  
**Date:** April 7, 2026  
**Time:** America/New_York

## Purpose

This canvas defines how the BDS Precomputed Context Program determines what is authoritative, how repo type changes derivation behavior, and how ambiguity must be escalated.

This canvas exists because the prior planning set relied on the statement that canonical source remains upstream, but did not define how canonicality is resolved when multiple source families disagree.

That is not safe enough for implementation.

---

## Core Judgment

Authority must be explicit.

No repo may enter artifact derivation without:

- a declared repo archetype
- an authority precedence stack
- an approved source-family list
- ambiguity handling rules
- a rule for when derivation must fail closed

The system must never silently compress disagreement into a seemingly authoritative derivative artifact.

---

## Repo Archetype Model

### Archetype A — Service / API Repo

Typical characteristics:

- backend runtime code
- service contracts
- typed schemas
- integration surfaces
- operational validation commands
- system docs or protocol overlays

Typical authority precedence:

1. service contracts and schemas  
2. runtime code implementing those contracts  
3. authoritative verification tests  
4. system docs and repo truth docs  
5. protocol overlays and architecture notes

Typical first-wave eligible artifact classes:

- Repo Navigation Map
- Key File Packet
- Validation Command Packet

### Archetype B — Desktop / App Repo

Typical characteristics:

- routed UI structure
- client-side feature boundaries
- typed interface surfaces
- UI and command integration behavior
- internal system docs

Typical authority precedence:

1. typed interfaces and contracts  
2. route or feature implementation code  
3. authoritative tests  
4. SYSTEM.md or equivalent repo truth docs  
5. protocol overlays

Typical first-wave eligible artifact classes:

- Repo Navigation Map
- Key File Packet
- Validation Command Packet

### Archetype C — Protocol / Documentation Repo

Typical characteristics:

- protocol documents
- policy documents
- architecture decisions
- supporting schemas or diagrams
- governance change records

Typical authority precedence:

1. formal protocol documents  
2. architecture decisions and approved addenda  
3. supporting diagrams and schemas  
4. mirrors, summaries, or generated views

Typical first-wave eligible artifact classes:

- Repo Navigation Map
- Key File Packet
- Validation Command Packet where operational validation exists

### Archetype D — Shared Contract / Library Repo

Typical characteristics:

- shared schemas
- interface contracts
- reusable library code
- compatibility and migration surfaces

Typical authority precedence:

1. schemas and interface contracts  
2. library code  
3. compatibility tests  
4. migration notes and release truth docs

Typical first-wave eligible artifact classes:

- Repo Navigation Map
- Key File Packet
- Validation Command Packet

---

## Required Repo Onboarding Record

Every repo admitted into the program must produce an **Authority Resolution Record**.

Required fields:

- `repo_id`
- `repo_name`
- `repo_archetype`
- `authority_order[]`
- `approved_source_families[]`
- `disallowed_source_families[]`
- `ambiguity_rules[]`
- `escalation_required_conditions[]`
- `approved_derivation_scope[]`
- `operator_review_required_conditions[]`
- `notes_on_known_authority_gaps`

This record is mandatory before discovery rules are written.

---

## Source Family Model

Starter source families:

- code_runtime
- contract_schema
- test_verification
- repo_truth_doc
- protocol_doc
- architecture_decision
- generated_output
- advisory_note

### Rules

1. Not all source families are equally authoritative.  
2. Some source families may be allowed as support but not as authority.  
3. Generated outputs may support derivation but must not become primary authority unless explicitly declared.  
4. Advisory or convenience notes must never be treated as canonical without explicit approval.

---

## Authority Precedence Rules

### Rule 1 — Precedence is repo-specific but declared

The system may not assume a universal authority stack across all repos.

### Rule 2 — Derivation must record which authority level was used

Every artifact must retain explicit source support and authority provenance.

### Rule 3 — Weak support must remain weak

If an artifact relies primarily on lower-precedence support, that weakness must remain visible and may restrict admissibility.

### Rule 4 — Consumer-ready packets may not hide authority ambiguity

If any required constituent artifact is blocked by unresolved authority conflict, the packet must fail normal admission.

---

## Ambiguity Handling Rules

### Ambiguity Condition Examples

- code and SYSTEM.md disagree on entry point ownership
- protocol document and runtime behavior diverge
- tests imply a different invariant than the current documentation
- repo structure changed but validation docs were not updated

### Mandatory response

If an ambiguity condition affects artifact trust:

1. do not silently select a winner unless precedence rules are explicit  
2. mark candidate artifact as `blocked` or `remediation_required`  
3. create an ambiguity remediation item  
4. require operator or governance review where policy says so

### Never allowed

- narrative smoothing over conflict
- implicit majority-rule authority
- consumer admission of a packet whose required artifact is blocked by unresolved authority conflict

---

## ForgeCommand First-Wave Classification

Before discovery begins, ForgeCommand must be explicitly classified.

Required decision output:

- selected repo archetype
- precedence stack
- approved source families
- known authority gaps
- initial derivation scope

No extraction rule or artifact schema instance should be applied to ForgeCommand before this classification is complete.

---

## Derivation Scope Rules

Each repo onboarding record must declare what may be derived in the current wave.

Example starter scope:

- repo navigation structure
- key file importance and caution notes
- validation commands and pass conditions

The following should be excluded from first-wave scope unless separately approved:

- interpretive tribal knowledge capture
- compatibility traps
- failure-mode summaries
- protocol compliance judgments beyond explicit source support

---

## Fail-Closed Rules

Derivation must fail closed when:

- repo archetype is not assigned
- authority precedence is not approved
- source family classification is missing
- required source refs cannot be resolved
- ambiguity affects required artifact truth
- onboarding record is incomplete

---

## Review and Change Control

Any change to a repo’s authority resolution record must trigger:

- review of affected artifacts
- reevaluation of packet constituents
- potential invalidation or downgrade where required

Authority changes are not advisory. They are governance-relevant events.

---

## Final Judgment

Authority resolution is not documentation garnish.
It is a prerequisite control surface.

Without it, derivative artifacts will eventually become polished conflict amplifiers instead of trustworthy operational aids.

