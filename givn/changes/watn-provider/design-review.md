# Design Review: watn-provider

## Review Basis

The review cross-checked `proposal.md`, the change feature, `design.md`,
`arc42.md`, all twelve durable arc42 chapters, ADR-0011, the existing provider,
config, model, CLI, and HTTP source modules, and the feature runner and shared
step fixtures. The proposal scope remains unchanged. No `tasks.md` was created
and no implementation source was edited.

## Ranked Findings

1. **Critical: automatic onboarding contradicted the existing CLI contract.**
   The original design resumed the request after onboarding, used ratatui for
   every missing-provider path, and did not distinguish explicit provider
   selection. Resolved with an explicit selection-source matrix: only implicit
   TTY first use may onboard; explicit `--provider` and `WATN_PROVIDER` retain
   unknown-provider and missing-key errors; implicit non-TTY first use prints
   guidance and exits 1; successful setup stops before the original request.
2. **Critical: model setup could terminate the process and lose branch state.**
   Resolved with typed provider/model setup results, caller-owned status mapping,
   provider persistence before model setup, Escape status 1, Ctrl-C status 130,
   and no original request after model cancellation or failure.
3. **High: the E2E endpoint seam could falsify persisted configuration and cover
   only model discovery or chat.** Resolved with an ephemeral override applied
   only at HTTP construction, never used by readiness or persistence, with
   separate assertions for `/models`, `/chat/completions`, and the exact saved
   OpenRouter endpoint.
4. **High: config safety promised more than the existing writer provides.**
   Resolved by requiring mode `0600` after every existing direct write and
   explicitly removing any atomic temp-file/rename promise.
5. **High: verification filters were not mutually consistent.** Resolved by
   excluding `@e2e` from regular verification and coverage, retaining the E2E
   filter, using `%p-%m.profraw`, and removing `--tags` from every named
   scenario command because the runner rejects `--name` combined with tags.
6. **High: E2E step registration was split into a namespace the runner does not
   register.** Resolved by specifying one global
   `tests/steps/provider_setup_steps.rs` capability module for regular and PTY
   bindings; tags filter scenarios, and no `tests/e2e_steps` namespace is used.
7. **High: the provider feature lacked negative, persistence, precedence, and
   no-request assertions.** Resolved with `@givn.added @wip` scenarios for
   validation, missing references, explicit variables, endpoint precedence and
   normalization, rerun preservation, cancellation, model failure, non-TTY
   guidance, secure mode repair, literal precedence, and explicit env selection.
8. **Medium: non-E2E provider scenarios could accidentally become piped-stdin
   ratatui tests.** Resolved by requiring the renderer-independent setup state
   machine/config seam for regular scenarios; only the two inventory scenarios
   use a PTY.
9. **Medium: generated provider names and rerun collisions were unspecified.**
   Resolved with fixed `openrouter` and `custom` names, intentional replacement
   of only the selected entry, and preservation of unrelated providers and
   configuration in design, ADR-0011, and arc42 risks.
10. **Medium: credential precedence could overwrite references or fall through
    after a missing reference.** Resolved with authoritative saved literals and
    exact `${VARIABLE}` references, use-time expansion, authentication errors
    for missing references, and provider-specific then generic fallback only
    when `api_key` is absent.
11. **Medium: arc42 contained stale terminal claims.** Resolved by removing the
    stale `console`/raw-picker model from chapter 8, defining stdin as the TTY
    source in chapter 12, and aligning chapters 1 through 12 with the hardened
    runtime and persistence behavior.

## Resolved Decisions

- Automatic onboarding is TTY-only and is allowed only for implicit provider
  selection. Explicit `--provider` and `WATN_PROVIDER` selection preserve the
  existing unknown-provider and missing-key errors. Implicit non-TTY first use
  prints actionable `watn provider` and config-path guidance, exits 1, and does
  not initialize ratatui.
- E2E uses an ephemeral test transport endpoint override at HTTP construction.
  The override is never persisted or used for readiness and covers both
  `/models` and `/chat/completions`; persisted OpenRouter configuration remains
  exactly `https://openrouter.ai/api/v1`.
- Every configuration save uses the existing direct-write mechanism and applies
  Unix mode `0600`. Atomic temp-file/rename behavior is not promised.
- Provider and model setup return typed results rather than exiting internally.
  A provider is saved before model setup. Model cancellation/failure preserves
  that provider, stops onboarding, and sends no original request. Escape is 1,
  Ctrl-C is 130. Successful automatic setup stops after model selection and
  does not resume the original request.
- Regular filters exclude `@e2e`; the E2E filter remains. Coverage uses the
  same filters and `coverage/profraw/%p-%m.profraw`. Named scenario commands
  use `--name` alone.
- All provider setup bindings, including PTY bindings, are globally registered
  from `tests/steps/provider_setup_steps.rs`; no separate E2E namespace exists.
- The change feature has exactly two `@e2e` scenarios, one per inventory entry.
  All other provider scenarios use the renderer-independent setup/config seam.
- The feature preserves the existing provider regression scenarios and adds a
  saved default model prerequisite to the saved custom-provider bypass.
- Onboarding uses fixed names `openrouter` and `custom`. A rerun intentionally
  replaces the selected fixed entry while preserving unrelated providers and
  configuration. This collision behavior is documented in design, ADR-0011,
  and chapter 11.
- Saved literal credentials and exact saved environment references are
  authoritative. Defaults are `${OPENROUTER_API_KEY}` for OpenRouter and
  `${WATN_API_KEY}` for custom endpoints. Only absent `api_key` permits
  provider-specific then generic `WATN_API_KEY` fallback; references expand at
  use time and missing references are authentication errors.

## Branch Sign-Off

### Scope

PASS. The proposal's endpoint, credential-source, persistence, and automatic
setup goals remain represented. The review narrows the triggering conditions and
completion behavior without adding a new product capability or editing the
proposal scope. Existing provider regression scenarios remain in place.

### Tech

PASS. Ratatui/crossterm remains the terminal renderer consistent with the model
settings dialog. Dialoguer is recorded as the rejected simpler alternative in
ADR-0011. Typed results and a renderer-independent setup/config seam avoid
process-exit coupling and keep regular scenarios deterministic. The HTTP seam
is construction-time only and requires no persisted schema field or dependency.

### Missing Scenarios

PASS. The delta feature now covers invalid endpoint, empty credential, missing
reference authentication/no request, saved OpenRouter endpoint precedence,
explicit variable names, trailing-slash normalization, rerun preservation,
Escape and Ctrl-C cancellation, model catalog failure, explicit command
termination before model setup, non-TTY guidance, world-readable-file repair,
literal/reference precedence, and explicit environment selection. The saved
custom-provider bypass includes a default model prerequisite.

### Testability

PASS. Regular provider scenarios drive the renderer-independent state machine,
config seam, readiness, credential resolver, and mocked transport. Then steps
assert exact endpoint strings, config representations, Unix modes, exit codes,
request counts/paths, no ratatui initialization, and preserved unrelated
configuration. No regular scenario relies on piped stdin to drive ratatui.

### E2E Fidelity

PASS. Exactly two change-feature scenarios retain `@e2e`, matching the two
inventory entries. Both use `portable-pty` against the real CLI. The first
covers explicit `watn provider` interaction and `/chat/completions`; the second
covers automatic provider-to-model interaction and `/models`, then asserts no
chat request is sent. The loopback transport is ephemeral and the persisted
OpenRouter endpoint is asserted exactly.

### Interaction Matrix

PASS. Every inventory entry has one non-empty matrix row in `design.md`:

| Inventory entry | Scenario | Driving mechanism |
|---|---|---|
| run `watn provider` and complete the interactive provider setup | Configure OpenRouter with an environment-backed credential | `portable-pty` plus a real CLI subprocess |
| run a normal `watn` command with no recognized provider and complete automatic provider and model setup | First normal use starts provider setup and then model setup | persistent `portable-pty` session across both dialogs |

### Risk

PASS. The primary implementation risk is partial onboarding state: provider
save, model setup, and original request resumption can be accidentally coupled.
The typed-result flow, save ordering, explicit no-request assertions, and
R-013/R-015 mitigations isolate those branches. R-016, R-017, and R-018 cover
fixed-name collision, direct-write interruption, and transport-seam leakage.

### Arc42

PASS. The twelve-row `arc42.md` assessment is preserved with `STATUS: DONE`.
All twelve chapter files exist and contain decision-specific content. Chapters
8 and 12 no longer claim a `console` raw picker or stdout-based TTY check.
Chapter 9 records the dialoguer alternative and fixed-name consequences.
Chapter 11 records every ADR-0011 bad consequence, including TTY/catalog
dependence, explicit-selection errors, no-resume behavior, partial onboarding,
literal secrets, fixed-name collisions, direct writes, and the E2E seam. No
ASCII-art diagrams were introduced.

## Validation

- `givn lint --change watn-provider`: structurally valid; exit 2 is expected
  because the 21 scenarios remain tagged `@wip` for the next implementation
  stage.
- `git diff --check`: passed.
- `givn check --change watn-provider arc42-docs`: passed.
- `givn check --change watn-provider design-review`: passed.

DESIGN-REVIEW: PASS
