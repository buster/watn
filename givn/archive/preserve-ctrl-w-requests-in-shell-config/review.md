# Review: Preserve Ctrl-W Requests In Shell Config

## Fabrication Audit

### Step and tag audit

- Delta feature file scanned: `givn/changes/preserve-ctrl-w-requests-in-shell-config/specs/interactive-shell-shortcut/interactive-shell-shortcut.feature`.
- The added Bash E2E scenario retains `@e2e`.
- The modified permanent Bash E2E scenario retains `@e2e`.
- No `@e2e` tag was removed from the delta feature history.
- All `@wip` tags are removed; `givn lint` reports clean.
- Changed step-definition files scanned: `tests/steps/preserve_ctrl_w_requests_steps.rs` and `tests/steps/interactive_shell_shortcut_e2e_steps.rs`.
- `0` empty, trivial, or unimplemented step bodies found across the changed step-definition files. Every new body performs a real Bash execution, filesystem assertion, generated-block assertion, or E2E buffer assertion.
- Strict-mode proof is recorded in `tasks.md` Setup evidence: the registered `unimplemented!()` step matched, panicked with `not implemented`, and the targeted Cucumber process exited non-zero. The runner uses `Cucumber::fail_on_skipped()`.

### Implementation and commit audit

| Scenario or task | Commit | Implementation evidence |
|---|---|---|
| A successful generation keeps the original request visible as a comment | `3ac22d8` | `src/shell_shortcut.rs` changes the Bash widget to flatten the request and assign comment plus result; the capability step module is registered. |
| Only the generated command executes when the buffer is committed | `c806db3` | A real `bash -c` step executes the produced buffer and asserts generated-command and comment-command filesystem effects. |
| Requests with metacharacters and embedded newlines remain one comment line | `7d37e64` | The capability step decodes escaped control characters, runs the shared Bash harness, and asserts one comment line. |
| Failed or empty generation preserves the original buffer | `6735dbd` | Existing failure/empty fixtures and exact buffer assertions pass unchanged; this is an intentional pure verification/reuse scenario. |
| Zsh and Fish widgets preserve the request as a comment | `8b7e722` | `src/shell_shortcut.rs` changes both native blocks; generated content and shell syntax are asserted. |
| The generated Bash widget keeps the request visible and does not evaluate the command | `d7ecebc` | The real Bash subprocess E2E steps assert returned buffer text, request-comment shape, invocation input, and no evaluation. |
| Modified permanent scenarios | `793be0e` | Durable permanent expectations and `@givn.modified` entries are synchronized; the title-targeted regular and E2E runs pass. |

The later verification commits intentionally contain step infrastructure or durable-spec updates rather than repeated production edits. The production behavior is implemented in `3ac22d8` and `8b7e722`; the later scenarios exercise that same generated widget through distinct safety and shell-boundary assertions. No checked task is represented only by an empty stub.

### Design conformance

- `src/shell_shortcut.rs` contains the promised `BASH_BLOCK`, `ZSH_BLOCK`, and `FISH_BLOCK` changes.
- `tests/steps/preserve_ctrl_w_requests_steps.rs` exists and is registered from `tests/steps/mod.rs`.
- E2E updates are in the design-named `tests/steps/interactive_shell_shortcut_e2e_steps.rs`.
- The configured regular command filters `not @wip and not @e2e`.
- The configured E2E command filters `@e2e and not @wip`; it is not identical to the regular command.
- The interface is CLI/terminal execution, not a browser UI. The E2E driver is a real `bash --noprofile --norc -c` subprocess, as specified in `design.md`.
- No browser, HTTP shortcut, `fetch`, or alternate E2E implementation exists for this capability.
- The local environment requires no server or digital twin. The fake `watn` executable on `PATH` is the only external fixture.

### E2E scope and interface

The inventory contains one user-facing action: pressing Ctrl-W in an installed shell widget and observing the original request above the generated command. The design matrix maps it to the added scenario `The generated Bash widget keeps the request visible and does not evaluate the command`. The modified permanent E2E scenario has the same interface because it is the existing regression scenario being updated, not a second variant or a new inventory action; its `@e2e` tag is retained and its expectations are synchronized.

Both E2E scenarios drive the generated widget through a real Bash process. Their Then steps assert the process-produced command buffer and the absence of execution. The E2E step file records the fake `watn` invocation and checks the returned `LINE<<...>>` buffer; no repository-only substitute is used.

### Interaction coverage cross-reference

| User Interaction Inventory entry | Design matrix row | Delta E2E scenario | Step definition and driving mechanism | Result |
|---|---|---|---|---|
| Press Ctrl-W in an installed Bash, Zsh, or Fish widget and observe the original request preserved as a comment above the generated command | Same inventory entry, real Bash widget in a Bash subprocess | `The generated Bash widget keeps the request visible and does not evaluate the command` | `tests/steps/interactive_shell_shortcut_e2e_steps.rs` and the shared Bash fixture; `bash --noprofile --norc -c` sources the installed block, sets input, invokes `_watn_widget`, and reads the editable buffer | Covered |

The Zsh and Fish portions are covered at generated-block contract and syntax-check level because their interactive `zle` and `commandline` APIs require an interactive shell session. Bash is covered through both the fixture subprocess and the E2E subprocess.

### Runner isolation

The literal configured commands in `givn/commands.yaml` build the default and `test-support` binaries and invoke `cargo test --test features_runner --features test-support` with different tag expressions. The regular run passed with `19 features`, `105 scenarios`, and `604 steps`. The E2E run passed with `23 features`, `68 scenarios`, and `473 steps`. The E2E scenario count is strictly smaller, proving the filter is active.

## Arc42 Implementation Conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 1. Introduction and goals: request remains visible and generated output is never evaluated | `docs/arc42/01-introduction-and-goals.md:31-33,50` | Affected; refines the shell-shortcut goal | Success produces `# <flattened request>` plus result; result is assigned, never evaluated | Successful, commit-isolation, E2E, and modified scenarios pass | `src/shell_shortcut.rs:455-459`; Bash subprocess assertions | Yes |
| 3. Context and scope: line-editor boundary returns replacement buffer text | `docs/arc42/03-context-and-scope.md:31-32,45-46` | Affected; extends line-editor boundary output | Native widget replaces the editable buffer | Bash, Zsh, Fish contract scenarios pass | `BASH_BLOCK`, `ZSH_BLOCK`, `FISH_BLOCK` assign native buffers | Yes |
| 4. Solution strategy: native widgets preserve comment and avoid evaluation | `docs/arc42/04-solution-strategy.md:27-29` | Affected; extends ADR-0018 widget strategy | Uses `command watn -- "$question"`, trailing normalization, flattening, and capture-only assignment | Production changes limited to the three blocks | `src/shell_shortcut.rs:439-527` | Yes |
| 5. Building blocks: Shell Shortcut and Line editor boundary responsibilities | `docs/arc42/05-building-block-view.md:56-57` | Affected; updates widget responsibilities | Generated blocks build comment-plus-command buffer | Bash, Zsh, Fish, and E2E tasks pass | Native block constants and registered capability steps | Yes |
| 6. Runtime: Ctrl-W flow, failure preservation, Enter execution isolation | `docs/arc42/06-runtime-view.md:138-165` | Affected; updates Ctrl-W flow | Empty/failure preserve; success flattens and inserts; Enter ignores comment | Failure, commit-isolation, flattening, and E2E evidence | Real `bash -c` commit step and process E2E | Yes |
| 8. Cross-cutting concepts: request preservation and no-evaluation safety | `docs/arc42/08-crosscutting-concepts.md:58-69` and `docs/arc42/11-risks-and-technical-debt.md:57` | Affected; documents flattening and no-evaluation guarantees | CR/LF/TAB become spaces; embedded generated breaks remain text | Flattening and no-evaluation scenarios pass | Bash and shell-block assertions | Yes |
| 9. Architecture decisions: ADR-0018 summary | `docs/arc42/09-architecture-decisions.md:104-116`; `docs/adr/0018-safe-shell-shortcut-installation-and-native-widgets.md:69-85` | Affected; ADR summary updated | Portable comment fallback retained across shells | All widget scenarios and E2E pass | Three native blocks and atomic installer unchanged | Yes |
| 10. Quality requirements: QS-051, QS-052, QS-055 | `docs/arc42/10-quality-requirements.md:110-114` | Affected; adds request-preservation and execution-isolation quality scenario | Explicit failure, multiline, cursor, no-evaluation, and comment assertions | Full regular and E2E suites pass | `105` regular scenarios and `68` E2E scenarios | Yes |
| 11. Risks: shell differences, unsafe output, flattened comment | `docs/arc42/11-risks-and-technical-debt.md:50-57` | Affected; documents testability and flattening risk | Bash subprocess is the real interface; Zsh/Fish use content and syntax checks | Zsh/Fish scenario and Bash E2E pass | `zsh -n` is conditional when the executable is unavailable; Fish syntax and generated contracts pass | Yes |
| 12. Glossary: Shell widget, Request comment, Request flattening | `docs/arc42/12-glossary.md:72-78` | Affected; adds preservation terms | Uses the same terms in design and feature specification | All scenarios use the terms consistently | Feature text and generated block implementation match | Yes |

**ARC42 CONFORMANCE: CLEAN**

## Coverage

Coverage was measured, not inferred. `./measure-coverage.sh` completed both regular and E2E instrumented runs and wrote `coverage/non-e2e-cobertura.xml` and `coverage/e2e-cobertura.xml`; `./merge-coverages.sh` wrote `coverage/cobertura-coverage.xml`.

- Merged line coverage: `9164/10128` (`90.48%`).
- `src/shell_shortcut.rs`: `271/319` lines (`84.95%`) in the merged report.
- The Gherkin runner binary was included in both instrumented runs.
- The changed Bash success and flattening paths are exercised by regular and E2E scenarios. Zsh and Fish construction paths are exercised by generated-block assertions; Fish syntax is checked locally, while the local environment has no `zsh` executable. Missing Zsh/Fish executables do not fail local tests; CI sets the required checks and installs the shell dependencies.
- The remaining uncovered lines in the affected module are existing installer error/platform branches or interactive shell redraw behavior. The latter is legitimately hard to test without an interactive ZLE/Fish session; generated syntax/content and the real Bash line-editor subprocess cover the portable behavior promised by this change. No new missing scenario was identified.
- The current givn artifact manifest exposes no separate `coverage` artifact, so `givn check coverage` is not available; the configured coverage commands produced the measured Cobertura reports directly.

## Verification

- `givn lint --change preserve-ctrl-w-requests-in-shell-config`: clean.
- Regular runner: `19 features`, `105 scenarios`, `604 steps`, all passed.
- E2E runner: `23 features`, `68 scenarios`, `473 steps`, all passed.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `git diff --check`: passed.
- `tasks.md`: `33/33` tasks checked with scenario commit hashes recorded.

## Sign-Off

- Fabrication audit: clean.
- Strict-mode proof: present and non-zero for the registered stub.
- Production implementation: present in `src/shell_shortcut.rs`.
- E2E interface: real Bash subprocess with primary buffer/output assertions.
- Arc42 implementation conformance: clean.
- Coverage: measured across regular and E2E runner binaries.
- No `@wip` tags remain in the delta.
- Both configured runners pass and the E2E filter is a strict subset.
- Interaction inventory maps to the design matrix, feature scenario, and step driver.

REVIEW: PASS
