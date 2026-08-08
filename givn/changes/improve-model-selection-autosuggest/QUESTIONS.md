# QUESTIONS

## Blocking issue: archive verify gate red due to pre-existing failures in unrelated features

The change `improve-model-selection-autosuggest` is fully implemented (all
scenarios GREEN, interactive search picker wired into `watn models`, PTY e2e
harness). `givn lint` is clean.

However, `givn archive` (and `givn check tasks`) run the full-suite verify
hook (`cargo test --test features_runner -- --tags 'not @wip'`), which is red:
**9 failures, all in already-archived features (ask, config, reasoning,
models) — none caused by this change.**

### Exact failing scenarios (full `not @wip` run)

| # | Feature | Scenario | Failing assertion | Root cause |
|---|---|---|---|---|
| 1 | ask | Ask with default tier returns a copy-pasteable command | output should contain a model name | Assertion expects `model:` label; binary outputs compact `{model} · {n} tok/s · …` (see `src/output/render.rs`) |
| 2 | ask | Execute flag with "n" answer skips execution | command should not have been executed | Step expects a single output line; binary emits blank-line-wrapped command |
| 3 | ask | Cost is displayed when pricing is configured | output should contain a cost value | Assertion expects `cost: $`; binary outputs `$0.0002` |
| 4 | ask | Tokens/second is displayed after response completes | regex `tokens/s:\s+\d+\.?\d*` | Spec regex matches old `tokens/s:` label; binary outputs `· 14152 tok/s` |
| 5 | config | Environment variable overrides config file | request should be sent to provider "custom" | Chat mock not hit; provider resolution/env setup issue |
| 6 | config | Model pricing configured for cost display | output should contain a cost estimate | Assertion expects `cost:` label; binary outputs `$…` |
| 7 | config | Config file with syntax error produces diagnostic | exit status should be 1 | Binary returns 0 (config parse error appears not surfaced; `run_models`/ask path) |
| 8 | models | Model explorer without LiteLLM endpoint configured | exit status should be 0 | Binary exits 1 after resolving a real provider and fetching live models |
| 9 | reasoning | Verbose flag with default tier does not alter existing model behavior | output should contain a model name | Same as #1 |

### Working assumption / decision made

These failures predate and are unrelated to `improve-model-selection-autosuggest`
(the change touches `src/models/*` and model-autosuggest specs/steps only; the
failing features are ask/config/reasoning/models-explorer with stale
output-format assertions and environment/network-dependent behavior). Fixing
them correctly requires editing permanent specs and production behavior in
already-archived features, which is a separate givn change, not this one.

Therefore the final verification tasks (verify.command / verify.e2e_command →
zero exit) are intentionally left **unchecked**, and the change is **not
archived**, because doing so would require either editing out-of-scope
permanent specs or falsely reporting a green gate.

### Open question for the maintainer

Should the stale output-format assertions (ask/config/reasoning) and the
config-error/network behavior be fixed as a separate change before
archiving this one, or is the archive verify gate expected to be satisfied
by fixing them here?
