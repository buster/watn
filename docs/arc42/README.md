# Architecture Documentation (arc42)

This directory contains the project's arc42 architecture documentation.
Maintained by the `arc42-docs` givn artifact — updated with each change via `givn`.

The current architecture includes TTY-gated provider onboarding through
`watn provider`, environment-backed credentials, and automatic first-use
provider/model setup that stops before the original request.

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
