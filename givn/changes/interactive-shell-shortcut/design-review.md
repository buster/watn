# Design Review: interactive-shell-shortcut

## Review Method

An independent fresh-context review re-read the revised proposal, feature,
design, task list, implementation surface, runner configuration, and all twelve
Arc42 chapters. The review treated the user's no-PTY direction as a design
constraint: generated configuration syntax and deterministic Bash subprocess
behavior are the verification boundary.

## Resolved Decisions

| Topic | Resolution |
|---|---|
| Interactive E2E scope | Do not drive a terminal emulator or claim visible Readline redraw. E2E runs `bash -n`, `fish -n`, and a non-interactive Bash process against isolated generated artifacts. |
| Parser availability | Bash and Fish are mandatory for this change's E2E verification. The current environment provides `/usr/bin/bash` and `/usr/bin/fish`; Zsh remains a static contract because it is unavailable. |
| Bash subprocess E2E | Retain it because it sources the installed generated block in a fresh Bash process, exercises PATH resolution and buffer assignment, and asserts no evaluation. Regular probes cover the same logic with lower-level fixture reuse; E2E proves the installed artifact boundary. |
| Prompt redraw | Native shell behavior remains part of the generated widget contract, but visible terminal redraw is not claimed or asserted by this change. Bash uses the Readline binding; Zsh and Fish retain explicit redisplay/repaint commands. |
| Setup testing boundary | Regular steps may use the typed shortcut-selection/install seam without a PTY. E2E is reserved for real shell parser/process boundaries; setup-screen interaction is covered by the existing setup wizard capability rather than duplicated here. |

## Hardened Findings

- The stale PTY-era E2E scenarios were replaced with syntax-focused Bash/Fish
  parser validation and a fresh Bash-process widget scenario.
- `interactive_shell_shortcut_e2e_steps.rs` must implement the exact four new
  parser/process steps before either E2E scenario leaves `@wip`; no obsolete PTY
  step remains part of the feature contract.
- The inventory has exactly two entries and the design matrix has exactly two
  matching rows with real shell-process driving mechanisms.
- Parser exit status is the primary E2E assertion. Filesystem checks remain
  secondary and all targets are isolated from the developer's HOME.
- Regular Bash probes continue to assert buffer/cursor, quoting, status,
  trailing/embedded line behavior, and no evaluation. They do not claim visible
  prompt redraw.
- Marker validation, atomic replacement, independent aggregate reporting,
  leading-option/reserved-token handling, and failure preservation remain
  covered by regular scenarios.
- Arc42 remains a twelve-chapter impact. The glossary collision was removed by
  distinguishing `Ratatui widget` and `Shell widget`; ADR-0018 and chapter 11
  now describe parser/process verification rather than PTY evidence.

## Residual Risks

- Setup-screen interaction is not re-tested through a new PTY in this change;
  the existing unified setup wizard E2E remains the owner of that boundary.
- Visible prompt redraw is delegated to each shell's native line-editor contract
  and is not mechanically verified without a terminal emulator.
- Zsh cannot be parser-checked in the current environment; its generated block
  is checked through exact API/binding contracts and remains a documented local
  environment limitation.

## Verification

- `givn lint --change interactive-shell-shortcut`: expected `@wip` findings only.
- `git diff --check`: passed before this review update.
- Strict mode remains `.fail_on_skipped()` in `tests/features_runner.rs`.
- Regular and E2E commands remain `./run-tests.sh` and
  `./run-tests.sh --e2e`, with distinct tag filters.

## Sign-Off

DESIGN-REVIEW: PASS
