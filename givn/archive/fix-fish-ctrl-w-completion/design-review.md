# Design Review: Fix Fish Ctrl-W Completion Insertion

## Grilling Outcome

The fresh review found the initial plan too broad for its executable evidence:
the scenario captured the Fish buffer but did not commit it. The user chose to
narrow the requirement rather than add execution coverage. The proposal and
scenario now require only an actual line break in the editable Fish buffer;
committing the buffer and Fish-specific failure, empty-output, and
multiline-output branches are explicitly out of scope.

## Scope

The narrowed proposal, feature, design, and quality scenario describe the same
observable behavior: pressing Ctrl-W in Fish produces a comment line followed
by generated text with a real line break. Bash, Zsh, command generation, and
execution behavior are outside this change.

## Technical Choices

`printf '%s\\n%s' ... | string collect` is the smallest Fish-native solution
that produces one buffer value containing an actual newline. The real Fish
reader runs under `portable-pty`, which is required because `commandline` has
no meaningful editable buffer in a non-interactive Fish process. The existing
Fish source-contract step was updated alongside the new capability-specific
e2e steps.

## Scenario and E2E Review

The feature has one exhaustive user interaction inventory entry and exactly one
matching `@e2e` scenario. The scenario asserts the real Fish `commandline`
buffer, not repository state. The configured Cucumber runner uses
`.fail_on_skipped()`. The non-e2e run completed with `103 scenarios (103
passed)` and the e2e run completed with `68 scenarios (68 passed)`.

## Arc42 Review

The independent twelve-row assessment marks context and scope, building blocks,
runtime view, deployment verification, cross-cutting concepts, architecture
decisions, quality requirements, and risks as affected. The change assessment and durable docs
agree on those rows. Chapter 9 updates ADR-0018 rather than creating a second
decision; chapter 11 updates the existing shell-version and PTY risk
mitigations. All twelve chapter files contain substantive content and use
Mermaid for diagrams.

## Hardening Applied

- Renamed the scenario and matrix entry to match the narrowed buffer-only contract.
- Added the existing Fish source-contract step file to the design and task inventory.
- Updated runtime, context, building-block, deployment, cross-cutting, quality, risk, README, and ADR documentation.
- Made the Fish PTY child use the isolated temporary `HOME` and `XDG_CONFIG_HOME`.
- Ran `givn lint --change fix-fish-ctrl-w-completion` successfully.

## Status

DESIGN-REVIEW: PASS
