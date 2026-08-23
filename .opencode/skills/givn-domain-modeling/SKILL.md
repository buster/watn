---
name: givn-domain-modeling
description: Model a change's domain before designing — surface ubiquitous language, aggregates, and invariants that the specs and design must honour.
---

# givn-domain-modeling

Model the domain for change `<change-id>` before writing the design.

## Context

- Proposal: `givn/changes/<change-id>/proposal.md`
- Specs: `givn/changes/<change-id>/specs/`
- Design (next): `givn/changes/<change-id>/design.md`

## What to produce

A short domain model that the design layer builds on:

- **Ubiquitous language**: the nouns and verbs the user and code share. Define
  each term in one sentence. Resolve ambiguities now, not in the design.
- **Aggregates / boundaries**: which entities form a consistency boundary, and
  which are independent. Name the root of each aggregate.
- **Invariants**: the rules that must always hold (e.g. "a change id is unique
  and kebab-case"). Mark where each is enforced (data model, command, gate).
- **Lifecycle events**: the state transitions that matter to the user
  (created -> planned -> implemented -> archived).

## Rules

- Stay in domain language. No class names, DB schemas, or framework types yet.
- Derive everything from the proposal and specs — do not invent new behaviour.
- If a term is ambiguous, pick the interpretation the proposal supports and flag
  it. Do not leave it open for the design to guess.
- Keep it short. This is a scaffold for the design, not a separate artifact.
