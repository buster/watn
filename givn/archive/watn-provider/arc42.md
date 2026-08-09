# arc42 impact: watn-provider

| # | Chapter | Affected | Reason and change summary |
|---|---|---|---|
| 1 | Introduction and Goals | Yes | Adds TTY-gated first-use provider onboarding, credential-source choice, and credential-safety goals without resuming the original request. |
| 2 | Architecture Constraints | Yes | Adds ratatui/crossterm terminal interaction, typed setup results, environment-reference handling, local readiness, and direct-write permission constraints. |
| 3 | Context and Scope | Yes | Adds the `watn provider` terminal interface, non-TTY setup guidance, provider endpoint/credential interactions, and the ephemeral E2E transport boundary. |
| 4 | Solution Strategy | Yes | Adds the fixed-name ratatui setup strategy, credential precedence, TTY gate, in-process provider/model setup, and stop-after-selection behavior. |
| 5 | Building Block View | Yes | Adds provider setup results, provider-readiness and credential-resolution building blocks, config permission enforcement, and the HTTP construction seam. |
| 6 | Runtime View | Yes | Adds explicit provider setup, implicit TTY first-use branches, non-TTY guidance, cancellation/failure branches, and no-resume automatic completion. |
| 7 | Deployment View | No | The single-binary deployment and lack of runtime infrastructure do not change. |
| 8 | Crosscutting Concepts | Yes | Adds exact credential precedence/expansion, TTY detection, typed cancellation, direct-write mode enforcement, fixed-name replacement, and ephemeral transport rules. |
| 9 | Architecture Decisions | Yes | Updates ADR-0011 with the dialoguer alternative, fixed-name collision/preservation consequences, TTY boundary, typed results, and direct-write/E2E trade-offs. |
| 10 | Quality Requirements | Yes | Adds measurable onboarding, non-TTY, credential-precedence, cancellation, persistence, and no-resume scenarios. |
| 11 | Risks and Technical Debt | Yes | Records all ADR-0011 bad consequences, including TTY/catalog dependence, explicit-selection errors, no-resume behavior, partial onboarding, fixed-name collisions, direct writes, literal secrets, and the E2E seam. |
| 12 | Glossary | Yes | Corrects stdin TTY terminology and adds fixed provider names, typed setup outcomes, transport override, and no-resume onboarding terms. |

## Status

STATUS: DONE
