# GEMINI.md
Date: 2026-04-17
Time: 03:14 AM America/New_York

## Repo Identity
precomputed-context-core is part of the BDS / Forge ecosystem.
Treat it as a governed core subsystem for deterministic precomputed context construction and related identity/connectivity behavior.
Do not frame this repo as an MVP or as a generic utility package.

## Primary Working Rules
- Prefer bounded, explicit changes.
- Prefer minimal edits over broad refactors.
- Do not invent bundle fields, identity rules, hash semantics, connectivity fields, or downstream assumptions.
- Preserve deterministic context construction and bundle integrity.
- Keep responses copy/paste friendly.
- If a contract detail is uncertain, inspect the repo first.

## Output Contract
- Show the exact file path first when changing files.
- Prefer full-file replacement output when practical.
- Provide exact commands only.
- Do not provide pseudo-commands.
- State root cause before or with the fix when debugging.
- If verification was not run, say so explicitly.

## Repo Purpose
precomputed-context-core exists to support disciplined, deterministic construction of precomputed context bundles or closely related context artifacts.
Treat it as a core truth layer for context assembly behavior, not as an app-specific formatting helper.

## Core Preservation Rules
- Preserve deterministic context assembly.
- Preserve identity and hash meaning.
- Preserve bundle composition discipline.
- Preserve lineage and connectivity field carriage when relevant.
- Preserve separation between context construction and application-specific orchestration.
- Do not casually broaden the bundle with extra context “just in case.”

## Architecture Boundaries
Preserve boundaries between:
- source collection
- normalization or shaping
- bundle construction
- identity / hashing
- connectivity field carriage
- downstream transport or orchestration concerns

This repo should not absorb app-specific runtime policy.

## Contract Rules
When changing bundle or context behavior:
- identify the current contract first
- identify affected fields explicitly
- preserve current identity semantics unless the task explicitly changes them
- call out any change that affects compatibility
- verify hash and identity behavior explicitly when relevant

Do not casually rename, remove, or repurpose contract fields.

## Determinism Rules
This repo is determinism-sensitive.
When making changes:
- preserve stable ordering where ordering matters
- preserve deterministic hash or ID derivation where applicable
- avoid hidden nondeterminism
- avoid silent inclusion of unstable or incidental context

If output identity changes, treat that as high risk and explain why.

## Connectivity Rules
When this repo participates in context connectivity flows:
- preserve lineage fields
- preserve bundle identity behavior
- separate bundle construction from downstream delivery
- verify that bundle changes propagate correctly into connected systems when relevant

Important fields to preserve when relevant:
- `task_intent_id`
- `context_bundle_id`
- `context_bundle_hash`

Do not casually rename, drop, or synthesize these fields.

## Cross-Repo Rules
This repo may sit upstream of other systems.
When working cross-repo:
- do not assume downstream systems interpret a bundle the same way unless verified
- separate bundle-shape issues from transport issues
- separate identity drift from runtime failures
- verify integration assumptions explicitly

## Efficiency Rules
This repo contributes to context discipline.
When changing assembly logic:
- avoid “more context is always better” assumptions
- avoid uncontrolled context growth
- preserve selective, bounded context construction
- prefer deliberate inclusion rules over catch-all inclusion

## Testing and Verification Rules
Before calling a task done:
1. identify whether the change affects construction, identity, hashing, or connectivity
2. run the narrowest relevant tests first
3. if hash or identity behavior changed, verify before/after expectations explicitly
4. if connectivity overlay behavior is involved, verify lineage fields are preserved
5. report exactly what passed or failed

## Documentation Rules
When creating or updating docs in this repo:
- keep docs aligned to implementation
- do not write aspirational behavior as if it is already live
- include date and time
- use direct, operational language

## Context Priorities
When reasoning about precomputed-context-core, inspect in this order when relevant:
1. target file(s)
2. schema / model / bundle construction logic
3. identity / hash logic
4. tests covering bundle construction or connectivity
5. docs describing bundle semantics
6. adjacent integration points

## Do Not
- do not invent bundle fields
- do not weaken determinism
- do not silently expand bundle contents
- do not collapse construction and transport into one concern
- do not claim compatibility without verification

## Preferred Default Work Pattern
1. Identify whether the task is construction, identity, hashing, or connectivity related.
2. Identify the exact files.
3. State the narrow verification target.
4. Make the bounded change.
5. Run or recommend the narrow verification.
6. Report exact results and next step.