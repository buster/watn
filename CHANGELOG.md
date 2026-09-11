# Changelog

All notable changes to watn are documented in this file.

The release sections are generated with [git-cliff](https://git-cliff.org/).
Versions are selected manually and use annotated `vX.Y.Z` Git tags.
## [Unreleased]

## [0.4.1] - 2026-09-11

### Bug Fixes

- **ci:** Install givn for specification fixtures

- **ci:** Pin givn fixture installer to git revision

- **ci:** Remove Watn Givn fixture dependency

- **security:** Update h2 0.4.15 -> 0.4.16 (RUSTSEC-2026-0258)

- **tests:** Rename body_contains to body_includes for httpmock 0.8

- **tests:** Rename hits to calls for httpmock 0.8

- **release:** Exclude AGENTS.md from package

- **quicksetup:** Use valid OpenRouter default model

- Resolve clippy lints blocking the release

- **interactive-shell-shortcut:** Keep the raw candidate visible in the stage stack


### Documentation

- **upgrade:** Record aggregate migration plan migrate-0-2-0-to-0-3-0

- Update README to streamline demo and shortcuts

- **readme:** Document the quick setup flow

- **specs:** Strict use-case relationship bullets in group.md files

- **specs:** Strict relationship bullets and reconciled interaction inventories

- **upgrade:** Record aggregate migration plan migrate-0-3-0-to-0-5-0

- **migrate-0-3-0-to-0-5-0:** Arc42 assessment and design-review PASS

- **migrate-0-3-0-to-0-5-0:** Tasks evidence and review PASS

- **migrate-0-3-0-to-0-5-0:** Empty disposition tables removed for clean gate

- **migrate-0-3-0-to-0-5-0:** Complete task evidence

- Record Watn migration evidence

- **upgrade:** Record aggregate migration plan migrate-0-5-0-to-0-6-0

- **usecase:** Preserve shell-completions interactions after archive merge

- **readme:** Document the review card and its toggles

- **readme:** Document the card-only review controls

- **arc42:** Record the single review card and its controls

- **arc42:** Single review card, color property, and in-panel controls

- **readme:** Document persisted review panel toggles

- **arc42:** Record persisted review preference

- **arc42:** Record flow-first decisions, model chooser, and regeneration risk

- **arc42:** Emphasize shortcut letters in the review hints


### Features

- **quicksetup:** Quick setup without a terminal prints guidance instead of asking

- **quicksetup:** A model question without a suggestion requires a non-empty answer

- **quicksetup:** Quick setup does not ask reasoning questions and stores no reasoning

- **quicksetup:** An OpenAI endpoint suggests the OpenAI credential and no model

- **quicksetup:** Shell integrations are pre-selected only for shells available on the path

- **quicksetup:** Explicit provider selection skips the first-run quick setup

- **quicksetup:** Aborting explicit quick setup leaves the previous configuration unchanged

- **quicksetup:** A failed configuration write installs no shell integration

- **quicksetup:** A failed shell installation keeps the saved configuration

- **quicksetup:** An invalid endpoint value re-asks for the endpoint

- **quicksetup:** An unknown shell name shows an error and keeps the list open

- **quicksetup:** A re-asked small model stores the later answer on completion

- Migrate Watn specs to use cases

- **interactive-shell-shortcut:** The review surface explains a complex command flow

- **interactive-shell-shortcut:** A complete candidate is buffered before review

- **interactive-shell-shortcut:** Direct command editing preserves the original intent

- **interactive-shell-shortcut:** Escape discards a direct command edit

- **interactive-shell-shortcut:** Edited candidate purpose refresh failure remains reviewable

- **interactive-shell-shortcut:** The compact review surface cycles three focus regions

- **interactive-shell-shortcut:** Review actions use Enter and Escape

- **interactive-shell-shortcut:** Stage purposes can load after a structured candidate appears

- **interactive-shell-shortcut:** A command-only response shows purpose-unavailable

- **interactive-shell-shortcut:** Unsupported command flow remains reviewable

- **interactive-shell-shortcut:** Enhanced renderer failure falls back to the inline review surface

- **interactive-shell-shortcut:** Portable review-surface failure releases no candidate

- **interactive-shell-shortcut:** Provider failure preserves a selected candidate during review

- **interactive-shell-shortcut:** Disabled review preserves direct Ctrl-W replacement

- **interactive-shell-shortcut:** Disabled review preserves direct positional output

- **interactive-shell-shortcut:** Disabled review preserves -x confirmation

- **interactive-shell-shortcut:** Non-review -x preserves confirmation

- **interactive-shell-shortcut:** Rephrasing starts a new candidate cycle

- **interactive-shell-shortcut:** Regeneration replaces the current candidate by default

- **interactive-shell-shortcut:** Higher-tier review generates a candidate at the next configured tier

- **interactive-shell-shortcut:** Highest-tier review opens explicit provider catalog model selection

- **interactive-shell-shortcut:** Rejected candidate returns to the current intent

- **interactive-shell-shortcut:** Retained candidates can be compared and one selected

- **interactive-shell-shortcut:** Interrupting an in-progress review operation preserves the selected candidate

- **interactive-shell-shortcut:** Shell repaint remains owned by the line editor after review

- **interactive-shell-shortcut:** A narrow terminal keeps the inline review bounded and readable

- **interactive-shell-shortcut:** A markdown-fenced structured response is still explained

- **interactive-shell-shortcut:** An invalid structured response with a command stays reviewable

- **interactive-shell-shortcut:** A provider payload without a usable command releases nothing

- **interactive-shell-shortcut:** A multiline provider payload keeps every rendered value on one row

- **interactive-shell-shortcut:** An unknown purpose status keeps matching model-written purposes

- **interactive-shell-shortcut:** A command broken across lines is explained on one row per stage

- **interactive-shell-shortcut:** Mismatched stage text still shows purpose-unavailable

- **interactive-shell-shortcut:** A provider stage split that covers the command is shown with its purposes

- **interactive-shell-shortcut:** Provider stages that do not cover the command are not trusted

- **interactive-shell-shortcut:** The enhanced review card frames the command, stage, and actions

- **interactive-shell-shortcut:** The card shows one stage at a time and moves stages with the arrow keys

- **interactive-shell-shortcut:** The card marks unsupported stage syntax

- **interactive-shell-shortcut:** Disabling the enhanced card preserves the plain review surface

- **interactive-shell-shortcut:** A color-incapable terminal falls back to the plain review surface

- **interactive-shell-shortcut:** The card's edit shortcut opens the command editor

- **interactive-shell-shortcut:** The review card is the only review panel

- **interactive-shell-shortcut:** A failing review card releases no candidate

- **interactive-shell-shortcut:** A color-incapable terminal shows the review card without color

- **interactive-shell-shortcut:** Disabling the review panel preserves the original command handling

- **interactive-shell-shortcut:** The panel can permanently disable the review

- **interactive-shell-shortcut:** The -x confirmation offers to explain the command

- **interactive-shell-shortcut:** Enabling the review panel from the command line persists it

- **interactive-shell-shortcut:** Disabling the review panel from the command line persists it

- **interactive-shell-shortcut:** The review card opens on the first command-flow stage

- **interactive-shell-shortcut:** The review card exposes the direct decision shortcuts

- **interactive-shell-shortcut:** Enter accepts the current candidate

- **interactive-shell-shortcut:** The accept shortcut accepts the current candidate

- **interactive-shell-shortcut:** The cancel shortcut cancels the review

- **interactive-shell-shortcut:** Rejecting a candidate opens the model chooser

- **interactive-shell-shortcut:** A configured tier can be chosen with its number

- **interactive-shell-shortcut:** A model name is suggested from the provider catalog while typing

- **interactive-shell-shortcut:** A typed model name works when the catalog is unavailable

- **interactive-shell-shortcut:** The model chooser ignores incomplete choices

- **interactive-shell-shortcut:** A failed regeneration preserves the previous candidate

- **interactive-shell-shortcut:** Leaving the model chooser preserves the candidate

- **interactive-shell-shortcut:** The command editor moves the insertion point with arrows and Home and End

- **interactive-shell-shortcut:** Backspace and Delete remove text at the insertion point

- **interactive-shell-shortcut:** The explanation card ignores review decisions

- **interactive-shell-shortcut:** The review card emphasizes the decision shortcut keys

- **interactive-shell-shortcut:** Emphasize shortcut letters inside the hint words

- **interactive-shell-shortcut:** The model chooser explains how to choose

- **interactive-shell-shortcut:** The first suggestion is ready to choose

- **interactive-shell-shortcut:** The card marks nested shell syntax

- **interactive-shell-shortcut:** The card marks a stage it cannot decompose

- **interactive-shell-shortcut:** The simple review view names only the model without provider or tier

- **interactive-shell-shortcut:** The simple view stacks the command flow with its separators

- **interactive-shell-shortcut:** A long stage continues on the next stack rows and keeps its separator

- **interactive-shell-shortcut:** The selected stage is marked and its purpose follows the arrow keys

- **interactive-shell-shortcut:** The purpose below the stack is marked and readable

- **interactive-shell-shortcut:** The view toggle switches between the simple and detailed reviews

- **interactive-shell-shortcut:** A stage longer than the stack window is truncated with a marker

- **interactive-shell-shortcut:** A narrow terminal keeps the inline review bounded and readable

- **interactive-shell-shortcut:** The panel can permanently disable the review

- **interactive-shell-shortcut:** The review surface switches configure without a request

- **interactive-shell-shortcut:** Unsupported command flow remains reviewable without a marker

- **interactive-shell-shortcut:** The review card stops advertising cancel

- **interactive-shell-shortcut:** The detailed view drops the flow and stage rows

- **interactive-shell-shortcut:** The panel reports its disable clearly

- **quicksetup:** Quick setup documents its parameters in help

- **quicksetup:** Quick setup parameters prefill the dialog

- **quicksetup:** A small-model parameter seeds the remaining tiers

- **quicksetup:** Tier parameters override the shared model prefill

- **interactive-shell-shortcut:** The review card exposes the enter accept and disable-review wording

- **interactive-shell-shortcut:** The detailed view shares the simple command block

- **models:** Model picker shows metadata when available

- **models:** Non-terminal model assignment records catalog prices

- **ratatui-model-picker:** Model entry shows additional metadata when available

- **ratatui-model-picker:** Interactive model table shows published prices per million tokens

- **streamlined-setup:** Detected shells are preselected on the shell pages

- **streamlined-setup:** Shell setup prefills installed integrations and removes only managed blocks when deselected

- **unified-setup-wizard:** Setup surfaces use the review visual language


### Other Changes

- Delete docs/givn-refactor-0.3 directory

- Delete docs/givn-embedding-idea.md

- Cargo fmt and fix clippy match-single-binding


### Refactoring

- **watn-consolidation:** Retain exact Bash failure assertion

- **watn-consolidation:** Capture fixture review output

- **watn-consolidation:** Verify archived fixture titles

- **ci:** Remove Givn tag filters from Watn

- **quicksetup:** Move shells_available_on_path to shell_shortcut per design

- **quicksetup:** Drop redundant validators and flatten endpoint answer fallback

- **specs:** Regroup the spec corpus into use-case groups

- **review:** Drop the superseded derived-stage purpose match

## [0.4.0] - 2026-09-11

### Bug Fixes

- **interactive-shell-shortcut:** Keep the raw candidate visible in the stage stack


### Features

- **interactive-shell-shortcut:** The simple review view names only the model without provider or tier

- **interactive-shell-shortcut:** The simple view stacks the command flow with its separators

- **interactive-shell-shortcut:** A long stage continues on the next stack rows and keeps its separator

- **interactive-shell-shortcut:** The selected stage is marked and its purpose follows the arrow keys

- **interactive-shell-shortcut:** The purpose below the stack is marked and readable

- **interactive-shell-shortcut:** The view toggle switches between the simple and detailed reviews

- **interactive-shell-shortcut:** A stage longer than the stack window is truncated with a marker

- **interactive-shell-shortcut:** A narrow terminal keeps the inline review bounded and readable

- **interactive-shell-shortcut:** The panel can permanently disable the review

- **interactive-shell-shortcut:** The review surface switches configure without a request

- **interactive-shell-shortcut:** Unsupported command flow remains reviewable without a marker

- **interactive-shell-shortcut:** The review card stops advertising cancel

- **interactive-shell-shortcut:** The detailed view drops the flow and stage rows

- **interactive-shell-shortcut:** The panel reports its disable clearly

- **quicksetup:** Quick setup documents its parameters in help

- **quicksetup:** Quick setup parameters prefill the dialog

- **quicksetup:** A small-model parameter seeds the remaining tiers

- **quicksetup:** Tier parameters override the shared model prefill

- **interactive-shell-shortcut:** The review card exposes the enter accept and disable-review wording

- **interactive-shell-shortcut:** The detailed view shares the simple command block

## [0.3.3] - 2026-09-10

### Bug Fixes

- Resolve clippy lints blocking the release


### Documentation

- **usecase:** Preserve shell-completions interactions after archive merge

- **readme:** Document the review card and its toggles

- **readme:** Document the card-only review controls

- **arc42:** Record the single review card and its controls

- **arc42:** Single review card, color property, and in-panel controls

- **readme:** Document persisted review panel toggles

- **arc42:** Record persisted review preference

- **arc42:** Record flow-first decisions, model chooser, and regeneration risk

- **arc42:** Emphasize shortcut letters in the review hints


### Features

- **interactive-shell-shortcut:** The review surface explains a complex command flow

- **interactive-shell-shortcut:** A complete candidate is buffered before review

- **interactive-shell-shortcut:** Direct command editing preserves the original intent

- **interactive-shell-shortcut:** Escape discards a direct command edit

- **interactive-shell-shortcut:** Edited candidate purpose refresh failure remains reviewable

- **interactive-shell-shortcut:** The compact review surface cycles three focus regions

- **interactive-shell-shortcut:** Review actions use Enter and Escape

- **interactive-shell-shortcut:** Stage purposes can load after a structured candidate appears

- **interactive-shell-shortcut:** A command-only response shows purpose-unavailable

- **interactive-shell-shortcut:** Unsupported command flow remains reviewable

- **interactive-shell-shortcut:** Enhanced renderer failure falls back to the inline review surface

- **interactive-shell-shortcut:** Portable review-surface failure releases no candidate

- **interactive-shell-shortcut:** Provider failure preserves a selected candidate during review

- **interactive-shell-shortcut:** Disabled review preserves direct Ctrl-W replacement

- **interactive-shell-shortcut:** Disabled review preserves direct positional output

- **interactive-shell-shortcut:** Disabled review preserves -x confirmation

- **interactive-shell-shortcut:** Non-review -x preserves confirmation

- **interactive-shell-shortcut:** Rephrasing starts a new candidate cycle

- **interactive-shell-shortcut:** Regeneration replaces the current candidate by default

- **interactive-shell-shortcut:** Higher-tier review generates a candidate at the next configured tier

- **interactive-shell-shortcut:** Highest-tier review opens explicit provider catalog model selection

- **interactive-shell-shortcut:** Rejected candidate returns to the current intent

- **interactive-shell-shortcut:** Retained candidates can be compared and one selected

- **interactive-shell-shortcut:** Interrupting an in-progress review operation preserves the selected candidate

- **interactive-shell-shortcut:** Shell repaint remains owned by the line editor after review

- **interactive-shell-shortcut:** A narrow terminal keeps the inline review bounded and readable

- **interactive-shell-shortcut:** A markdown-fenced structured response is still explained

- **interactive-shell-shortcut:** An invalid structured response with a command stays reviewable

- **interactive-shell-shortcut:** A provider payload without a usable command releases nothing

- **interactive-shell-shortcut:** A multiline provider payload keeps every rendered value on one row

- **interactive-shell-shortcut:** An unknown purpose status keeps matching model-written purposes

- **interactive-shell-shortcut:** A command broken across lines is explained on one row per stage

- **interactive-shell-shortcut:** Mismatched stage text still shows purpose-unavailable

- **interactive-shell-shortcut:** A provider stage split that covers the command is shown with its purposes

- **interactive-shell-shortcut:** Provider stages that do not cover the command are not trusted

- **interactive-shell-shortcut:** The enhanced review card frames the command, stage, and actions

- **interactive-shell-shortcut:** The card shows one stage at a time and moves stages with the arrow keys

- **interactive-shell-shortcut:** The card marks unsupported stage syntax

- **interactive-shell-shortcut:** Disabling the enhanced card preserves the plain review surface

- **interactive-shell-shortcut:** A color-incapable terminal falls back to the plain review surface

- **interactive-shell-shortcut:** The card's edit shortcut opens the command editor

- **interactive-shell-shortcut:** The review card is the only review panel

- **interactive-shell-shortcut:** A failing review card releases no candidate

- **interactive-shell-shortcut:** A color-incapable terminal shows the review card without color

- **interactive-shell-shortcut:** Disabling the review panel preserves the original command handling

- **interactive-shell-shortcut:** The panel can permanently disable the review

- **interactive-shell-shortcut:** The -x confirmation offers to explain the command

- **interactive-shell-shortcut:** Enabling the review panel from the command line persists it

- **interactive-shell-shortcut:** Disabling the review panel from the command line persists it

- **interactive-shell-shortcut:** The review card opens on the first command-flow stage

- **interactive-shell-shortcut:** The review card exposes the direct decision shortcuts

- **interactive-shell-shortcut:** Enter accepts the current candidate

- **interactive-shell-shortcut:** The accept shortcut accepts the current candidate

- **interactive-shell-shortcut:** The cancel shortcut cancels the review

- **interactive-shell-shortcut:** Rejecting a candidate opens the model chooser

- **interactive-shell-shortcut:** A configured tier can be chosen with its number

- **interactive-shell-shortcut:** A model name is suggested from the provider catalog while typing

- **interactive-shell-shortcut:** A typed model name works when the catalog is unavailable

- **interactive-shell-shortcut:** The model chooser ignores incomplete choices

- **interactive-shell-shortcut:** A failed regeneration preserves the previous candidate

- **interactive-shell-shortcut:** Leaving the model chooser preserves the candidate

- **interactive-shell-shortcut:** The command editor moves the insertion point with arrows and Home and End

- **interactive-shell-shortcut:** Backspace and Delete remove text at the insertion point

- **interactive-shell-shortcut:** The explanation card ignores review decisions

- **interactive-shell-shortcut:** The review card emphasizes the decision shortcut keys

- **interactive-shell-shortcut:** Emphasize shortcut letters inside the hint words

- **interactive-shell-shortcut:** The model chooser explains how to choose

- **interactive-shell-shortcut:** The first suggestion is ready to choose

- **interactive-shell-shortcut:** The card marks nested shell syntax

- **interactive-shell-shortcut:** The card marks a stage it cannot decompose


### Refactoring

- **review:** Drop the superseded derived-stage purpose match

## [0.3.1] - 2026-09-01

### Bug Fixes

- **quicksetup:** Use valid OpenRouter default model


### Features

- **quicksetup:** Quick setup without a terminal prints guidance instead of asking

- **quicksetup:** A model question without a suggestion requires a non-empty answer

- **quicksetup:** Quick setup does not ask reasoning questions and stores no reasoning

- **quicksetup:** An OpenAI endpoint suggests the OpenAI credential and no model

- **quicksetup:** Shell integrations are pre-selected only for shells available on the path

- **quicksetup:** Explicit provider selection skips the first-run quick setup

- **quicksetup:** Aborting explicit quick setup leaves the previous configuration unchanged

- **quicksetup:** A failed configuration write installs no shell integration

- **quicksetup:** A failed shell installation keeps the saved configuration

- **quicksetup:** An invalid endpoint value re-asks for the endpoint

- **quicksetup:** An unknown shell name shows an error and keeps the list open

- **quicksetup:** A re-asked small model stores the later answer on completion


### Refactoring

- **quicksetup:** Move shells_available_on_path to shell_shortcut per design

- **quicksetup:** Drop redundant validators and flatten endpoint answer fallback

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

[0.3.1]: https://github.com/buster/watn/compare/v0.3.0...v0.3.1

[0.3.3]: https://github.com/buster/watn/compare/v0.3.2...v0.3.3

[0.4.0]: https://github.com/buster/watn/compare/v0.3.3...v0.4.0

[0.4.1]: https://github.com/buster/watn/compare/v0.4.0...v0.4.1
