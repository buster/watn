# Architecture Documentation (arc42)

This directory contains the project's arc42 architecture documentation.
Maintained by the `arc42-docs` givn artifact — updated with each change via `givn`.

The current architecture includes TTY-gated provider onboarding through
`watn provider`, environment-backed credentials, and automatic first-use
provider/model setup that stops before the original request. The setup flows use
structured Ratatui widgets so choices, metadata, status, tier tabs, and long
catalog position remain visible in the terminal.
Provider and model onboarding now share a five-page setup wizard with explicit
page, cursor, and save/discard state.
Model discovery resolves a dedicated catalog source: configured LiteLLM is used
for model listing, pagination, and search, while chat remains on the selected
provider. Credential sources remain literal values or exact environment
references through discovery and partial setup saves; model reasoning defaults
are validated centrally and stale search generations cannot overwrite results
from a newer user-entered search.
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
