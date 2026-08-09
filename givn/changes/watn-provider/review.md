# Review: watn-provider

## Review Result

The implementation is ready for archive. The review was rerun after replacing
world-state markers with production setup/result seams, adding unconditional
loopback request assertions, integrating typed model outcomes, adding provider
review/confirmation, and bounding PTY teardown.

## Verification Evidence

| Command | Result |
|---|---|
| `cargo test --test features_runner -- --tags 'not @wip and not @e2e'` | 43 scenarios, 234 steps passed |
| `cargo test --test features_runner -- --tags '@e2e and not @wip'` | 34 scenarios, 189 steps passed |
| `givn lint --change watn-provider` | Clean; no WIP findings |
| Non-E2E configured coverage command | Cobertura saved; 872/1689 lines, 51.63% in the non-E2E filtered report |
| E2E configured coverage command | Cobertura saved; 1163/1689 lines, 68.86% in the E2E filtered report |
| `cargo llvm-cov report --summary-only` | Merged workspace summary: 1777/2670 lines, 66.63%; production paths are non-zero |

The two Cobertura percentages are intentionally filter-specific. The merged
summary includes the union of instrumented workspace paths and is not compared
as if it were either filtered report. Both reports contain the same source
universe and were regenerated from the final source/profile state.

## Fabrication Audit

### Step bodies

- Zero empty, bare-pass, bare-return, or `unimplemented!` step bodies remain in
  the changed or shared step modules.
- Cancellation steps call the production `cancellation_result` seam, preserve
  the config bytes, and assert unconditional zero-hit chat mocks.
- Provider-command regular steps call production draft/result and config APIs;
  model setup absence is asserted against an unconditional `/models` mock.
- Automatic model-failure steps call the production `run_models_result` seam in
  a worker thread, assert its typed failure, and use real `/models` and chat
  mocks.
- URL assertions call production `models_url` and
  `chat_completions_url` helpers rather than duplicating URL construction in
  the step body.
- `no_original_chat_completion_request` and `no_request_should_be_sent` always
  require a mock handle and assert hit count zero.
- E2E environment-variable selection validates the requested variable name
  instead of discarding the step argument.
- PTY completion polls for ten seconds and kills a stuck child before output
  collection.

### Checked task traceability

Every scenario task has a commit hash. The regression scenarios whose direct
RED/GREEN/REFACTOR commits were test-fixture-only have explicit production
remediation links in `tasks.md` to `2cc0a17`, `624dec3`, `d58047b`, `e3be6f5`,
or `8a39d90`. Those remediation commits contain the shared production behavior
they exercise: typed model results, endpoint/credential resolution, setup
results, transport URL construction, config permissions, and CLI gating. No
checked task is represented only by a feature-file or empty-step commit.

The strict-mode proof is recorded in the setup task and shows a non-zero
targeted run from `unimplemented!`. The final feature contains exactly two
`@e2e` scenarios, matching the two inventory entries; no `@e2e` tag was removed.

### Promised components

| Design promise | Result |
|---|---|
| CLI `provider` dispatch and TTY-gated onboarding | Implemented in `src/main.rs` |
| Provider setup renderer | Implemented in `src/provider/setup.rs` with Endpoint, CredentialSource, CredentialValue, Review, and Confirmed stages |
| Typed provider/model setup outcomes | Implemented and mapped at the CLI boundary |
| Provider readiness and exact credential expansion | Implemented in `src/config/mod.rs` and `src/config/env.rs` |
| Saved OpenRouter precedence and fixed provider names | Implemented and covered |
| Transport endpoint override | Implemented for model and chat HTTP construction only |
| Secure direct config writes | Every save applies Unix mode `0600` |
| Bounded PTY cleanup | Implemented in `tests/steps/mod.rs` |

### E2E fidelity

The real interface is CLI/terminal. Both E2E scenarios use `portable-pty` to
launch the compiled `watn` binary and send keyboard input. Their primary
assertions are terminal output, process exit, and loopback HTTP requests. TOML
assertions are additional persistence checks.

| Scenario | Driver | Primary assertion |
|---|---|---|
| Configure OpenRouter with an environment-backed credential | PTY `watn provider`, then real CLI request | Terminal shows setup prompts; loopback `/chat/completions` receives the resolved key |
| First normal use starts provider setup and then model setup | Persistent PTY across both ratatui dialogs | Terminal shows model setup; process exits after tier persistence; loopback `/models` is hit and chat hit count is zero |

No browser or HTTP shortcut is used for the terminal interaction. No live
provider is contacted.

### Interaction Coverage

| Inventory entry | Matching scenario | Driver | Result |
|---|---|---|---|
| run `watn provider` and complete the interactive provider setup | Configure OpenRouter with an environment-backed credential | `portable-pty` starts `watn provider`, sends endpoint/source/confirmation keys, then a real CLI subprocess sends the request | PASS |
| run a normal `watn` command with no recognized provider and complete automatic provider and model setup | First normal use starts provider setup and then model setup | Persistent `portable-pty` holds one `watn "hello"` child through both dialogs and captures terminal output | PASS |

The matrix in `design.md`, the feature inventory, the two scenario titles, and
the globally registered capability step module agree. `verify.e2e_command`
invokes `tests/features_runner.rs` with the `@e2e and not @wip` filter. There is
no second E2E implementation in the tree.

### Local Runnability

The local run command is:

```text
cargo test --test features_runner -- --tags '@e2e and not @wip'
```

It starts the Gherkin runner, real CLI subprocesses, persistent PTYs, and a
random-port `httpmock::MockServer` digital twin per scenario. There is no
database, queue, application server, container, shared network, or live
third-party dependency.

### Design conformance

- `run_models_result` returns `Saved`, `Cancelled`, or `Failed`; process exit
  mapping is in `main.rs`.
- Provider setup contains the reviewed confirmation states and restores the
  terminal on all exits.
- Provider-specific and generic fallback lookup is in `config/env.rs`.
- The E2E transport override is ephemeral and excluded from readiness and
  persistence.
- `verify.command` and `verify.e2e_command` match `givn/commands.yaml` and are
  distinct filtered commands.
- The design contains an explicit Local Runnability section and the PTY helper
  implements its bounded wait/kill decision.

## Coverage Classification

Coverage was measured with instrumented Gherkin and CLI processes. No
change-specific production path is unmeasured: `src/provider/setup.rs`,
`src/config/mod.rs`, `src/main.rs`, `src/models/mod.rs`,
`src/provider/openai_compat.rs`, and `src/provider/transport.rs` all have
non-zero line coverage in the generated reports.

### Dead code

No change-specific dead code remains. The unused endpoint helper and unused
search-error fixture identified by the first audit were removed.

### Missing test coverage

No change-specific missing scenario remains. Provider setup, readiness,
credential precedence/expansion, endpoint normalization, cancellation, config
permissions, TTY gating, model failure, explicit command termination, and both
real-interface flows are covered by the delta or updated permanent features.

### Legitimately hard to test

The remaining uncovered regions in pre-existing request rendering, shell
execution, spinner animation, and provider error formatting are outside the
provider-onboarding paths and require OS-specific terminal/signal or transport
failure injection. They are classified here as legitimately hard to test in
this change because the repository has no stable production seam for those
unrelated implementations, while all new branches have deterministic feature
coverage through PTY or loopback fixtures.

## Sign-off Checklist

| Check | Status |
|---|---|
| Fabrication audit clean | PASS |
| Every checked task has verified production remediation traceability | PASS |
| Promised components exist and conform | PASS |
| Strict-mode proof present and non-zero | PASS |
| `verify.command` exits 0 | PASS |
| `verify.e2e_command` exits 0 | PASS |
| E2E count is isolated and smaller | PASS: 34 E2E vs 43 regular filtered scenarios |
| Coverage includes runner and spawned CLI | PASS for line coverage; Cobertura branch fields are unavailable |
| Coverage classified using the three required buckets | PASS |
| No WIP tags remain | PASS |
| E2E scenarios use real CLI/PTY interaction and primary interface assertions | PASS |
| Exactly one E2E scenario per inventory action | PASS |
| Local digital twin and PTY stack starts cleanly | PASS |
| Implementation matches the reviewed design | PASS |
| Inventory, matrix, scenarios, and drivers cross-reference cleanly | PASS |

REVIEW: PASS
