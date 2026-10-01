# Tasks: omit-request-defaults

## Domain constraints (apply to every scenario task)

From `givn/changes/omit-request-defaults/proposal.md` and the `ask` capability
in `givn/specs/fragments/fragment.md`:

- Watn sends only the parameters a request explicitly requires. It never
  invents a sampling temperature or an output-token limit; when no explicit
  value is configured, neither parameter appears in the request and the
  provider's own default applies (`Provider default`, `docs/arc42/12-glossary.md`).
- The rule is shared by every provider-backed path: ask, review, and
  explanation.
- Reasoning effort behavior is unchanged: a non-`off` value is still sent, and
  `off` still omits the field.
- Usage, cost, and Amount behavior are unchanged.
- Removing the two `RequestOptions` fields is a source-breaking change to the
  published crate's Rust API; the implementing revision is marked breaking
  (`feat(ask)!:` with a `BREAKING CHANGE` footer) per design-review Q2.
- Persona lens: `terminal-developer--interactive` — the developer whose
  requests currently fail on models that reject a fixed temperature or a fixed
  output-token limit must be able to use those models.

Design references: `design.md` (field removal, body construction, mock
wiring), `design-review.md` (Q1 regular scenario, Q2 breaking removal),
`arc42.md` (chapters 05, 08, 11, 12).

## Setup task

- [x] **T1 — Prove strict mode before the scenario.**
  - Runner config is already in place: `verify.command` is `./run-tests.sh`
    (`givn/commands.yaml`), strict mode is `.fail_on_skipped()` at
    `tests/features_runner.rs:210` plus the skipped-count exit at `:227`.
  - Proof of strictness: run the new scenario by name while it is still
    `@wip` and has no step definition; `--name` bypasses the tag filter, so an
    undefined step must produce a non-zero exit.
  - Command: `./run-tests.sh --name "Ask succeeds against a provider that rejects generation parameters"`
  - Evidence: non-zero exit (status 1), undefined step:
    ```
     ✘  And a provider that rejects a request carrying a sampling temperature or an output-token limit
        Step failed:
        Defined: givn/changes/omit-request-defaults/specs/fragments/ask.feature:8:5
        Step doesn't match any function
    [Summary] 1 feature, 1 scenario (1 failed), 2 steps (1 passed, 1 failed)
    error: test failed ... (exit status: 1)
    ```

## Scenarios — non-@e2e, in feature-file order

### S1 — Ask succeeds against a provider that rejects generation parameters

- [x] **RED** — remove `@wip` from this scenario only. Add the test setup that
      makes the provider hostile: the `WatnWorld` flag, the blocking-twin
      registration in both `ensure_test_env` mock-setup branches, and the
      Given step in `tests/steps/ask_steps.rs`. Do not touch production code.
      Run:
      `./run-tests.sh --name "Ask succeeds against a provider that rejects generation parameters"`
      Non-zero exit required, failing on the exit-status assertion because the
      binary still sends `temperature` and `max_tokens` and the blocking twin
      answers HTTP 400.
  - Evidence: non-zero exit (status 1), the binary exiting 2 on the twin's 400:
    ```
     ✔  Given a configured default provider "openai"
     ✔  And a provider that rejects a request carrying a sampling temperature or an output-token limit
     ✔  When I run `watn "list go files"`
     ✘  Then the exit status should be 0
        Step panicked. Captured output: assertion `left == right` failed: expected exit status 0, got Some(2). stderr: warning: config file is world-readable (644)
        API error (400): {"error":"generation defaults must not be sent"}
    [Summary] 1 feature, 1 scenario (1 failed), 4 steps (3 passed, 1 failed)
    error: test failed ... (exit status: 1)
    ```
- [x] **GREEN** — remove `temperature` and `max_tokens` from
      `RequestOptions` (`src/provider/mod.rs`), remove the two body keys from
      `src/provider/openai_compat.rs`, and drop the fields from all five
      literals (`src/main.rs:496`, `:962`, `:1247`;
      `tests/steps/interactive_shell_shortcut_steps.rs:3173`, `:3374`).
      Production files modified: `src/provider/mod.rs`,
      `src/provider/openai_compat.rs`, `src/main.rs`. Compile first; then the
      same command → zero exit.
  - Evidence: `cargo build --locked` finished; scenario passes:
    ```
     ✔  Given a configured default provider "openai"
     ✔  And a provider that rejects a request carrying a sampling temperature or an output-token limit
     ✔  When I run `watn "list go files"`
     ✔  Then the exit status should be 0
     ✔  And the output should contain "find"
    [Summary] 1 feature, 1 scenario (1 passed), 5 steps (5 passed)
    ```
- [x] **REFACTOR** — clean up without changing behaviour; run the full regular
      suite `./run-tests.sh` → zero exit, no regressions; run the e2e suite
      `./run-tests.sh --e2e` → zero exit.
  - Evidence: `cargo fmt` applied and `cargo fmt --check` clean; `cargo clippy
    --locked --all-targets` clean;
    ```
    ./run-tests.sh         → 24 features, 275 scenarios (275 passed), 1678 steps (1678 passed)
    ./run-tests.sh --e2e   → 26 features, 92 scenarios (92 passed), 699 steps (699 passed)
    ```
- [x] **COMMIT** — one atomic revision for RED+GREEN+REFACTOR:
      `givn commit` with the release note as the subject and a breaking
      marker plus the scenario title in the body, e.g.
      subject: `feat(ask)!: Watn no longer sends a sampling temperature or an output-token limit, so every request relies on the provider's own defaults and models that reject those parameters work.`
      body: the scenario title and `BREAKING CHANGE: RequestOptions no longer
      exposes temperature and max_tokens.`
  - Revision: `eb97fcc51ab86dbaff8466759350c89ef4ad5ab0`

## Post-implementation checks

- [x] `givn lint --change omit-request-defaults` exits 0 or 2 with no `@wip`
      finding remaining.
  - Evidence: `givn lint: 1 file(s) checked — clean`
- [x] Confirm no `@wip` and no `@e2e` tag change on the delta scenario.
  - Evidence:
    ```
      @givn.added
      Scenario: Ask succeeds against a provider that rejects generation parameters
    ```