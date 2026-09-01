# Design Review: quicksetup-first-run

## Phase 1: Grilling

Grilling was performed by a fresh-context subagent over the full planning set
(proposal, both delta features, design, arc42 assessment, ADR-0026) and the
relevant codebase (main.rs entry gate, config load/save, provider setup seams,
shell install machinery, PTY test harness). Findings and resolutions:

### Blocking

1. **Space-toggle shell list is unimplementable without raw mode.**
   `design.md` specified a Space-toggled `[x]` list but also a plain
   `read_line` flow; canonical mode buffers keystrokes until Enter, so
   per-row Space presses are unobservable. Resolution: typed answer contract
   — shell names toggle rows and re-render the list, an empty line confirms
   the current selection. Design and arc42 wording (03/06) updated. No raw
   mode introduced (consistent with ADR-0026).

2. **PATH prepend does not hide real shells.** The detection scans all
   `$PATH` directories; prepending the stub dir still finds the runner's real
   `/usr/bin/fish`, so the pre-selection scenario would fail as designed.
   Resolution: `isolate_quicksetup_env` now **replaces** `PATH` with exactly
   `<tmp>/bin`. Safe because the watn binary is spawned by absolute path and
   shell targets resolve via `HOME`/`XDG_CONFIG_HOME`. Design updated.

### Major

3. **"Both integrations per shell" under-asserted.** The completion e2e only
   asserted one block type per shell. Resolution: added the missing
   Both-block assertions for Bash, Zsh, and Fish.

4. **Scenario 5 accepted a credential suggestion that could not exist**
   (no key env set; isolation strips inherited credentials). Resolution: the
   pre-selection scenario now sets `OPENROUTER_API_KEY`.

5. **Explicit provider/model bypass unpinned.** The first-run quick setup
   sits inside the existing `!explicit_provider && !explicit_model` gate
   (main.rs:211-216); a refactor hoisting the config-existence check could
   hijack explicit-provider runs. Resolution: new regular scenario pins that
   `WATN_PROVIDER` runs never show the quick setup; design states the
   placement contract explicitly.

6. **First-run scenario asserted "no original request" without a sentinel.**
   Resolution: the catalog-request sentinel is now installed in the first-run
   and abort scenarios (without `WATN_PROVIDER`, which would skip the branch)
   and in the overwrite scenario.

### Minor (all resolved)

7. Config-location output promise had no assertion → added a Then step.
8. The abort e2e drove the explicit command instead of the first-run trigger →
   now driven by `watn "hello"` with announcement and sentinel assertions; the
   explicit-run abort remains a regular scenario.
9. Wrong step file named for the authoritative-tree update → corrected to
   `tests/steps/shell_completions_e2e_steps.rs` (contains-based step body).
10. Confirm-time failure paths unpinned → two new scenarios: config-write
    failure installs nothing (`WATN_TEST_FAIL_CONFIG_WRITE` seam); shell
    install failure keeps the saved configuration and reports nonzero.
11. OpenAI-mapped endpoint variant unpinned → new scenario: openai endpoint
    suggests `${OPENAI_API_KEY}` and no model.
12. Stdout flushing unspecified → design requires `flush()` before each read.
13. shell-completions delta lacked the inventory comment block → added.
14. "Five questions" vs six → aligned to six plain-line interactions
    (endpoint, credential, three models, shell selection) in ADR-0026 and
    chapter 04.

### Verified during grilling (no action)

- Ctrl-C abort via PTY is observable and side-effect free (no trap installed
  on the quick setup path; nothing persists before confirm); abort scenarios
  correctly assert state, not exit status (signal death is not exit code 130
  in the harness).
- Persistence seams verified: `normalize_endpoint`, `provider_name`,
  `build_provider_draft`, `${VAR}` verbatim persistence with request-time
  expansion, atomic 0600 save, openai→custom migration semantics matching the
  overwrite scenario, aggregated install reports.
- `config_file_exists()` helper is justified: `load_config()` cannot
  distinguish missing from empty files.
- Isolation plumbing verified: `world.env_vars` applied last in subprocess and
  PTY paths; `WatnWorld` env scrub on drop; `.fail_on_skipped()` strict mode
  configured; `verify.e2e_command` matches `givn/commands.yaml`.
- Inventory normalization: 4 entries ↔ 4 `@e2e` scenarios; regular scenarios
  assert distinct invariants (no duplicate-layer scenarios).
- ADR-0026 qualification: all five mandatory dimensions pass with evidence;
  NEW_ADR is correct (ADR-0011 is already superseded, so amending it would be
  wrong); register row and summary present.
- arc42: all 12 chapters independently re-derived; no diff with the
  self-report; chapter 09 and 11 carry the ADR and its consequences; blocking
  findings 1/2 also triggered wording fixes in chapters 03/06.

## Phase 2: Hardening

Applied to the artifacts:

- `specs/quicksetup/quicksetup.feature`: shell-answer contract realized in
  steps; added Both-block assertions, config-location assertion, key-env
  Given, sentinel Givens, first-run abort via `watn "hello"`, and five new
  regular scenarios (provider bypass, OpenAI variant, write failure, install
  failure, plus the existing set). 13 scenarios, all `@givn.added @wip`
  (4 with `@e2e`).
- `specs/shell-completions/shell-completions.feature`: inventory block added;
  the five modified authoritative-tree scenarios now include `quicksetup`.
- `design.md`: typed shell answer contract, PATH replacement, flush
  contract, entry-gate placement contract, untrapped Ctrl-C rationale,
  save/install failure semantics, sentinel placement, step-file correction,
  matrix wording.
- arc42: chapters 03, 04, 06 wording aligned; ADR-0026 question count fixed.
- `givn lint --change quicksetup-first-run`: 2 files checked, 13 findings,
  all expected `@wip` markers — exit condition satisfied (lint reports WIP
  tags only; no structural findings).

## Sign-off

All blocking and major findings resolved; minors dispositioned. The two
blocking findings were contract-level (interaction mechanics, test
isolation) and are fixed in design and fixtures, not deferred to tasks.

DESIGN-REVIEW: PASS
