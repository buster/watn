# 1. Introduction and Goals

## Requirements overview

This change introduces `watn`, a CLI tool for generating shell commands from
LLMs. The tool is optimised for one-shot command generation with model tiering,
optional execution with confirmation, and metadata output (model, speed, cost).

Top requirements:
1. Ask a question and receive a copy-pasteable shell command
2. Three model tiers (small/fast via `-1`, normal via `-2`, thinking via `-3`)
3. The thinking tier sends a reasoning-effort signal to the API
4. A `-v`/`--verbose` flag prints the model's reasoning content to stderr
5. Optional execution with user confirmation (`-x` prompts "Execute now? [Y/n]")
6. Model discovery via optional LiteLLM endpoint (`watn models` interactive)
7. Layered configuration: CLI flags > env vars > user config > built-in defaults
8. Auto-init template: first run writes a commented-out config file to the standard XDG path
9. TTY-gated provider onboarding with OpenRouter defaults, environment-backed credentials, and automatic first-use model setup that stops before the original request
10. Structured terminal setup views make credential-source choices, provider details, model tiers, and long model catalogs scannable
11. One setup wizard makes the current page, editable line, cursor, and save/discard state explicit
12. Test routing must be isolated from normal and release-profile binaries; configured endpoints, readiness, and persisted configuration remain authoritative
13. Model discovery must preserve credential sources, select LiteLLM independently from chat, and use exact endpoint and Authorization behavior
14. Provider confirmation must survive catalog failure without changing unconfirmed model tiers or sending the original question
15. Reasoning defaults and persisted values must resolve consistently across interactive and non-interactive model selection
16. Overlapping model searches must leave the newest result visible and clean up older search work

See `givn/changes/watn-cli/specs/` for the executable Gherkin specification.

## Quality goals

| Priority | Quality attribute | Motivation |
|---|---|---|
| 1 | Usability | One-shot shell command generation; confirmation-prompted execution; scannable interactive setup views |
| 2 | Flexibility | Any OpenAI-compatible API; any model; model tier assignment |
| 3 | Portability | Single static binary, no runtime dependencies beyond the OS |
| 4 | Security | Prefer environment-backed credentials, mask pasted credentials, enforce private config permissions, and keep resolved secrets out of diagnostics |
| 5 | Observability | Model name, tokens/second, cost (when priced), and reasoning content (when verbose) printed per response; exit codes for scripting |
| 6 | Test isolation | Loopback transport overrides are available only to debug test-support binaries and cannot redirect release or normal invocations |
| 7 | Correctness | Catalog source, credential-source, setup-save, reasoning, and stale-search policies are shared across all model-discovery paths |

## Stakeholders

| Role | Expectation |
|---|---|
| Developer (end user) | Ask for shell commands from terminal, copy or execute immediately |
| Power user | Configure custom providers, model tiers, pricing for cost tracking |
| CI/user | Pipe questions in, get clean output with exit codes and metadata
| Test maintainer | Run deterministic local-provider scenarios without changing release-binary behavior or persisted user configuration |
