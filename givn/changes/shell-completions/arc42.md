# arc42 Documentation Update: shell-completions

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Shell completion generation adds a user-facing CLI goal and deterministic, script-safe output requirements. | Add completion generation to the requirements, stakeholders, and quality goals. |
| 2 | 02 architecture-constraints | Yes | The closed selector, literal parser error, authoritative command metadata, and stdout-only behavior constrain implementation choices. | Record the supported shell boundary, parser contract, and side-effect constraints. |
| 3 | 03 context-and-scope | Yes | `watn completions <SHELL>` creates a new user-facing interface between the CLI, caller, and shell parser. | Add the completion caller and supported shell interfaces to the context and interface tables. |
| 4 | 04 solution-strategy | Yes | Completion generation requires a direct strategy using the existing Clap command definition and an early dispatch branch. | Record authoritative metadata reuse, closed selector mapping, and direct stdout generation. |
| 5 | 05 building-block-view | Yes | A local `CompletionShell` selector and completion-generation branch are new CLI building blocks. | Add the selector, generator boundary, and test-step ownership to the building-block view. |
| 6 | 06 runtime-view | Yes | Supported generation, help, invalid input, determinism, parser validation, and no-side-effect execution are new runtime flows. | Add a Mermaid generation flow and explicit no-config, help, and unsupported-selector runtime contracts. |
| 7 | 07 deployment-view | No | The change adds no service, package, artifact, or deployment topology. | |
| 8 | 08 crosscutting-concepts | Yes | Error wording, stdout/stderr routing, shell validation, determinism, and configuration/provider isolation are cross-cutting contracts. | Document completion output, error, help, and side-effect behavior. |
| 9 | 09 architecture-decisions | Yes | Choosing a local closed selector and the authoritative Clap definition is an architecture tradeoff. | Add ADR-0017 for completion generation. |
| 10 | 10 quality-requirements | Yes | The feature adds measurable usability, correctness, determinism, portability, and side-effect scenarios. | Add quality scenarios for all supported shells and the no-config contract. |
| 11 | 11 risks-and-technical-debt | Yes | Completion output can drift, shell parsers vary, and a reserved token changes question parsing. | Add risks and mitigations for generator drift, parser availability, and the reserved command token. |
| 12 | 12 glossary | Yes | `CompletionShell`, completion script, selector value suggestions, and reserved token are new domain terms. | Add completion-specific terms and their exact contract meanings. |

## Notes

Archived snapshots remain untouched. All new architecture diagrams use Mermaid
fenced blocks; no ASCII diagrams are introduced.

## Status

STATUS: DONE
