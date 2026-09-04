---
name: givn-reverse-engineer
description: Reverse-engineer an existing capability into descriptive Gherkin specs — discovery, feature writing, characterization tasks.
---

# givn-reverse-engineer

Reverse-engineer an existing capability into **descriptive Gherkin specs** for
change `<change-id>`. No proposals, no designs — just `.feature` files +
characterization tasks that make them executable.

## Context

- Change: `givn/changes/<change-id>/`
- Test runner: `./run-tests.sh`
- Features path: `GIVN_FEATURES=givn/specs`

## Stance: descriptive, not prescriptive

You are capturing what the system **does** today, not what it *should* do.
Every spec assertion is probed from the real code. Suspected bugs are flagged,
never fixed. Production code behaviour is never changed — it is refactored
only for testability seams, with a behaviour-preservation guarantee.

## When to use

- Reverse-engineering an existing codebase that has no givn specs.
- Creating the initial baseline spec for a capability.
- Before dogfooding givn on its own repo (or any repo with existing code).

## What not to do

- Do NOT write prescriptive specs (what the system *should* do). Write
  descriptive specs (what the system *does*).
- Do NOT change production code behaviour to match a spec. If they disagree,
  the spec is wrong — correct it.
- Do NOT fix bugs discovered during characterization. Flag them as follow-up
  `@givn.modified` candidates.
- Do NOT delete dead code. Flag it as a follow-up `@givn.removed` candidate.
- Do NOT skip the discovery phase — it is always interactive.
- Do NOT write proposal.md, design.md, or design-review.md. The reverse flow
  has no such artifacts — they are skipped via `givn new --skip`.

## Discovery phase

Before writing any artifact, explore the codebase and propose ONE capability
with a clear black-box boundary. Present the proposal to the user
interactively. Do not proceed until the user confirms.

One capability per change. If multiple capabilities are needed, suggest
separate changes.

## Artifacts produced (only 2)

- `specs/<group>/<cap>.feature` — descriptive scenarios (`@givn.added @wip`),
  all provisional until validated during characterization. The Feature:
  description block contains the black-box boundary and suspected bugs.
- `tasks.md` — first task is harness setup; subsequent tasks are one
  RED/GREEN/REFACTOR characterization cycle per scenario (see `givn-steps`
  skill for the shared loop mechanics).

## What is NOT produced

- No `proposal.md` — the black-box boundary and suspected bugs go in the
  Feature: description block.
- No `design.md` — seam strategy goes per-task in `tasks.md`; verify.command
  choice goes in the first setup task.
- No `design-review.md` — the discovery phase is already interactive; grilling
  a descriptive spec is unnecessary.

## Transition out

When planning is complete, suggest:
`/givn-characterize` — execute the RED/GREEN/REFACTOR characterization loop.

## Verify command

```
./run-tests.sh
```
