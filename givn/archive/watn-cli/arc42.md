# Arc42 Impact Assessment: watn-cli

## 12-row assessment

| # | Chapter | Affected? | Action |
|---|---|---|---|
| 1 | Introduction and Goals | YES | Rewrote requirements around command generation, model tiers, execution mode |
| 2 | Architecture Constraints | YES | Added constraints for model tier flags, LiteLLM integration, execution confirmation |
| 3 | Context and Scope | YES | Added LiteLLM and system shell as external systems; removed session store |
| 4 | Solution Strategy | YES | Updated for tier dispatch, execution mode, LiteLLM discovery; removed JSONL session storage reference |
| 5 | Building Block View | YES | Replaced session module with models (explorer) and exec modules; removed ConfirmationPrompt sub-component |
| 6 | Runtime View | YES | Replaced session flow with model explorer flow; added execution flow; added config loading flow |
| 7 | Deployment View | YES | Replaced scaffold with truthful minimal content — cargo install or copy binary to PATH |
| 8 | Cross-cutting Concepts | YES | Added tier resolution, cost tracking, tok/s timing, execution mode |
| 9 | Architecture Decisions | YES | Split inline ADRs into standalone MADR files under docs/adr/ (0001-0006); 09-architecture-decisions.md now references them |
| 10 | Quality Requirements | YES | Rewrote quality scenarios for command generation, execution, tier resolution |
| 11 | Risks and Technical Debt | YES | Added risks for LiteLLM schema drift, destructive command execution, and execution flow UX surprise (R-006) |
| 12 | Glossary | YES | Added tier, LiteLLM, tokens/second, pricing, execution mode terms |

STATUS: DONE
