# 1. Introduction and Goals

## Requirements overview

This change introduces `watn`, a CLI tool for generating shell commands from
LLMs. The tool is optimised for one-shot command generation with model tiering,
optional execution with confirmation, and metadata output (model, speed, cost).

Top 8 requirements:
1. Ask a question and receive a copy-pasteable shell command
2. Three model tiers (small/fast via `-1`, normal via `-2`, thinking via `-3`)
3. The thinking tier sends a reasoning-effort signal to the API
4. A `-v`/`--verbose` flag prints the model's reasoning content to stderr
5. Optional execution with user confirmation (`-x` prompts "Execute now? [Y/n]")
6. Model discovery via optional LiteLLM endpoint (`watn models` interactive)
7. Layered configuration: CLI flags > env vars > user config > system config > built-in defaults
8. Auto-init template: first run writes a commented-out config file to the standard XDG path

See `givn/changes/watn-cli/specs/` for the executable Gherkin specification.

## Quality goals

| Priority | Quality attribute | Motivation |
|---|---|---|
| 1 | Usability | One-shot shell command generation; confirmation-prompted execution |
| 2 | Flexibility | Any OpenAI-compatible API; any model; model tier assignment |
| 3 | Portability | Single static binary, no runtime dependencies beyond the OS |
| 4 | Observability | Model name, tokens/second, cost (when priced), and reasoning content (when verbose) printed per response; exit codes for scripting |

## Stakeholders

| Role | Expectation |
|---|---|
| Developer (end user) | Ask for shell commands from terminal, copy or execute immediately |
| Power user | Configure custom providers, model tiers, pricing for cost tracking |
| CI/user | Pipe questions in, get clean output with exit codes and metadata