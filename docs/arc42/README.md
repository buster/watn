# Architecture Documentation (arc42)

This directory contains the project's arc42 architecture documentation.
Maintained by the `arc42-docs` givn artifact — updated with each change via `givn`.

The current architecture includes four TTY-gated setup entry points:
`watn setup`, `watn provider`, `watn models`, and `watn shell`. They use a
shared in-memory draft and structured Ratatui questions. Coordinated setup
shows provider, completion endpoint, credential, provider-local catalog,
separate model/reasoning questions, shell desired states, and a final review.
It writes configuration only after final confirmation; focused provider and
model flows save only their owned domain, while shell setup changes only target
files.
Model discovery is provider-derived. A saved or edited provider-local catalog
endpoint is probed with the provider credential; the legacy `[litellm]` section
is retained as unrelated configuration but is not contacted by setup or model
discovery. Credential sources remain literal values or exact environment
references. Reasoning accepts any non-empty value verbatim, with `off` as the
only omission sentinel, and stale search generations cannot overwrite results
from newer user-entered searches.
The outbound transport boundary keeps configured endpoints authoritative for
normal and release-profile binaries; only a debug `test-support` binary may
route requests to a loopback test twin, and that route is never persisted or
used for readiness. Debug transport verification uses two copied binaries from
Cargo's shared target cache. Release verification inspects the built artifact
for target-dependent dynamic runtime libraries; no universal static-deployment
claim is made.
Chat completion responses are consumed as OpenAI-compatible SSE through a
synchronous content callback with no worker channel. Command content is flushed
incrementally, reasoning is buffered and printed only after successful
completion under `-v`, and `[DONE]` is mandatory; truncated or failed streams
preserve visible prefixes but do not print success metadata or execute.
A run in progress is cancellable: Ctrl+C stops the stream on the next SSE event
via a worker-thread watchdog bounded by a 500 ms grace, exits 130 without an
error message, and preserves already-streamed output.
The CLI also generates completions with `watn completions <SHELL>` for the closed
set `bash`, `elvish`, `fish`, `powershell`, and `zsh`. Scripts come from the authoritative Clap command
definition, are deterministic, are written only to stdout, and bypass config
initialisation and provider requests. Unsupported shell values use the literal
`unsupported shell '<value>'; choose bash, elvish, fish, powershell, or zsh` parser contract.

The shared setup flow also offers an optional Ctrl-W shortcut for Bash, Zsh, and
Fish, including during implicit first-use onboarding. Installation owns one
marked startup-file block per selected shell, uses atomic replacement, reports
independent target results, and invokes `command watn -- "$question"` without
evaluating generated output. The widget records the request as a `#`-prefixed
history comment (recallable and re-askable via the shell history) and leaves
only the generated command in the buffer; pressing Enter executes that command
as its own history entry.

The permanent Gherkin tree is also treated as one behavior inventory. Scenario
ownership, overlap dispositions, and consolidation receipts are recorded by
the givn workflow; see [ADR-0025](../adr/0025-repository-wide-specification-ownership.md).
This changes repository evidence only, not the Watn runtime or release artifact.

## Archive Status

The files in this directory describe the current architecture. Archived givn
change artifacts, including historical Arc42 assessments, are retained under
[`givn/archive/`](../../givn/archive/) as historical records. They are not
current architecture snapshots and are not rewritten when active documentation
changes.

| Chapter | File | Description |
|---|---|---|
| 1 | [Introduction and Goals](01-introduction-and-goals.md) | Requirements, quality goals, stakeholders |
| 2 | [Architecture Constraints](02-architecture-constraints.md) | Technical, organisational, legal constraints |
| 3 | [Context and Scope](03-context-and-scope.md) | System boundaries, external interfaces |
| 4 | [Solution Strategy](04-solution-strategy.md) | Fundamental decisions and approaches |
| 5 | [Building Block View](05-building-block-view.md) | Static decomposition into modules/components |
| 6 | [Runtime View](06-runtime-view.md) | Important runtime scenarios and flows |
| 7 | [Deployment View](07-deployment-view.md) | Infrastructure and deployment topology |
| 8 | [Cross-cutting Concepts](08-crosscutting-concepts.md) | Error handling, security, config, etc. |
| 9 | [Architecture Decisions](09-architecture-decisions.md) | ADRs — context, decision, rationale |
| 10 | [Quality Requirements](10-quality-requirements.md) | Measurable quality scenarios |
| 11 | [Risks and Technical Debt](11-risks-and-technical-debt.md) | Known risks and debt |
| 12 | [Glossary](12-glossary.md) | Domain and technical terms |

Completion generation is further recorded in [ADR-0017](../adr/0017-completion-generation-from-authoritative-command-definition.md).
Shell shortcut installation and widget safety are recorded in
[ADR-0018](../adr/0018-safe-shell-shortcut-installation-and-native-widgets.md).
Interruptible streaming cancellation is recorded in
[ADR-0019](../adr/0019-interruptible-completion-via-worker-thread.md).
