# Design: Setup Refactoring

## Domain Model

### Ubiquitous language

| Term | Meaning |
|---|---|
| Persisted config | The supported TOML configuration read from the configured path before setup starts. |
| First run | The physical config path does not exist. A legacy comment-only template is still an existing config file. |
| Setup draft | The complete in-memory provider, model-role, reasoning, and shell-intent state edited by one wizard session. |
| Field origin | `Loaded from config`, `Detected from environment`, `Recommended default`, or `Entered by you`. |
| Credential source | A literal credential, an environment-variable reference, or a missing suggested variable. |
| Catalog status | The draft provider's model-discovery result: available, unavailable, incomplete, or stale after a provider change. |
| Model role | One saved tier: Small / fast, Balanced / normal, or Thinking. |
| Shell intent | The desired installed state for a completion or shortcut marker block in a supported shell startup file. |
| Finish | The only configuration persistence boundary in a wizard session. |

### Invariants

- Only physical config-path absence starts first-run onboarding.
- Reading a missing config path never creates a directory, template, or file.
- A detected credential is represented only by its variable name and non-empty
  presence. Its resolved secret never enters renderer state, diagnostics, or
  persisted discovery data.
- Existing persisted provider, credential source, model roles, and reasoning are
  authoritative when a config exists. Discovery must not silently replace them.
- A draft provider change invalidates model-role review without overwriting a
  value the user entered or explicitly selected.
- A role with manually entered or metadata-unknown model information persists
  `off` reasoning.
- Finish writes supported TOML state at most once. Escape, Ctrl-C, validation
  failure, and a pre-Finish catalog failure make no durable config change.
- A shell integration is represented by its marker block in a startup file, not
  by a TOML field. Unrelated startup-file content remains unchanged.
- A partial shell failure never makes watn claim that all setup operations
  completed; configuration may still be committed.

## Technical Decisions

| Concern | Decision | Rationale |
|---|---|---|
| Config loading | Split read-only config loading from explicit config commit. The read result includes `exists`. | First-run detection must precede parsing side effects and template generation. |
| Config saving | Use one secure atomic writer at Finish: restrictive temporary file permissions, flush/sync, atomic rename, then preserve or tighten Unix mode to `0600`. | Finish-only persistence must not expose a literal credential or leave partial TOML after interruption. |
| Wizard state | Use one `SetupDraft` with typed field origins, credential source, catalog status, model-role state, and shell intent. | Presentation and validation need more information than the persisted TOML schema contains. |
| Provider choices | Make OpenRouter, OpenAI, and Custom explicit choices. | The UI must make the documented endpoint and credential mappings understandable. |
| Discovery | Read a finite allowlist by name; use only variable names and presence flags. Permit deliberate manual variable names after validation. | Prevent arbitrary environment scanning and secret disclosure while retaining flexible custom setups. |
| Catalog source | Preserve configured catalog-source precedence while resolving provider-backed catalog operations from the draft. A provider edit marks all roles for review. | Existing LiteLLM behavior remains supported without allowing a stale provider/model pairing to pass Finish silently. |
| Manual model fallback | Permit all manually entered required roles after any catalog failure, surface an unverified warning in Review, and use `Reasoning: off`. | Setup remains usable for compatible providers without a reliable catalog, without claiming verification that did not occur. |
| Shell state | Derive checkboxes from existing completion and shortcut marker blocks in Bash, Zsh, and Fish startup files. | The installed files are the existing source of truth and avoid false TOML state after a failed install. |
| CLI surface | Keep `watn setup`, `watn completions`, `-1`, `-2`, and `-3`; remove focused setup commands and provider/model overrides. | Configuration has one review and persistence boundary; request-tier selection remains a runtime behavior. |
| Verification | Add active-change `@wip` Gherkin scenarios first, then remove `@wip` one scenario at a time with strict step bindings. | The Cucumber runner rejects skipped steps, while the change remains handoff-only until implementation begins. |

## Architecture Impact

### CLI and first-use dispatch

`src/main.rs` retains CLI parsing, request dispatch, and exit mapping.

- Remove `Commands::Provider` and `Commands::Models`.
- Remove `Cli.provider`, `Cli.model`, `Cli.set_small`, `Cli.set_normal`, and
  `Cli.set_thinking` along with their generated completion entries.
- Keep `Commands::Setup`, `Commands::Completions`, and the three tier flags.
- Do not apply `WATN_PROVIDER` or `WATN_MODEL` as runtime configuration
  overlays. Credential variables remain available only through normal
  credential-source resolution and setup discovery.
- Before normal request readiness is evaluated, obtain the read-only config
  result. If its path is absent, interactive stdin opens the full wizard and
  non-interactive stdin prints guidance to stderr and exits 1.
- With an existing config, retain the normal request path when it is ready;
  incomplete existing configuration opens the repair wizard only on an
  interactive terminal.
- A successful implicit onboarding writes `Setup complete. Retry your command.`
  to stderr, returns 0, and returns before a chat provider or original request
  is constructed.

### Configuration read, draft, and commit

`src/config/` gains a read-only boundary such as:

```text
PersistedConfig {
    config: Config,
    exists: bool,
}
```

The path check happens before parsing. An absent file returns default runtime
configuration with `exists: false`; it does not call the current template
writer. Existing configs parse normally and are cloned into a `SetupDraft`.
The draft retains only supported semantic config fields. Saving may normalize
or remove unknown TOML, comments, formatting, and key order, as explicitly
accepted by this change.

`SetupDraft` owns runtime-only values:

```text
SetupDraft {
    config: Config,
    provider: ProviderDraft,
    roles: [RoleDraft; 3],
    shell_intent: ShellIntent,
    first_run: bool,
}

ProviderDraft {
    identity: OpenRouter | OpenAi | Custom,
    endpoint: DraftValue<String>,
    credential: CredentialDraft,
    catalog_status: CatalogStatus,
}

DraftValue<T> {
    value: T,
    origin: LoadedConfig | DetectedEnvironment | RecommendedDefault | UserEntered,
}
```

`CredentialDraft` distinguishes a masked literal, an environment-variable
name, and a recommended but absent variable. Environment discovery returns
names and presence only. Environment references are validated as variable
names and persist as `${NAME}`; resolution happens only when an authenticated
catalog or chat request is made. Secret-bearing types must redact `Debug` and
`Display` output, or avoid those implementations entirely.

On Finish, validation produces one supported-config overlay. The writer merges
the provider, all three role assignments, and their reasoning into a clone of
the loaded config, then commits it once. This replaces the current provider
checkpoint and separate tier write. Literal credentials reach the filesystem
only through the secure writer. First-run cancellation leaves the path absent;
existing-config cancellation leaves the original bytes unchanged.

### Provider discovery and validation

Provider rendering and pure validation remain in provider-oriented modules,
but no provider module writes a config or owns a terminal loop.

The Provider topic offers these identities:

| Identity | Endpoint choice | Credential suggestions |
|---|---|---|
| OpenRouter | Built-in OpenRouter endpoint | `OPENROUTER_API_KEY`, then the generic `WATN_API_KEY` suggestion where applicable. |
| OpenAI | Built-in OpenAI endpoint | `WATN_OPENAI_API_KEY` and `OPENAI_API_KEY` when OpenAI is selected. |
| Custom | User-entered HTTP(S) endpoint | `WATN_API_KEY`; provider-specific names only after identity and endpoint are known. |

Multiple detected variables are displayed as explicit choices in deterministic
allowlist order. A detected provider-specific credential is never carried to a
different endpoint without explicit confirmation. Changing the endpoint may
recalculate an untouched automatic suggestion, but cannot replace a user-edited
or explicitly selected credential source.

Endpoint syntax and credential-source syntax are required. A catalog failure,
including an authentication, transport, timeout, or malformed-catalog failure,
does not by itself block Finish when all model roles are entered manually. The
Review page must identify that state as unverified rather than marking the
provider complete.

### Four-topic wizard

Replace the current `src/setup.rs` with a cohesive `src/setup/` module tree
behind the existing public setup facade:

```text
setup/
  draft.rs       # typed draft, origins, validation, invalidation
  discovery.rs   # allowlisted environment suggestions without secret values
  model_roles.rs # suggestions, picker overlay, role/reasoning state
  shell.rs       # marker-block detection and Finish reconciliation intent
  render.rs      # Ratatui topics, progress rail, contextual help
  mod.rs         # public run and outcome boundary
```

Exact file splits may vary, but renderer, mutable draft state, discovery, and
external side effects must not remain entangled in one page enum.

The topic rail is fixed:

```text
Provider -> Model roles -> Shell integration -> Review
```

`watn setup` always starts on Provider. Tab and Shift-Tab move among controls
and cross topic boundaries. Completed topics remain revisitable. Changing the
Provider topic changes Model roles to `Needs attention` until each retained or
new assignment is explicitly reviewed. Escape opens a leave/discard prompt;
Back never discards the draft; Ctrl-C exits with the established interrupt
status and never saves.

On wide terminals, render settings and a persistent contextual-help pane in two
columns. The renderer uses two columns only when both panes can meet their
minimum content width; otherwise it stacks help below settings. The same four
help sections always appear for the active control:

1. What it is
2. What it enables
3. Recommendation
4. Tradeoff or requirement

Provider displays a first-run discovery banner when `first_run` is true.
Model roles display all three assignments at once. Entering a model field opens
a searchable picker overlay with catalog metadata. The main wizard page does
not become a permanent full-screen browser. Suggested values are visibly marked
as suggestions; no reliable candidate renders `Needs selection` instead of a
hard-coded identifier.

Review summarizes endpoint, provider identity, credential storage without a
resolved secret, role IDs, reasoning, catalog status, shell changes, and every
warning. Finish is disabled for invalid provider input, missing credential
source, missing required model roles, or roles still requiring review. Catalog
warnings do not disable Finish once manual required roles are present.

### Shell integration and partial outcomes

`src/shell_completion.rs` and `src/shell_shortcut.rs` keep their marker-block
installers and injectable `ShellEnvironment`. The setup draft reads markers in
Bash, Zsh, and Fish startup files to derive initial checkboxes. Completion and
shortcut remain independent selections.

At Finish, after the config commit:

1. Install a checked missing marker block.
2. Remove an unchecked existing marker block.
3. Preserve every non-watn byte and the other integration's block.
4. Collect per-shell results without rolling back successfully changed files.

The wizard returns either a complete success or a `SavedWithShellFailures`
outcome. The latter prints which config was saved, reports each failed shell
operation with retry guidance, and exits nonzero. It never prints the automatic
onboarding retry-success message. A startup file that cannot be safely read or
reconciled is `Needs attention`, not silently interpreted as unchecked.

## Data Model and Compatibility

No persisted schema is added or changed. Existing provider entries, literal
credentials, `${VARIABLE}` references, tier names, reasoning strings, pricing,
and LiteLLM configuration remain supported semantic fields. The serializer is
not a document-preserving TOML editor; unknown values and comments are outside
the compatibility guarantee.

Draft validation writes `off` for every manual or metadata-unknown role,
including Thinking. That explicit persisted value prevents the existing
missing-value fallback from turning a manual Thinking role into `high`. Existing
saved explicit reasoning continues to be loaded and displayed until a
provider/model edit makes it require review.

The existing catalog-source precedence remains in effect for configured
LiteLLM. Catalog requests using the selected provider use the in-memory draft
endpoint and credential source; a provider change still invalidates model-role
review even if the catalog source is independent.

## Test Seams and Specification Migration

The Gherkin delta lives at:

```text
givn/changes/setup-refactoring/specs/setup-refactoring/setup-refactoring.feature
```

Its scenarios remain `@wip` until their strict bindings and implementation are
introduced. The normal runner filters `not @wip and not @e2e`; the E2E runner
filters `@e2e and not @wip`. Remove `@wip` one scenario at a time only after
the matching real interface proof is green.

Use these test seams:

| Concern | Primary proof | Supporting seam |
|---|---|---|
| Config path and Finish boundary | Isolated XDG directory and real CLI | Read/commit unit tests with byte assertions and Unix modes. |
| Credential discovery | Pure presence-only environment lookup | Values must not appear in draft debug, output, or TOML. |
| Topic layout/navigation | Ratatui buffer tests and `portable-pty` | Wide/narrow dimensions and visible contextual-help assertions. |
| Catalog failure/manual roles | Loopback `httpmock` and PTY | Exact catalog request, unverified review warning, and `off` reasoning. |
| Implicit onboarding | Real CLI subprocess/PTY | stderr-only success guidance, exit status, and zero original chat requests. |
| Shell reconciliation | Isolated `ShellEnvironment` home | Marker removal/install, user-content preservation, and partial failure ordering. |
| CLI removals | Real CLI help/parser invocation | Removed commands/flags fail and generated completions omit them. |

The implementation must replace, not duplicate, incompatible permanent
specifications:

- `givn/specs/auto-init-config/auto-init-config.feature`
- `givn/specs/provider-setup/provider-setup.feature`
- `givn/specs/setup-persistence/setup-persistence.feature`
- `givn/specs/unified-setup-wizard/unified-setup-wizard.feature`
- standalone `watn models` coverage in `givn/specs/models/`,
  `givn/specs/model-autosuggest/`, and related picker/catalog specifications
- provider/model override and completion expectations in config, provider, ask,
  and shell-completion specifications

Existing test modules may be reused where their observable boundary remains
valid, but a dedicated setup-refactoring step module is preferable to adding
new wizard behavior to standalone-model step files. Every new Gherkin step must
be globally unique because the Cucumber registry is shared.

## Implementation Order

1. Introduce the read-only config result and secure one-shot writer, then prove
   path absence, cancellation, and existing-config preservation.
2. Remove the focused CLI commands and provider/model overrides while retaining
   tier selectors and credential resolution.
3. Build the typed draft and pure allowlisted discovery, including provenance
   labels and provider-change invalidation.
4. Refactor the wizard into the Provider and Model roles topics with the
   responsive help renderer and catalog/manual fallback states.
5. Add Shell integration detection/reconciliation and Review/Finish outcomes.
6. Migrate permanent specifications, documentation, and ADRs that encode the
   replaced page, persistence, command, and first-run contracts.
