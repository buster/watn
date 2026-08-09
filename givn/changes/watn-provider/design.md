# Design: watn-provider

## Technical Summary

Implement provider onboarding inside the existing Rust CLI. The command parser
gains a `provider` subcommand. The subcommand runs the provider setup renderer
only when it has a TTY, validates one OpenAI-compatible provider, saves it, and
returns without starting model setup.

Normal question execution performs a local readiness check before resolving the
provider. Automatic onboarding is permitted only when provider selection is
implicit: no `--provider` flag and no `WATN_PROVIDER` selection environment
variable. If the implicit provider is not ready, stdin being a TTY is required
to start the provider and model setup flow. A non-TTY request prints actionable
setup guidance and exits with status 1 without initializing ratatui.

An explicit `--provider` or `WATN_PROVIDER` selection never starts automatic
onboarding. Unknown providers and missing credentials retain their existing
errors and exit-code mapping. A ready saved provider or recognized supported
environment credential bypasses onboarding and proceeds directly to the
request.

Provider setup and model setup return typed results. The caller, not either
setup function, maps cancellation and failures to process status. Automatic
onboarding saves the provider before model discovery, so a later model
cancellation or failure preserves the provider. Successful automatic setup
stops after model selection with status 0; it does not send or resume the
original question. The user runs the original question again after setup.

No new third-party dependency is required. Existing locked dependencies provide
`ratatui`, `crossterm`, `dialoguer`, `reqwest`, `portable-pty`, `httpmock`, and
the Gherkin runner.

## Architecture Impact

### Production modules

- `src/main.rs`
  - Add `Commands::Provider`.
  - Track whether provider selection came from `--provider`, `WATN_PROVIDER`,
    or the implicit default.
  - Dispatch explicit setup and models before question execution.
  - Apply the TTY-gated automatic onboarding branch.
  - Map typed setup results to status 0, 1, 2, 3, or 130.
  - Stop after successful automatic model setup instead of resuming the
    original question.
- `src/provider/mod.rs`
  - Export provider readiness, setup result, and cancellation types.
- `src/provider/setup.rs`
  - New ratatui/crossterm provider setup state machine.
  - Own endpoint input, credential-source selection, validation, review, and
    terminal restoration.
  - Return a draft result; do not save configuration or call process exit from
    the renderer.
- `src/config/types.rs`
  - Keep `ProviderConfig.api_key` as `Option<String>`.
  - Store either a literal credential or a complete reference such as
    `${OPENROUTER_API_KEY}`.
- `src/config/mod.rs`
  - Add local provider readiness detection.
  - Expand exact environment references at use time.
  - Prefer a saved `[providers.openrouter]` entry over the built-in OpenRouter
    endpoint.
  - Apply Unix mode `0600` after every direct configuration write.
- `src/config/env.rs`
  - Centralize provider-specific and generic credential fallback lookup.
  - Keep `WATN_PROVIDER` an explicit provider selection signal.
  - Do not treat arbitrary environment variables as provider configuration.
- `src/models/mod.rs`
  - Extract model setup into a reusable function returning a typed result.
  - Remove process exits from model setup and let the caller handle errors and
    cancellation.
  - Keep `watn models` as the explicit entry point and call the same function
    from automatic onboarding.
- HTTP construction used by `src/provider/openai_compat.rs` and
  `src/models/list.rs`
  - Accept the ephemeral test transport endpoint override at client/request
    construction time.
  - Never expose that override as a persisted configuration field or use it in
    readiness detection.
- `givn/commands.yaml`
  - Exclude `@e2e` from regular verification and coverage filters.
  - Keep the E2E filter.
  - Use collision-safe LLVM profile paths for both the runner and child CLI.

### Selection-source matrix

Provider readiness and provider selection are separate decisions. The source
of the selected provider controls whether automatic onboarding is allowed.

| Selection source | Provider not ready | TTY behavior | Non-TTY behavior |
|---|---|---|---|
| `--provider NAME` | Existing unknown-provider or missing-key error | Never starts onboarding | Same existing error |
| `WATN_PROVIDER=NAME` | Existing unknown-provider or missing-key error | Never starts onboarding | Same existing error |
| Implicit default | Automatic onboarding | Starts provider setup, then model setup | Prints setup guidance and exits 1 |
| Ready saved provider or recognized supported env credential | Not applicable | Sends the request directly | Sends the request directly |

An implicit default is the built-in/default selection used when the user did
not explicitly select a provider through the CLI or provider-selection
environment variable. A configured ready provider is detected from actual
endpoint and credential data rather than from the default provider name alone.

## Setup Flow And Typed Results

The setup state is separate from persistence and request execution. The result
types are conceptually:

```text
SetupCancellation = Escape | CtrlC
ProviderSetupResult = Configured(ProviderDraft) | Cancelled(SetupCancellation)
ModelSetupResult = Saved | Cancelled(SetupCancellation) | Failed(Error)
```

The production implementation may choose equivalent Rust names, but the
observable contract is fixed:

- Provider setup validates all input before returning `Configured`.
- Escape cancellation maps to status 1.
- Ctrl-C cancellation maps to status 130.
- Provider cancellation leaves the existing provider configuration untouched.
- The caller saves a successful provider result before invoking model setup.
- Model cancellation or failure leaves that saved provider in place and does
  not send the original request.
- Model failures use the existing `Error` to exit-code mapping. A mocked HTTP
  500 model catalog failure therefore exits with status 2.
- No setup function calls `std::process::exit`.

```mermaid
flowchart TD
    A[watn command] --> B{Explicit provider selection?}
    B -- --provider or WATN_PROVIDER --> C[Resolve provider normally]
    C --> D[Existing unknown or missing-key error]
    C --> E[Send request when ready]
    B -- no --> F[Load config and check local readiness]
    F --> G{Provider ready?}
    G -- yes --> E
    G -- no and stdin is not TTY --> H[Print setup guidance]
    H --> I[Exit 1; no ratatui]
    G -- no and stdin is TTY --> J[Run provider setup]
    J --> K{Provider result}
    K -- Escape --> L[Preserve config; exit 1]
    K -- Ctrl-C --> M[Preserve config; exit 130]
    K -- Configured --> N[Save provider]
    N --> O[Run model setup]
    O --> P{Model result}
    P -- Saved --> Q[Exit 0; do not send original request]
    P -- Cancelled --> R[Keep provider; exit 1 or 130]
    P -- Failed --> S[Keep provider; map Error; no request]
```

The explicit `watn provider` path is:

1. Require an interactive terminal for the setup renderer; a non-TTY explicit
   setup request prints the same actionable setup guidance and exits 1.
2. Run provider setup and return a typed result.
3. Save the provider only after confirmation.
4. Exit without invoking model setup or making a model catalog request.

The automatic path is:

1. Load configuration and inspect actual provider data and recognized
   credentials without network access.
2. If the implicit provider is missing and stdin is not a TTY, print guidance
   naming `watn provider`, the configuration path, and the environment-backed
   credential options, then exit 1.
3. If stdin is a TTY, run provider setup and save the confirmed provider.
4. Run the existing model setup function in-process.
5. On model success, persist all three tier selections and exit 0. Do not
   construct a chat request and do not resume the original question.

## Provider Configuration Model

### Persisted representation

The existing TOML structure remains the storage contract. Setup writes the
selected fixed provider name as the default provider and creates or replaces
that provider entry:

```toml
[defaults]
provider = "openrouter"

[providers.openrouter]
endpoint = "https://openrouter.ai/api/v1"
api_key = "${OPENROUTER_API_KEY}"
```

A custom endpoint uses the fixed `custom` name:

```toml
[defaults]
provider = "custom"

[providers.custom]
endpoint = "https://llm.example.com/v1"
api_key = "sk-custom-key"
```

The names are not generated from the URL:

- The exact normalized OpenRouter endpoint
  `https://openrouter.ai/api/v1` always maps to `openrouter`.
- Every other endpoint maps to `custom`.
- Re-running setup intentionally replaces the existing fixed entry with the
  new endpoint and credential representation. This is a documented collision
  with a manually maintained `openrouter` or `custom` entry, not a new provider
  name allocation.
- Replacement updates `defaults.provider` and only the selected fixed provider
  entry. Unrelated provider entries, model tiers, pricing, LiteLLM settings,
  schema metadata, and other config fields remain unchanged.
- Manually maintained arbitrary provider names remain valid for existing
  configuration and explicit provider selection; onboarding does not rename or
  migrate them.

The optional `default_model` field is not synthesized by provider setup. Model
setup remains responsible for populating `[tiers]`. A saved custom-provider
bypass test must provide a saved default model or saved tier assignment so that
request model resolution is not an unrelated failure.

### Endpoint handling

- The OpenRouter default is the exact string
  `https://openrouter.ai/api/v1`.
- Input is trimmed and trailing `/` characters are removed before provider-name
  classification and persistence.
- A custom endpoint must be non-empty and use an HTTP or HTTPS URL.
- Setup does not probe the endpoint before saving. Model discovery performs the
  first network operation and reports its existing diagnostics.
- Request URL construction appends exactly `/models` or
  `/chat/completions` after trimming the persisted endpoint.

### Credential representation and precedence

The credential source has two variants:

- `Literal(value)`: save the pasted value directly in `api_key`.
- `Environment(name)`: validate the name and save the complete string
  `${NAME}`.

The default environment references are:

- OpenRouter: `${OPENROUTER_API_KEY}`.
- Any custom endpoint: `${WATN_API_KEY}`.
- An explicitly entered valid uppercase shell-style variable name: the exact
  entered reference.

At use time, credential resolution follows this strict order:

| `api_key` state | Resolution | Fallback allowed? |
|---|---|---|
| `Some(literal)` where the value is not an exact reference | Use the literal | No |
| `Some(${VARIABLE})` | Expand `VARIABLE` from the environment | No; missing/empty is `AuthError` |
| `None` | Try provider-specific environment key | Then try generic `WATN_API_KEY` |
| `None` and all fallback keys absent/empty | Return `AuthError` | No request |

For the absent-key fallback, OpenRouter uses `OPENROUTER_API_KEY` as its
provider-specific key. A named provider uses the existing
`WATN_<PROVIDER>_API_KEY` convention, followed by `WATN_API_KEY`. A saved
literal or saved reference is authoritative even when fallback environment
variables are present. A missing saved reference is an authentication error,
not permission to fall through to another environment variable.

Environment references are expanded only when model discovery or a request
needs the credential. The expansion is exact, not substring interpolation:
only a value matching `${[A-Z_][A-Z0-9_]*}` is a reference. Resolved values are
never written back to configuration or printed in setup status.

### Readiness

Readiness is local and network-free. It returns ready only when the selected
provider has a usable endpoint and either a literal credential, a set saved
environment reference, or an available fallback credential when `api_key` is
absent. It returns missing for an absent/invalid endpoint, absent credential,
or missing saved reference. The ephemeral E2E transport override is never
consulted by readiness.

## Configuration Persistence And File Safety

All configuration saves use the existing direct `std::fs::write` mechanism.
After every successful direct write, including template, provider, and model
saves, Unix builds apply mode `0600` with the existing permissions API. An
existing world-readable file is corrected on its next save. Loading may still
warn about a pre-existing world-readable file before a save repairs it.

The design does not promise atomic temp-file/rename behavior. Direct writes may
retain the existing write interruption risk; the mitigation is permission
enforcement on every save and explicit documentation of the behavior.

## Ratatui Provider Setup State Machine

`src/provider/setup.rs` implements a deterministic state machine:

1. `Endpoint`: show the OpenRouter endpoint as the editable default; accept it
   or replace it with a custom endpoint.
2. `CredentialSource`: offer pasted credential and environment variable, with
   the provider-specific suggestion preselected when present.
3. `CredentialValue`: accept masked literal input or a validated environment
   variable name.
4. `Review`: display the normalized endpoint and whether the credential is
   literal or environment-backed, never the resolved secret.
5. `Confirmed`: return `ProviderSetupResult::Configured` to the caller.

Enter advances or confirms. Escape returns a cancellation classified as status
1. Ctrl-C returns a cancellation classified as status 130. Validation errors
keep the user in the relevant state and show an inline message. All exits
restore the terminal. Provider setup does not launch model setup.

The renderer is not the behavior under test for regular scenarios. The
renderer-independent setup state machine and configuration seam are exercised
directly so invalid input, normalization, credential precedence, replacement,
and cancellation can fail in RED without piped stdin or ratatui.

## Runtime Integration

The command dispatch order becomes:

1. Parse CLI arguments.
2. Dispatch `provider` directly if requested.
3. Dispatch `models` directly if requested.
4. Resolve a question and load configuration.
5. Determine provider selection source and check readiness without network
   access.
6. For an explicit selection, preserve the existing resolution and error
   behavior.
7. For an implicit missing provider, either print non-TTY guidance or run
   provider setup and save it.
8. Run model setup only in the successful automatic branch, then exit without
   sending the original request.
9. For a ready provider, resolve the model and expanded API key and send the
   request through the existing OpenAI-compatible provider.

The OpenRouter resolver must prefer a saved `[providers.openrouter]` entry and
its saved endpoint/credential representation. It falls back to the built-in
endpoint only when that entry does not exist. All providers pass through the
same exact-reference expansion function before an HTTP provider is built.

## Testing Design

### Gherkin runner

The existing `tests/features_runner.rs` recursively loads both `givn/specs/**`
and `givn/changes/**/specs/**`. It calls `.fail_on_skipped()`, which is strict
for the locked cucumber-rs runner. New step bodies must never be empty; a
RED-stage binding uses a failing placeholder such as
`unimplemented!("provider setup step not implemented")` until its scenario is
implemented.

Regular verification excludes E2E scenarios. The exact configured commands are:

```text
cargo test --test features_runner -- --tags 'not @wip and not @e2e'
cargo test --test features_runner -- --tags '@e2e and not @wip'
```

The runner rejects combining `--name` with `--tags`. Therefore every
single-scenario command uses `--name` alone:

```text
cargo test --test features_runner -- --name '^Configure OpenRouter with an environment-backed credential$'
cargo test --test features_runner -- --name '^First normal use starts provider setup and then model setup$'
```

These commands intentionally run the named scenario regardless of its current
`@wip` state during RED/GREEN work.

### Step definition module

All provider-setup step bindings, including PTY bindings for the two E2E
scenarios, live in one globally registered capability module:

- `tests/steps/provider_setup_steps.rs`

There is no `tests/e2e_steps` namespace and no separately registered E2E
module. Cucumber v0.23 registers step definitions globally; tags filter
scenarios, not step modules. Existing generic step patterns from
`tests/steps/ask_steps.rs` must not be duplicated.

Regular provider scenarios call the renderer-independent setup state machine,
config seam, and mocked transport directly. They do not pipe stdin into
ratatui. The two E2E scenarios use PTY only because they verify the real CLI
terminal interaction.

### E2E infrastructure and transport seam

The real interface type is **CLI/terminal**. The E2E driver is
`portable-pty`. It launches the compiled `watn` binary with
`TERM=xterm-256color`, sends user key sequences, and reads rendered terminal
output.

The E2E harness starts an isolated `httpmock::MockServer` on a random loopback
port. The test transport endpoint override is ephemeral test-harness state
passed to the child process and consumed only when HTTP clients are
constructed. It is not a TOML field, is never persisted, and is not consulted
by readiness. Both HTTP construction paths must honor it:

- model discovery requests to `/models` (including the automatic model setup
  path), and
- chat requests to `/chat/completions`.

The OpenRouter E2E scenario asserts the persisted endpoint remains exactly
`https://openrouter.ai/api/v1` while its chat request is routed to the loopback
transport. The automatic first-use E2E scenario asserts the same persisted
endpoint while its model catalog request is routed to loopback. No E2E
scenario contacts OpenRouter or another live provider. The override must not
make an invalid persisted endpoint appear ready.

Exactly two scenarios in the change feature carry `@e2e`, matching the two
User Interaction Inventory entries:

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism | Required external path |
|---|---|---|---|---|
| run `watn provider` and complete the interactive provider setup | Configure OpenRouter with an environment-backed credential | CLI/terminal | `portable-pty` starts `watn provider`, sends endpoint and credential keys, then a real CLI subprocess sends a request through the loopback twin | `/chat/completions` |
| run a normal `watn` command with no recognized provider and complete automatic provider and model setup | First normal use starts provider setup and then model setup | CLI/terminal | `portable-pty` keeps interactive `watn "hello"` alive across provider and model screens and asserts the terminal transition and final exit | `/models` |

The first scenario verifies request-time environment expansion and chat
transport. The second verifies automatic model discovery and explicitly
asserts that no original chat request is sent after setup.

### Coverage and profile paths

Coverage uses the same scenario filters as verification and passes a
collision-safe profile pattern to both the instrumented Gherkin runner and
spawned CLI:

```text
coverage/profraw/%p-%m.profraw
```

`%p` separates processes and `%m` separates instrumented binaries. The
configured coverage commands create `coverage/profraw`, clean the workspace,
build/run the instrumented `watn` binary, run the instrumented feature runner,
and export separate Cobertura files. Non-E2E coverage uses
`not @wip and not @e2e`; E2E coverage uses `@e2e and not @wip`.

The coverage boundary must include provider setup confirmation, config mode
enforcement, exact reference expansion, both `/models` and
`/chat/completions` transport construction, and the no-resume automatic branch.

### Scenario coverage map

The change feature covers:

- OpenRouter and custom fixed-name persistence, literal and environment-backed
  credentials, including explicit variable names.
- Endpoint validation, empty credential validation, and trailing-slash
  normalization.
- Saved-reference authentication failure, literal/reference precedence, saved
  OpenRouter endpoint precedence, and fallback ordering.
- Rerun replacement with unrelated configuration preservation and secure mode
  repair from `0644` to `0600`.
- Provider Escape/Ctrl-C cancellation, model catalog failure after provider
  save, and explicit provider command termination before model setup.
- TTY-gated automatic onboarding, actionable non-TTY guidance, and the rule
  that successful automatic setup does not send the original request.

## Decisions And Constraints

- No new dependency or version pin is introduced; existing lockfile versions
  are reused.
- No network call is made to decide whether onboarding is needed.
- A saved literal or environment reference is never replaced by a resolved
  secret.
- `openrouter` and `custom` are the fixed onboarding names; a rerun replaces
  only the selected fixed entry and preserves unrelated configuration.
- Explicit `--provider` and `WATN_PROVIDER` selections do not trigger
  onboarding and retain existing unknown-provider and missing-key errors.
- Automatic onboarding is TTY-only. Non-TTY first use emits guidance and exits
  1 without ratatui.
- The explicit `watn provider` command saves only provider configuration and
  does not invoke model setup.
- Provider and model setup return typed results and never call process exit
  internally.
- If automatic model setup is cancelled or fails, the provider remains saved,
  onboarding stops, and the original request is not sent.
- Successful automatic setup stops after model selection; it does not resume
  the original question.
- Every direct configuration save enforces Unix mode `0600`; no atomic
  temp-file/rename behavior is promised.
- The E2E endpoint override is ephemeral, applied only at HTTP construction,
  never persisted, never used for readiness, and covers both required paths.
