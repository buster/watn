# ADR-0011: Interactive provider onboarding with environment-backed credentials

- **Status:** proposed
- **Date:** 2026-08-09
- **Decision-makers:** architect

## Context and Problem Statement

Watn already accepts OpenAI-compatible provider configuration, but first-run
users must know the TOML structure and credential precedence before they can
make a request. Model setup also depends on a provider endpoint and credential.
The CLI needs a guided terminal entry point that can use an existing shell
credential without copying the secret into the config file.

Automatic onboarding must not change established explicit-provider behavior.
An explicit `--provider` or `WATN_PROVIDER` selection must continue to report
unknown providers and missing credentials through the existing errors. Only an
implicit first-use selection may enter onboarding, and only when stdin is a
TTY. A non-TTY first-use request must provide setup guidance instead of
initializing a terminal renderer.

## Decision Drivers

- A new user must be able to configure an endpoint without hand-editing TOML.
- OpenRouter should work with a sensible default endpoint and key variable.
- Environment-backed credentials should remain references in persisted config.
- The interactive flow must use the existing terminal UI technology.
- Automatic onboarding must be TTY-gated and must not alter explicit-provider
  error behavior.
- A first normal request with no implicit ready provider should enter provider
  and model setup, but successful setup must stop before sending the original
  request.
- If model setup fails after provider setup, the provider must remain saved and
  the original request must not be sent.
- Existing manually maintained provider configuration must remain valid.
- Every config save must repair Unix permissions without promising atomic writes.
- E2E tests must never contact a live provider.

## Considered Options

- **Documentation-only setup:** document TOML examples and require users to
  edit the file. This keeps the binary unchanged but does not solve first-run
  usability or automatic model setup.
- **One-shot command-line flags:** add endpoint and key flags. This supports
  scripting but exposes secrets in shell history and does not provide a guided
  first-run flow.
- **Dialoguer prompts:** use the existing line/list prompt dependency for
  endpoint and credential questions. This is simple for linear input, but it
  does not provide the consistent keyboard-driven state machine, inline
  validation, masking, terminal restoration, and future review state required
  by the provider and model dialogs. It also makes TTY/non-TTY renderer
  boundaries less explicit.
- **Interactive ratatui setup with persisted credential references:** collect
  endpoint and credential source in a deterministic terminal state machine,
  save either the literal value or `${VARIABLE}`, and use typed results to
  control the automatic first-use branch. This reuses the existing ratatui and
  crossterm interaction model while keeping persistence and exit handling in
  the caller.

## Decision Outcome

Chosen: an interactive ratatui/crossterm provider setup flow. It uses
`https://openrouter.ai/api/v1` and `OPENROUTER_API_KEY` as OpenRouter defaults,
accepts custom OpenAI-compatible endpoints, and saves environment-backed
credentials as complete references. Trailing slashes are normalized before
classification and persistence.

Onboarding uses fixed provider names: `openrouter` for the normalized OpenRouter
endpoint and `custom` for every other endpoint. Re-running setup deliberately
replaces the selected fixed entry, including a collision with an existing
manually maintained entry of that name. It changes only that entry and the
default-provider field; unrelated providers, tiers, pricing, LiteLLM settings,
and other configuration are preserved. Existing arbitrary provider names
remain valid for manually maintained configuration.

Provider readiness is determined locally. A saved literal or exact
`${VARIABLE}` reference is authoritative. A missing saved reference is an
authentication error and does not fall through to another environment value.
Only an absent `api_key` permits provider-specific environment fallback followed
by generic `WATN_API_KEY`. The default setup stores `${OPENROUTER_API_KEY}` for
OpenRouter and `${WATN_API_KEY}` for custom endpoints; an explicitly entered
valid variable name is stored as that exact reference.

Automatic onboarding is allowed only for an implicit provider selection and a
TTY stdin. Explicit `--provider` and `WATN_PROVIDER` selections retain existing
unknown-provider and missing-key errors. An implicit non-TTY first-use request
prints actionable `watn provider` and config-path guidance, exits 1, and does
not initialize ratatui.

Provider setup returns a typed configured/cancelled result. Model setup returns
a typed saved/cancelled/failed result and never exits the process internally.
The caller saves a confirmed provider before model setup. Escape maps to status
1 and Ctrl-C to status 130. Model cancellation or failure preserves that
provider, stops onboarding, and sends no original request. Successful automatic
setup stops after model selection with status 0; it does not resume the
original request. The explicit `watn provider` command always ends after
provider configuration and never invokes model setup.

All config saves retain the existing direct-write mechanism and then enforce
Unix mode `0600`. Atomic temp-file/rename behavior is not part of this decision.

## E2E Transport Boundary

E2E tests may pass an ephemeral endpoint override at outbound HTTP construction
time. The override is test-harness state, not a config field. The only compiled
branch that reads it is guarded by
`cfg(all(feature = "test-support", debug_assertions))`. Therefore a
default-feature release binary and a release binary built with `test-support`
both use the configured endpoint.

URL builders remain pure and receive the already resolved endpoint. The
override is never consulted by configuration loading, readiness, persistence,
or endpoint display. Missing or whitespace values fall back to the configured
endpoint. Touched model requests and chat requests use separate loopback twins
and assert exact full URL, method/path, request count, Authorization header,
competing-server zero hits, response source, and unchanged persisted endpoint.

Transport verification builds the default-feature and `test-support` binaries
sequentially for the debug profile through Cargo's shared default target cache,
then copies them to unique temporary paths. Only those two absolute paths are
passed to the subprocess harness; scenarios do not discover a stale
`target/debug/watn` or build during execution. Release-profile runtime
verification inspects the exact release artifact and its target runtime
libraries.

## Consequences

- **Good:** first-run users get a guided endpoint and credential workflow.
- **Good:** environment-backed credentials keep the secret out of the
  persisted representation and are expanded only at use time.
- **Good:** saved provider entries, including OpenRouter, are honored instead
  of being discarded in favor of a built-in endpoint.
- **Good:** fixed names make the setup result predictable and preserve all
  unrelated configuration on replacement.
- **Good:** the existing model picker remains the single model-selection path.
- **Good:** typed setup results make cancellation, partial onboarding, and exit
  status behavior testable without process termination inside lower layers.
- **Good:** the single binary and OpenAI-compatible provider boundary are
  preserved.
- **Bad:** interactive first-use setup depends on a usable TTY and a reachable
  model catalog; non-interactive users receive guidance and must configure or
  rerun from a terminal.
- **Bad:** explicit CLI and environment provider selections do not receive
  automatic onboarding, so their existing unknown-provider and missing-key
  errors remain a first-use obstacle by design.
- **Bad:** successful automatic setup does not resume the original request;
  the user must run it again.
- **Bad:** model cancellation or catalog failure can leave a provider saved
  without model tiers, requiring a repeatable model setup command.
- **Bad:** the literal credential option intentionally leaves a secret on disk,
  so mode enforcement and warnings remain important.
- **Bad:** fixed `openrouter`/`custom` names can overwrite an existing entry of
  the same name when setup is rerun; unrelated entries are preserved but the
  collision is intentional.
- **Bad:** direct writes do not provide an atomic temp-file/rename guarantee,
  so an interrupted write can leave incomplete configuration.
- **Bad:** E2E coverage needs a test-only HTTP construction seam, two sequential
  debug builds with copied binary paths, per-child accounting, and exact twin
  assertions; the extra setup is deliberate because a broad mock or stale
  binary would create false-green transport tests. Release-profile runtime
  proof is recorded by the release artifact inspection.

## Confirmation

Gherkin scenarios cover the three transport E2E inventory entries plus validation,
credential precedence, saved OpenRouter endpoint precedence, fixed-name
replacement, TTY gating, cancellation statuses, model failure preservation,
direct-write permissions, and the explicit command boundary in
`givn/changes/isolate-test-transport/specs/transport/transport.feature` and the
permanent provider-setup specifications.
