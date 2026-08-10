# watn Improvement Plan

The review baseline was a clean `main` worktree. The report maps into four
implementation changes, completed in this order:

1. `isolate-test-transport`
2. `model-discovery-and-setup-correctness`
3. `incremental-sse-rendering`
4. `release-truth-and-repository-cleanup`

Each change must use the givn workflow: propose, spec, design,
design-review, tasks, implement, review, and archive. One active change is
kept at a time. Each completed scenario receives one atomic commit, and its
commit hash is recorded in `tasks.md`.

## Baseline Contracts

- Commands are written to stdout.
- Metadata, prompts, spinners, and diagnostics are written to stderr.
- LiteLLM is used for model discovery only, never for chat completions.
- `watn models` changes model tiers without replacing the active provider.
- `watn setup` and implicit first-use setup save a valid provider before the
  first model catalog request.
- Absent thinking-tier reasoning retains the existing `high` default.
- Empty and unknown persisted reasoning values disable reasoning.
- The endpoint test seam is not compiled into normal or release builds.
- The current dynamically linked release binary is documented truthfully unless
  static release artifacts become an explicit product requirement.

## Change 1: Isolate Test Transport

### Production

- Remove the unconditional `WATN_TEST_ENDPOINT_OVERRIDE` lookup from normal
  builds.
- Prefer explicit endpoint injection through provider and model request paths.
- If PTY subprocess tests need implicit OpenRouter routing, put the override
  behind a `test-support` Cargo feature.
- Keep URL builders pure. They must not consult process-global environment
  state.
- Keep readiness checks, persisted endpoints, and configuration serialization
  independent of test transport.

### Tests

- Enable `test-support` only for tests that need it.
- Prove that a default-feature build ignores `WATN_TEST_ENDPOINT_OVERRIDE`.
- Assert exact request URLs instead of only mock hit counts.
- Assert actual Authorization headers.
- Preserve assertions that the saved OpenRouter endpoint remains
  `https://openrouter.ai/api/v1` while test requests use loopback.
- Make stale-search tests genuinely concurrent with deterministic delays or
  barriers.
- Remove write-only search delay state after the new test is in place.

### Completion

- A normal release binary cannot be redirected by the test override.
- PTY tests still have deterministic loopback routing.
- LiteLLM URL tests fail when the wrong endpoint is used.
- Late search results cannot replace newer results.

## Change 2: Model Discovery and Setup Correctness

### Credentials

- Preserve credential source information, not only resolved secrets.
- Seed the TTY wizard with an environment-backed source when the built-in
  OpenRouter provider has no configured key but `OPENROUTER_API_KEY` exists.
- Use the environment value for discovery without displaying or persisting the
  secret.
- Persist `${OPENROUTER_API_KEY}` when that source is confirmed.
- Preserve literal credentials and explicit environment references as
  authoritative.

### LiteLLM

- Use `[litellm]` endpoint and optional credential for `/models` and search
  requests when configured.
- Otherwise use the selected provider endpoint and credential.
- Never use LiteLLM for `/chat/completions`.
- Expand LiteLLM environment references at request time.
- Allow unauthenticated LiteLLM catalog requests when no key is configured.
- Keep the catalog source separate from the active provider draft.

### Partial Saves

- `watn provider` saves after the API-key page is confirmed.
- `watn setup` and implicit onboarding save the provider before catalog
  discovery.
- Catalog failure preserves the provider and leaves tiers unchanged.
- Cancellation before credential confirmation writes nothing.
- Cancellation after provider confirmation preserves the provider.
- `watn models` updates tiers without replacing provider configuration with the
  discovery endpoint.
- The real unified wizard must drive the catalog-failure scenario; tests must
  not call `save_provider_draft()` directly to simulate the flow.

### Reasoning

- Recognize `off`, `low`, `minimal`, `medium`, and `high` only.
- Resolve empty and unknown persisted values to no reasoning.
- Use `default_enabled` when selecting a model's initial reasoning state.
- Mandatory models must never expose `off`.
- Ignore unknown supported efforts.
- Use valid `default_effort`, then the first valid supported effort.
- Remove non-TTY writes of empty reasoning strings.
- Preserve existing reasoning settings when no valid model default exists.

### Required Scenarios

- TTY `watn models` with implicit OpenRouter and `OPENROUTER_API_KEY`.
- Exact LiteLLM discovery URL, with and without a key.
- Non-TTY selection never persists empty reasoning.
- Disabled, mandatory, minimal, and unknown reasoning behavior.
- Real unified-wizard catalog failure preserving the provider.
- Catalog failure sends no chat request.
- Model-only setup preserves provider and LiteLLM configuration.

## Change 3: Incremental SSE Rendering

### Provider

- Replace `response.bytes()` buffering with incremental SSE parsing.
- Emit content and reasoning events as they arrive.
- Return final metadata and accumulated content after stream completion.
- Use a synchronous event sink for the single blocking consumer instead of a
  worker channel solely to match stale documentation.
- Extract usage and model fields from the top-level event even when `choices`
  is empty.
- Handle `[DONE]`, blank lines, partial reads, and malformed nonessential data.
- Measure elapsed time from the first stream event.

### Output

- Stop and clear the spinner when the first content token arrives.
- Flush content to stdout immediately.
- Accumulate content for `-x`, verbose reasoning, and final metrics.
- Do not print streamed content a second time after completion.
- Preserve stdout/stderr separation.
- Finish the spinner and report an error on mid-stream failure.

### Tests

- Use a deterministic loopback server that flushes one event, waits, and then
  sends the final event.
- Assert that the first token is visible before response completion.
- Test usage-only final events, reasoning events, `[DONE]`, malformed lines,
  and partial reads.
- Test `-x` after complete command receipt.
- Test spinner cleanup on success, failure, and drop.
- Add a raw-TTY confirmation scenario separate from piped stdin.

## Change 4: Release Truth and Repository Cleanup

### Version

- Derive CLI version from Cargo package metadata.
- Assert the actual package version in the version scenario.
- Do not bump the package version unless releasing a new version.

### Deployment

- Correct deployment documentation to describe the current dynamically linked
  binary and target-dependent runtime libraries.
- Add `file` and `ldd` release verification.
- Do not claim static artifacts until musl/TLS/compression dependencies are
  verified. Static artifacts, if required, are a separate release-engineering
  change.

### Documentation

- Update Arc42 context, strategy, building blocks, runtime, deployment,
  cross-cutting concepts, and risks to match the implementation.
- Remove dialoguer, nonexistent table, deleted PTY helper, plain-`r`, missing
  XDG data directory, and incorrect output-stream claims.
- Document LiteLLM discovery-only behavior, test-support isolation, actual
  reasoning strengths, and partial-save behavior.
- Mark archived Arc42 files as historical and non-normative.
- Remove conflicting hardcoded README coverage values and document one
  authoritative coverage command.

### Dead Code and Hygiene

- Keep `LiteLLMConfig` and `ModelReasoning.default_enabled` once consumed.
- Remove provider setup wrappers only after confirming the crate is treated as
  a binary application with no library compatibility requirement.
- Remove the unused `build_registry` config parameter.
- Reassess whether a one-provider registry is justified.
- Remove write-only test-world state and obsolete helpers.
- Make the repository rustfmt-clean without mixing unrelated formatting noise
  into behavioral changes.

## Final Verification

```text
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
cargo test --test features_runner --features test-support -- --tags 'not @wip and not @e2e'
cargo test --test features_runner --features test-support -- --tags '@e2e and not @wip'
cargo build --release
git diff --check
```

Regenerate coverage after all regression scenarios are complete. Do not report
branch coverage from a stable-toolchain run that emits a `0/0` denominator.
