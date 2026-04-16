# Canvas 12 — Slice 12 Verification Matrix v1

Date: 2026-04-15
Time: 20:05 ET
Repo: `Boswecw/precomputed-context-core`
Scope: `Verification matrix for Slice 12 durable evidence and replay`

## Verification doctrine

This matrix exists to prevent Slice 12 from becoming a storage feature instead of a governed proof slice.

Every test in this slice must prove one of the following:

- evidence is durably recorded
- evidence is structurally valid
- evidence reflects the already-governed decision path
- replay from evidence reproduces final posture exactly
- tampering or ambiguity fails closed

## Test matrix

| ID | Area | Case | Expected result |
|---|---|---|---|
| S12-U01 | event receipt | valid accepted receipt | passes validation |
| S12-U02 | event receipt | duplicate receipt | passes validation with duplicate outcome |
| S12-U03 | event receipt | poison receipt | passes validation with poison outcome |
| S12-U04 | event receipt | missing idempotency key | fails closed |
| S12-U05 | coalesced batch | valid batch record | passes validation |
| S12-U06 | coalesced batch | empty member set | fails closed |
| S12-U07 | artifact invalidation | valid causal linkage | passes validation |
| S12-U08 | artifact invalidation | missing cause events | fails closed |
| S12-U09 | packet reevaluation | valid trigger linkage | passes validation |
| S12-U10 | packet reevaluation | empty trigger set | fails closed |
| S12-U11 | remediation | blocking plan with actions | passes validation |
| S12-U12 | remediation | blocking plan without actions | fails closed |
| S12-U13 | replay manifest | valid referenced records | passes validation |
| S12-U14 | replay manifest | dangling record reference | fails closed |
| S12-I01 | live execution | governed scenario emits event receipt evidence | passes |
| S12-I02 | live execution | coalesced batch emits membership evidence | passes |
| S12-I03 | live execution | source overlap invalidation emits artifact record | passes |
| S12-I04 | live execution | authority invalidation emits artifact record | passes |
| S12-I05 | live execution | packet reevaluation emits packet record | passes |
| S12-I06 | live execution | blocking remediation emits remediation record | passes |
| S12-I07 | duplicate handling | repeated event yields one accepted and later duplicate receipts | passes |
| S12-I08 | poison handling | poison event is durably recorded | passes |
| S12-R01 | replay | clean evidence replays to same artifact posture | passes |
| S12-R02 | replay | clean evidence replays to same packet posture | passes |
| S12-R03 | replay | tampered evidence fails closed | passes |
| S12-R04 | replay | ordering drift is detected or normalized deterministically | passes |
| S12-R05 | replay | override presence does not mutate historical truth | passes |

## Required proof outputs

Slice 12 should emit proof artifacts that are easy to inspect manually.

Recommended outputs:

- `proof_artifacts/event_receipts.jsonl`
- `proof_artifacts/coalesced_batches.jsonl`
- `proof_artifacts/artifact_invalidation_records.jsonl`
- `proof_artifacts/packet_reevaluation_records.jsonl`
- `proof_artifacts/remediation_records.jsonl`
- `proof_artifacts/replay_bundle_manifest.json`
- `proof_artifacts/replay_equivalence_report.json`

## `proof_check` reporting requirements

`cargo run --bin proof_check` should report at minimum:

- accepted event receipt count
- duplicate receipt count
- poison receipt count
- artifact invalidation evidence count
- packet reevaluation evidence count
- remediation evidence count
- replay bundle generated true/false
- replay equivalence true/false
- tamper detection proof true/false

## Completion gate

Slice 12 is not complete unless:

1. all matrix tests pass
2. replay equivalence is proven
3. proof artifacts are emitted deterministically
4. no existing governed-flow behavior regresses
5. the proof output is human-inspectable without needing hidden runtime state
