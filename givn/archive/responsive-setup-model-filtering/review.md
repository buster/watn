# Review: responsive-setup-model-filtering

## Fabrication Audit

### Tag Integrity

The delta feature contains four scenarios in
`givn/changes/responsive-setup-model-filtering/specs/responsive-setup-model-filtering/responsive-setup-model-filtering.feature`:

- `A complete catalog is filtered locally`
- `A catalog requiring more data uses provider-backed filtering`
- `A newer model query remains authoritative`
- `The terminal model filter stays responsive during a delayed search` (`@e2e`)

The single `@e2e` scenario retains its `@e2e` tag. No `@wip` tags remain
(`givn lint --change responsive-setup-model-filtering` is clean). No tag was
removed to bypass the E2E gate.

### Step-Body Audit

`tests/steps/responsive_setup_model_filtering_steps.rs` was scanned for
`unimplemented!()`, `todo!()`, empty bodies, and no-op returns. Zero empty or
stub step bodies were found. All steps start real loopback provider twins, drive
the real PTY session, reconstruct the terminal screen, or assert request counts
and rendered query/result text. None is repository-only.

### Task And Commit Audit

Every checked task has evidence and commits touching production source or
genuine step infrastructure:

| Scenario | Commits | Implementation evidence |
|---|---|---|
| A complete catalog is filtered locally | `6a62cb5` | `src/setup.rs` adds catalog completeness, local filtering, visible query title, retained worker handles, and shutdown join |
| A catalog requiring more data uses provider-backed filtering | `c88d6fe` | Reuses remote worker path from `6a62cb5`; new 50-model fixture and search-request assertion |
| A newer model query remains authoritative | `144adc5`, follow-up `7d9eb2c` | Generation-ordered delayed `gpt` / immediate `o3` twins; existing worker/generation checks in `src/setup.rs` |
| Paginated-catalog compatibility fix | `7d9eb2c` | `src/models/list.rs` adds `ModelPage` with `meta.has_more`; `src/setup.rs` uses `fetch_models_page_info`; permanent `ask_steps.rs` fixture declares pagination |
| Terminal model filter stays responsive (E2E) | `2bd4627` | Real PTY against delayed twins; renders query/result; proves continued input response |

The strict-mode proof is present in the setup and RED tasks with non-zero
targeted runs. Every scenario either introduced production code directly or is
backed by the follow-up production fix; no task is completed by a spec-only or
stub-only commit.

### Components And Design Conformance

The capability file `tests/steps/responsive_setup_model_filtering_steps.rs` is
registered from `tests/steps/mod.rs`. The implementation matches `design.md`
for commands, step location, `portable-pty` E2E driving, `httpmock` twins, the
200 ms debounce, single generation authority, and joined worker lifecycle. The
one refinement — explicit `meta.has_more` for catalog completeness — is the
mechanism `design.md` left to the models module and is recorded in ADR-0009.

### Real-Interface E2E Audit

The E2E scenario starts the compiled `watn models` process in a real
`portable-pty` session, types `gpt`, replaces it with `o3` while the `gpt`
provider response is delayed, asserts the reconstructed terminal screen shows
`Filter: o3` and `o3-pro`, then changes the filter to `gpt` and asserts the
terminal accepts it. Primary assertions are terminal output, not repository or
request state. `httpmock` is only the provider digital twin; no browser driver
applies for this CLI capability and no HTTP/fetch() substitution stands in for
the PTY interaction.

### Command Isolation And Local Stack

- Regular: `./run-tests.sh` → `98 scenarios`, `570 steps` passed.
- E2E: `./run-tests.sh --e2e` → `66 scenarios`, `463 steps` passed.
- `66 < 98`, so E2E is a strict subset. The runner rejects combining `--tags`
  and `--name`, so targeted E2E proof used the name-filtered command while the
  full gate used the tag-filtered command.
- The local stack builds the default and `test-support` binaries, starts the
  per-scenario loopback `httpmock` twin, and shuts down through the PTY
  harness. No Docker, external service, or live provider is required.

### Interaction Coverage Cross-Reference

| Inventory entry | Matrix row | `@e2e` scenario | Real interface | Driving mechanism |
|---|---|---|---|---|
| type a model filter in the setup wizard and observe the query and matching results update while the catalog search is delayed | matching `design.md` row | The terminal model filter stays responsive during a delayed search | CLI/terminal UI | `portable-pty` starts `watn models`, sends filter and replacement keystrokes before the delayed response, and asserts the reconstructed screen |

One inventory entry maps to exactly one matrix row and exactly one `@e2e`
scenario. No excess E2E scenario duplicates an input variant.

## Arc42 Implementation Conformance

`addons.arc42` is enabled. The independent chapter re-derivation matches
`arc42.md`: chapters 01, 03, 04, 05, 06, 08, 09, 10, 11, and 12 are affected;
chapters 02 and 07 are not.

| Chapter / fact | Durable source | `arc42.md` | `design.md`/tasks | Implementation evidence | Match |
|---|---|---|---|---|---|
| Visible filter query and delayed-search responsiveness | Ch. 01, 03, 04, 06, 08, 10 | Yes | Design sections and QS-054 | `Filter: <query>` title in `draw_model`; E2E terminal assertions | Yes |
| Hybrid local/remote filtering | Ch. 04, 05, 08, ADR-0009 | Yes | Design Search Lifecycle | Local path for complete catalogs; debounced remote path otherwise | Yes |
| Catalog completeness via pagination metadata | Ch. 05, 09, ADR-0009 | Yes | Follow-up fix `7d9eb2c` | `ModelPage.complete` from `meta.has_more` | Yes |
| Worker lifecycle and generation authority | Ch. 04, 05, 06, 08, 11 | Yes | R-020 strengthened | Retained handles, reap, join on `Drop` | Yes |
| Glossary terms | Ch. 12 | Yes | Design decisions | Catalog completeness, local model filter, search worker | Yes |

`ARC42 CONFORMANCE: CLEAN`

## Coverage Measurement

`measure-coverage.sh` instruments the library, the `features_runner` binary,
and the copied `watn` children used by PTY. Profiles use the collision-safe
`LLVM_PROFILE_FILE=coverage/profraw/%p-%m.profraw` pattern and are flushed on
process exit; `merge-coverages.sh` produced a fresh merged report.

| Report | Covered / valid lines | Rate | Branch status |
|---|---:|---:|---|
| Non-E2E | 5,791 / 10,041 | 57.67% | Not claimed: 0 / 0 |
| E2E | 6,106 / 10,041 | 60.81% | Not claimed: 0 / 0 |
| Merged | 9,077 / 10,041 | 90.40% | Not claimed: 0 / 0 |
| Merged `src/setup.rs` | 843 / 1,030 | 81.84% | Not claimed |
| Merged `src/models/list.rs` | 179 / 257 | 69.65% | Not claimed |
| Merged `tests/features_runner.rs` | 72 / 81 | 88.89% | Not claimed |

The merged first line reports `line-rate="0.9039936261328553"` and
`lines-valid="10041"`. Exercised production paths (local filter, remote search,
query rendering, generation checks, worker join, completeness parsing) all have
non-zero coverage through the instrumented PTY child.

### Coverage Classification

- **Dead code:** None. No unused filter/complete/worker path remains.
- **Missing test coverage:** None for the four stated behaviours. Each
  regular scenario asserts concrete query, suggestion, or request-count
  outcomes; the E2E scenario asserts the rendered terminal during a delayed
  search.
- **Legitimately hard to test:** Remaining uncovered regions in broad modules
  are pre-existing terminal event/read failures, HTTP/network error branches,
  and OS-level process failures that require fault injection at the real
  terminal/network boundary. Normal add/backspace editing, local filtering,
  remote search, delayed ordering, and worker shutdown are covered by the
  runner scenarios.

No gap was classified outside the three permitted buckets.

## Verification

- `givn lint --change responsive-setup-model-filtering`: clean.
- `./run-tests.sh`: 18 features, 98 scenarios, 570 steps passed.
- `./run-tests.sh --e2e`: 22 features, 66 scenarios, 463 steps passed.
- Targeted scenario runs: local (1 scenario, 7 steps), provider-backed (1
  scenario, 6 steps), stale-result (1 scenario, 4 steps), E2E (1 scenario,
  9 steps).
- `./measure-coverage.sh` and `./merge-coverages.sh`: fresh reports produced.
- `cargo fmt --all -- --check`, `cargo check --locked`, `git diff --check`:
  passed.
- Branch data: `0/0` on this toolchain; branch coverage is not claimed.

## Sign-Off

- [x] Fabrication audit clean (tags intact, no stubs, no spec-only commits).
- [x] Every checked task has evidence and production-backed commits.
- [x] Promised components exist and match `design.md`.
- [x] Strict-mode proof present with non-zero RED evidence.
- [x] Regular and E2E verification pass; E2E is a strict subset (66 < 98).
- [x] Coverage measured across runner and instrumented child processes.
- [x] Coverage gaps classified under the three permitted buckets.
- [x] Arc42 conformance clean.
- [x] Interaction coverage cross-reference clean.
- [x] No `@wip` tags remain; exactly one `@e2e` scenario per inventory entry.

REVIEW: PASS