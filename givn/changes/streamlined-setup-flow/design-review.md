# Design Review: streamlined-setup-flow

## Review Scope

The review covered the proposal, active Gherkin delta, design, permanent setup
and catalog specifications, setup implementation, Cucumber runner, ADRs, and all
twelve arc42 chapter files. The change has five interaction-inventory entries
and exactly one `@e2e` scenario for each entry. The real interface is the CLI
terminal, driven through the existing portable-pty harness.

## Ranked Findings And Resolutions

### P0: Coordinated persistence boundary

The existing provider-first save behavior conflicted with the approved
proposal. The user chose final-confirmation-only persistence for coordinated
`watn setup`. The design now treats credential acceptance, catalog probing,
model selection, reasoning selection, navigation, and review opening as
non-persistent. A missing destination remains absent until final confirmation;
an existing destination remains byte-for-byte unchanged after cancellation or
draft failure. Focused commands retain their own final-confirmation boundaries.

The active feature now covers cancellation after provider validation, after a
successful catalog probe, after catalog failure, and with no existing config.

### P0: Catalog source

The user chose provider-derived catalog discovery only. The design now derives
or reuses a provider-local catalog endpoint, uses the provider credential, and
never contacts the legacy `[litellm]` source for setup or model discovery. The
legacy section remains readable and is carried through unrelated config writes;
it is not migrated or silently used as a fallback.

The active feature now covers conflicting provider and LiteLLM sources, exact
provider authorization, provider-local pagination/search, and zero requests to
the legacy source.

### P0: Reasoning values

The user chose arbitrary non-empty reasoning values. The design now treats
`off` as the only omission sentinel, rejects whitespace-only custom input, and
persists/transmits every other value verbatim, including existing unknown
values and surrounding whitespace. Catalog metadata supplies suggestions and
mandatory/off validation but does not create a closed persistence enum.

The active feature now covers request-body round trips, existing unknown values,
whitespace rejection, and custom values alongside catalog-supported choices.

### P0: Provider-name migration

The user chose migration of a selected arbitrary provider name to the fixed
`custom` entry. The design defines source-key removal, deterministic collision
handling, default-model preservation, credential-source authority, unrelated
provider preservation, and idempotent reruns. The old selected key is removed
only in the successful final write.

The active feature now covers migration without collision, destination default
model preservation, and idempotence.

### P0: Atomic configuration replacement

Coordinated and focused configuration saves use one candidate snapshot and a
same-directory temporary-file replacement. A failed final write leaves the
previous file untouched and prevents shell operations from starting. Shell
files remain independent and are not part of the configuration transaction.

The active feature now covers a failed final write and the no-shell-operation
boundary.

### P1: Catalog failure and manual fallback

The design defines missing, available, existing-failed, edited-failed-with-
prior-available, and edited-failed-without-prior-available states. Invalid
response shapes, empty data, missing identifiers, and duplicate identifiers
switch to manual mode without inventing models. A failed replacement never
promotes its endpoint.

The active feature now covers successful edits, failed edits with and without a
saved endpoint, invalid catalog data, manual identifier persistence, and
provider-change revalidation.

### P1: Review and navigation

The design defines a read-only review containing provider, endpoint, catalog
status, credential source/masking, every model/reasoning pair, migration notice,
and shell desired state. Confirmation is blocked for missing or stale values.
Back navigation preserves the draft and marks downstream catalog-backed values
stale when the provider changes.

The active feature now covers review contents, secret absence, blocked
confirmation, and back-navigation preservation.

### P1: Shell desired state

Shell completion and Ctrl-W are separate desired states. Filesystem inspection
prefills existing valid managed blocks. Deselecting removes only one valid
managed block; malformed marker layouts fail before writes. Declining both
questions performs no target inspection or file creation. Independent target
failures retain successful earlier changes.

The active feature now covers no-I/O decline, byte preservation, managed-block
removal, malformed markers, and independent failure behavior.

### P1: First-use readiness

Provider readiness remains local and side-effect free. A usable provider does
not bypass setup when a required model role is missing. Interactive incomplete
requests enter the coordinator and do not replay the original request;
non-interactive requests print guidance without catalog or chat traffic.

The active feature covers missing-role onboarding and the non-interactive
negative path.

## Scope And Technology

The spec matches the proposal's four focused commands, coordinated flow,
provider/catalog credential handling, model/reasoning separation, shell safety,
first-use behavior, and malformed-config behavior. The change keeps Rust,
Ratatui, Crossterm, blocking Reqwest, Cucumber, httpmock, and portable-pty.
No new runtime or test dependency is needed. The existing model filtering,
pagination, stale-search generation, terminal restoration, and shell marker
validation remain reusable boundaries.

The broad setup replacement is justified because the current module combines
page state, persistence transitions, catalog loading, reasoning policy, and
shell installation in one state machine. The new draft/result boundary is
smaller than preserving the old shared save transitions and is testable through
the same real PTY boundary.

## Testability And E2E Fidelity

Every active scenario uses concrete observable assertions: terminal labels and
review text, exit status, exact configuration fields and bytes, shell target
bytes, exact HTTP method/path/query/Authorization, and request counts. The
runner uses `Cucumber::fail_on_skipped()`. WIP scenarios remain excluded until
their step definitions and production behavior are implemented; no WIP or E2E
tag is removed to bypass a gate.

The five E2E scenarios correspond one-to-one with the five inventory entries and
drive real subprocesses in a PTY. Loopback httpmock servers are deterministic
digital twins; no live provider is contacted.

## Arc42 Audit

The independent twelve-row impact assessment is recorded in the change-local
`arc42.md`. Chapters 01, 02, 03, 04, 05, 06, 08, 09, 10, 11, and 12 are
affected. Chapter 07 is explicitly unaffected because production deployment
topology does not change. Durable chapter updates and superseding ADR entries
are required before archive and are included in the implementation work.

## Remaining Implementation Risks

- The permanent executable specs still contain historical scenarios for the old
  LiteLLM source, provider-first persistence, closed reasoning, and arbitrary
  provider preservation. They must be modified or superseded before the full
  suite can be green.
- The current configuration model lacks a provider-local catalog endpoint and
  the current setup module lacks shell desired-state removal; both require
  focused production changes.
- Step registration is global in cucumber-rs, so new setup step expressions must
  avoid collisions with existing generic steps.

These are implementation tasks, not unresolved design questions.

DESIGN-REVIEW: PASS
