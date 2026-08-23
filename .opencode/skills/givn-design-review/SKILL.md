---
name: givn-design-review
description: Stress-test the plan before tasks are written — grill the design one question at a time, harden the artifacts, then sign off.
---

# givn-design-review

Stress-test the complete plan for change `<change-id>` before the task list
is written. Grill every branch of the design, harden the artifacts with what
you learn, then sign off.

## Mandatory, non-skippable, fresh context

Never skip this step. Never run it as a continuation of the session that
wrote the design — a model grilling its own just-written plan will confirm
its own assumptions instead of finding blind spots. Spawn the grilling
subagent into a genuinely fresh context via the Task tool. If the harness
lets you choose a model or reasoning-effort tier for the spawned subagent,
choose the most capable one available and set reasoning/thinking effort to
high — this step exists to catch subtle scope gaps and untestable scenarios
that a fast/low-effort model tends to rubber-stamp instead of catching.

## Context

- Design review file: `givn/changes/<change-id>/design-review.md`
- Instructions: run `givn instructions design-review --change <change-id>`
- Runs after: design
- Runs before: tasks

## Two phases

### Phase 1: Grilling (subagent → orchestrator → user)

A grilling subagent reads all planning artifacts (proposal, specs, design) and
the relevant codebase, then returns a ranked question list with recommended
answers. The orchestrator presents questions to the user one at a time — waiting
for each answer before the next.

Required branches:
- **Scope** — spec matches proposal exactly?
- **Tech choices** — right stack for these scenarios? Simpler alternatives?
- **Missing scenarios** — observable behaviours without a scenario? Error paths?
- **Testability** — every scenario can genuinely fail in RED?
- **E2E fidelity** — read `givn instructions specs --change <change-id>`
  and verify its normalized action scope, interface type, real driver, and
  primary assertion rules are applied without redefinition.
- **Interaction Coverage** — does every entry in the spec's User Interaction
  Inventory correspond to a row in the design's Interaction Coverage Matrix?
  Does every matrix row name a valid, non-empty driving mechanism (real
  browser driver for Web UI; real HTTP client for HTTP API; real subprocess
  for CLI)? Cross-reference the inventory comment in the `.feature` file
  against the matrix table in `design.md` — any inventory entry without a
  matching matrix row is a finding.
- **Risk** — most likely failure mode and mitigation?
- **ADR qualification** — use the complete procedure from the canonical Arc42
  addon instruction. Only a structured verdict with every mandatory gate
  passing requires a MADR. A non-qualifying choice names exactly one canonical
  lower-level artifact; a refinement amends an existing ADR and a replacement
  supersedes it. `Must be shared` is supporting evidence only. This is guidance,
  not runtime enforcement.
- **Architecture documentation (arc42)** — N/A if `addons.arc42` is not
  enabled. Otherwise, independently re-derive the expected chapter set
  before trusting `arc42.md`'s claim: walk all 12 rows of the arc42-docs
  instruction's selection table yourself against proposal.md/design.md,
  form your own Yes/No per row, then diff against `arc42.md`'s table. A row
  you mark "Yes" that `arc42.md` marks "No" or omits is a finding —
  omission is the failure mode that slips through, not just contradiction.
   For every "Yes" row (either party), confirm the chapter content actually
   matches design.md, not placeholder text or a stale contradiction. Does
   chapter 09 have a MADR entry for every qualified ADR candidate in design.md,
   and does chapter 11 reflect that decision's consequences? A non-qualifying
   design choice is complete when its canonical destination is recorded.
  `arc42.md`'s `STATUS: DONE` is a self-report, not verification — treat any
  mismatch, including a missing row, as a finding.

If a question can be answered by exploring the codebase, the subagent does that
and reports the finding instead of asking the user.

### Phase 2: Hardening (hardening subagent)

Applies decisions reached during grilling:
- Update `design.md` if tech decisions changed.
- Add missing scenarios to `specs/**/*.feature` with `@givn.added @wip`.
- **Never remove `@e2e` tags from any scenario.** If the e2e infrastructure
  (`verify.e2e_command`) is not configured yet, leave the `@e2e` tags in
  place. The verify-e2e hook will fail at check/archive time — that failure
  is correct: an `@e2e` scenario without a configured runner IS an unenforced
  gap. Removing `@e2e` tags to bypass the gate is a procedure violation and
  will be caught by the review fabrication audit. If the e2e infrastructure
  is genuinely not buildable yet, record this as a tracked open question in
  `design-review.md` with a concrete date or condition for resolving it —
  never as a tag removal.
- Run `givn lint --change <change-id>` — must exit 0 or 2.
- Does NOT write `tasks.md` — that is the next step.

## Sign-off

When both phases are complete, `design-review.md` ends with:

```
DESIGN-REVIEW: PASS
```

All questions resolved. Otherwise: `DESIGN-REVIEW: FAIL`.

## After sign-off

```
givn status --change <change-id>
```

Next step: `/givn-tasks`.
