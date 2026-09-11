# Design Review: quicksetup-parameters

## Review Scope

Grilling covered `proposal.md`, the delta feature, `design.md`, `arc42.md`,
the permanent `configure-interactive` contract and quick-setup scenarios,
`src/main.rs` (CLI and first-run wiring), `src/quicksetup.rs`,
`src/provider/setup.rs`, the quick-setup PTY harness, and the changed arc42
chapters. The grilling subagent ran in a fresh context; findings are
dispositioned below.

## Findings And Dispositions

### The catalog assertion had no sentinel

**Finding (blocker):** the prefill scenario asserted "no model catalog request
should be sent" without the `provider requests are captured by a sentinel`
given, so the assertion would panic on a missing mock.

**Disposition:** Resolved. The sentinel given was added to all three e2e
scenarios.

### Interaction inventory was stale

**Finding (major):** the matrix claimed two inventory entries that did not
exist in `usecase.md`.

**Disposition:** Resolved. The change-side `configure-interactive/usecase.md`
adds three rows: prefill with parameters, seed remaining tiers from a
small-model parameter, and override tiers with parameters, each mapped to one
of the three e2e scenarios.

### The README examples were not the tested commands

**Finding (major):** scenario 2 added `--model` while README example 2 does
not, and the `--model-normal`/`--model-thinking` prefills had no behavioral
scenario.

**Disposition:** Resolved. Scenario 2 is now README example 2 exactly
(`--model-small` only, asserting normal/thinking fall back to the accepted
small answer). Scenario 3 exercises `--model` plus `--model-small` and
`--model-thinking` overrides and asserts the per-tier result.

### Literal-key exposure was unrecorded

**Finding (major):** the README recommends a literal `--key` example with no
security note, and arc42 marked chapters 10 and 11 unchanged.

**Disposition:** Resolved. The README states that a literal key is visible in
shell history and process listings and that the `${ENV_VAR}` form is preferred;
`arc42.md` marks chapter 11 affected and R-011 records the argv exposure.

### Empty flag values, arc42 wording, and `@wip`

**Finding (minor):** empty flag strings were unspecified; chapter 06 omitted
`--model-normal`/`--model-thinking`; chapter 04 still said "five suggested
questions"; the design did not state the `@wip` discipline.

**Disposition:** Resolved. Empty flag values normalize to `None`; chapter 06
lists all six flags; chapter 04 says six; the test-runner section records the
`@wip` clearing per task.

## E2E Fidelity And Interaction Coverage

The three e2e scenarios run the real watn binary in a PTY and inspect the
persisted config; the help scenario runs a real subprocess. The sentinel proves
no catalog request. Every new inventory row maps to exactly one e2e scenario.

## ADR Qualification

`NOT_QUALIFIED` with this design as the canonical artifact. ADR-0026's
plain-line quick-setup decision is unchanged; prefills refine it without a new
boundary.

## Arc42 Independent Walk

| # | Independent result | Disposition |
|---:|---|---|
| 3 | Yes | User input names the parameter prefills. |
| 4 | Yes | First-run strategy records prefills; the "six questions" wording is fixed. |
| 5 | Yes | `quicksetup` module row records prefills. |
| 6 | Yes | First-run scenario names the six flags. |
| 11 | Yes | R-011 records the literal `--key` argv exposure. |
| 12 | Yes | Quick setup definition mentions parameter prefills. |
| 1, 2, 7, 8, 9, 10 | No | No goals, constraints, deployment, crosscut, decision, or quality-scenario change. |

## Ubiquitous Language

No term is added; Quick setup keeps its definition and gains the parameter
prefill detail. No undefined or inconsistent term remains.

## Hardening Receipt

- `design.md`: empty-value normalization, README/README-example mapping,
  literal-key note, three matrix rows, chapter 11 affected, `@wip` note.
- feature delta: sentinel added; scenario 2 is the README example; scenario 3
  adds tier-override precedence.
- change-side `usecase.md`: three new interaction rows; base hash rebased.
- Arc42 chapters 04, 06, 11 updated; `arc42.md` corrected.
- README Quick setup rewritten with the install command, both examples, the
  test query, and the literal-key warning.
- `givn lint --change quicksetup-parameters`: `@wip` notices only.
- `tasks.md`: not written.

## Status

DESIGN-REVIEW: PASS
