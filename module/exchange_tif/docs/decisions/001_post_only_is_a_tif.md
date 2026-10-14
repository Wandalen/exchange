# ADR-001: Post-Only Is a `Tif` Value

**Date**: 2026-10-06
**Status**: Accepted
**Deciders**: Anatolii Shliakhto

## Context

A market maker needs a guarantee that an order only adds liquidity. The
source design names no such option
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:546-548`).

## Decision

`Tif::PostOnly`: rests like `Gtc`, and is refused whole if it would take on
arrival. `tif_takes` is the query; `exchange_match::cross` enforces it and
`exchange_core` reports `RejectReason::PostOnlyWouldTake`.

## Alternatives Considered

### Option 1: A `post_only : bool` field on `Order`

Rejected: every `Order` literal in the family changes, and the field admits
combinations with no meaning — a post-only IOC can never trade, a post-only
FOK can never fill.

## Consequences

**Positive:** one variant, no `Order` change, no meaningless combination.

**Negative:** none known — a post-only order rests by definition, so it never
needs a second disposition.
