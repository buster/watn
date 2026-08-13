# Review: streamlined-setup-flow

## Fabrication Audit

### Tag Integrity

The active delta feature is
`givn/changes/streamlined-setup-flow/specs/streamlined-setup/streamlined-setup.feature`.
It contains exactly five `@givn.added @e2e` scenarios and no `@wip` tags:

- `Coordinated setup completes provider models reasoning and shell choices`
- `Provider setup configures an OpenAI provider with an environment credential`
- `Models setup configures all three roles from an available catalog`
- `Shell setup independently configures completion and Ctrl-W integrations`
- `Incomplete interactive request opens setup and does not send the original request`

The tags were preserved. The configured E2E command selects `@e2e and not
@wip` through `./run-tests.sh --e2e`.

### Step-Body Audit

All registered step modules under `tests/steps/` were scanned for
`unimplemented!()`, `todo!()`, empty bodies, bare no-op returns, and trivial
step implementations. Zero fabricated step bodies were found. The PTY steps
start the compiled CLI, send real key sequences, inspect terminal output, and
clean up child processes. Other steps use real subprocesses, loopback HTTP
twins, filesystem assertions, or domain assertions.

### Task And Commit Audit

All `206/206` task checkboxes are complete. The `56` unique commit hashes
recorded in `tasks.md` resolve to commits. Some hashes intentionally contain
only test/spec/evidence changes because their task explicitly reuses an
already implemented production path; each newly introduced production
behavior has a corresponding implementation commit touching `src/`.
No missing commit or spec-only implementation was found.

### Components And Design Conformance

Every component named by `design.md` exists and is registered or used:

- `src/setup.rs` owns coordinated and focused draft state machines, validation, catalog state, review, and terminal restoration.
- `src/config/mod.rs` owns atomic candidate replacement, mode enforcement, and provider migration.
- `src/models/list.rs` and `src/models/picker.rs` own provider-local discovery, pagination, filtering, and stale-result protection.
- `src/provider/setup.rs` owns endpoint normalization, credential-source validation, and provider draft construction.
- `src/shell_completion.rs` and `src/shell_shortcut.rs` own independent shell operations.
- `tests/steps/streamlined_setup_steps.rs` contains regular capability steps.
- `tests/steps/streamlined_setup_e2e_steps.rs` contains the five real CLI PTY smoke flows.
- `tests/steps/mod.rs` registers both capability-specific modules and the permanent E2E modules.

The implementation uses the reviewed Rust, Ratatui/Crossterm,
cucumber-rs, portable-pty, and httpmock choices.

### Strict-Mode Proof

The setup task records a non-zero strict-run proof for an undefined setup
step. The runner uses `Cucumber::fail_on_skipped()` in
`tests/features_runner.rs`, so undefined and pending steps fail the build.

### Real-Interface E2E Audit

All five delta E2E scenarios drive the actual compiled `watn` binary through
`portable-pty`. Their primary assertions inspect rendered terminal questions,
model/reasoning progression, shell prompts, review/output text, and exit
status. Config files, shell target files, and mock request counts are
secondary assertions.

The permanent setup E2E scenarios use the same real PTY boundary through
`setup_wizard_steps.rs`, `provider_setup_steps.rs`,
`model_picker_layout_steps.rs`, and
`highlight_active_setup_input_steps.rs`. There is no browser capability and
no HTTP or in-page `fetch()` shortcut replacing a user-interface driver.

### Command Isolation And Local Runnability

`givn/commands.yaml` configures distinct commands:

- Regular: `./run-tests.sh` — `148` scenarios and `851` steps passed.
- E2E: `./run-tests.sh --e2e` — `75` scenarios and `555` steps passed.

The E2E set is a strict subset. `run-tests.sh` builds the normal and
`test-support` binaries to temporary paths, runs the Cucumber feature runner,
and uses loopback `httpmock` twins plus isolated `portable-pty` sessions. No
Docker service or live provider is required by this CLI capability.

### Interaction Coverage Cross-Reference

| Inventory entry | Matrix scenario | Delta scenario | Step implementation | Driver verified |
|---|---|---|---|---|
| Invoke `watn setup` and complete the coordinated configuration flow | Coordinated setup completes provider models reasoning and shell choices | Present, `@e2e` | `tests/steps/streamlined_setup_e2e_steps.rs` drives provider, endpoint, credential, catalog, model, reasoning, shell, review, and output | `portable-pty` CLI with loopback catalog/chat twins |
| Invoke `watn provider` and configure a provider independently | Provider setup configures an OpenAI provider with an environment credential | Present, `@e2e` | The same E2E step module selects OpenAI, accepts the endpoint, selects the environment source, and asserts terminal/config result | `portable-pty` CLI |
| Invoke `watn models` and configure the three model roles independently | Models setup configures all three roles from an available catalog | Present, `@e2e` | The same E2E step module drives all model and reasoning pages and asserts completion/config result | `portable-pty` CLI with loopback catalog twin |
| Invoke `watn shell` and configure shell integrations independently | Shell setup independently configures completion and Ctrl-W integrations | Present, `@e2e` | The same E2E step module drives independent completion and shortcut selections and inspects target files | `portable-pty` CLI with isolated HOME/XDG targets |
| Invoke an interactive `watn "question"` request when setup is incomplete | Incomplete interactive request opens setup and does not send the original request | Present, `@e2e` | The same E2E step module drives first-use setup, observes prefilled values, cancels, and checks the chat twin | `portable-pty` CLI with zero-hit chat twin |

The inventory, design matrix, feature scenarios, and step definitions have a
one-to-one mapping. No excess E2E scenario was added for an input variant or
error case of these five actions.

### Coverage Measurement Validity

Coverage was measured with `./measure-coverage.sh` and merged with
`./merge-coverages.sh`. The scripts instrument the Cucumber runner and the
test-support Watn binaries, use collision-safe
`coverage/profraw/%p-%m.profraw` paths, and merge both source reports.

Fresh merged Cobertura output:

- Non-E2E: `9,459 / 13,997` lines, line rate `0.6757876687861685`.
- E2E: `7,580 / 13,997` lines, line rate `0.5415446167035793`.
- Merged: `12,832 / 13,997` lines, line rate `0.9167678788311782`.
- `src/setup.rs`: `1,266 / 1,483` lines, line rate `0.8536749831422792`.
- Branch data: the Cobertura export reports `0 / 0`; no branch denominator is emitted by the configured Rust coverage export.

The merged report includes the runner and production processes, and a known
production path (`src/setup.rs`) has non-zero coverage. Coverage measurement
is valid.

## Arc42 Implementation Conformance

| Arc42 chapter/fact | Durable documentation | `arc42.md` claim | Design/tasks | Implementation evidence | Match |
|---|---|---|---|---|---|
| 1. Goals | `docs/arc42/01-introduction-and-goals.md` | Setup safety, focused commands, catalog, reasoning, and final confirmation affected | Design technical direction and scenario tasks | `src/main.rs`, `src/setup.rs`, provider/model/shell tests | Yes |
| 2. Constraints | `docs/arc42/02-architecture-constraints.md` | Rust, TTY UI, credential references, atomic config, closed completion selector | Design hardening and persistence sections | Rust implementation, `src/config/mod.rs`, `src/main.rs`, `run-tests.sh` | Yes |
| 3. Context/scope | `docs/arc42/03-context-and-scope.md` | Four setup commands and provider-local catalog boundaries affected | Design CLI entry-point and catalog sections | `src/main.rs`, `src/setup.rs`, provider-local HTTP paths | Yes |
| 4. Strategy | `docs/arc42/04-solution-strategy.md` | Draft state machine, migration, catalog, reasoning, and atomicity affected | Design technical direction and final-confirmation sections | `src/setup.rs`, `src/config/mod.rs`, `src/models/list.rs` | Yes |
| 5. Building blocks | `docs/arc42/05-building-block-view.md` | Setup, config, catalog, provider, shell, and completion blocks affected | Design architecture-impact module map | `src/setup.rs`, `src/config/mod.rs`, `src/models/*`, `src/provider/setup.rs`, `src/shell_*` | Yes |
| 6. Runtime | `docs/arc42/06-runtime-view.md` | Coordinated/focused setup and completion runtime sequences affected | Design state-machine and E2E matrix | PTY scenarios and `tests/steps/streamlined_setup_e2e_steps.rs` | Yes |
| 7. Deployment | `docs/arc42/07-deployment-view.md` | Explicitly marked no impact | Arc42 assessment says no deployment change | No production service/artifact topology added; test twins are local | Yes |
| 8. Cross-cutting concepts | `docs/arc42/08-crosscutting-concepts.md` | Atomicity, credentials, source isolation, reasoning, and cancellation affected | Design hardening, persistence, and transport sections | `src/config/mod.rs`, `src/setup.rs`, `src/provider/*`, tests | Yes |
| 9. Decisions | `docs/arc42/09-architecture-decisions.md` and ADR-0020..0024 | Superseding decisions recorded | Design binding decisions and task evidence | Final-confirmation snapshots, provider-local catalog, verbatim reasoning, migration, atomic replacement | Yes |
| 10. Quality | `docs/arc42/10-quality-requirements.md` | New measurable setup/catalog/reasoning/atomicity scenarios | `tasks.md` has `206/206` tasks and count evidence | `148/851` regular and `75/555` E2E pass | Yes |
| 11. Risks | `docs/arc42/11-risks-and-technical-debt.md` | Collision, source ambiguity, final-write failure, shell partial failure, and E2E risks updated | Design hardening and risk scenarios | Migration, atomic write, shell result, transport and PTY tests | Yes |
| 12. Glossary | `docs/arc42/12-glossary.md` | Catalog source, draft, confirmation, migration, reasoning, PTY, and shell terms updated | Design terminology and feature language | Matching code and Gherkin vocabulary | Yes |

`ARC42 CONFORMANCE: CLEAN`

## Coverage Classification

- **Dead code:** None identified in the changed production paths.
- **Missing test coverage:** None for the requested setup, catalog, reasoning, provider migration, atomicity, shell, readiness, transport, completion, or help behavior. The feature runner covers all checked scenarios and both E2E gates are green.
- **Legitimately hard to test:** Low-level terminal draw/descriptor failures, exact process-kill timing during PTY handoff, and unavailable optional shell executables remain environment-specific. Forcing those boundaries would corrupt or terminate the test harness rather than validate a user-observable contract. Normal Escape/Ctrl-C, catalog failure, shell failure, and parser availability paths are covered.

## Verification

```text
cargo clippy --locked --all-targets -- -D warnings
clean

cargo fmt --all -- --check
clean

givn lint --change streamlined-setup-flow
clean

./run-tests.sh
148 scenarios passed, 851 steps passed

./run-tests.sh --e2e
75 scenarios passed, 555 steps passed

./measure-coverage.sh
./merge-coverages.sh
fresh merged Cobertura report generated
```

All `206/206` tasks are checked, all active delta scenarios retain `@e2e`, no
`@wip` tags remain, the exact verify commands are distinct, the local PTY and
loopback test infrastructure runs without external services, and the
implementation matches the reviewed design.

REVIEW: PASS
