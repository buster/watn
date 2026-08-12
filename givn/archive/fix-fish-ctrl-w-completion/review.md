# Review: Fix Fish Ctrl-W Completion Insertion

## Fabrication Audit

### Step and tag audit

- Delta feature file scanned: `givn/changes/fix-fish-ctrl-w-completion/specs/fish-ctrl-w-completion/fish-ctrl-w-completion.feature`.
- The scenario retains `@givn.added @e2e`; no `@e2e` tag was removed.
- `givn lint --change fix-fish-ctrl-w-completion` reports clean and no `@wip` tags remain.
- `tests/steps/fish_ctrl_w_completion_e2e_steps.rs` and `tests/steps/preserve_ctrl_w_requests_steps.rs` were scanned; `0` empty, trivial, or unimplemented bodies were found.
- Strict-mode proof is recorded in `tasks.md`: the registered `unimplemented!()` step matched, panicked, and caused a non-zero Cucumber exit. The runner uses `.fail_on_skipped()`.

### Implementation and commit audit

| Scenario or task | Commit | Implementation evidence |
|---|---|---|
| Fish inserts a real line break after Ctrl-W | `1ebd11e`, hardening `fc55fae` | `1ebd11e` changes `src/shell_shortcut.rs` from a literal Fish `\\n` separator to a collected buffer with an actual newline; the step module and permanent source-contract assertion are registered. `fc55fae` narrows the reviewed contract, updates durable documentation, and isolates the Fish PTY environment. |

The implementation commit predates the design-review scope narrowing and uses
the former scenario title. The hardening commit uses the current scenario title
verbatim. The checked scenario therefore has a production implementation commit
and a follow-up hardening commit; it is not represented only by a feature file
or a stub.

### Design conformance

- `src/shell_shortcut.rs` contains the promised Fish widget change.
- `tests/steps/fish_ctrl_w_completion_e2e_steps.rs` exists at the design-named path and drives the real Fish reader through `portable-pty`.
- `tests/steps/preserve_ctrl_w_requests_steps.rs` exists and contains the updated existing Fish source-contract assertion.
- `tests/steps/mod.rs` registers both step modules as designed.
- The configured regular command filters `not @wip and not @e2e`.
- The configured e2e command filters `@e2e and not @wip`; the commands are not identical.
- The interface is CLI/terminal interaction, not a browser UI. The e2e driver uses a real interactive `fish` process in a PTY, not HTTP, `fetch`, or repository inspection.
- The local environment requires no server, database, network, or digital twin. The fake `watn` executable and temporary Fish startup file are isolated fixtures.

### E2E scope and interface

The inventory contains one user-facing action: press Ctrl-W in an installed Fish
shortcut and observe the generated command in the editable line. The design
matrix maps it to `Fish inserts a real line break after Ctrl-W`. The scenario's
Then step asserts the real Fish `commandline` buffer, including the actual
newline and absence of literal `\\n` text. It does not claim to test committing
the buffer or Fish failure/empty/multiline branches; those were explicitly
removed from the narrowed proposal.

### Interaction coverage cross-reference

| User Interaction Inventory entry | Design matrix row | Delta E2E scenario | Step definition and driving mechanism | Result |
|---|---|---|---|---|
| press Ctrl-W in an installed Fish shortcut and observe the generated command in the editable command line | Same inventory entry, real Fish terminal buffer | `Fish inserts a real line break after Ctrl-W` | `tests/steps/fish_ctrl_w_completion_e2e_steps.rs`; `portable-pty` starts interactive Fish, sends request text and Ctrl-W, invokes a test capture binding, and reads the resulting `commandline` buffer | Covered |

### Runner isolation

The literal configured commands in `givn/commands.yaml` build the default and
`test-support` binaries and invoke the same Cucumber runner with different tag
expressions. The regular run passed with `18 features`, `103 scenarios`, and
`594 steps`. The e2e run passed with `23 features`, `68 scenarios`, and `471
steps`. The e2e count is strictly smaller, proving that the filter is active.

The local run command starts the complete capability fixture: an isolated
interactive Fish process under a PTY, a temporary HOME/XDG tree, and a fake
`watn` executable. No external service or digital twin is required.

## Arc42 Implementation Conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 3. Context and scope: Fish line-editor boundary receives a real newline | `docs/arc42/03-context-and-scope.md` line-editor table | Affected; replacement-buffer contract clarified | Real Fish commandline buffer is the assertion boundary | Focused scenario records exact buffer | `fish_ctrl_w_completion_e2e_steps.rs` captures Fish `commandline` output | Yes |
| 5. Building blocks: Shell Shortcut and line-editor responsibilities | `docs/arc42/05-building-block-view.md` Shell Shortcut and Line editor rows | Affected; existing responsibilities clarified | Only the existing generator changes | Production file is `src/shell_shortcut.rs` | Fish block assembles `buffer` with `printf` and `string collect` | Yes |
| 6. Runtime view: Ctrl-W replacement flow | `docs/arc42/06-runtime-view.md` Ctrl-W scenario | Affected; Fish separator is an actual newline | PTY drives request and Ctrl-W, then captures buffer | Green focused scenario | Captured buffer equals `# show available diskspace\ndf -h` with one actual newline | Yes |
| 7. Deployment view: isolated shortcut verification | `docs/arc42/07-deployment-view.md` shortcut verification | Affected; interactive Fish PTY is documented | Temporary startup files and fake PATH executable | E2E setup evidence | PTY child receives isolated HOME/XDG values and sources only the temporary target | Yes |
| 8. Cross-cutting concepts: shell widget text safety | `docs/arc42/08-crosscutting-concepts.md` shell shortcut section | Affected; literal `\\n` is distinguished from a real newline | Fish uses shell-produced newline in one collected value | Green buffer assertion | `set -l buffer (printf '%s\\n%s' ... | string collect)` then `commandline -r -- "$buffer"` | Yes |
| 9. ADR-0018: native widget decision | `docs/arc42/09-architecture-decisions.md` and `docs/adr/0018-safe-shell-shortcut-installation-and-native-widgets.md` | Affected; existing ADR clarified, no new ADR | `printf` and `string collect` avoid Fish's literal `\\n` behavior | Hardening documentation commit recorded | ADR-0018 documents the collected Fish buffer decision | Yes |
| 10. QS-056: Fish buffer contract | `docs/arc42/10-quality-requirements.md` QS-056 | Affected; exact buffer metric added | Actual line break and no visible escape text | Focused and full e2e runs pass | Feature Then step asserts exact real-interface buffer | Yes |
| 11. R-046/R-055: shell and redraw risks | `docs/arc42/11-risks-and-technical-debt.md` | Affected; Fish PTY mitigation and buffer risk updated | Real Fish PTY covers the changed representation | E2E setup and green evidence recorded | Interactive Fish test passes under isolated PTY | Yes |

**ARC42 CONFORMANCE: CLEAN**

## Coverage

Coverage was measured, not inferred. `./measure-coverage.sh` completed both
instrumented regular and e2e runs and wrote
`coverage/non-e2e-cobertura.xml` and `coverage/e2e-cobertura.xml`.
`./merge-coverages.sh` wrote `coverage/cobertura-coverage.xml`.

- Merged line coverage: `9321/10302` (`90.48%`).
- `src/shell_shortcut.rs`: `271/319` lines (`84.95%`) in the merged report.
- The Gherkin runner and all child production binaries are instrumented by `cargo llvm-cov`; per-process `LLVM_PROFILE_FILE=coverage/profraw/%p-%m.profraw` output avoids collisions.
- The changed Fish buffer behavior is exercised by the real Fish e2e scenario. The generated Fish block is a Rust string constant, so LLVM coverage does not assign executable hits to source line 519; the surrounding `Shell Shortcut` installation code is included in the measured `271/319` module result.
- No dead code was identified.
- No missing test coverage was identified for the narrowed behavior: the exact real Fish buffer contract has a focused `@e2e` scenario and the existing Fish source contract remains green.
- Remaining uncovered lines in the affected module are existing installer OS-error/platform branches or generated-shell/interactive redraw paths. They are legitimately hard to test in this Rust process without manufacturing platform failures or replacing the real shell reader; generated syntax/content checks and the real Fish PTY cover the changed portable behavior. This is classification bucket 3, not an unrecorded gap.

**COVERAGE MEASUREMENT: CLEAN**

## Verification

- `givn lint --change fix-fish-ctrl-w-completion`: clean.
- Focused runner: `1 scenario`, `3 steps`, all passed for `Fish inserts a real line break after Ctrl-W`.
- Regular configured runner: `18 features`, `103 scenarios`, `594 steps`, all passed.
- E2E configured runner: `23 features`, `68 scenarios`, `471 steps`, all passed.
- `./measure-coverage.sh`: both instrumented runs passed and reports were written.
- `./merge-coverages.sh`: passed and produced the merged report.
- `cargo fmt --all -- --check`: passed.
- `git diff --check`: passed before the hardening commit.
- Tasks: all `5/5` checked; implementation and hardening commit hashes are recorded.

## Sign-Off

- Fabrication audit: clean.
- Strict-mode proof: present and non-zero for the registered stub.
- Production implementation: present in `src/shell_shortcut.rs`.
- E2E interface: real Fish PTY with primary editable-buffer assertion.
- Arc42 implementation conformance: clean.
- Coverage: measured across the Gherkin runner and child production binaries.
- No `@wip` tags remain in the delta.
- Both configured runners pass and the e2e filter is a strict subset.
- The interaction inventory maps to the design matrix, feature scenario, and step driver.

REVIEW: PASS
