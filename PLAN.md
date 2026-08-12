# watn Improvement Handoff Plan

Handoff snapshot: 2026-08-12

This file is the working handoff for the next agent. It describes the current
repository state, completed decisions, and remaining implementation work.

## Current Repository State

- Repository: `/home/buster/projects/watn`
- Branch: `main`
- Worktree: clean on `main` after archiving `preserve-ctrl-w-requests-in-shell-config`
- Remote/upstream: `origin` configured
- Active givn change: none
- Archived transport work: `incremental-sse-rendering`, `isolate-test-transport`
- Archived release work: `release-truth-and-repository-cleanup`, `shell-completions`
- Archived setup work: `interactive-shell-shortcut`, `highlight-active-setup-input`, `responsive-setup-model-filtering`
- Current package version: `0.1.4` in `Cargo.toml`
- Current CLI version: read from `CARGO_PKG_VERSION` in `src/main.rs`

Do not amend existing commits. Do not push unless explicitly requested.

## Required Session Start

Run this before exploring or editing:

```text
givn instructions
```

The project uses givn. The required lifecycle is:

```text
new -> propose -> spec -> design -> design-review -> tasks -> implement -> review -> archive
```

Keep exactly one active change. Complete one scenario at a time. Use RED,
GREEN, REFACTOR, and one atomic scenario commit. Record the commit hash in
`tasks.md` immediately after the scenario commit.

## Baseline Contracts

These contracts must remain true unless a later change explicitly changes them
through a reviewed proposal and specification:

- Commands and generated command content go to stdout.
- Metadata, prompts, spinners, setup guidance, and diagnostics go to stderr.
- LiteLLM is a model-discovery service only. It must never replace the active
  chat-completion provider.
- `watn models` changes model tiers without replacing the active provider.
- `watn setup` and implicit first-use setup save a valid provider before the
  first model-catalog request.
- An absent thinking-tier reasoning value retains the existing `high` default.
- Empty and unknown persisted reasoning values disable reasoning.
- The test endpoint seam is not available in normal or release-profile builds.
- A release binary is currently dynamically linked. Do not claim universal
  static deployment without producing and verifying static artifacts.
- Saved literal credentials and exact environment references are authoritative.
- A missing saved environment reference is an authentication error and does not
  fall through to another environment variable.
- Successful Ctrl-W generation displays the flattened request as a shell comment
  above the generated command; failed or empty generation preserves the original
  buffer, and generated text is never evaluated during replacement.

## Coverage And Test Command Contract

- Coverage and test commands must be `.sh` scripts configured in
  `givn/commands.yaml`.
- Those scripts must compute overall line and branch coverage by merging E2E and
  non-E2E coverage.
- The README must contain the resulting overall line and branch coverage counts
  once, rather than multiple different counts.

## Completed Work

- `incremental-sse-rendering` is archived and verified.
- `release-truth-and-repository-cleanup` is archived and verified.
- `shell-completions` is implemented, verified, and archived.
- `interactive-shell-shortcut` is implemented, verified, and archived in
  commit `fe8b0f9`.
- `preserve-ctrl-w-requests-in-shell-config` is implemented, reviewed, verified,
  and archived in commit `ce6a7fb`. Bash, Zsh, and Fish widgets now preserve the
  request comment, flatten CR/LF/TAB safely, isolate commit-time execution, and
  preserve the original buffer on failure or empty output.
- `highlight-active-setup-input` is implemented, reviewed, archived, and pushed
  in commit `81bdf63`. Its four PTY scenarios cover URL, credential,
  model/reasoning, and shortcut focus borders while preserving inactive styling.
- `responsive-setup-model-filtering` is implemented, reviewed, archived, and
  pushed in commit `eb4149c`. Complete catalogs filter locally, incomplete
  catalogs use debounced provider search, the current query stays visible, stale
  results are discarded, and search workers are joined on exit.
- The shortcut supports optional setup selection for Bash, Zsh, and Fish,
  marked atomic startup-file replacement, idempotent installation, independent
  target reporting, native generated widgets, and non-evaluating
  `command watn -- "$question"` invocation.
- The final archive verification passed 110 regular scenarios and 69 E2E
  scenarios, with 628 regular steps and 477 E2E steps.
- Merged coverage is 90% line coverage (`9171/10135`); branch coverage is
  unavailable (`0/0`).
- `givn lint`, formatting, compilation, clippy, documentation tests, release
  build, coverage measurement/merge, review, and archive gates passed.
- Zsh and Fish syntax checks are optional when those executables are absent from
  local environments. CI installs both dependencies and requires their checks.
- Interactive ZLE/Fish redraw behavior and wrapped-line rendering remain
  classified as legitimately hard to measure outside real interactive shells;
  generated-block assertions, syntax checks, and the real Bash subprocess cover
  the implemented contract.

## Remaining Work

No active implementation work remains. Start a new reviewed givn change for any
additional shell-history, interactive redraw, wrapped-line, or terminal-resize
behavior.

## Handoff Rules

- Read `givn instructions` before acting.
- Inspect `givn status --change <id>` before editing an active change.
- Preserve user changes and never reset or checkout unrelated work.
- Use `apply_patch` for manual edits.
- Use one active change at a time.
- Use one scenario commit for RED, GREEN, and REFACTOR.
- Do not amend existing commits.
- Do not push unless the user explicitly requests it.
- Keep secrets out of diagnostics, persisted test output, and commits.
- Prefer the smallest correct implementation. Do not add compatibility layers
  without a concrete persisted-data or external-consumer requirement.
