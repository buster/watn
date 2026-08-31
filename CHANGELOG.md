# Changelog

All notable changes to watn are documented in this file.

The release sections are generated with [git-cliff](https://git-cliff.org/).
Versions are selected manually and use annotated `vX.Y.Z` Git tags.
## [Unreleased]

## [0.3.0] - 2026-08-31

### Bug Fixes

- **release:** Publish repository changelog notes

- **release:** Link crates.io package versions

- **ci:** Install givn for specification fixtures

- **ci:** Pin givn fixture installer to git revision

- **ci:** Remove Watn Givn fixture dependency

- **security:** Update h2 0.4.15 -> 0.4.16 (RUSTSEC-2026-0258)

- **tests:** Rename body_contains to body_includes for httpmock 0.8

- **tests:** Rename hits to calls for httpmock 0.8

- **release:** Exclude AGENTS.md from package


### Documentation

- **upgrade:** Record aggregate migration plan migrate-0-2-0-to-0-3-0

- Update README to streamline demo and shortcuts


### Features

- **shortcut:** Record Ctrl-W requests in shell history


### Other Changes

- Delete docs/givn-refactor-0.3 directory

- Delete docs/givn-embedding-idea.md


### Refactoring

- **watn-consolidation:** Retain exact Bash failure assertion

- **watn-consolidation:** Capture fixture review output

- **watn-consolidation:** Verify archived fixture titles

- **ci:** Remove Givn tag filters from Watn

## [0.2.1] - 2026-08-13

### Bug Fixes

- **release:** Pass repository to GitHub CLI

## [0.2.0] - 2026-08-13

### Bug Fixes

- **incremental-sse-rendering:** Close stream review gaps

- **incremental-sse-rendering:** Align renderer review evidence

- **incremental-sse-rendering:** Remove unused stream render wrappers

- **release:** Preserve annotated tags during validation

- **release:** Harden recovery and publication flow

- **release:** Create GitHub releases

- **ci:** Install fish for shell syntax checks

- **highlight-active-setup-input:** Preserve shell focus baseline after rebase

- **responsive-setup-model-filtering:** Preserve paginated catalog search

- **preserve-ctrl-w-requests-in-shell-config:** Align delta feature with shell shortcut spec

- **preserve-ctrl-w-requests-in-shell-config:** Match modified delta spec path

- **preserve-ctrl-w-requests-in-shell-config:** Use permanent shell shortcut delta tag

- **shell-shortcut:** Undef tty werase before binding Bash Ctrl-W

- **cancel-running-completion:** Harden cancellation and test isolation

- **streamlined-setup:** Invalidate catalog after provider change

- **setup:** Align legacy request contracts with role readiness

- **reasoning:** Expose off for non-mandatory catalog models


### Documentation

- **release-truth-and-repository-cleanup:** Active documentation describes current command streaming

- **release-truth-and-repository-cleanup:** Active documentation distinguishes archived historical snapshots

- **shell-completions:** Completion help documents the supported selector and output contract

- Refresh improvement handoff plan

- Refresh improvement handoff plan

- Refresh improvement handoff plan

- Record archived setup focus change

- **responsive-setup-model-filtering:** Record local filtering scenario

- **responsive-setup-model-filtering:** Record provider filtering scenario

- **responsive-setup-model-filtering:** Record stale-result scenario

- Record archived responsive filtering change

- Update improvement handoff plan

- Capture setup wizard refactoring idea

- Clarify default model tier

- Explain why branch coverage is n/a in README

- Add screencasts to README

- Align README with verified behavior

- Clarify provider and Ctrl-W setup

- Propose streamlined setup flow


### Features

- **incremental-sse-rendering:** A usage-only final event supplies cost and throughput metadata

- **incremental-sse-rendering:** A DONE event completes a stream successfully

- **incremental-sse-rendering:** Partial network reads are reassembled into complete events

- **incremental-sse-rendering:** Malformed nonessential events do not discard valid content

- **incremental-sse-rendering:** EOF without DONE is a truncated stream

- **incremental-sse-rendering:** Output failure preserves the visible prefix and skips completion actions

- **release-truth-and-repository-cleanup:** Release artifact reports target-dependent runtime libraries

- **shell-completions:** Each supported shell exposes the authoritative command tree

- **shell-completions:** Unsupported shell returns actionable guidance

- **shell-completions:** Completion generation does not load configuration or contact a provider

- **shell-completions:** Every native clap_complete shell exposes the authoritative command tree

- **shell-completions:** The reserved completion token can remain question text after --

- **interactive-shell-shortcut:** Enter accepts the default decline for shortcut setup

- **interactive-shell-shortcut:** Selecting no shells leaves shell configuration unchanged

- **interactive-shell-shortcut:** The shell basename alone controls shortcut preselection

- **interactive-shell-shortcut:** Multiple selected shells are installed independently

- **interactive-shell-shortcut:** A partial multi-shell failure reports every result without rollback

- **interactive-shell-shortcut:** Missing parent directories are created only for selected shells

- **interactive-shell-shortcut:** Installing again replaces the generated block without disturbing user content

- **interactive-shell-shortcut:** A shell configuration failure reports the exact target and reason

- **interactive-shell-shortcut:** Invalid marker layouts fail before any target write

- **interactive-shell-shortcut:** Generated shell blocks use the installed watn command and preserve shell syntax

- **interactive-shell-shortcut:** A successful widget inserts one normalized command and moves the cursor to its end

- **interactive-shell-shortcut:** Embedded multiline output remains buffer text without evaluation

- **interactive-shell-shortcut:** Empty input does not invoke watn or change the command line

- **interactive-shell-shortcut:** Failed or empty output preserves the original command line

- **interactive-shell-shortcut:** Non-zero watn status discards partial stdout

- **interactive-shell-shortcut:** The complete command line is passed as one quoted question

- **interactive-shell-shortcut:** Leading-option and reserved-token questions remain one argument

- **interactive-shell-shortcut:** Setup reports the exact reload instruction for every modified shell

- **interactive-shell-shortcut:** The optional setup result includes only explicitly selected shells

- **setup:** Add shell integration tabs

- **highlight-active-setup-input:** The initial URL input has a green border

- **highlight-active-setup-input:** The green border follows API key focus

- **highlight-active-setup-input:** The green border follows model focus

- **highlight-active-setup-input:** The green border follows optional shortcut focus

- **responsive-setup-model-filtering:** A complete catalog is filtered locally

- **preserve-ctrl-w-requests-in-shell-config:** A successful generation keeps the original request visible as a comment

- **preserve-ctrl-w-requests-in-shell-config:** Only the generated command executes when the buffer is committed

- **preserve-ctrl-w-requests-in-shell-config:** Requests with metacharacters and embedded newlines remain one comment line

- **preserve-ctrl-w-requests-in-shell-config:** Failed or empty generation preserves the original buffer

- **preserve-ctrl-w-requests-in-shell-config:** Zsh and Fish widgets preserve the request as a comment

- **spinner:** Animate thinking face during request

- **cancel-running-completion:** One Ctrl+C cancels a completion waiting for streamed output

- **cancel-running-completion:** One Ctrl+C cancels a completion waiting for a connection

- **streamlined-setup:** Coordinated setup displays one separate reasoning question after each model

- **streamlined-setup:** Rerunning coordinated setup prefills current values and preserves a masked literal credential

- **streamlined-setup:** Cancelling coordinated setup leaves an existing configuration unchanged

- **streamlined-setup:** Provider setup requires a custom endpoint

- **streamlined-setup:** Provider setup refuses an unresolved environment credential

- **streamlined-setup:** Provider setup preserves unrelated settings

- **streamlined-setup:** Provider setup does not probe the catalog

- **streamlined-setup:** Models setup gives guidance when no provider is configured

- **streamlined-setup:** Available catalog restricts model choices

- **streamlined-setup:** Unavailable catalog allows manual model identifiers

- **streamlined-setup:** Catalog metadata selects supported reasoning efforts for the chosen model

- **streamlined-setup:** Missing reasoning metadata provides generic efforts and free-form input

- **streamlined-setup:** Off reasoning omits the reasoning setting from a request

- **streamlined-setup:** Shell setup prefills installed integrations and removes only managed blocks when deselected

- **streamlined-setup:** Shell setup refuses malformed managed markers

- **streamlined-setup:** Shell failure does not discard successful shell changes or configuration

- **streamlined-setup:** Non-interactive incomplete request prints setup guidance without probing

- **streamlined-setup:** Malformed configuration is reported without modification

- **streamlined-setup:** Cancelling after provider and credential validation does not create a config file

- **streamlined-setup:** Cancelling after a successful catalog probe leaves the baseline unchanged

- **streamlined-setup:** Catalog failure does not persist an unconfirmed provider

- **streamlined-setup:** A successful edited catalog endpoint is promoted only at final confirmation

- **streamlined-setup:** A failed edited catalog endpoint preserves the previous endpoint

- **streamlined-setup:** A failed new catalog endpoint remains unset

- **streamlined-setup:** Invalid catalog data switches to manual model selection

- **streamlined-setup:** Catalog entries without unique non-empty identifiers are rejected

- **streamlined-setup:** Provider catalog takes precedence over a conflicting legacy LiteLLM source

- **streamlined-setup:** Provider catalog pagination and search use the provider source

- **streamlined-setup:** Manual model identifiers are persisted exactly after catalog failure

- **streamlined-setup:** Changing provider invalidates catalog-backed model choices

- **streamlined-setup:** The final review shows all draft domains without exposing a secret

- **streamlined-setup:** Final confirmation is blocked while a required draft value is invalid

- **setup:** Back navigation preserves draft values across model and reasoning questions

- **provider:** Selected provider migration moves an arbitrary provider to custom

- **reasoning:** Free-form reasoning survives persistence and request construction

- **reasoning:** Existing unknown reasoning remains active after rerunning setup

- **reasoning:** Whitespace-only custom reasoning is rejected

- **shell:** Declining shell setup performs no target inspection or write

- **shell:** Shell removal preserves bytes outside the managed block

- **setup:** Missing model roles trigger implicit setup even with a usable provider

- **models:** Focused model setup preserves provider-owned and unrelated fields

- **setup:** A failed final config write prevents shell operations


### Other Changes

- Set package ecosystem to rust-toolchain

- Merge remote-tracking branch 'origin/main'

# Conflicts:
#	README.md
#	givn/commands.yaml

- Delete QUESTIONS.md

- Merge remote-tracking branch 'origin/main'

- Revert "feat(spinner): animate thinking face during request"

This reverts commit 610446a17fb408a177dd93c64bc9bbb349f46f71.

- **cancel-running-completion:** Apply cargo fmt

- Complete cancel-running-completion

- Define streamlined setup behavior

- Harden streamlined setup design

- Break down streamlined setup scenarios

- Complete streamlined setup flow


### Refactoring

- **release-truth-and-repository-cleanup:** Remove confirmed dead repository code

## [0.1.4] - 2026-08-11

### Bug Fixes

- **release:** Preserve annotated tags during validation

## [0.1.3] - 2026-08-11

### Bug Fixes

- Cancel execute prompt with Esc and Ctrl-C

- **provider-setup:** Harden baseline fixtures and coverage

- **provider-setup:** Remediate review findings

- **provider-setup:** Finalize review traceability

- **provider-setup:** Complete production result seam

- **unified-setup-wizard:** Remove obsolete model dialog path

- **unified-setup-wizard:** Invalidate searches when discarding setup

- **unified-setup-wizard:** Remove obsolete provider dialog path

- **unified-setup-wizard:** Align modified feature with permanent spec

- **transport:** Keep debug transport verification clippy-clean

- **output:** Remove extra response blank line

- **reasoning:** Validate unknown persisted request effort


### Documentation

- Rewrite README usage section, add examples, license badge


### Features

- Add pulsing request spinner

- **provider-setup:** Configure a custom endpoint with a pasted credential

- **provider-setup:** Configure a custom provider with the generic environment variable

- **provider-setup:** A recognized environment credential skips automatic provider setup

- **provider-setup:** A saved provider with a default model skips automatic provider setup

- **provider-setup:** Invalid endpoint remains in provider setup for correction

- **provider-setup:** Empty credential remains in provider setup for correction

- **provider-setup:** A missing saved environment reference fails authentication without a request

- **provider-setup:** A saved OpenRouter endpoint takes precedence over the built-in endpoint

- **provider-setup:** An explicitly named environment variable is persisted and expanded at use time

- **provider-setup:** Trailing slashes are normalized before persistence and requests

- **provider-setup:** Rerunning provider setup preserves unrelated configuration

- **provider-setup:** Escape cancellation preserves the existing provider configuration

- **provider-setup:** Ctrl-C cancellation preserves the existing provider configuration

- **provider-setup:** Model catalog failure after provider setup preserves the provider and sends no request

- **provider-setup:** The explicit provider command ends without model setup

- **provider-setup:** Non-TTY first use prints setup guidance instead of starting ratatui

- **provider-setup:** A literal saved credential is authoritative over environment fallback

- **provider-setup:** Explicit provider selection from the environment preserves missing-key errors

- **provider-setup:** Saving provider configuration secures a world-readable file

- **provider-setup-widget-layout:** Provider setup separates choices, details, and guidance

- **provider-setup-widget-layout:** Model picker makes tiers and long model lists easy to scan

- **unified-setup-wizard:** Provider setup separates choices, details, and guidance

- **unified-setup-wizard:** Setup wizard guides provider and model configuration page by page

- **unified-setup-wizard:** Models command opens the shared wizard on Small Model

- **unified-setup-wizard:** Escape asks whether to save or discard current setup

- **credentials:** A missing saved environment credential fails before discovery

- **credentials:** Provider-specific environment fallback precedes generic fallback

- **catalog:** LiteLLM discovery without a key sends no authorization header

- **reasoning:** A disabled model default selects off even when a default effort is present

- **catalog:** Provider discovery is used when LiteLLM is absent

- **reasoning:** Mandatory reasoning excludes off

- **catalog:** Catalog pagination and search use the configured catalog source

- **reasoning:** Unknown persisted reasoning sends no reasoning request

- **reasoning:** Non-TTY model assignment never persists empty reasoning values

- **reasoning:** Existing reasoning survives selection without a valid replacement

- **search:** The newest search result stays visible when an older result arrives later


### Other Changes

- Merge pull request #1 from buster/bright-fireant

Bright fireant


### Refactoring

- Remove obsolete code paths


## [0.1.2] - 2026-08-09

- Published to crates.io from the verified source commit `d5ddb36`.
- This already-published version is excluded from automated publishing.

## [0.1.1] - 2026-08-08

- Historical crates.io publication before the tagged release workflow existed.
- No Git tag was recorded for this publication.

## [0.1.0] - 2026-08-08

- Initial crates.io publication before the tagged release workflow existed.
- No Git tag was recorded for this publication.

[0.2.0]: https://github.com/buster/watn/compare/v0.1.4...v0.2.0

[0.2.1]: https://github.com/buster/watn/compare/v0.2.0...v0.2.1

[0.3.0]: https://github.com/buster/watn/compare/v0.2.1...v0.3.0
