---
description: Stress-test the plan before tasks are written — grill the design, harden the artifacts, sign off
---

Stress-test the plan (proposal, spec, design) before the task list is written.
Catches scope, architecture, and testability problems when they cost a markdown
edit rather than a rewrite.

**Input**: Optionally specify a change ID after `/givn-design-review`. If
omitted, infer from conversation context, or auto-select if only one active
change exists. If ambiguous, list changes and ask.

---

## Mandatory, non-skippable, fresh context

This gate exists to catch a flawed plan before it produces flawed tasks and
flawed code. Two conditions make it work; violating either one defeats its
purpose:

- **Never skip it.** Do not let this step be bypassed because "the design
  looks fine" or "there's no time." A design that looks fine to the person
  who just wrote it is exactly what this gate exists to check.
- **Never run it as a continuation of the session that wrote the design.**
  Both subagents below MUST be spawned via the Task tool into a genuinely
  fresh context — not the same conversation, not a subagent that has already
  seen the architect's reasoning. A model that just produced the design
  cannot credibly grill its own blind spots; it will confirm its own
  assumptions. If invoked directly after `/givn-propose` in the same
  conversation, still spawn the grilling subagent fresh — do not let it
  inherit the orchestrator's or the architect's context.
- **Use a capable reasoning model with high thinking/reasoning effort for
  the grilling subagent.** This step depends on catching subtle scope gaps,
  wrong tech choices, and untestable scenarios — the same failure modes a
  fast, low-effort model is prone to missing or rubber-stamping. When the
  harness lets you choose a model or reasoning-effort tier for the spawned
  subagent, choose the most capable one available and set reasoning/thinking
  effort to high. Do not spawn the grilling subagent on a fast/cheap/
  low-reasoning-effort tier to save time — that produces a review that
  passes everything, which is worse than no review at all because it
  creates false confidence.

---

## Role: orchestrator

This command is an **orchestrator** with two subagents and an interactive loop
in between:

1. **Grilling subagent** — reads all planning artifacts and the codebase; returns
   a ranked list of questions with recommended answers.
2. **Interactive loop** — the orchestrator presents questions one at a time to
   the user, records answers.
3. **Hardening subagent** — applies all decisions reached to the artifacts and
   writes `design-review.md`.

The interactive loop runs in the orchestrator's own context so the
one-question-at-a-time dialogue works naturally.

---

## Steps

### 1. Resolve the change

```sh
givn status --change <id> --json
```

Identify the active change. Announce: "Using change: <id>".

Check the `artifacts` array:
- `design` must be `"status": "done"` before proceeding.
- If `design` is not done, stop: "Design must be complete before design-review."

### 2. Collect instructions

```sh
givn instructions design-review --change <id>
```

When `--json` is supplied, capture `id`, `generates`, `requires`, and
`instruction`. Use `generates` and `requires` to orient the review; the
instruction text is the resolved policy. Without `--json`, capture the
resolved instruction text only.

### 3. Spawn the grilling subagent

Use the **Task tool** to spawn a grilling subagent in a genuinely fresh
context — never a continuation of this conversation or of whatever session
wrote the design. If the harness exposes a model or reasoning-effort choice
for the spawned subagent, select the most capable model available with
reasoning/thinking effort set to high. This subagent's entire value is
catching what the design's author missed; a low-effort model in the same
context as the author defeats the purpose.

Pass a self-contained prompt:

```
You are a grilling subagent for givn change '<id>'.

Your job: read the full plan and return a prioritised list of questions that
must be answered before implementation starts. You do NOT ask the user anything
directly — you return the questions so the orchestrator can present them one at
a time.

## Read everything first

Read all of these in full before forming any questions:
- givn/changes/<id>/proposal.md
- givn/changes/<id>/specs/**/*.feature
- givn/changes/<id>/design.md
- Existing codebase files named in design.md (integration points, affected
  modules, patterns). If a question can be answered by exploring the codebase,
  explore it and record the finding as a resolved item — do not include it as
  a question for the user.
- Before opening `arc42.md`, walk all 12 rows of the arc42-docs instruction's
  chapter-selection table (`givn instructions arc42-docs`) yourself against
  proposal.md/specs/design.md, and form your own Yes/No per row. Only after
  that, open `givn/changes/<id>/arc42.md` and every chapter file under
  fixed `docs/arc42/` path that the marker file claims to have touched. This is
  required input for the Architecture
  documentation branch below — do not skip it because arc42.md says
  `STATUS: DONE`; that marker is a self-report from the same agent that
  wrote the docs, not verification.

## Required branches to cover

Walk the design tree and produce at least one question per branch (or record it
as "no issue found" if genuinely clean):

- **Scope** — does the spec match what the proposal asked for? Anything extra?
  Anything missing? Is "done" unambiguous?
- **Tech choices** — is the stack in design.md the right fit? Are there simpler
  alternatives that satisfy all the observable behaviours in the spec?
- **Missing scenarios** — are there observable behaviours implied by the proposal
  but absent from the spec? Error paths? Boundary conditions?
- **Testability** — can every scenario fail in RED with no production code?
  Do Then-steps assert concrete values, not "it ran"? Does design.md name a
  concrete strict-mode mechanism, not-implemented stub pattern, and
  single-scenario run command? Missing or vague on any of the three is a
  finding — this is the root cause of past fabricated changes.
- **E2E fidelity** — read `givn instructions specs --change <id>` and verify
  that design.md applies its normalized action scope, real-interface rule, and
  concrete driving mechanism. A silent redefinition, vague interface type,
  HTTP/fetch plan for a browser UI, or duplicate coverage of one action is a
  finding.
- **Risk** — what is the single most likely way this plan fails during
  implementation? What is the mitigation?
- **Architecture documentation (arc42)** — this branch is an independent audit, not
  a review of `arc42.md`'s self-report — do the steps in this order:
  1. Before opening `arc42.md`, walk all 12 rows of the arc42-docs
     instruction's chapter-selection table yourself against the actual
     scope of `proposal.md`/`design.md` (new components/modules →
     building-block-view, new runtime flows → runtime-view, deployment
     changes → deployment-view, a new ADR's "Bad, because..." consequence →
     risks-and-technical-debt, etc.). Record your own Yes/No per row.
  2. Open `arc42.md` and its 12-row table. Diff row by row against your own
     assessment. **Any row you marked "Yes" that `arc42.md` marks "No",
     leaves blank, or omits entirely is a finding** — this
     omission is the actual failure mode seen in production use (an agent
     stopping after finding 2-3 obviously-affected chapters instead of
     checking all 12), and it is more dangerous than a contradiction
     because nothing points at it. Do not treat "arc42.md only lists 3
     chapters and doesn't mention the rest" as implicitly meaning "checked
     and found not applicable" — the instruction requires every row to be
     explicit; a missing row is unverified, not resolved.
  3. For every chapter marked "Yes" by either party: open it and confirm the
     content actually reflects design.md — not placeholder text, not a
     stale description that contradicts design.md's Architecture Impact or
     Technology Decisions sections. A contradiction is a finding.
   4. For every ADR candidate in design.md, delegate qualification to the
      canonical Arc42 ADR instruction and verify its structured verdict. Only
      a candidate with all mandatory gates `PASS` requires a chapter-09 MADR
      entry (status/date/decision-makers, considered options, decision outcome,
      consequences), and its consequences must be reflected in chapter 11.
      A non-qualifying choice must instead name exactly one canonical feature,
      design, Arc42, process, project-documentation, or code destination. A
      refinement must amend an existing ADR; a replacement must supersede it.
      `Must be shared` is supporting evidence only. A missing MADR is a finding
      only when a qualified candidate has no corresponding entry.
   5. A chapter file still containing only its scaffolded placeholder text,
 while `arc42.md` claims that chapter was "updated," is a
  finding — the same fabrication pattern as an empty step body reported
  as passing.

## Return format

Return a JSON array of objects, ordered by priority (most critical first):

[
  {
    "branch": "<branch name>",
    "question": "<the question>",
    "recommended": "<your recommended answer>",
    "codebase_finding": null   // or a string if you resolved it by exploring
  },
  ...
]

Also return a separate array of items you resolved by codebase exploration
(so the orchestrator can report them):

{
  "questions": [ ... ],
  "resolved_by_codebase": [
    { "branch": "...", "finding": "..." }
  ]
}

Return only the JSON — no prose before or after it.
```

### 4. Interactive grilling loop

Wait for the grilling subagent to return.

If any `resolved_by_codebase` items were found, report them first:

```
## Design review: <id>

**Resolved by codebase exploration:**
- [Branch]: <finding>
```

Then work through `questions` one at a time:

For each question:
1. Present it to the user:
   ```
   **Q<N> (<branch>):** <question>
   **Recommended:** <recommended answer>
   ```
2. **Wait for the user's response.** Do not present the next question until
   the current one is answered.
3. Record the outcome: agreed / overridden (note what the user decided) /
    deferred (flag as open question).

When all questions are answered, summarise:

```
**Grilling complete.** <N> questions answered.
Open questions: <list or "none">

Proceeding to hardening...
```

If any open question remains unresolved, stop and report the
blocker. Do not proceed to hardening until the user resolves it.

### 5. Spawn the hardening subagent

Use the **Task tool** to spawn a hardening subagent in a clean context.

Pass a self-contained prompt that includes the full grilling outcome:

```
You are a hardening subagent for givn change '<id>'.

The plan has been grilled. Your job: apply the decisions reached to the
existing artifacts and write design-review.md.

## Grilling outcome

### Resolved by codebase exploration
<paste resolved_by_codebase items>

### Questions and answers
<paste each question, recommended answer, and the user's decision>

### Open questions
<list, or "none">

## Your tasks

### 1. Apply hardening edits

Based on the grilling outcome, edit the existing artifacts as needed:

- `givn/changes/<id>/design.md` — update if any tech decisions changed.
- `givn/changes/<id>/specs/**/*.feature` — add missing scenarios with
  `@givn.added @wip` if any were identified. Then run:
  ```sh
  givn lint --change <id>
  ```
  Exit 0 or 2 = correct. Exit 1 = parse error — fix before continuing.
- Do NOT create tasks.md — tasks has not been written yet and is not your
  responsibility.
- Do NOT change proposal.md unless the grilling revealed a fundamental scope
  mismatch that the user explicitly decided to fix — flag that as a blocker.

### 2. Write design-review.md

Write `givn/changes/<id>/design-review.md` following this template structure:

## Grilling log
| # | Branch | Question | Recommended | Outcome |

## Resolved by codebase exploration
| Branch | Finding |

## Open questions
| # | Question |

## Architecture documentation (arc42) check
Fill in all 12 rows — do not
leave blank and do not only list the rows arc42.md claims:
| # | Chapter | Grilling subagent's own Yes/No | arc42.md's Yes/No | Match? | Content matches design.md? |
(one row per chapter, 1-12)
- [ ] All 12 rows independently assessed against proposal.md/design.md before opening arc42.md.
- [ ] No row where the subagent said "Yes" and arc42.md said "No"/omitted it (any such row is a blocker, listed in Open questions).
- [ ] Every qualified ADR candidate in design.md has a MADR entry in chapter 09.
- [ ] Every MADR "Bad, because..." consequence has a counterpart in chapter 11.
- [ ] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening
| Artifact | Change summary |

## Sign-off
- [ ] All branches walked.
- [ ] All open questions resolved.
- [ ] design.md reflects decisions reached.
- [ ] specs/*.feature updated for any missing scenarios.
- [ ] givn lint exits 0 or 2.
- [ ] Architecture documentation (arc42) check completed above.

DESIGN-REVIEW: PASS

Write `DESIGN-REVIEW: PASS` as the final line ONLY if all questions are resolved.
If there are unresolved open questions, write `DESIGN-REVIEW: FAIL`
instead and list them.

### 3. Report back with
- Artifacts edited (list, or "none")
- Lint result (exit code)
- Whether DESIGN-REVIEW: PASS or FAIL was written, and why
- Path written: givn/changes/<id>/design-review.md
```

### 6. Wait for the hardening subagent to complete

If the subagent reports `DESIGN-REVIEW: FAIL`, surface the blockers to the user
and stop. Do not suggest next steps until blockers are resolved.

### 7. Show final status

```sh
givn status --change <id>
```

Display next step: "Run `/givn-tasks` to write the task list."

---

## Output on completion

```
## Design review complete: <id>

**Grilling:** <N> questions across <M> branches
**Resolved by codebase:** <N> items
**Hardening:** <summary of edits, or "no changes needed">
**Open questions:** <list or "none">

**Review written:** givn/changes/<id>/design-review.md  (DESIGN-REVIEW: PASS)

**Next:** Run `/givn-tasks` to write the task list.
```

## Output on blocker

```
## Design review blocked: <id>

**Unresolved question:** <question>
**Status:** DESIGN-REVIEW: FAIL

Resolve the blocker before proceeding.
```

---

## Guardrails

- The orchestrator presents exactly one question per turn. Never batch questions.
- Do not proceed to hardening while any question is unanswered.
- The hardening subagent must not write tasks.md — that is the next artifact.
- Hardening edits do not reset design or specs status — they are ordinary edits.
- `DESIGN-REVIEW: PASS` requires all questions resolved. If any question remains
  unanswered, write `DESIGN-REVIEW: FAIL` and surface them to the user.
- `givn lint` must exit 0 or 2 after any .feature edits before signing off.
