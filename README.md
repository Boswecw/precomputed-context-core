# precomputed-context-core

Core Rust crate for the BDS Precomputed Context Program proof slice.

## Current scope

- authority resolution record validation
- lifecycle and freshness state algebra
- artifact admissibility rules
- packet admissibility rules
- event dedupe and coalescing substrate
- override non-mutation discipline

## Why this crate exists

This crate is the governed proof-slice core for the precomputed-context program. It exists to keep the first implementation slice contract-first, authority-first, and event-first rather than letting later service integration redefine the meaning of state, admissibility, or override behavior.

## Not in scope yet

- storage substrate
- repo discovery
- ForgeCommand UI trust surfaces
- packet composition runtime
- invalidation worker orchestration
- RBAC enforcement layer
- durable audit persistence

## Current proof targets

- authority resolution fails closed when precedence or allowed sources are invalid
- artifact lifecycle and freshness combinations are validated
- packet admissibility fails closed when required constituents degrade
- event dedupe works by idempotency key
- event batches coalesce by repo plus correlated source scope
- overrides do not mutate underlying governed truth

## Check

Run: cargo test
