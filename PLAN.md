# watn Improvement Handoff Plan

Handoff snapshot: 2026-08-10

This file is the working handoff for the next agent. It describes the current
repository state, the decisions already made, and the remaining implementation
work.

## Current Repository State

- Repository: `/home/buster/projects/watn`
- Branch: `main`
- Worktree: clean before this plan update
- Remote/upstream: none configured
- Active givn change: none
- Archived transport work: `isolate-test-transport`
- Current package version: `0.1.2` in `Cargo.toml`
- Current CLI version: still hardcoded as `0.1.0` in `src/main.rs`

Do not amend existing commits. Configure a remote before attempting to push.

## Required Session Start

Run this before exploring or editing:

```text
givn instructions
```

The project uses givn. The required lifecycle is:

```text
new -> propose -> spec -> design -> design-review -> tasks -> implement -> review -> archive
```

Keep exactly one active change. Complete one scenario at a time. Use RED,
GREEN, REFACTOR, and one atomic scenario commit. Record the commit hash in
`tasks.md` immediately after the scenario commit. Do not batch-check task
boxes.

## Baseline Contracts

These contracts must remain true unless a later change explicitly changes them
through a reviewed proposal and specification:

- Commands and generated command content go to stdout.
- Metadata, prompts, spinners, setup guidance, and diagnostics go to stderr.
- LiteLLM is a model-discovery service only. It must never replace the active
  chat-completion provider.
- `watn models` changes model tiers without replacing the active provider.
- `watn setup` and implicit first-use setup save a valid provider before the
  first model-catalog request.
- An absent thinking-tier reasoning value retains the existing `high` default.
- Empty and unknown persisted reasoning values disable reasoning.
- The test endpoint seam is not available in normal or release-profile builds.
- A release binary is currently dynamically linked. Do not claim universal
  static deployment without producing and verifying static artifacts.
- Saved literal credentials and exact environment references are authoritative.
- A missing saved environment reference is an authentication error and does
  not fall through to another environment variable.

## Change 2: Model Discovery and Setup Correctness

Create this as the next active givn change:

```text
givn new model-discovery-and-setup-correctness
```

Use the full givn workflow. The scope covers report findings 1, 4, 5, 6, and 8,
plus the stale-search test defect that was explicitly deferred from change 1.

### Workstream A: Credential Source Preservation

Current defect:

- `src/models/mod.rs::run_models_result()` resolves an API key and discards
  the result before entering the TTY wizard.
- The built-in OpenRouter provider in `src/config/mod.rs` has `api_key: None`.
- `src/setup.rs::SetupWizard::from_config()` initializes an empty configuration
  credential.
- `load_catalog()` then rejects the empty credential.

Required behavior:

- With no saved OpenRouter key and `OPENROUTER_API_KEY` set, TTY
  `watn models` must discover models successfully.
- The wizard must treat the credential as environment-backed, not literal.
- The secret must not appear in terminal output or persisted TOML.
- Confirming that source persists `${OPENROUTER_API_KEY}`.
- A saved literal key remains authoritative over environment fallback.
- A saved exact environment reference remains authoritative and reports a
  missing variable as an authentication error.

Likely implementation direction:

- Preserve a credential source representation through `run_models_result()` and
  `SetupWizard` initialization.
- Distinguish `None`, literal, and complete `${VARIABLE}` references.
- Use `config::get_provider_api_key()` only for the outbound discovery secret;
  retain the source representation for persistence and UI state.
- Avoid displaying resolved secrets.

Required scenarios:

- TTY model setup with implicit built-in OpenRouter and `OPENROUTER_API_KEY`.
- Environment-backed confirmation persists the reference, not the value.
- Missing saved environment reference fails without a request.

### Workstream B: Catalog Source Resolution

Current defect:

- `Config.litellm` is parsed but production model discovery ignores it.
- `run_models_result()` uses the active provider endpoint for catalog requests.
- The permanent LiteLLM scenario in `givn/specs/providers/providers.feature`
  only checks that some mock was hit; `tests/steps/ask_steps.rs` ignores the
  expected URL.

Required behavior:

- When `[litellm]` exists, `/models`, paginated catalog, and search requests use
  the LiteLLM endpoint.
- A LiteLLM key is optional. No Authorization header is sent when absent.
- A configured LiteLLM environment reference expands at request time.
- Without `[litellm]`, discovery falls back to the selected provider endpoint.
- Chat completions always use the selected provider endpoint, never LiteLLM.
- The active provider draft and catalog source remain separate.
- Exact URL, method, path, query, and Authorization assertions must be used.

Likely implementation direction:

- Add one catalog-source resolver used by non-TTY model setup and the shared
  wizard.
- Pass endpoint and optional credential explicitly into model-list functions.
- Keep `LiteLLMConfig` as a production-consumed configuration type.
- Update `should_query_models_at()` to assert the supplied URL rather than only
  checking `mock.hits() > 0`.
- Add exact model search and pagination URL assertions where applicable.

Required scenarios:

- LiteLLM configured with a key uses exact `/models` URL and Authorization.
- LiteLLM configured without a key uses exact `/models` URL without auth.
- Provider discovery is used when LiteLLM is absent.
- Chat requests remain on the active provider when LiteLLM is configured.
- Search and pagination use the correct catalog source.

### Workstream C: Partial Save Through The Real Wizard

Current defect:

- Documentation claims the provider is saved before model discovery.
- `apply_result()` currently persists only after the wizard returns a complete
  result.
- Catalog failure in `move_next()` returns to the API-key page but does not save
  the confirmed provider.
- The current catalog-failure step bypasses the real wizard by calling
  `save_provider_draft()` directly.

Required behavior:

- Provider setup saves only after valid credential confirmation.
- Full setup and implicit onboarding save the provider before the first catalog
  request.
- Catalog failure leaves the provider persisted and tiers unchanged.
- Cancellation before credential confirmation does not write.
- Cancellation after credential confirmation preserves the provider.
- No original chat request is sent after setup failure or cancellation.
- `watn models` changes tiers without replacing provider or LiteLLM settings.

Required test correction:

- Drive catalog failure through the actual unified wizard or a reviewed seam
  that exercises the same persistence boundary.
- Do not use a direct `save_provider_draft()` call as the primary simulation of
  automatic onboarding.
- Assert terminal/CLI behavior first and config persistence second.

### Workstream D: Reasoning Defaults And Persistence

Current defects:

- Non-TTY `watn models` creates three empty reasoning strings and overwrites
  existing reasoning configuration.
- `ModelReasoning.default_enabled` is parsed but ignored.
- `TierReasoning::effort()` forwards arbitrary values such as `bogus`.
- Reasoning metadata behavior is missing tests for `minimal`, mandatory,
  disabled defaults, and unknown strengths.

Required policy:

- Valid strengths are `off`, `low`, `minimal`, `medium`, and `high`.
- Empty and unknown persisted values resolve to no reasoning.
- A non-mandatory model with `default_enabled = false` defaults to `off`.
- A mandatory model cannot select `off`.
- A valid `default_effort` is preferred when enabled and supported.
- Otherwise select the first valid supported effort.
- Unknown supported efforts are ignored.
- Non-TTY model assignment preserves existing reasoning unless a valid model
  default replaces it.
- No empty reasoning strings are serialized.

Likely implementation direction:

- Centralize parsing and resolution in a small pure policy function.
- Reuse the policy in TTY wizard synchronization and non-TTY selection.
- Keep the thinking-tier absent-value compatibility default explicit.
- Add unit tests for policy boundaries plus Gherkin scenarios for observable
  request bodies and persisted TOML.

Required scenarios:

- Disabled default selects `off` even when a default effort is present.
- Mandatory reasoning excludes `off`.
- `minimal` is persisted and sent.
- Unknown configured strength sends no reasoning.
- Non-TTY model selection never writes empty strings.
- Existing reasoning survives model selection when no valid replacement exists.

### Workstream E: Stale Search Concurrency

Current defect:

- `tests/steps/ask_steps.rs` waits for the slow search before starting the fast
  search.
- `search_query_delays` is populated but does not control actual overlap.
- The current scenario can pass even if the generation guard is removed.

Required behavior:

- Start the slow query and fast query before either result is fully applied.
- The fast/newest result becomes visible.
- A late slow/older result cannot replace it.
- The assertion checks exact final IDs: includes the fast result and excludes
  the stale result.
- Search workers are cleaned up before scenario exit.

Likely implementation direction:

- Add a deterministic barrier or channel to the test twin.
- Dispatch both real search operations concurrently.
- Apply results through the same generation guard as production.
- Remove `search_query_delays` if the corrected test no longer needs it.

## Change 3: Incremental SSE Rendering

Create after change 2 is archived:

```text
givn new incremental-sse-rendering
```

This covers report finding 2, usage-only parsing from finding 7, and the output
and spinner coverage gaps.

### Provider API

Current defect:

- `src/provider/openai_compat.rs` calls `response.bytes()` and buffers the full
  response before parsing.
- `src/provider/mod.rs::Provider` returns only a final `StreamingResponse`.
- `src/main.rs` prints only after the provider returns.

Required behavior:

- Parse SSE incrementally from the blocking response reader.
- Emit content and reasoning events as they arrive.
- Accumulate the same content for final metadata, verbose output, and `-x`.
- Return final model, usage, elapsed time, accumulated content, and reasoning.
- Propagate mid-stream transport errors after preserving any already-visible
  output and cleaning the spinner.

Recommended design:

- Keep `reqwest::blocking`; do not add async solely for streaming.
- Use a synchronous event sink or callback owned by the single CLI consumer.
- Parse complete SSE lines/events with a buffered reader.
- Do not introduce a worker channel unless it is required by a concrete
  concurrency need.

### SSE Parsing Rules

- Handle `data:` lines and `[DONE]`.
- Ignore blank and non-data lines.
- Tolerate malformed nonessential JSON events without crashing the whole stream.
- Extract `content` and `reasoning` from choice deltas.
- Extract `usage` from the top-level event even when `choices` is empty.
- Extract the response model from the top-level event independently of choices.
- Measure elapsed time from the first received stream event.
- Preserve correct cost and tok/s when usage appears in a final usage-only event.

### CLI Output Rules

- Start the spinner before request execution.
- Clear or stop the spinner when the first content token arrives.
- Flush content to stdout immediately.
- Print final metadata only after stream completion.
- Never print the complete command a second time after incremental output.
- Keep reasoning on stderr and only print it under `-v`.
- Prompt for `-x` only after the complete command is received.

### Required Tests

- A local provider flushes one SSE event, waits, and then flushes the final
  event. The test observes the first token before the delayed response ends.
- Usage-only final event produces non-zero cost/tok-s values when configured.
- Reasoning and content are emitted separately.
- `[DONE]` terminates cleanly.
- Partial network reads are parsed correctly.
- Malformed nonessential SSE lines are tolerated.
- Mid-stream failure returns a non-zero status and cleans the spinner.
- Spinner startup, worker lifecycle, cleanup, and Drop are covered where
  observable.
- Raw TTY confirmation is tested separately from piped stdin confirmation.

## Change 4: Release Truth And Repository Cleanup

Create after change 3 is archived:

```text
givn new release-truth-and-repository-cleanup
```

This covers findings 9 and 10, documentation drift, and remaining dead-code
candidates.

### Version

- Replace the hardcoded `0.1.0` in `src/main.rs` with Cargo package metadata.
- Make the version scenario assert the package version from `Cargo.toml` or
  equivalent runtime metadata.
- Do not bump `0.1.2` unless a release is explicitly being prepared.

### Deployment Truth

- Current `cargo build --release` output is dynamically linked.
- Update `docs/arc42/07-deployment-view.md` to state target-dependent runtime
  library requirements.
- Add release verification using `file` and `ldd`.
- If static artifacts become a requirement, make that a separate release
  engineering decision involving musl, TLS, compression, and CI artifact
  verification.

### Documentation Reconciliation

Update the active Arc42 and README claims for:

- Incremental versus buffered streaming.
- LiteLLM discovery-only behavior.
- Actual shared setup wizard behavior.
- Actual PTY helper names.
- Ctrl-R rather than plain `r` for reasoning focus.
- Four reasoning strengths plus `minimal`.
- stdout command output and stderr metadata/prompt behavior.
- Config-only XDG storage rather than an unimplemented data directory.
- Debug-only test-support routing and deferred release verification.
- Historical status of archived Arc42 snapshots.
- One authoritative coverage command and current measured values.

### Dead Code And Hygiene

- Keep `LiteLLMConfig` after adding its production consumer.
- Keep `ModelReasoning.default_enabled` after adding its behavior.
- Remove provider setup result wrappers only after confirming there are no
  external library consumers. This repository is currently structured as a
  binary application, but public modules exist in `src/lib.rs`.
- Remove the unused `_config` parameter from `build_registry()`.
- Reassess whether `ProviderRegistry` is useful for one active provider.
- Remove write-only fields from `WatnWorld` after their scenarios are corrected.
- Remove obsolete helper names and archived documentation claims.
- Decide separately whether to format the entire repository. Avoid mixing a
  repository-wide rustfmt rewrite into behavioral commits.

## Additional Planned Work

### 4. Watn Bash, Zsh, And Fish Completions Through Clap

Create this as a separate givn change after the release/documentation cleanup
unless reprioritized:

```text
givn new shell-completions
```

Add a `watn completions <shell>` command backed by Clap completion generation.
The supported shell values are `bash`, `zsh`, and `fish`. The command writes
only the generated completion script to stdout, does not load configuration,
does not contact a provider, and reports an unsupported shell as a normal CLI
error. Do not hand-maintain shell completion scripts when Clap can generate
them from the authoritative command tree.

Required behavior:

- Completion output includes the current commands, flags, subcommands, and
  value suggestions exposed by the Clap CLI.
- Bash, Zsh, and Fish output is deterministic for a fixed CLI definition.
- `watn completions --help` documents the supported shells and output purpose.
- Completion generation does not write files or modify shell configuration.
- The command uses the package's actual CLI metadata and does not duplicate
  command names in a second registry.

Required tests:

- `watn completions bash` exits successfully and emits Bash completion syntax.
- `watn completions zsh` exits successfully and emits Zsh completion syntax.
- `watn completions fish` exits successfully and emits Fish completion syntax.
- An unsupported shell returns a non-zero status with actionable guidance.
- The generated output contains the current setup, provider, models, and
  completion command options.
- Existing command behavior and stdout/stderr contracts remain unchanged.

The implementation should use the repository's existing CLI test and Gherkin
fixture conventions. If a completion dependency is required, keep it limited
to completion generation and verify that the generated scripts are sourced by
the intended shell versions.

### 5. Interactive Shell Shortcut For Watn

Create this as a separate givn change after shell completions, unless the
implementation order is deliberately changed through a reviewed proposal:

```text
givn new interactive-shell-shortcut
```

#### Overview

Extend the existing multi-step `watn setup` wizard with an optional step that
installs a shell key binding. The default binding is Ctrl-W. It reads the
entire current shell command-line buffer, passes that buffer to `watn` as one
quoted question, and replaces the current buffer with the generated command.
The generated command is inserted but never executed automatically.

Example:

```text
$ find all images<Ctrl-W>
$ find . -type f \( -iname '*.jpg' -o -iname '*.png' \)
```

The user can inspect or edit the replacement before pressing Enter.

#### Setup Flow

Add the following optional flow to the existing setup wizard:

1. Ask: `Configure a shell shortcut for generating commands with watn?`
2. If enabled, show a multi-select list containing Bash, Zsh, and Fish.
3. Preselect shells detected from the current environment when appropriate,
   without preventing selection of any supported shell.
4. Allow zero, one, or multiple selected shells.
5. Install the selected shell configuration.
6. Report modified files and shell-specific reload instructions.

If the user declines or selects no shells, leave shell configuration untouched
and continue setup. The shell choice is independent of the user's default
shell; users may select shells that are not currently running.

#### Shortcut Contract

For every selected shell, the generated widget must:

- Read the complete current command-line buffer.
- Avoid invoking `watn` for empty input, or otherwise leave the line unchanged.
- Invoke `watn "$question"` with quoted expansion so the entire buffer is one
  question.
- Capture the generated command without evaluating it.
- Replace the current buffer only when `watn` succeeds and output is non-empty.
- Move the cursor to the end of the replacement.
- Redisplay the prompt.
- Preserve the original input when `watn` fails or produces no output.
- Never execute the generated command automatically.
- Normalize trailing newlines so the inserted value remains a single command
  line.
- Keep stderr visible or handle it consistently with the existing CLI output
  contract.

#### Bash Implementation

Install a function and Readline binding using the current line and cursor:

```bash
# >>> watn shell shortcut >>>
_watn_widget() {
    local question=$READLINE_LINE
    local result

    if result=$(watn "$question") && [[ -n "$result" ]]; then
        READLINE_LINE=$result
        READLINE_POINT=${#READLINE_LINE}
    fi
}

bind -x '"\C-w":_watn_widget'
# <<< watn shell shortcut <<<
```

Bash-specific requirements:

- Use `READLINE_LINE` for the current buffer.
- Use `READLINE_POINT` to position the cursor.
- Register the function with `bind -x`.
- Load the binding when the shell starts.
- Explicitly document that the selected shell's existing Ctrl-W binding is
  overridden, or preserve it if the project decides that is required.

#### Zsh Implementation

Install a ZLE widget and binding:

```zsh
# >>> watn shell shortcut >>>
_watn_widget() {
    local question=$BUFFER
    local result

    if result=$(watn "$question") && [[ -n "$result" ]]; then
        BUFFER=$result
        CURSOR=${#BUFFER}
    fi

    zle redisplay
}

zle -N _watn_widget
bindkey '^W' _watn_widget
# <<< watn shell shortcut <<<
```

Zsh-specific requirements:

- Use `$BUFFER` for the current line.
- Set `$CURSOR` after replacement.
- Register the widget using `zle -N`.
- Bind the widget using `bindkey`.
- Bind the default map and, when applicable, `viins` as well:
  `bindkey -M viins '^W' _watn_widget`.

#### Fish Implementation

Install a Fish function and binding:

```fish
# >>> watn shell shortcut >>>
function _watn_widget
    set -l question (commandline)
    set -l result (watn "$question" | string collect)
    set -l status_code $pipestatus[1]

    if test $status_code -eq 0; and test -n "$result"
        commandline -r -- "$result"
    end

    commandline -f repaint
end

bind \cw _watn_widget
# <<< watn shell shortcut <<<
```

Fish-specific requirements:

- Use `commandline` to read the current buffer.
- Use `commandline -r -- "$result"` to replace it.
- Repaint after the command completes.
- Preserve the original line on failure or empty output.
- Bind Ctrl-W in the appropriate default mode; support insert mode if the
  project's Fish configuration conventions require it.

#### Configuration Installation

Follow existing setup conventions for prompt rendering, multi-select inputs,
configuration discovery, status output, error handling, and generated blocks.
The typical target files are:

| Shell | Typical configuration file |
|---|---|
| Bash | `~/.bashrc` |
| Zsh | `~/.zshrc` |
| Fish | `~/.config/fish/config.fish` |

The installer must:

- Use existing shell and configuration path detection when available instead of
  hard-coded assumptions.
- Create missing parent directories, especially for Fish.
- Append a clearly delimited shell-appropriate generated block.
- Use the markers `# >>> watn shell shortcut >>>` and
  `# <<< watn shell shortcut <<<`.
- Replace an existing exact generated block rather than append a duplicate.
- Preserve unrelated user configuration.
- Define the behavior for manually customized content inside an existing
  marked block; the preferred default is replacing only exact generated-marker
  blocks, with confirmation if the project chooses to preserve custom edits.
- Fail clearly when a target file cannot be read or written.
- Ideally create a backup before modifying an existing file, following any
  existing repository backup convention.
- Use the installed `watn` executable resolved naturally from the user's PATH;
  never embed a development-time or repository-local executable path.

#### Setup Edge Cases And Runtime Reporting

- Running setup twice is idempotent.
- Declining the shortcut makes no shell-file changes.
- Selecting no shells makes no shell-file changes.
- Selecting multiple shells updates each selected file independently.
- Existing Ctrl-W behavior is overridden only for selected shells.
- Empty input, non-zero `watn` status, and empty output preserve the original
  buffer.
- Generated output is inserted as text and is never shell-evaluated during
  replacement.
- Report each modified file and how to reload it. Examples:

```text
Configured Bash in ~/.bashrc.
Run: source ~/.bashrc

Configured Zsh in ~/.zshrc.
Run: source ~/.zshrc

Configured Fish in ~/.config/fish/config.fish.
Run: source ~/.config/fish/config.fish
```

It is also valid to state that the shortcut becomes available in newly
started shells.

#### Required Tests

Use temporary HOME/XDG-style directories and shell fixture files so tests do
not modify a developer's real configuration. Cover:

- Declining the wizard step leaves all shell files byte-for-byte unchanged.
- Zero, one, and multiple shell selections behave correctly.
- Environment-based preselection does not restrict manual selection.
- Missing parent directories are created only when installation is selected.
- Bash, Zsh, and Fish blocks contain their required widget APIs and bindings.
- Repeated setup replaces or preserves the generated block without duplicates.
- Unrelated user configuration remains byte-for-byte unchanged.
- Read/write failures produce actionable errors.
- A successful widget replaces the line, moves the cursor, repaints, and does
  not execute the generated command.
- A failed or empty `watn` result preserves the original line.
- The complete input is passed as one quoted question, including spaces and
  shell metacharacters.
- Trailing output newlines are normalized consistently.
- Reload instructions identify the exact modified file for every selected
  shell.

### 6. Highlight Active Setup Input

Improve the setup dialog's visual indication of where user input is currently
being entered. The border or box surrounding the active input location shall be
green; inactive input locations retain their existing styling.

### 7. Responsive Setup Model Filtering

Improve the setup dialog's model filter so the typed query remains visible while
the user is entering it. Typing must remain responsive while model searches run
in the background, and filter updates shall be debounced by 200 ms. The model
list shall update continuously as the debounced query changes, without blocking
further input on an in-flight search.

When the complete model list fits in one catalog request (for example, fewer
than 500 models), load the list once and filter it client-side instead of making
server-side search requests. Use server-side filtering when the catalog requires
multiple requests or otherwise cannot be loaded in one request. Results from an
older in-flight search must not replace the results for a newer query.

## Handoff Rules

- Read `givn instructions` before acting.
- Inspect `givn status --change <id>` before editing an active change.
- Preserve user changes and never reset or checkout unrelated work.
- Use `apply_patch` for manual edits.
- Use one active change at a time.
- Use one scenario commit for RED, GREEN, and REFACTOR.
- Do not amend existing commits.
- Do not push unless the user explicitly requests it.
- Keep secrets out of diagnostics, persisted test output, and commits.
- Prefer the smallest correct implementation. Do not add compatibility layers
  without a concrete persisted-data or external-consumer requirement.
