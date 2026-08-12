# Arc42 Impact Assessment: Setup Refactoring

The change replaces field-oriented setup with a reviewed, four-topic draft
flow. It changes first-run dispatch, configuration persistence, credential
discovery, model-role review, shell reconciliation, and the associated terminal
contracts.

## Chapter Assessment

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and goals | Yes | First-run setup becomes an explicit reviewed onboarding experience with credential-safety and cancellation goals. |
| 2 | Architecture constraints | Yes | Finish-only persistence, read-only first-run detection, allowlisted discovery, and the retained Rust/Ratatui/TTY constraints narrow implementation choices. |
| 3 | Context and scope | Yes | The user-facing setup surface changes to Provider, Model roles, Shell integration, and Review; removed CLI commands and overrides change the boundary. |
| 4 | Solution strategy | Yes | The architecture moves from seven pages and early writes to one in-memory draft and one commit boundary. |
| 5 | Building-block view | Yes | Draft, discovery, role, shell-intent, rendering, and commit responsibilities become explicit building blocks. |
| 6 | Runtime view | Yes | First-run, review/finish, cancellation, catalog failure, and partial shell-failure flows are new or changed sequences. |
| 7 | Deployment view | Yes | Per-user shell startup files are a deployment integration whose marker blocks are now inspected, removed, and reconciled after Finish. |
| 8 | Cross-cutting concepts | Yes | Secret handling, config durability, validation, cancellation, and partial side-effect reporting change across the CLI. |
| 9 | Architecture decisions | Yes | The change records reviewed draft state, explicit provider identities, and Finish-only persistence as durable decisions. |
| 10 | Quality requirements | Yes | New measurable requirements cover no-secret leakage, responsive help, no premature writes, and first-run behavior. |
| 11 | Risks and technical debt | Yes | A larger draft and shell reconciliation introduce failure and migration risks that require explicit mitigation. |
| 12 | Glossary | Yes | Terms such as setup draft, field origin, catalog status, shell intent, and Finish become part of the shared language. |

## Updated Chapters

The durable chapter files under `docs/arc42/` were updated for the affected
rows. Chapter 7 records the changed lifecycle of per-user shell startup files;
binary and service deployment topology remains unchanged.

## Status

STATUS: DONE
