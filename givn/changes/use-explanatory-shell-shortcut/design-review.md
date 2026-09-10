# Design Review: use-explanatory-shell-shortcut

## Review Scope

The review covered `proposal.md`, the change feature, the change use case,
`design.md`, the permanent `use-shell` contract, the confirmed
`terminal-developer--interactive` Persona, the ideation handoff, all twelve
Arc42 chapter files, the Arc42 assessment, and active ADR records.

The permanent owner is `use-shell`; the changed capability is
`interactive-shell-shortcut`. `review-command-candidate` and
`route-reviewed-command` remain design subfunctions. No new use-case root,
Persona, capability, or shell-completions interaction is introduced.

## Findings And Dispositions

### Scope

**Finding:** The prior design deferred model-written purposes, treated purposes
as deterministic local text, and did not define the review provider response.

**Disposition:** Resolved. Review mode now requires a structured review response
with `review_version`, complete `command`, exact `stage_text`, model-written
`purpose`, and `purpose_status`. Delayed loading is shown only when the
structured response supports it; command-only, invalid, stale, or mismatched
responses show `purpose-unavailable`. No local purpose text is substituted.

**Finding:** The prior design opened review before a complete Candidate and did
not make `[DONE]` a review release boundary.

**Disposition:** Resolved. The existing progress line remains first. Review-mode
Candidate text is buffered until `[DONE]`; the review surface opens only after
the complete Candidate is assembled. Final acceptance is the only release
gate.

**Finding:** Direct positional, interactive-stdin, Ctrl-W, disabled, and `-x`
paths were not all specified with separate output and authorization rules.

**Disposition:** Resolved. The design contains a consumer routing matrix and
failure table. Direct accepted output remains the existing command-output
channel behavior. Ctrl-W records history and changes the shell line-editor
buffer only after acceptance. Eligible `-x` acceptance is the sole execution
authorization. Disabled and non-review `-x` retain `Execute now?` confirmation.

**Finding:** Candidate refinement lifecycle was incomplete.

**Disposition:** Resolved. Rephrase, regeneration, next-tier escalation,
highest-tier explicit provider catalog model selection, rejection, comparison,
candidate selection, interruption, direct command editing, purpose refresh, and
failure behavior are defined in the lifecycle table and feature scenarios.

### Technology Choices

**Finding:** The previous deterministic-purpose approach was simpler but did not
meet the confirmed product decision.

**Disposition:** Rejected. The structured provider response is the minimum
contract that preserves model-written purposes without inventing local prose.
The existing synchronous provider boundary remains; review mode adds only a
mode-specific buffered sink and response validation.

**Finding:** A full-screen terminal UI or alternate screen would simplify layout
but violate the product boundary.

**Disposition:** Rejected. The portable inline review surface is bounded and
transient. A portable `Presentation adapter` is mandatory; enhanced adapters
are optional and failure-safe.

**Finding:** A new provider endpoint or a new permanent review capability would
make the shared behavior more explicit but would expand ownership and protocol
scope.

**Disposition:** Rejected. The structured response is carried through the
existing provider completion path, and review remains an interaction of
`interactive-shell-shortcut` owned by `use-shell`.

### Missing Scenarios

The following gaps were added to the feature:

- Complete Candidate buffering before `[DONE]` and progress ordering.
- Structured ready fixture, delayed purposes, command-only fallback, purpose
  refresh failure, and unsupported flow.
- Direct editor Enter commit and separate editor Escape discard.
- Exact Flow/Candidates/Actions focus cycle, arrows, Tab, Shift-Tab, Enter, and
  review Escape.
- Enhanced adapter fallback and portable review-surface failure.
- Provider failure while a selected Candidate exists.
- Disabled Ctrl-W, direct positional output, interactive stdin output, disabled
  `-x`, and non-review `-x` confirmation.
- Rephrase, regeneration, higher tier, highest-tier explicit catalog model,
  rejection, comparison, selection, interruption, and shell repaint ownership.
- Bounded layout under narrow terminal dimensions.

`@e2e` tags were retained. No shell-completions row was added to the inventory
or matrix. The existing `corpus-infra` include remains because this capability
uses existing streaming and interruption infrastructure; no dangling new
relationship was introduced.

### Testability And E2E Fidelity

Every feature scenario has concrete visible assertions in domain language. The
four normalized interaction inventory rows each map to exactly one `@e2e`
scenario:

| Interaction | E2E scenario | Real driver and primary assertion |
|---|---|---|
| Ctrl-W review and acceptance | Developer accepts an explained candidate from Ctrl-W | Real Bash PTY; prompt, buffer, history, and no execution |
| Ctrl-W cancellation | Developer cancels a review without changing the shell buffer | Real Bash PTY; unchanged buffer/history and no release |
| Direct interactive review and acceptance | Developer accepts a candidate from an interactive terminal request | Real terminal subprocess; command-output channel contains only accepted Candidate |
| Eligible `-x` review and execution | Developer accepts an eligible `-x` candidate and it executes once | Real PTY subprocess; one execution and no second confirmation |

The E2E command is configured as `./run-tests.sh --e2e`; the regular command is
`./run-tests.sh`. The structured provider fixture and deterministic terminal
writer/Presentation adapter seams are specified without replacing the real
CLI boundary. `givn lint --change use-explanatory-shell-shortcut` reports only
the expected `@wip` findings after the planning edits; the disabled positional
and interactive-stdin assertions share one scenario to avoid a redundant subset
contract. The feature remains WIP because no implementation or step definitions
are being written in this planning change.

### Risk

The most likely implementation failure is contamination of stdout by review
surface bytes or release of a partial Candidate before final acceptance. The
mitigation is explicit channel vocabulary, controlling-terminal rendering, a
review-only buffered sink, `[DONE]` gating, cleanup-before-release ordering,
and separate direct/Ctrl-W/`-x` assertions. Secondary risks are stale purpose
responses and prompt repaint corruption; Candidate identity validation, visible
`purpose-unavailable`, deterministic renderer seams, bounded layout, and
shell-line-editor-owned repaint address them.

### Domain Context

The canonical glossary terms are `Intent`, `Candidate`, `Command flow`, `Stage
text`, `Stage purpose`, `Purpose status`, `Review surface`, `Review decision`,
`Presentation adapter`, `Review outcome`, `Command-output channel`,
`Controlling-terminal channel`, `Shell line-editor buffer`, and `Review history`.
The artifacts use these terms consistently. Generic `result`, `answer`, `panel`
as a domain boundary, and `proposal` as the selectable command are not used as
canonical terms.

The confirmed Persona `terminal-developer--interactive` is used only as a
stakeholder review lens. `Terminal developer` and `Shell line editor` are
interaction actors. No Persona was invented or promoted.

### ADR Qualification

The complete structured verdicts are recorded in `design.md`.

The review-mode buffering choice is **QUALIFIED** as an amendment to ADR-0015:

- Alternatives: PASS. Incremental review output and complete-Candidate
  buffering are materially different release boundaries.
- Architectural impact: PASS. The choice fixes the command-output release
  boundary while preserving the synchronous provider callback.
- Durable consequence: PASS. Reversal changes Ctrl-W, direct output, and
  eligible `-x` authorization across consumers.
- Lower-level artifact: PASS. ADR-0015 already owns the stream/completion
  boundary; this is a durable refinement of that boundary, not a parallel ADR.
- Existing-ADR check: PASS. Active ADR-0015 was found; no duplicate review
  buffering ADR exists.
- Routing: `AMEND_ADR` targeting ADR-0015. No new ADR file is created.

The structured response shape, renderer seam, keyboard contract, and consumer
routing are **NOT_QUALIFIED** for a new ADR. Their durable rationale is fully
owned by this design, the Gherkin specification, and affected Arc42 chapters.
Their canonical destination is exactly `design.md` for implementation-level
contract detail, with Arc42 chapters for durable architecture facts.

### Arc42 Independent Walk

The independent twelve-row assessment matches `arc42.md`:

| # | Independent result | Disposition |
|---:|---|---|
| 1 | Yes | Review comprehension and terminal usability goals are affected. |
| 2 | Yes | Controlling-terminal, no alternate-screen, structured-response, and release constraints are affected. |
| 3 | Yes | The review surface and command-output/controlling-terminal boundaries are new context. |
| 4 | Yes | Buffering, structured response, presentation fallback, and consumer routing alter strategy. |
| 5 | Yes | Review surface, response validation, and presentation adapter responsibilities are added. |
| 6 | Yes | Candidate generation, purpose loading, lifecycle, cleanup, and routing flows are added. |
| 7 | No | No production deployment artifact, service, or topology changes. |
| 8 | Yes | Output channels, terminal restoration, keyboard, configuration precedence, and non-evaluation are cross-cutting. |
| 9 | Yes | ADR-0015 is amended with review-mode buffering and release semantics. |
| 10 | Yes | Review usability, structured-purpose correctness, failure recovery, and `-x` authorization scenarios are added. |
| 11 | Yes | Response drift, buffering latency, terminal repaint, channel contamination, and routing risks are added. |
| 12 | Yes | The glossary gains the review domain vocabulary and exact channel terms. |

All twelve chapter files exist and contain project-specific content. Mermaid is
used for all architecture diagrams; no ASCII box-drawing or pseudo-diagram was
found. Chapter 07 remains unchanged because deployment topology is unchanged.
Chapter 09 records the ADR-0015 amendment without creating a new ADR. Chapter
11 records its buffering and output-isolation consequences.

## Hardening Receipt

- `proposal.md`: retains model-written purposes, defines structured response and
  channel boundaries, preserves owner and consumer scope, and removes the
  remaining open design ambiguity.
- `specs/use-shell/usecase.md`: distinguishes actors from the confirmed Persona,
  keeps one capability owner, retains the valid infrastructure include, removes
  shell-completions from this interaction inventory, and records routing rules.
- `specs/use-shell/interactive-shell-shortcut.feature`: adds the complete
  lifecycle, disabled paths, keyboard contract, failure outcomes, structured
  fixture, and bounded-layout scenarios without removing `@e2e` tags.
- `design.md`: defines the canonical vocabulary, structured response shape,
  lifecycle, keyboard and channel contracts, renderer seam, consumer matrix,
  failure outcomes, interaction matrix, and ADR routing.
- Arc42 assessment and affected chapters: updated with real content; row 7 is
  explicitly No.
- `docs/arc42/12-glossary.md`: records all new terms and anti-ambiguity
  definitions.
- `tasks.md`: not written or modified.

## Status

DESIGN-REVIEW: PASS
