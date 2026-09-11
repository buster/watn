# Design: quicksetup-parameters

## Use-Case Traceability

| Contract (usecase.md) | Design decision | Evidence |
|---|---|---|
| Quick setup persists a working configuration | `--url`, `--key`, `--model`, and tier flags prefill the existing questions; saving and shell installation are unchanged | Parameter prefill scenarios, existing quick-setup scenarios |
| Cancellation leaves no unconfirmed state | Prefills are in-memory suggestions only; the write still happens at the end of the dialog | Existing Ctrl-C scenario |
| Active inputs remain visible | The dialog still runs with the prefilled suggestion in brackets | `endpoint question should suggest` |

## CLI Surface

`Commands::Quicksetup` becomes a struct variant with optional parameters:

```rust
Quicksetup {
    #[arg(long = "url", value_name = "URL", help = "Prefill the completion endpoint")]
    url: Option<String>,
    #[arg(long = "key", value_name = "KEY", help = "Prefill the credential, literal or ${ENV_VAR}")]
    key: Option<String>,
    #[arg(long = "model", value_name = "MODEL", help = "Prefill the small, normal, and thinking models")]
    model: Option<String>,
    #[arg(long = "model-small", value_name = "MODEL", help = "Prefill the small model")]
    model_small: Option<String>,
    #[arg(long = "model-normal", value_name = "MODEL", help = "Prefill the normal model")]
    model_normal: Option<String>,
    #[arg(long = "model-thinking", value_name = "MODEL", help = "Prefill the thinking model")]
    model_thinking: Option<String>,
}
```

`run_quicksetup_command` builds `quicksetup::QuickSetupDefaults` and calls
`watn::quicksetup::run_with_defaults(defaults)`. The implicit first-run path
keeps calling `run()` as a defaults-empty wrapper, so it is unchanged.

## Suggestion Resolution

`QuickSetupDefaults` carries the six optional strings. `run_with_defaults`:

- endpoint suggestion: `defaults.url` else `OPENROUTER_ENDPOINT`; the existing
  `normalize_endpoint` validation and re-ask loop are unchanged.
- credential suggestion: `defaults.key` else the existing
  `${SUGGESTED_ENV}` detection; a `${VAR}` value is passed verbatim to
  `build_provider_draft`, which already stores it as an environment-backed
  credential.
- small suggestion: `defaults.model_small` else `defaults.model` else the
  OpenRouter haiku suggestion (only when the endpoint resolver says
  OpenRouter).
- normal suggestion: `defaults.model_normal` else `defaults.model` else the
  accepted small answer.
- thinking suggestion: `defaults.model_thinking` else `defaults.model` else the
  accepted small answer.
- Empty flag values (`--key ""`, `--model-small ""`) normalize to `None` before
  the dialog, so they behave as if omitted.

Every prompt still asks; an empty answer accepts the prefilled suggestion, and
the shell question always runs. `--model` therefore seeds all three tiers while
a tier flag wins over it.

## README

The Quick setup section gains the install command, both example invocations
(single-quoted environment reference and plain key with `--model-small`), and
the `watn "find the 5 largest files in the commit history"` test query. The
examples are the exact commands exercised by the e2e scenarios.

## Interfaces And Step Migration

- `src/main.rs`: `Commands::Quicksetup` variant and `run_quicksetup_command`
  wiring.
- `src/quicksetup.rs`: `QuickSetupDefaults`, `run_with_defaults`, suggestion
  plumbing through `ask_endpoint`, `ask_credential`, and the three model asks.
- `tests/steps/quicksetup_steps.rs`: new step
  `I start \`watn quicksetup\` with these parameters:` (docstring of
  `--flag value` lines) and `I run \`watn quicksetup --help\``.
- Delta feature:
  `givn/changes/quicksetup-parameters/specs/configure-interactive/quicksetup.feature`.
- Permanent scenario bodies are not modified; the delta only adds scenarios.

## Test Runner

- Unit/integration: `./run-tests.sh`
- E2E: `./run-tests.sh --e2e`
- Single scenario:
  `./run-tests.sh --e2e --name 'Quick setup parameters prefill the dialog with an environment credential'`
- Strict mode: `tests/features_runner.rs:208` calls `.fail_on_skipped()`;
  not-yet-implemented steps use `unimplemented!()`.
- Formatting and lints: `cargo fmt --all -- --check` and
  `cargo clippy --locked --all-targets -- -D warnings`.
- Toolchain: `rust-toolchain.toml` pins `1.97.1`; no new dependencies.
- `@wip` scenarios are excluded from both suites until their steps exist; each
  task clears `@wip` with its RED/GREEN commit.

## Interaction Coverage Matrix

| Inventory entry (configure-interactive) | @e2e scenario | Real interface | Driving mechanism |
|---|---|---|---|
| start quick setup on first run | First run without a configuration starts the quick setup | watn CLI in a PTY | PTY first-run trigger |
| persist quick setup answers | Quick setup stores answers and installs integrations | watn CLI in a PTY | PTY answers and shell confirmation |
| overwrite explicit quick setup | Explicit quick setup overwrites an existing configuration | watn CLI in a PTY | PTY answers and shell confirmation |
| abort first-run quick setup | Aborting quick setup with Ctrl-C on the first run leaves no configuration | watn CLI in a PTY | PTY Ctrl-C before confirm |
| prefill quick setup with parameters | Quick setup parameters prefill the dialog with an environment credential | watn CLI in a PTY | PTY: start with flags, accept suggestions, inspect config |
| seed remaining tiers from a small-model parameter | A small-model parameter seeds the remaining tiers | watn CLI in a PTY | PTY: start with `--model-small`, accept suggestions, inspect config |
| override tiers with parameters | Tier parameters override the shared model prefill | watn CLI in a PTY | PTY: start with `--model` plus tier flags, accept suggestions, inspect config |

The help scenario is a regular subprocess assertion; it does not add a consumer
action beyond running `watn quicksetup --help`.

## Failure Outcomes

| Condition | Outcome |
|---|---|
| Invalid `--url` | The dialog shows it as the suggestion; pressing Enter reports the existing endpoint error and re-asks |
| Missing flag value | Clap rejects the missing value with a non-zero argument error |
| Empty flag value string | Treated as omitted |
| No flags | Existing quick setup behavior |
| `${VAR}` key | Stored as an environment-backed credential |

## ADR Qualification And Routing

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "FAIL",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "NO",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/quicksetup-parameters/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [],
    "architectural_impact": [],
    "durable_consequence": [],
    "lower_level_artifact": [
      "Optional CLI parameters that only prefill an existing dialog are owned by this design and the Gherkin scenarios."
    ],
    "existing_adr_check": [
      "ADR-0026 owns the plain-line quick-setup decision; prefills do not change that boundary."
    ]
  }
}
```

## Architecture Impact

- Chapter 03 context-and-scope: the User input line names the quick-setup
  parameters.
- Chapter 04 solution-strategy: the quick-setup strategy records parameter
  prefills.
- Chapter 05 building-block-view: the `quicksetup` module row records parameter
  prefills and the help surface.
- Chapter 06 runtime-view: the first-run quick-setup scenario mentions
  prefilled parameter suggestions.
- Chapter 12 glossary: the Quick setup definition mentions optional parameter
  prefills.
- Chapter 11 risks-and-technical-debt: R-011 records that a literal `--key`
  argument is visible in shell history and process listings, with the
  environment reference as the documented preference.
- Chapters 01, 02, 07, 08, 09, 10: unchanged.

## Verification Contract

The existing Cucumber runner (`./run-tests.sh` and `./run-tests.sh --e2e`)
remains the executable specification; the delta `.feature` file is the
authoritative test surface.
