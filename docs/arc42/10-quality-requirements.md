# 10. Quality Requirements

## Quality tree

- **Usability** — Command generation, execution confirmation, sensible defaults
  - QS-001: Default tier returns a shell command immediately
  - QS-002: `-x` prompts and executes on Enter
- **Onboarding** — A first-time TTY user can reach model setup without learning TOML, while non-TTY use remains actionable and script-safe
  - QS-011: Provider setup offers a usable OpenRouter default
   - QS-012: Incomplete provider or model roles automatically open coordinated setup
  - QS-019: Setup information is separated into scannable terminal regions
    - QS-020: Background model search keeps the newest query authoritative
    - QS-054: Complete catalogs filter locally and delayed searches keep the query visible and responsive
   - QS-021: The setup wizard makes the active page and cursor explicit
   - QS-022: Coordinated setup separates model and reasoning questions and ends in review
   - QS-053: The setup wizard marks the focused input region with a green border
   - QS-023: Test transport cannot redirect release-profile binaries by source guard
   - QS-024: Test transport assertions identify the exact endpoint and credential
   - QS-027: Normal debug transport ignores a non-empty test override
- **Flexibility** — Provider-agnostic, config-driven, model tiering
  - QS-003: Custom OpenAI-compatible provider
  - QS-004: Model tier assignment via config
   - QS-005: Model discovery uses the selected provider's catalog source
- **Security** — Credentials are not unnecessarily copied or exposed
  - QS-013: Environment-backed credentials remain references in config
  - QS-014: Literal credentials are masked during entry
  - QS-015: Every config save repairs Unix mode to `0600`
- **Portability** — One release executable with verified target runtime requirements
  - QS-006: Release artifact and target libraries are identified for the selected host
- **Observability** — Model, speed, cost, reasoning in output
  - QS-007: Tokens/second displayed after response
  - QS-008: Cost displayed when pricing configured
  - QS-009: Buffered reasoning content displayed on stderr after successful completion when verbose flag is set
  - QS-010: Exit code categories for scripting
  - QS-032: Command content is visible before a delayed stream completes
  - QS-033: A completed stream is distinguished from EOF without `[DONE]`
  - QS-034: Partial output and output failures have safe cleanup behavior
  - QS-039: Version output matches the package metadata
- **Completion fidelity and script safety** — Current command tree, closed shell support, deterministic output, and no side effects
  - QS-040: Help states the exact completion usage, supported values, and stdout purpose
  - QS-041: Every supported script exposes the authoritative root tree and selector values
  - QS-042: Repeated generation is byte-for-byte deterministic
  - QS-043: Each generated script is accepted by its target shell parser
  - QS-044: Completion generation bypasses config creation and provider requests
  - QS-045: Unsupported shell errors contain the literal actionable contract
  - QS-046: Successful completion writes only the script to stdout
  - QS-047: The reserved `completions` token consequence is documented and stable
- **Shell integration safety** — Optional setup, safe startup-file ownership, non-evaluating line replacement, and request preservation
  - QS-048: Shortcut setup is opt-in and independent of provider/model selection
  - QS-049: Selected targets are installed independently and every result is reported
  - QS-050: Marker validation prevents malformed or duplicate block writes
  - QS-051: The widget preserves the buffer on empty, failed, or empty results
   - QS-052: The widget inserts successful output without evaluating it
   - QS-055: The widget records the original request as a `#`-prefixed history comment, leaves only the generated command in the buffer, and only the generated command runs on Enter
   - QS-056: Recalled history comments can be edited and re-asked because the widget strips one leading `# ` comment prefix

## Quality scenarios

| ID | Quality attribute | Scenario | Metric / Acceptance criterion |
|---|---|---|---|
| QS-001 | Usability | User runs `watn "find . -name *.rs"` | Output contains the find command; tokens/sec and model name are printed |
| QS-002 | Usability | User runs `watn -x "echo ok"` and presses Enter | Command executes; "ok" appears on stdout |
| QS-003 | Flexibility | User configures custom provider in config | Request is sent to the configured endpoint |
| QS-004 | Flexibility | User assigns models to tiers in config | `-1` uses small model, `-3` uses thinking model |
 | QS-005 | Flexibility | User runs `watn models` with a provider-local catalog endpoint | Interactive selection works; tier config is persisted and a conflicting legacy LiteLLM source receives zero requests |
| QS-006 | Portability | User deploys a release artifact on its verified target | `file` identifies the artifact as dynamically linked and `ldd` (Linux) or `otool -L` (macOS) identifies successful target-library requirements; no static portability claim is made |
| QS-007 | Observability | Response completes | Output contains "tokens/s" with a numeric value |
| QS-008 | Observability | Pricing configured in config | Output contains "cost:" with a monetary value |
| QS-009 | Observability | User runs `watn -3 -v "..."` and API returns reasoning | Reasoning content printed to stderr on its own line |
| QS-010 | Observability | API returns HTTP 429 | Exit code 2; stderr contains "rate limit" |
| QS-011 | Onboarding | User runs `watn provider` and accepts the default | The setup displays `https://openrouter.ai/api/v1` and persists it as the provider endpoint |
 | QS-012 | Onboarding | User runs a normal command with no ready provider or missing model role from a TTY | Coordinated setup starts, existing values are prefilled, setup exits after final confirmation or cancellation, and no original chat request is sent |
| QS-013 | Security | User chooses `OPENROUTER_API_KEY` as the credential source | Config contains `${OPENROUTER_API_KEY}`, not its resolved value; a later request uses the environment value |
| QS-014 | Security | User pastes a credential into provider setup | Terminal output masks the input and the resolved secret is absent from setup status output |
| QS-015 | Security | A provider or model save updates an existing config file | Unix mode is exactly `0600` after the direct write |
| QS-016 | Onboarding | User runs first use without a TTY and without a ready implicit provider | Exit 1; stderr names `watn provider` and the config path; no ratatui, `/models`, or chat request starts |
 | QS-017 | Onboarding | Coordinated setup is cancelled or catalog/model setup fails | Existing configuration remains byte-for-byte unchanged; a missing first-run file remains absent; the original chat request is not sent |
| QS-018 | Flexibility | A saved `[providers.openrouter]` entry exists | Its endpoint and credential representation take precedence over the built-in OpenRouter fallback |
| QS-019 | Onboarding / Usability | User opens provider or model setup in a terminal | Provider setup exposes a titled border, selectable credential list, aligned detail rows, and guidance paragraph; model setup exposes a titled border, three tier tabs, aligned model columns, and a scrollbar for an overflowing catalog |
| QS-020 | Usability / Responsiveness | User changes the model filter while a provider search is delayed | The UI remains able to redraw and accept input; after the debounce only the newest query's results are applied, and an older result cannot replace them |
| QS-021 | Onboarding / Usability | User opens setup and edits a page | The active tab, page number, prompt, and visible cursor identify exactly what is being edited |
| QS-022 | Onboarding / Usability | User advances through setup | URL, API key, Small Model, Middle Model, and Large Model appear as one ordered wizard; Enter/Tab advances, Shift-Tab returns, and Escape presents save/discard |
| QS-023 | Security / Portability | The transport resolver is compiled for a release profile with `test-support` enabled | The negated `cfg(all(feature = "test-support", debug_assertions))` branch is selected and the release binary has no active override lookup; the release artifact and target runtime libraries are inspected by release verification |
| QS-024 | Security / Testability | A debug test-support request uses a local twin | The expected full URL and method/path are exact, request count is exactly 1, Authorization is exactly `Bearer sk-configured`, and the persisted configured endpoint is unchanged |
| QS-025 | Security / Testability | The override is absent or whitespace | The configured endpoint receives exactly 1 request with the exact Authorization header; the competing endpoint receives exactly 0; persisted TOML is unchanged |
| QS-026 | Correctness | Readiness is evaluated while a competing override is present | Readiness is true from configuration alone; both local twins receive exactly 0 requests |
| QS-027 | Security / Testability | One default-feature debug request runs with a non-empty override pointing at a competing local twin | The configured twin receives exactly 1 request with the exact method/path and `Authorization: Bearer sk-configured`; the competing twin receives exactly 0 requests; output comes from the configured twin and persisted TOML is unchanged |
 | QS-028 | Correctness / Security | User runs `watn models` with a conflicting legacy `[litellm]` section | List, pagination, and search use the exact provider-local endpoint and provider Authorization; the legacy source receives zero requests |
 | QS-029 | Correctness / Recovery | User confirms or cancels a complete coordinated draft | Final confirmation writes one complete snapshot; cancellation or catalog failure preserves the baseline and does not send the original request |
 | QS-030 | Correctness | Model metadata or persisted config contains disabled, mandatory, custom, empty, or `off` reasoning values | Non-empty values round-trip verbatim, mandatory models reject `off`, whitespace-only input is rejected, and `off` emits no reasoning field |
| QS-031 | Responsiveness / Correctness | Slow and fast model searches overlap | Fast/newest IDs remain visible after the slow result completes; stale IDs are absent and all workers are cleaned up before scenario exit |
| QS-032 | Responsiveness / Usability | A provider flushes a first command event and delays a later event | The first content is observable before release; the spinner was visible beforehand and a clear-line cleanup is observable after first content; the complete generated command appears exactly once after success |
| QS-033 | Correctness / Recovery | A provider sends `[DONE]` and keeps the connection open, or closes without `[DONE]` | `[DONE]` permits exit 0 before connection close; EOF without `[DONE]` preserves visible content, reports a network error, emits no success metadata or prompt, and exits 3 |
| QS-034 | Observability / Correctness | Verbose reasoning and command content arrive in the same stream | Command stdout is visible before completion; reasoning is absent before completion, then appears on stderr only under `-v` after `[DONE]`; stdout never contains reasoning |
| QS-035 | Correctness / Observability | A choices-empty usage event supplies response model and usage after content | Final metadata names the response model rather than the requested model, uses pricing for that response model, and contains a non-zero cost and positive throughput |
| QS-036 | Recovery / Usability | A provider resets or reaches EOF after a visible command prefix | The prefix remains visible, spinner clear-line cleanup is observable, mapped network status is 3, final success metadata is absent, and no execute prompt appears |
| QS-037 | Recovery / Correctness | A command-output write or flush fails after a visible prefix | The existing I/O error status 1 is returned; the prefix and spinner cleanup remain observable; final metadata and execution confirmation are omitted |
| QS-038 | Correctness / Usability | A command is confirmed from a raw terminal or a pipe | The generated command is one complete line exactly once; execution output is asserted separately and occurs only after successful completion and confirmation |
| QS-039 | Correctness / Release truth | User runs `watn --version` from the release artifact | Exit status is 0 and the output contains the exact Cargo package version used to build the binary |
| QS-040 | Completion fidelity / Usability | User runs `watn completions --help` | Exit status is 0; stdout contains `Usage: watn completions <SHELL>`, `bash`, `zsh`, `fish`, and the instruction that the script is written to stdout for installation or sourcing; stderr is empty |
| QS-041 | Completion correctness | User runs `watn completions <SHELL>` for each supported shell | The script derives root options, positional-argument acceptance, every root subcommand, and selector suggestions `bash`, `elvish`, `fish`, `powershell`, and `zsh` from the authoritative command definition; renderers need not emit a literal name for the free-form `question` positional |
| QS-042 | Completion determinism | User generates the same shell script twice from the same binary | The stdout byte sequences are identical |
| QS-043 | Completion portability | User validates a generated Bash, Elvish, Fish, PowerShell, or Zsh script with its corresponding shell executable | The target parser accepts the script when installed; a missing shell executable is reported as an explicit environment limitation rather than a syntax success |
| QS-044 | Completion safety | User generates Bash completion with an absent isolated XDG config file and a zeroed provider-request sentinel | The config file remains absent, no file is written in the isolated config directory, the sentinel remains at zero, and no provider or network request occurs |
| QS-045 | Completion correctness / Usability | User runs `watn completions nushell` | Exit status is non-zero and stderr contains exactly `unsupported shell 'nushell'; choose bash, elvish, fish, powershell, or zsh` as the parser-owned literal |
| QS-046 | Completion safety / Observability | User generates a supported completion script | Stdout contains only the generated script and stderr is empty; no shell startup file is changed |
| QS-047 | Completion compatibility | User has question text whose first unquoted token is `completions` | The token dispatches to the completion subcommand; question text must be quoted or passed after `--`, and this consequence is documented in help/CLI documentation |
| QS-048 | Shell integration / Onboarding | User completes explicit or implicit first-use setup and accepts the shortcut default or selects no shells | Enter/no or an empty selection leaves every shell target byte-for-byte unchanged; provider/model setup behavior remains unchanged |
| QS-049 | Shell integration / Recovery | User selects Bash, Zsh, and Fish and one target fails | Every selected target is attempted; successful targets contain one generated block, the failed target is unchanged, every result is reported, and the aggregate status is non-zero |
| QS-050 | Shell integration / Integrity | A target contains duplicate, unmatched, or reversed shortcut markers | Installation fails before write; the target and its parent directory remain byte-for-byte unchanged |
| QS-051 | Shell integration / Correctness | User presses Ctrl-W with empty input, non-zero `watn`, or empty output | `watn` is not called for empty input; otherwise the original buffer remains exactly unchanged and stderr remains visible |
| QS-052 | Shell integration / Safety | User presses Ctrl-W and `watn` returns a successful command containing trailing or embedded line breaks | Trailing CR/LF is removed, embedded breaks remain text in the buffer, the cursor moves to the end, the prompt redraws, and no returned text executes |
| QS-053 | Onboarding / Usability | User moves between URL, credential, model, reasoning, and optional shortcut inputs in the setup wizard | The widget receiving keyboard input has a green border; inactive input widgets retain their existing border styling; layout, key behavior, and visible cursor remain unchanged |
| QS-054 | Onboarding / Responsiveness | User types a model filter against a complete or delayed catalog | The query remains visible; complete catalogs update locally without a search request; delayed searches do not block another query; only the newest result is applied and workers are joined on exit |
| QS-055 | Shell integration / Correctness and safety | User presses Ctrl-W and `watn` returns a successful, empty, or failed result | On success the shell history gains a `# flattened request` comment entry and the buffer holds only the generated command, only the generated command runs on Enter, and the text is never evaluated; on failure or empty output the original buffer remains unchanged and nothing is recorded |
 | QS-056 | Shell integration / Fish compatibility | User recalls a `# show available diskspace` comment from the shell history and presses Ctrl-W | The stripped buffer `show available diskspace` is asked as one question and the buffer holds only the generated command |
 | QS-057 | Onboarding / Recovery | User cancels after editing provider, catalog, model, reasoning, and shell values | Existing config and all shell targets remain byte-for-byte unchanged; an absent config remains absent |
 | QS-058 | Correctness / Security | Provider and legacy LiteLLM sources return different catalogs | Only provider-local list/page/search requests occur, with the provider credential; the legacy source receives zero requests |
 | QS-059 | Flexibility / Compatibility | A selected arbitrary provider key is configured through setup | The selected key migrates to `custom`, collision/default-model rules are deterministic, saved credential representation is preserved, and unrelated providers remain unchanged |
 | QS-060 | Correctness / Compatibility | A custom reasoning value is saved and used after reload | The exact non-empty value appears in TOML and the next request body; `off` is absent from the request body |
 | QS-061 | Recovery / Integrity | Final config serialization or rename fails | The previous config remains unchanged and no shell operation begins |
| QS-062 | Shell integration / Integrity | A shell integration is deselected | Only one valid Watn-managed block is removed; surrounding user bytes remain unchanged; malformed markers are refused |
| QS-063 | Specification maintainability | The active permanent tree is reviewed before consolidation | Duplicate titles are reported deterministically, every delta finding has a disposition, and the review receipt contains the net delta |
| QS-064 | Specification correctness | A consolidation archives removed and added scenarios | The permanent tree has no duplicate scenario titles, the full runner remains green, and no runtime CLI behavior changes |
| QS-065 | Specification ownership | A weaker scenario is subsumed by a stronger scenario | The weaker scenario is absent after archive, the retained scenario remains executable, and the review records the retained contract |
