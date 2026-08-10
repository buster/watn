# Arc42 Impact Assessment: release-truth-and-repository-cleanup

The change corrects release identity and deployment claims, reconciles active
architecture documentation with the current CLI and SetupWizard boundaries, and
records the constrained repository cleanup. The assessment below was made
independently for all twelve Arc42 chapters after reading the proposal,
specification, design, current documentation, and repository consumers.

## Assessment

| # | Chapter | Affected | Reason | Durable summary |
|---|---|---|---|---|
| 1 | Introduction and Goals | Yes | Release version identity, target-dependent deployment, and observable output are user and maintainer goals. | Record package-derived version output, target-aware release requirements, and the current stdout/stderr behavior. |
| 2 | Architecture Constraints | Yes | XDG storage and release-library assumptions constrain configuration and packaging decisions. | Make XDG config-only and document per-target dynamic library inspection without a static-deployment promise. |
| 3 | Context and Scope | Yes | The release artifact and the SetupWizard/model-picker terminology are part of the documented system boundary. | Correct the user-facing flow names, command channels, and configuration boundary. |
| 4 | Solution Strategy | Yes | Package metadata, host inspection, documentation reconciliation, and conservative cleanup add strategy decisions. | Replace stale static/deferred claims and describe the actual SetupWizard and model-picker approach. |
| 5 | Building Block View | Yes | Current building-block names and retained public boundaries must match the implementation consumers. | Use SetupWizard/model-picker terminology and preserve ProviderRegistry and setup result APIs. |
| 6 | Runtime View | Yes | Version/release verification and corrected streaming/setup interaction descriptions change documented flows. | Correct Ctrl-R, stdout/stderr ownership, and release verification wording in the runtime scenarios. |
| 7 | Deployment View | Yes | The release binary is dynamically linked for the verified target rather than universally static. | Document target-dependent runtime libraries and `file`/`ldd` or `otool -L` verification. |
| 8 | Cross-cutting Concepts | Yes | Configuration location, output channels, terminal interaction names, and release verification cross-cut multiple concerns. | Correct config-only XDG, SetupWizard/model-picker, output, and release truth statements. |
| 9 | Architecture Decisions | Yes | Release truth is a new architecture decision and stale active decision summaries need reconciliation. | Add ADR-0016 and index its MADR; remove deferred verification and obsolete current-name wording. |
| 10 | Quality Requirements | Yes | Version and release-artifact truth require measurable acceptance criteria, and portability claims must be narrowed. | Add release-truth scenarios and target-aware portability criteria. |
| 11 | Risks and Technical Debt | Yes | Dynamic runtime requirements, version drift, output-channel confusion, ADR consequences, and cleanup limits create explicit risks. | Add mitigations and record ADR-0016 consequences plus the conservative cleanup boundary. |
| 12 | Glossary | Yes | XDG, SetupWizard/model-picker, and release-runtime terminology currently contains stale definitions. | Replace obsolete helper names and define config-only XDG and target-dependent release terms. |

## Overall Impact

This is a release-truth, documentation, and repository-hygiene change. It does
not add a deployment topology, provider behavior, setup behavior, or output
contract; it makes the durable documentation and release evidence describe those
existing boundaries accurately. Historical snapshots under `givn/archive/`
remain historical and are not rewritten.

## Status

STATUS: DONE
