# Arc42 Impact Assessment: streamlined-setup-flow

This change replaces the setup state machine, catalog-source policy, coordinated
configuration write boundary, reasoning value policy, and arbitrary-provider
migration behavior. The durable chapters must be updated before archive.

| # | Chapter | Impact | Summary |
|---:|---|---|---|
| 1 | Introduction and Goals | Yes | Setup rerun safety, focused commands, credential handling, catalog correctness, and reasoning flexibility change. |
| 2 | Architecture Constraints | Yes | Final-confirmation persistence, provider-derived catalog sourcing, open reasoning values, provider migration, and atomic config replacement become constraints. |
| 3 | Context and Scope | Yes | The four setup commands and provider-local catalog interaction change the user and external-system boundaries. |
| 4 | Solution Strategy | Yes | The draft state machine, catalog resolution, migration, and request reasoning strategy change. |
| 5 | Building Block View | Yes | Setup drafts, config snapshot writing, provider migration, catalog resolution, and shell desired state change responsibilities. |
| 6 | Runtime View | Yes | Coordinated setup, focused setup, probing, review, cancellation, migration, manual fallback, and first-use flows change. |
| 7 | Deployment View | No | The production binary and deployment topology do not change; PTY and loopback twins are test infrastructure only. |
| 8 | Cross-cutting Concepts | Yes | Configuration atomicity, credential authority, source isolation, error handling, reasoning serialization, and cancellation change. |
| 9 | Architecture Decisions | Yes | Existing catalog, persistence, reasoning, provider naming, and write-boundary decisions are superseded. |
| 10 | Quality Requirements | Yes | New measurable scenarios cover no-write-before-confirmation, exact catalog routing, migration, and verbatim reasoning. |
| 11 | Risks and Technical Debt | Yes | Collision migration, source ambiguity, final-write failure, stale configuration, and shell partial failure require new risk treatment. |
| 12 | Glossary | Yes | Catalog source, coordinated draft, final confirmation, provider migration, and reasoning effort terms change or are added. |

## Status

STATUS: DONE
