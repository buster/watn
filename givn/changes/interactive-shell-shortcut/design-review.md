# Design Review: interactive-shell-shortcut

## Review Method

An independent fresh-context review inspected the proposal, delta feature,
design, existing setup/PTY implementation, runner configuration, and all twelve
Arc42 chapters. The review identified product decisions and repository blockers;
the artifacts were hardened before sign-off.

## Resolved Decisions

| Topic | Resolution |
|---|---|
| First-use scope | The optional shortcut question is available in both explicit `watn setup` and implicit first-use onboarding. Enter accepts the default decline, so normal onboarding does not mutate shell files without an explicit opt-in. |
| Partial installation | Every selected target is attempted independently. All successes and failures are reported, successful writes remain, and any failure returns an aggregate non-zero setup result. No rollback is promised. |
| Embedded output line breaks | Only trailing CR/LF characters are removed. Embedded line breaks remain text in the editable buffer and are never evaluated. |
| Runtime E2E breadth | Bash receives the real interactive PTY path. Zsh and Fish receive regular generated-block syntax and contract coverage; the change does not claim runtime PTY evidence for them. |
| Existing setup compatibility | The optional interaction is attached to final Large Model confirmation and is not a sixth tab. The existing five-tab setup scenario remains valid when Enter accepts the default decline. |

## Hardened Findings

- `command watn -- "$question"` is required in every generated widget, keeping
  leading options and the reserved `completions` token inside one positional
  question while bypassing aliases/functions.
- Preselection uses only the basename of `SHELL`; existing target files do not
  silently select a shell.
- Marker replacement accepts zero markers or exactly one ordered pair. Duplicate,
  unmatched, and reversed layouts fail before any write and preserve the target.
- Existing targets are replaced through a same-directory temporary file and
  atomic rename; parent directories are created only after selection and target
  validation.
- The feature includes explicit scenarios for partial multi-shell failure,
  malformed markers, leading/reserved questions, non-zero status with partial
  stdout, embedded multiline output, and byte-for-byte failure preservation.
- Prompt redraw and cursor placement are split correctly: regular Bash probes
  assert buffer/cursor state, while the Bash PTY E2E scenario asserts visible
  redraw and no evaluation.
- The feature inventory has exactly two new interactions, each with one E2E
  scenario and a corresponding design matrix row.
- Arc42 assessment covers all twelve chapters. Durable chapters were updated,
  ADR-0018 was added, and its negative consequences are represented in chapter
  11 risks.

## Verification

- `givn lint --change interactive-shell-shortcut`: expected `@wip` findings only.
- `git diff --check`: passed.
- Runner strictness remains `.fail_on_skipped()` in
  `tests/features_runner.rs`; explicit RED step stubs are required before
  implementation.
- Regular and E2E commands remain `./run-tests.sh` and
  `./run-tests.sh --e2e`, with the existing tag filters.

## Sign-Off

DESIGN-REVIEW: PASS
