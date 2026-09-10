# Design Review: emphasize-review-shortcut-keys

## Review Scope

A fresh grilling subagent reviewed the proposal, the delta scenario, the
design, the card renderer and panel code, the step definitions, the use-case
inventory, and the arc42 chapters, and ran the library tests.

## Findings And Dispositions

### Missing step text (must-fix)

**Finding:** the scenario used `I discard the command editor`, which has no
binding; the existing step is `I press Escape in the command editor`.

**Disposition:** Resolved. The scenario now reuses the existing step text.

### Existing assertions broken by the new sequences (must-fix)

**Finding:** `review_accept_is_default` matches `[38;5;114m` and a contiguous
`⏎/a accept` tail; the bold sequence `[1;38;5;114m` and the reset before the
dim label break both. Two card unit tests pin `esc cancel` and `1-3 tier` as
contiguous dim text.

**Disposition:** Resolved. The design pins the raw-SGR matcher
(`[1;38;5;114m⏎/a` up to the reset) and moves the two unit tests to raw-SGR
checks. The E2E `accept ·` waits are unaffected because the label wait checks
whitespace-delimited words independently.

### False-GREEN risk (must-fix)

**Finding:** emphasis assertions built on the stripped-text render helper would
pass against the current dim renderer.

**Disposition:** Resolved. The design requires raw-SGR substring checks per
mode, so the steps fail before the renderer changes.

### Scope of explanation-only mode

**Finding:** the proposal said "each shortcut key in the card's hint lines",
which would include the explanation-only `esc close` hint with no scenario.

**Disposition:** Resolved. The proposal, design, and scenario scope the change
to the decision hints (review, command editor, model chooser); the
explanation-only card keeps its dim `esc close`. This is recorded in the
proposal's Out of Scope and the design.

### Accept versus other keys

**Finding:** "colored and bold" would pass even if every key were green.

**Disposition:** Resolved. The design pins the green `a` inside "accept" and
the label-colored first letter of "edit", "reject", "cancel", and "disable",
with dim remainders; the scenario's Then steps assert both sequences.

### Letter placement

**Finding:** the first rendering used a separate key token (`e edit`), which
duplicates the letter instead of pointing at it.

**Disposition:** Resolved. The review hint emphasizes the acting letter inside
the label word (`a`ccept, `e`dit, `r`eject, `c`ancel, `d`isable); only the
`⏎` and `esc` keys remain tokens. The editor and chooser hints keep their key
tokens because their keys are not letters.

### E2E fidelity

**Finding:** the design's rationale for the unaffected E2E wait was wrong, and
renderer-layer styling assertions are the right level.

**Disposition:** Corrected. Styling stays a renderer-layer assertion; the E2E
label-wait reason now names the word-split matcher. No interaction, inventory
row, or E2E scenario changes.

### ADR qualification

`NOT_QUALIFIED` with routing to the change design: local ANSI styling, no
alternatives or durable consequence. No ADR owns hint styling; ADR-0015 owns
stream completion.

### Arc42 independent walk

| # | Independent result | Disposition |
|---:|---|---|
| 5 | Yes | Review card renderer row records emphasized hint keys. |
| 10 | Yes | QS-067 records decision hints with emphasized keys. |
| 1-4, 6-9, 11, 12 | No | No flow, term, risk, or decision change. |

### Ubiquitous language

No term is added, renamed, or redefined. The change is presentation of the
existing `Review decision` keys.

### Lint

`givn lint --change emphasize-review-shortcut-keys` exits 2 with only the
expected `@wip` notice.

## Hardening Receipt

- `proposal.md`: scoped to decision hints; explanation-only card excluded.
- feature delta: one `@givn.added` scenario reusing existing step text.
- `design.md`: deterministic raw-SGR assertion contract; affected existing
  tests named; arc42 set 5 and 10.
- `arc42.md`: full 12-row table, `STATUS: DONE`.
- `tasks.md`: not written or modified.

## Status

DESIGN-REVIEW: PASS
