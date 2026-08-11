# watn Improvement Handoff Plan

Handoff snapshot: 2026-08-11

This file is the working handoff for the next agent. It describes the current
repository state, completed decisions, and remaining implementation work.

## Current Repository State

- Repository: `/home/buster/projects/watn`
- Branch: `main`
- Worktree: clean after merging origin/main; interactive-shell-shortcut archived in `fe8b0f9`
- Remote/upstream: `origin` configured
- Active givn change: none
- Archived transport work: `incremental-sse-rendering`, `isolate-test-transport`
- Archived release work: `release-truth-and-repository-cleanup`, `shell-completions`
- Archived setup work: `interactive-shell-shortcut`
- Current package version: `0.1.2` in `Cargo.toml`
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
- The shortcut supports optional setup selection for Bash, Zsh, and Fish,
  marked atomic startup-file replacement, idempotent installation, independent
  target reporting, native generated widgets, and non-evaluating
  `command watn -- "$question"` invocation.
- Shortcut verification passed 93 regular scenarios and 61 E2E scenarios,
  with 543 regular steps and 407 E2E steps.
- Merged coverage is 90.77% line coverage (`8115/8940`); branch coverage is
  unavailable (`0/0`).
- `givn lint`, formatting, compilation, clippy, documentation tests, release
  build, coverage measurement/merge, and review passed.

## Remaining Work

### 2. Highlight Active Setup Input

Improve the setup dialog's visual indication of where user input is currently
being entered. The border or box surrounding the active input location shall be
green; inactive input locations retain their existing styling.

Create this as a separate givn change:

```text
givn new highlight-active-setup-input
```

Use the full lifecycle and scenario-by-scenario TDD workflow. Preserve the
existing setup layout, keyboard behavior, and visible cursor contract.

### 3. Responsive Setup Model Filtering

Improve the setup dialog's model filter so the typed query remains visible while
the user is entering it. Typing must remain responsive while model searches run
in the background, and filter updates shall be debounced by 200 ms. The model
list shall update continuously as the debounced query changes, without blocking
further input on an in-flight search.

When the complete model list fits in one catalog request, load the list once and
filter it client-side instead of making server-side search requests. Use
server-side filtering when the catalog requires multiple requests or otherwise
cannot be loaded in one request. Results from an older in-flight search must not
replace the results for a newer query.

Create this as a separate givn change after the active-input styling change:

```text
givn new responsive-setup-model-filtering
```

The existing generation guard and worker cleanup are part of the baseline. Do
not regress the current newest-result-wins behavior.

### 8. Preserve Ctrl-W Requests In Shell Config

Extend the shell shortcut so the request that triggered Ctrl-W remains visible
after command generation. This must be implemented entirely in the generated Bash,
Zsh, and Fish configuration; `watn` itself should not change.

Investigate whether the widget can preserve the existing rendered shell line and
place the generated command on a new editable line without executing either line
during replacement. Account for shell-specific line-editor redraw behavior,
wrapped input, multiline input, terminal resizing, and stderr output.

At minimum, implement the portable shell-buffer fallback: replace the current
buffer with the original request as a shell comment, followed by a newline and
the generated command:

```text
# original request
generated command
```

Pressing Enter must ignore the comment and execute only the generated command.
Flatten or otherwise safely represent embedded newlines and control characters in
the original request so it remains one comment line. Preserve the original plain
buffer when `watn` fails or returns empty output. Do not evaluate the generated
command during replacement.

Do not promise shell-history persistence unless it is verified separately for
Bash, Zsh, and Fish. The required behavior is that the request remains visible in
the editable buffer and terminal transcript after Ctrl-W succeeds.

Required tests:

- Successful generation leaves the original request visible as a comment.
- Pressing Enter executes the generated command but not the comment.
- Requests containing spaces, shell metacharacters, and embedded newlines remain
  comments.
- Failed or empty generation preserves the original buffer.
- The generated command remains editable and is never executed automatically.
- Bash, Zsh, and Fish redraw the resulting multiline buffer correctly.
- The behavior is tested with wrapped input and visible stderr output.

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
