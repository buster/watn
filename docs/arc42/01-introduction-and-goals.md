# 1. Introduction and Goals

## Requirements overview

This change introduces `watn`, a CLI tool for generating shell commands from
LLMs. The tool is optimised for one-shot command generation with model tiering,
optional execution with confirmation, and metadata output (model, speed, cost).

Top requirements:
1. Ask a question and receive a copy-pasteable shell command
2. Three model tiers (small/fast via `-1`, normal via `-2`, thinking via `-3`)
3. The thinking tier sends a reasoning-effort signal to the API
4. A `-v`/`--verbose` flag prints the model's buffered reasoning content to stderr after successful completion
5. Optional execution with user confirmation (`-x` prompts "Execute now? [Y/n]")
6. Model discovery via optional LiteLLM endpoint inside the reviewed `watn setup` flow
7. Layered configuration with persisted values authoritative during setup and retained tier selectors at request time
8. A missing config path is a first-run signal without a read-time template side effect
9. TTY-gated onboarding reviews OpenRouter/provider suggestions, environment-backed credentials, and model roles before saving
10. One four-topic setup flow covers Provider, Model roles, Shell integration, and Review with contextual documentation
11. One in-memory draft and Finish-only commit make cancellation and persistence boundaries explicit
12. Test routing must be isolated from normal and release-profile binaries; configured endpoints, readiness, and persisted configuration remain authoritative
13. Model discovery must preserve credential sources, select LiteLLM independently from chat, and use exact endpoint and Authorization behavior
14. Setup must not persist provider or model changes before Review's Finish action, including when catalog discovery fails
15. Reasoning defaults and persisted values must resolve consistently across interactive and non-interactive model selection
16. Model filtering must keep the typed query visible and responsive, filter complete catalogs locally, use provider search only when the catalog is incomplete, leave the newest result visible, and clean up older search work
17. Generated command content must become visible and flushed before a delayed provider stream completes
18. Only an SSE stream terminated by `[DONE]` succeeds; truncated or failed streams preserve visible output and never execute it
19. Command-output write failures must retain the visible prefix, clean up progress, and use the existing I/O status without success metadata
20. `watn --version` must report the Cargo package version, and release documentation must match verified target runtime requirements
21. `watn completions <SHELL>` must generate the complete current command tree for the closed shell set `bash`, `elvish`, `fish`, `powershell`, and `zsh`, using stdout only without config/provider side effects
22. Setup may install an optional Ctrl-W command-line widget for selected Bash, Zsh, and Fish targets, including implicit first-use setup, without executing generated output
23. The setup wizard must mark the input region currently receiving keyboard input with a green border while preserving inactive styling and existing navigation
24. The Ctrl-W widget must keep the original request visible as a comment above the generated command while never evaluating the command
25. First-run credential detection must expose names and presence only, never resolved secret values
26. Existing configuration must remain byte-for-byte unchanged when setup is cancelled

See `givn/specs/` for the permanent executable Gherkin specifications.

## Quality goals

| Priority | Quality attribute | Motivation |
|---|---|---|
| 1 | Usability | One-shot shell command generation; confirmation-prompted execution; scannable interactive setup views with an unambiguous active-input indication |
| 2 | Flexibility | Any OpenAI-compatible API; any model; model tier assignment |
| 3 | Portability | A single release executable with runtime libraries documented and verified for its target |
| 4 | Security | Prefer environment-backed credentials, mask pasted credentials, enforce private config permissions, and keep resolved secrets out of diagnostics |
| 5 | Observability | Model name, tokens/second, cost (when priced), and buffered reasoning (when verbose after success) printed per response; exit codes for scripting |
| 6 | Test isolation | Loopback transport overrides are available only to debug test-support binaries and cannot redirect release or normal invocations |
| 7 | Correctness | Catalog source, credential-source, setup-save, reasoning, and stale-search policies are shared across all model-discovery paths |
| 8 | Responsiveness and recovery | Incremental content, deterministic completion, visible partial prefixes, and mapped stream/I/O failures must remain observable and safe |
| 9 | Completion fidelity and script safety | Completion output must match the authoritative command tree, remain byte-for-byte deterministic, parse in its target shell, and never initialise configuration or contact a provider |
| 10 | Shell integration safety | Shortcut installation must preserve user startup files, be idempotent, report independent target failures, and insert generated text without evaluation |

## Stakeholders

| Role | Expectation |
|---|---|
| Developer (end user) | Ask for shell commands from terminal, copy or execute immediately |
| Power user | Configure custom providers, model tiers, pricing for cost tracking |
| CI/user | Pipe questions in, get clean output with exit codes and metadata
| Test maintainer | Run deterministic local-provider scenarios without changing release-binary behavior or persisted user configuration |
| Release maintainer | Verify package version output and target-dependent runtime-library requirements before distribution |
| Shell user | Install or source a generated completion script, or opt into a Bash, Zsh, or Fish Ctrl-W widget that inserts a command without executing it |
