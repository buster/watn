# Design Review: explain-command

## Review Method

A fresh-context grilling subagent read the proposal, the delta feature, the
design, the Arc42 update, the glossary, and the runner configuration, then
returned a ranked defect list (F1–F11). The human resolved two decisions; the
hardening subagent applied them and all findings to the planning artifacts. No
production code and no task list were written.

## Grilling Decisions

| Decision | Resolution |
|---|---|
| Missing scenarios | Decision 1 (A): add all five missing scenarios as `@givn.added @wip` variants/error paths of the same explain action. No inventory entry, no Interaction Coverage Matrix row, and no new `@e2e` scenario. |
| Terminal visual baseline | Decision 2 (A): discharge the baseline with three explicit Then-steps asserted against the ANSI-stripped PTY snapshot instead of leaving it as prose in the Visual Design Contract. |

## Added Scenarios (5)

1. `Enter closes the explanation card without releasing the command` — Enter
   (not Escape) closes the card; exit 0, an empty command-output channel, and
   the absent marker `/tmp/watn-explain-enter-should-not-run` prove nothing was
   released or executed.
2. `A second positional argument is refused` — `watn explain 'echo one' 'echo
   two'` is a usage error (exit 2) and no card opens; this makes the coverage
   QS-076 already claimed executable.
3. `A piped command without the marker is explained` — `printf '%s' 'ls |
   wc -l' | watn explain` shows both stages, and the boundary step asserts two
   distinct stage rows.
4. `The review-panel switches are inert for explain` — `watn explain
   --no-review-panel 'ls'` still opens the card.
5. `A malformed configuration file is reported` — a broken `config.toml` exits
   1 with a parse error and no card opens.

Scenario count: 12 before, 17 after. The `@e2e` tag remains only on the existing
end-to-end scenario.

## Resolved Findings

- **F1** — The Enter-close scenario asserts no release and no execution: empty
  command-output channel plus the absent execution marker.
- **F2** — QS-076 already claims the second-positional and empty-input usage
  errors; the new scenario makes the claim executable, so the quality
  requirements wording is left unchanged.
- **F3** — The non-empty piped command without `-` is now a scenario
  (`printf '%s' 'ls | wc -l' | watn explain`) asserting both stages.
- **F4** — The no-terminal scenario now carries a non-empty command (``When I
  run `watn explain 'echo test'` without a controlling terminal``) so it
  reaches the terminal gate instead of the empty-input usage error; the design
  step table matches.
- **F5** — Decision 2 discharged the visual baseline with the steps `the
  explanation card should show the close hint "esc close"`, `the explanation
  card should name "watn"`, and `the explanation card should not show internal
  identifiers`; the design's baseline bullet names those steps.
- **F6** — The `@e2e` When step is now in the design step table with its
  `portable-pty`/`sh -c` argv behavior and its responsibility to write the
  ANSI-stripped transcript evidence.
- **F7** — `the explanation card should show the stage:` now specifies
  whitespace-collapsed snapshot matching, and the new boundary step `the
  explanation card should show the stages {string} and {string} as separate
  stages` asserts two distinct stage rows; the multi-line stdin and piped stdin
  scenarios use it.
- **F8** — `docs/design/design-system.md` now lists the frame glyphs as
  `U+250C, U+2510, U+2514, U+2518, U+2500, U+2502`; no literal box-drawing
  character remains in the file.
- **F9** — The proposal's Out of Scope now records that `explain` becomes a
  reserved first token (like `completions`) and that a question beginning with
  `explain` must be quoted or passed after `--`.
- **F10** — The crossterm note no longer claims a vendor directory; the
  `tty_fd()` `/dev/tty` behavior is attributed to the 0.29.0 source.
- **F11** — The `-x` rejection steps and their assertions now specify
  finishing/waiting the PTY session and asserting on `world.exit_status` plus
  the merged PTY transcript, not just the card snapshot.

## Branch Verdicts

- **Scope** — PASS: the feature covers the proposal's observable behaviour; the
  five additions are variants and error paths of the same explain action, so
  the inventory and Interaction Coverage Matrix gain no row.
- **Tech choices** — PASS: no new dependency; the change reuses the review
  card, the structured review response, and the provider/session machinery.
- **Missing scenarios** — PASS after Decision 1: Enter-close, second
  positional, piped stdin without the marker, inert review switches, and
  malformed config now have scenarios.
- **Testability** — PASS: every scenario drives the real binary; all new steps
  are undefined until implemented, so RED fails through
  `.fail_on_skipped()`/`unimplemented!()`; the boundary step prevents a
  joined-command false pass.
- **E2E fidelity** — PASS: the single `@e2e` scenario drives the compiled
  binary in a `portable-pty` session through `sh -c` with a real argv element,
  and the new step row records its driver and transcript evidence.
- **Visual contract** — PASS: `## Visual Design Contract` has its `Interface:`
  line and the terminal baseline is discharged by explicit assertable steps.
- **Interaction Coverage** — PASS: one inventory entry maps to one matrix row
  with a real CLI driving mechanism; no `@e2e` tag was removed.
- **Risk** — PASS: no-terminal determinism comes from the captured-stderr
  requirement, PTY timing from `pty_wait_for_label`, and provider shapes from
  the in-process `httpmock` twin.
- **Domain context** — PASS: `use-shell` ownership, the `explain-command`
  capability, and the `terminal-developer--interactive` persona are unchanged;
  no persona was invented or promoted.
- **ADR qualification** — PASS: the explain entry-point candidate remains
  NOT_QUALIFIED with `design.md` as its canonical destination; no MADR is
  required.
- **Architecture documentation** — PASS: `arc42.md` records the impacted
  chapters (01, 03, 05, 06, 08, 10, 11, 12) and the glossary already holds
  `Explained command`, `Explanation-only card`, and `Verbatim command
  delivery`; no discrepancy was raised.
- **Ubiquitous language** — PASS: the specs and design use the glossary terms
  `Explained command`, `Explanation-only card`, `Stage text`, `Stage purpose`,
  and `Purpose status` consistently.

## Verification

- `givn lint --change explain-command`: exit 2 — `@wip` findings and
  shape/subset advisories only, which is the allowed result; exit 1 did not
  occur.
- Scenario count: 12 before, 17 after; `@e2e` count unchanged at 1.
- `docs/design/design-system.md`: no literal box-drawing characters remain.

## Sign-Off

DESIGN-REVIEW: PASS
