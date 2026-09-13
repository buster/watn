# Review: explain-command

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, no assertion reference):

```
grep -nE "fn .*\{\s*\}|=> \{\}|unimplemented!|todo!|panic!\(\"TODO" tests/steps/explain_command_steps.rs tests/steps/explain_command_e2e_steps.rs
-> only match: tests/steps/explain_command_steps.rs:199 `_ => {}` — an ANSI
   state-machine match arm inside the local strip helper, not a step body.
0 empty step bodies found across 2 step definition files (474 + 141 lines).
Every step body either asserts, drives the real binary/PTY, configures a mock,
or reuses an existing proven step.
```

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit that touches production source (not just spec/stub files):

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Setup + S1 Enter closes | `2ac3c1e` | Yes: `src/main.rs`, `src/review/panel.rs`, `src/review/response.rs`, `src/review/session.rs`, `src/review/mod.rs` |
| S2 metacharacters verbatim | `0319495` | Test-only; production delivered in `2ac3c1e` (step-reuse/coverage commit) |
| S3 option terminator | `b4c3ec4` | Test-only; exercises `resolve_explain_command` from `2ac3c1e` |
| S4 stdin quoting/line breaks | `abc1a8c` | Yes: `src/review/panel.rs` `ControllingTerminal::open` eligibility |
| S5 piped without marker | `af0c529` | Test-only; exercises stdin resolution from `2ac3c1e` |
| S6 positional precedence | `9ca91e1` | Test-only; exercises precedence from `2ac3c1e` |
| S7 empty input rejected | `082b63f` | Test-only; exercises rejection from `2ac3c1e` |
| S8 second positional refused | `eb9caf1` | Test-only; exercises clap contract from `2ac3c1e` |
| S9 no credential, no request | `983a459` | Test-only; exercises readiness gate from `2ac3c1e` |
| S10 failed explanation | `a25714b` | Test-only; exercises degradation from `2ac3c1e` |
| S11 non-covering not trusted | `4d2d6e7` | Test-only; exercises `apply_explanation` from `2ac3c1e` |
| S12 never executes | `f47e40f` | Test-only; exercises `-x` rejection from `2ac3c1e` |
| S13 review disabled | `75fc633` | Test-only; exercises explain bypassing the preference from `2ac3c1e` |
| S14 switches inert | `4f52d82` | Test-only; exercises ignored flags from `2ac3c1e` |
| S15 terminal required | `c76e808` | Test-only; exercises `explanation_terminal_is_usable` from `2ac3c1e` |
| S16 malformed config | `fea98a4` | Test-only; exercises config error propagation from `2ac3c1e` |
| E1 @e2e explanation card | `7fd7258` | Yes: `src/review/panel.rs` explain-mode arrow navigation |
| CI formatting/clippy fix | `9593add` | Yes: `src/main.rs` (formatting only, no behavior change) |

No commit touches only the `.feature` file or an empty stub. Test-only commits
carry real step implementations and real assertions against production code
delivered in `2ac3c1e`, `abc1a8c`, and `7fd7258`.

**Promised-architecture cross-check** — every component design.md said would
be created actually exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| `Commands::Explain` + `run_explain_command` + `explain_system_prompt` | Yes | `src/main.rs` |
| `resolve_explain_command` / `read_stdin_command` | Yes | `src/main.rs` |
| `run_explanation_card(candidate, context) -> Result` | Yes | `src/main.rs` |
| `apply_explanation` | Yes | `src/review/response.rs` |
| `explain_command_candidate` | Yes | `src/review/session.rs` |
| `explanation_terminal_is_usable` | Yes | `src/review/panel.rs` |
| Regular steps file | Yes | `tests/steps/explain_command_steps.rs` |
| E2E steps file | Yes | `tests/steps/explain_command_e2e_steps.rs` |

**Strict-mode proof** — confirmed present and passing in tasks.md setup task:

```
./run-tests.sh --name "Scratch undefined step fails" -> exit 1
 ✘ Given a step that is not defined anywhere in this project
   Step failed: Step doesn't match any function
[Summary] 1 scenario (1 failed), 1 step (1 failed)
```

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps
checked for a primary assertion on the observable output, return value,
acknowledgement, or visible state of the interface named in design.md, not
repository-only; for a browser-UI capability, the step definitions checked
for actually driving the browser (click/type/navigate), not an HTTP request or
in-page `fetch()`:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Developer explains an existing command in the review card | Yes — asserts on the real PTY transcript of the explanation card (stage texts, model purposes, close hint, `watn` header, no internal identifiers) | N/A (CLI); driven as a real `watn` subprocess in a `portable-pty` session through `sh -c` with redirected stdout, real arrow keys, real `Escape` | No |

**E2e scope check** — each capability has one `@e2e` scenario per distinct
happy-path action, not one per variant/error case:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| explain-command | 1 | Yes — one action: explain an existing command in the review card | No |

**Duplicate-implementation check** — the exact file(s) `verify.e2e_command`
invokes, and confirmation the tree (including untracked files, via `git
status`) was searched for any other/parallel e2e implementation of the same
capability that might be stronger than the one being reviewed:

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| explain-command | `./run-tests.sh --e2e` -> `cargo test --test features_runner --tags '@e2e and not @wip'`; collects `tests/steps/explain_command_e2e_steps.rs` | No — only the two registered files (`git status` clean; `grep -rln "watn explain" tests/steps` finds only the new files) | N/A |

**Local environment check** — the local run command (design.md's Local
Runnability section) starts the full stack, including digital twins,
cleanly:

```
cargo run --release --locked -- --help -> exit 0, release profile finished cleanly
printf '' | ./target/debug/watn explain 'ls' -> "explain requires a terminal for the explanation card"
No containers, database, port, or live network: the product is a one-shot CLI,
and the only external service (the OpenAI-compatible endpoint) is replaced by
the in-process httpmock twin in every scenario. No digital twin is missing.
```

**E2e isolation check** — `verify.command` and `verify.e2e_command` read
literally from `givn/commands.yaml`; if identical, this is always a finding
(the "e2e run" is not isolating `@e2e` scenarios at all). Isolation proof
from the two scopes' scenario counts (the archive receipt will carry the
durable record):

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2e scope scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 238 | 89 | Yes |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `givn/specs/use-shell/usecase.md`: capability `explain-command` added, interaction row added; the explained command is never evaluated/executed; the card closes without releasing a command | Yes |
| Capability ownership and typed relationships preserved | Capability owned by `use-shell`; `givn/specs/use-shell/explain-command.feature` is the delta; no relationship removed | Yes |
| Confirmed Persona-relevant outcomes preserved | `terminal-developer--interactive` can read the full stage stack and every stage purpose (arrow navigation) and close without execution; verified by E1 and the persona transcript review | Yes |
| Actors remain distinct from Persona biographies | Gherkin uses the developer action; the Persona remains a review lens, never an actor | Yes |
| Migration ledger and coverage evidence complete, if applicable | No migration in this change | N/A |

**Design.md conformance check** — implementation's actual commands, file
layout, and framework/driver choices diffed against what design.md
explicitly named. Any deviation implemented without first updating
design.md and re-running design-review is a finding, not an acceptable
shortcut:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Runner | `./run-tests.sh` / `./run-tests.sh --e2e`, cucumber-rs with `.fail_on_skipped()` | Identical | Yes | — |
| Step files | one per capability: `tests/steps/explain_command_steps.rs` + `tests/steps/explain_command_e2e_steps.rs` | Both exist, registered separately | Yes | — |
| Argument contract | single positional, `--`, `-`/stdin, no join, one trailing newline stripped, empty exit 2 | Identical in `resolve_explain_command`/`read_stdin_command` | Yes | — |
| Purpose application | `apply_explanation` exact echo or verbatim covering split; never replaces the command | Identical in `src/review/response.rs` | Yes | — |
| Terminal eligibility | `explanation_terminal_is_usable` (stderr TTY + `/dev/tty` + `TERM != dumb`); `ControllingTerminal::open` validates only that availability | Identical | Yes | — |
| `-x` rejection | `watn -x explain` exit 2 with the never-execute message; `watn explain -x` clap usage error | Identical | Yes | — |
| Step-table step `the file "..." should not exist` | listed as new | Reused the existing proven step from `preserve_ctrl_w_requests_steps.rs` instead of duplicating it | Yes (step reuse, not a design decision) | — |
| `-x` rejection scenario mechanism | design-review F11 suggested a PTY session | Implemented as a non-PTY subprocess run asserting `exit_status == 2` and the stderr message on separate channels; design.md's step table only requires the rejection and no-execution proof, and this assertion is stronger | Yes | — |

**Interaction coverage verification** — spec's User Interaction Inventory
cross-referenced against design's Interaction Coverage Matrix, `.feature`
file, and step definitions. Every inventory entry must have a matrix row,
a matching `@e2e` scenario, and step definitions that use the promised
driving mechanism:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| explain-command / explain an existing command in the review card | Yes (`design.md:402`) | Yes — "Developer explains an existing command in the review card" (`explain-command.feature:6`) | Real `watn` binary in a `portable-pty` session; one argv element; PTY transcript; Escape; redirected stdout + marker file | `start_pty_command` (`sh -c '"$WATN_BIN" explain "$WATN_COMMAND" > "$WATN_OUT"'`), `pty_write` arrows/Escape, `pty_snapshot`/`finish_pty_session`, stdout file read, marker-file assertion | Yes |

## Visual Review

| @e2e Scenario | Viewport | Evidence file | Present and non-empty? |
|---|---|---|---|
| Developer explains an existing command in the review card | terminal PTY 120x40 (bounded inline card) | `givn/changes/explain-command/evidence/visual/developer-explains-an-existing-command-in-the-review-card/transcript.txt` | Yes (3,376 bytes; both card states) |

| confirmed persona | Goal | Finding | Disposition |
|---|---|---|---|
| `terminal-developer--interactive` | Understand an existing command as a stage stack with model-written purposes without execution | No findings. Baseline passes: card frame visible, `esc close` labeled, `watn · review` header, no `explain_only`, stages/purposes not duplicated, English, no error state. Arrow navigation exposes `↳ Explains stage 1` and `↳ Explains stage 2`. Observation (not a finding): the evidence command is short, so long-command wrapping is not exercised by this transcript. | Accepted |

**Baseline:** every control labeled, no internal identifier visible, a real
header, no duplicated visible content, no error state on the happy path, the
configured language, no motion. All pass on the committed transcript.

VISUAL-REVIEW: PASS

## Retirements

N/A — this change retires nothing.

## README impact

README-IMPACT: updated - Usage

The Usage section gained the `watn explain '<command>'` example, the
`explain` line in the captured `watn --help` command list, and the
"Explain an existing command" subsection describing the invocation forms,
verbatim-delivery guarantee, arrow navigation, and `purpose-unavailable`
degradation.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| Requirement 40 (explain an existing command) | `docs/arc42/01-introduction-and-goals.md:40` | Yes | `## Scope`, argument contract | S1-S16, E1 | `src/main.rs` `run_explain_command`; feature file 17 scenarios | Yes |
| Context/interface row and context diagram edge | `docs/arc42/03-context-and-scope.md:18,45` | Yes | CLI behavior decisions | S2-S16 | `watn explain` subcommand in `--help` output; E1 PTY run | Yes |
| Review surface building block gains the explain edge | `docs/arc42/05-building-block-view.md:42` | Yes | Architecture impact | S1, E1 | `run_explanation_card` reused; `explain_only` state | Yes |
| Runtime flow "Explain an existing command" | `docs/arc42/06-runtime-view.md:238-251` | Yes | `### Explain runtime flow` sequence | S1-S16, E1 | `resolve_explain_command` -> readiness -> `explain_command_candidate` -> card | Yes |
| Verbatim command delivery cross-cutting concept | `docs/arc42/08-crosscutting-concepts.md:190-209` | Yes | `## The argument-passing contract` | S2-S8, S14, S16 | `resolve_explain_command`, `read_stdin_command`; scenarios assert exact bytes | Yes |
| QS-075..QS-078 | `docs/arc42/10-quality-requirements.md:64,147-150` | Yes | Purpose application semantics, CLI behavior | E1, S2, S9-S11, S13 | E1 asserts stages/purposes/close; S9-S11 degradation; S13 preference independence | Yes |
| R-081..R-085 | `docs/arc42/11-risks-and-technical-debt.md:83-87,198-199` | Yes | `## Risks introduced`; ADR routing | S3, S4, S12, S15, S16 | Scenario coverage for each risk; no ADR required (routed to design.md) | Yes |
| Glossary: Explained command, Explanation-only card, Verbatim command delivery | `docs/arc42/12-glossary.md:128-130` | Yes | Domain language throughout | All | Spec, design, code, and README use these exact terms | Yes |
| ADR routing (NOT_QUALIFIED -> design.md) | `arc42.md` `## ADR qualification` | Yes | Rationale owned by design.md | No ADR task | No `docs/arc42/adr/` created; `docs/adr/` untouched; existing ADR-0017/0018 adjacent only | Yes |

- [x] Every chapter marked affected by either the independent review or
      `arc42.md` was opened and checked for stale, contradictory, or placeholder
      content.
- [x] Every Arc42 fact or architecture decision relevant to this change is
      represented in `design.md` and mapped to one or more tasks in `tasks.md`.
- [x] The completed implementation follows those facts and decisions; evidence
      names the built file, command, test, or observable result.
- [x] Architecture changes discovered in the completed implementation are
      recorded in the durable Arc42 chapter and the change-level assessment.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

**Result:** CLEAN.

---

## Coverage

No archive receipt exists yet for this change (it has not been archived).
Classification below is therefore from static inspection of the committed
production diff plus the scenario-to-code mapping; the archive will measure
the committed candidate and record the merged result.

**Coverage result:** no measured receipt yet — classified from inspection.

```
No `givn/archive/explain-command/verification.json` exists yet.
Both scopes ran GREEN during implementation and review preparation
(238 regular scenarios, 89 e2e scenarios), but the archive receipt is the
durable measured authority.
```

## Coverage gap classification

Every uncovered region must be classified as exactly one of:
1. **Dead code** (YAGNI/KISS) → delete it.
2. **Missing test coverage** → fix step definitions, or add a scenario and run RED/GREEN/REFACTOR.
3. **Legitimately hard to test** — rare; requires concrete technical justification.

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/main.rs` `Commands::Explain`, `run_explain_command`, `resolve_explain_command`, `read_stdin_command`, `explain_system_prompt` | new | Covered by scenarios | S1-S8, S14-S16 (argument/terminal/error paths), S9-S11 and E1 (provider paths) |
| `src/review/response.rs` `apply_explanation` | new | Covered by scenarios | E1 (Ready exact echo), S11 (non-covering fallback), S2/S10 (unavailable degradation); module unit tests also cover the response matrix |
| `src/review/session.rs` `explain_command_candidate` | new | Covered by scenarios | E1 (success), S10 (provider failure), S13 (non-JSON payload) |
| `src/review/panel.rs` `explanation_terminal_is_usable` + `ControllingTerminal::open` eligibility | new/changed | Covered by scenarios | S15 (no terminal), S4/S5 (piped stdin with PTY), all card scenarios |
| `src/review/panel.rs` explain-mode arrow navigation | changed | Covered by scenarios | E1 (arrow keys read each stage purpose) |
| `src/main.rs` refactored `run_explanation_card` | changed | Covered by scenarios | All card scenarios plus the existing `-x` explain-choice scenarios |

No dead code found. No uncovered region required a bucket-2 addition or a
bucket-3 justification.

---

## Scenario execution evidence

```
unproven candidate — no receipt yet (the change has not been archived).
The archive gate will run both scopes and write
givn/archive/explain-command/verification.json.
```

**Receipt status:** not archived yet

## Overlap dispositions

For every shape-match finding involving this change's delta, one row.
Decision must be one of: `duplicate`, `variant`, `boundary`.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| Developer explains an existing command in the review card | Enter closes the explanation card without releasing the command | boundary |
| A command beginning with a dash passes after the option terminator | The review-panel switches are inert for explain | boundary |
| A command read from standard input keeps its quoting and line breaks | A piped command without the marker is explained | variant |
| Empty input is rejected without opening a card | A second positional argument is refused | variant |

Subset (`[SUBST]`) lint heuristics name the option-terminator, failed-provider,
and inert-switch scenarios as subsets of broader scenarios. Disposition:
`boundary` for each — each asserts a distinct invocation boundary (option
terminator, provider failure, inert flag) that the broader scenario does not
assert. No pair is a duplicate; merging would delete a distinct invariant.

## Split-or-keep

No scenario exceeds the lint length threshold; `givn lint` reported no
long-scenario finding.

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified,
      all promised components exist, no downgraded e2e scenarios, no e2e
      scenario driven via HTTP/fetch() in place of real browser interaction,
      no over-produced e2e scope).
- [x] Scenario execution evidence recorded per scope in the archive
      verification receipt (review does not run the suite).
- [x] Every coverage gap is classified and resolved (or justified) from the
      most recent measured evidence plus static inspection.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the full stack, including digital twins,
      cleanly.
- [x] The exact file(s) `verify.e2e_command` invokes were identified, and
      the tree (including untracked files) was searched for a parallel e2e
      implementation before accepting a weaker one.
- [x] `verify.e2e_command` is not identical to `verify.command`; scenario
      counts prove real isolation (or "all scenarios are @e2e" is stated).
- [x] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed; any deviation went through a design.md update
      + design-review re-run, not a silent shortcut.
- [x] Interaction coverage verified: every spec inventory entry maps to a
      design matrix row, an existing `@e2e` scenario, and step definitions
      using the promised driving mechanism — no mismatches.
- [x] Design.md's `## Internal Primitives` section accurately lists every
      internal data structure and abstraction this change introduces; no
      production-code struct/fn/enum that the step definitions rely on is
      omitted.
- [x] arc42 slug-mapping verified: every arc42 chapter slug in arc42.md
      was mapped to a real `docs/arc42/<slug>.md` file.
- [x] No finding anywhere in this report is excused with a classification
      outside the three buckets.

**REVIEW: PASS**
