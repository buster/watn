# 1. Introduction and Goals

## Requirements overview

This change introduces `watn`, a CLI tool for generating shell commands from
LLMs. The tool is optimised for one-shot command generation with model tiering,
optional execution with confirmation, and metadata output (model, speed, cost).

Top 5 requirements:
1. Ask a question and receive a copy-pasteable shell command
2. Three model tiers (small/fast via `-1`, normal via `-2`, thinking via `-3`)
3. Optional execution with user confirmation (`-x` prompts "Execute now? [Y/n]")
4. Model discovery via optional LiteLLM endpoint (`watn models` interactive)
5. Layered configuration: CLI flags > env vars > user config > system config > built-in defaults

See `givn/changes/watn-cli/specs/` for the executable Gherkin specification.

## Quality goals

| Priority | Quality attribute | Motivation |
|---|---|---|
| 1 | Usability | One-shot shell command generation; confirmation-prompted execution |
| 2 | Flexibility | Any OpenAI-compatible API; any model; model tier assignment |
| 3 | Portability | Single static binary, no runtime dependencies beyond the OS |
| 4 | Observability | Model name, tokens/second, and cost (when priced) printed per response; exit codes for scripting |

## Stakeholders

| Role | Expectation |
|---|---|
| Developer (end user) | Ask for shell commands from terminal, copy or execute immediately |
| Power user | Configure custom providers, model tiers, pricing for cost tracking |
| CI/user | Pipe questions in, get clean output with exit codes and metadata