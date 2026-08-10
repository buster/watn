# Arc42 impact assessment: model-discovery-and-setup-correctness

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and goals | Yes | Adds correctness, credential-safety, setup-recovery, reasoning, and newest-search goals. |
| 2 | Architecture constraints | Yes | Adds catalog/chat separation, authoritative credential sources, and the closed reasoning set. |
| 3 | Context and scope | Yes | Clarifies LiteLLM discovery traffic versus active-provider chat traffic. |
| 4 | Solution strategy | Yes | Adds catalog-source resolution, partial provider save, shared reasoning policy, and generation guards. |
| 5 | Building block view | Yes | Changes Config, Models, and Setup Wizard responsibilities and introduces a runtime catalog source. |
| 6 | Runtime view | Yes | Adds catalog routing, provider-confirmation-before-failure, reasoning, and stale-search flows. |
| 7 | Deployment view | No | No deployment, packaging, profile, or runtime infrastructure behavior changes in this change. |
| 8 | Cross-cutting concepts | Yes | Changes credential handling, request routing, reasoning validation, persistence boundaries, and search lifecycle. |
| 9 | Architecture decisions | Yes | Adds ADR-0014 for independent catalog routing and provider confirmation persistence. |
| 10 | Quality requirements | Yes | Adds measurable quality scenarios for exact catalog routing, partial saves, reasoning, and concurrent search. |
| 11 | Risks and technical debt | Yes | Adds risks for source crossover, malformed reasoning, save timing, and weak concurrency evidence. |
| 12 | Glossary | Yes | Adds catalog source, credential source, provider draft, reasoning policy, and search-generation terms. |

## Status

STATUS: DONE
