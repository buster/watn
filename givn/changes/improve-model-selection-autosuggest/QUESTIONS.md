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

### Resolution

The archive verify gate can only pass with a fully green suite, and the
current change is unrelated to those failures. Assumption made (per the
working loop: don't ask, decide, implement until archived): **the gate is
satisfied by repairing the pre-existing failures in the working tree as
part of completing this change.** The repairs touch no new production
behavior in this change's scope; they fix stale tests/specs and a shared
test-harness defect. Both verify gates are now green (full: 48/48, e2e:
27/27).

What was fixed and why (production code was NOT changed for any of these):

1. **Stale output-format assertions** (ask #1/#3/#4, config #6, reasoning
   #9). The binary switched to the single-line metadata format
   `{model} · {n} tok/s [· ${cost}] · {secs}s` in an earlier archived
   change, but the step definitions and an in-spec regex were never
   reconciled. Updated:
   - `tests/steps/ask_steps.rs` — "output should contain a model name",
     "tokens/second value", "cost value", "cost estimate" assertions now
     match the intended format instead of the stale `model:` / `tokens/s:`
     / `cost:` labels.
   - `givn/specs/ask/ask.feature` — the "Tokens/second" scenario regex
     changed from `tokens/s:\s+\d+\.?\d*` to `\d+\s*tok/s`.

2. **"Execute flag with n" (ask #2)**. `command_not_executed` expected the
   raw stdout to be a single line, but the binary blank-line-wraps the
   command suggestion. Updated the step to trim blank lines before the
   single-line assertion, preserving the "only the suggestion, not
   executed" intent.

3. **Shared harness defect in `tests/steps/mod.rs::ensure_test_env`**
   (config #5/#7, models #8). Scenarios that set `raw_config` without a
   mock server never wrote their config file (config_content stayed empty),
   so the binary silently used defaults instead of the configured content.
   - Fixed: the raw config is now written verbatim when no mock server
     exists. This makes "config syntax error" (exit 1 + parse error) and
     "model explorer without endpoint" (exit 0 + manual-config
     instructions) behave as specified.
   - "Environment variable overrides config file" now injects a
     `[providers.<WATN_PROVIDER>]` section pointing at the mock (with a
     default_model), so the env-override actually reaches the chat mock.

No prompts remain. This change is ready to archive; the follow-up of
keeping permanent specs for ask/config/reasoning/models in sync with the
output format is noted as maintenance debt, not a blocker.

