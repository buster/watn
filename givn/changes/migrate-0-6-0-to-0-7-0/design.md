# Design: migrate-0-6-0-to-0-7-0

## Migration metadata

- source_version: `0.6.0`
- target_version: `0.7.0`
- upgrade_commit: `29b032cc1d6c10b0e87c4e05fd286dfdd4777acb`
- migration_type: aggregate maintenance change without a product specification

## Execution order

Complete the following phases strictly in sequence. Do not create independentparallel migration changes for these bundles. The complete sequence is alsorecorded in `migration.yaml`.

### Phase 1: migrate-0-6-0-to-0-7-0 (0.6.0 -> 0.7.0)

# Migration Design: 0.6.0 to 0.7.0 (archive proof)

This bundle is the project-specific 0.6.0 → 0.7.0 phase (Phase N in the
aggregate plan). Managed preparation was recorded in Commit A
`29b032cc1d6c10b0e87c4e05fd286dfdd4777acb`. Read this file completely before editing anything.

## When this applies

Apply this migration when the project archives changes with Givn
0.7.0 but was created with 0.6.0. The managed
refresh has already updated Givn-owned config, skills, commands, and
instructions. Do not rewrite behavior, specifications, or scenarios.

## Preflight

1. Record the Givn version (`givn --version`), the project marker
   (`givn_config_version` in `givn/config.yaml`), and the current commit.
2. Stop on a dirty worktree unless the dirt is the migration work itself.
3. Resolve the Git top level:
   `git rev-parse --show-toplevel`. If it fails, the project is not a Git
   repository. If it differs from the project root, the project is nested in
   a larger repository. Either case is a publication-boundary decision the
   human must make (initialize at the project root, or accept that Givn
   cannot archive here). Do not silently move or re-initialize anything.
4. Read `verify.command`, `verify.e2e_command`, and the coverage commands from
   `givn/commands.yaml`; open the files each command invokes.

## Phase 1: inventory the verification scopes

Write a short `runner-result-contract.md` in the maintenance change with one
row per scope:

| Scope | Command | Script/file that runs scenarios | Where counts come from today |
|---|---|---|---|
| regular | `verify.command` | | |
| e2e | `verify.e2e_command` (if configured) | | |

Skip the e2e row only when `verify.e2e_command` is empty or the sentinel
(`givn missing-e2e-runner`), and state that explicitly. An `@e2e` scenario
with no configured runner still blocks the archive before any change is
published; if the project has `@e2e` scenarios, configure the runner as part
of this migration.

## Phase 2: emit the runner result

Each scope command must write one JSON object to the path named by the
environment variable `GIVN_RESULT_FILE`:

```json
{"scope":"regular","total":12,"passed":12,"failed":0,"skipped":0}
```

Rules:

- `scope` is exactly `regular` for `verify.command` and exactly `e2e` for
  `verify.e2e_command`.
- Counts are scenario counts produced by the runner, never hardcoded and
  never inferred from the exit code alone. A wrapper that always writes
  `{"total":1,"passed":1}` is fabrication and must not be accepted.
- Fail-closed semantics: Givn rejects a missing file, malformed JSON, a wrong
  scope, `total == 0`, `failed != 0`, `skipped != 0`, and
  `passed + failed + skipped != total`.
- The command's exit code still reflects the run. Write the result whenever
  the runner produced counts, even when scenarios failed; Givn rejects the
  failure either way.
- When `GIVN_RESULT_FILE` is not set (developer runs the script directly),
  keep the previous behavior; only write when the variable is present.

How to get counts per runner family:

- **cucumber-rs**: run the world with the default `Summarize` writer and read
  `scenarios_stats()` (`total()`, `passed`, `failed`, `skipped`) after the run;
  write the JSON before exiting with the failure code.
- **cucumber-js**: run with `--format json:<file>`, then derive scenario
  statuses from the report's step results (a scenario is `passed` when every
  step passed, `failed` when any step failed, `skipped` when it was not
  executed) in a small Node script and write the result.
- **behave**: run with `--format json --outfile <file>` and derive each
  scenario's status from its steps (a scenario is `passed` when every step
  passed, `failed` when any step failed, `skipped` when it was not executed),
  then write the result. Do not use `--format plain` and count lines.
- **pytest / unittest**: run with `--junitxml` or the JSON reporter, parse the
  report, and write the result.
- **Other runners**: switch the invocation to the runner's machine-readable
  output and parse it. If no machine-readable output exists, stop and report
  the gap to the user instead of inventing counts.

Wrapper pattern for a project-owned script (replace the marked section with
the project's real count source):

```sh
#!/bin/sh
scope=regular  # use e2e in the script behind verify.e2e_command
# ... existing runner invocation, captured so counts can be read ...
runner_status=$?
result_file="${GIVN_RESULT_FILE:-}"
if [ -n "$result_file" ]; then
    # TODO: replace with the runner's real counts (never hardcode).
    total=0; passed=0; failed=0; skipped=0
    printf '{"scope":"%s","total":%s,"passed":%s,"failed":%s,"skipped":%s}\n' \
        "$scope" "$total" "$passed" "$failed" "$skipped" > "$result_file"
fi
exit "$runner_status"
```

`scope` is fixed per script: use `regular` in the script behind
`verify.command` and `e2e` in the script behind `verify.e2e_command`.

## Phase 3: route the commands

Keep `verify.command` and `verify.e2e_command` pointing at the project-owned
scripts; only the scripts change. If the scope wrapper is new, update the
commands file and the README development managed block through the normal
`givn` workflow (the archive refreshes the managed block itself).

## Phase 4: confirm the Git publication boundary

- The project root must equal `git rev-parse --show-toplevel`.
- Commit the migration work; the archive starts from a committed state. Only
  declared evidence paths may be dirty: the target archive directory always,
  `README.md` when the readme addon is enabled, and the configured coverage
  paths when the coverage addon is enabled.
- Legacy archives without `verification.json` stay historical and are never
  re-published; do not fabricate receipts for them.
- `givn check review` no longer runs the suite. Record the review evidence
  from the fabrication audit, static checks, and the archive receipt.

## Phase 5: validate

1. For each scope, run the command manually with a temporary result file and
   confirm the JSON parses and reconciles:

   ```sh
   result=$(mktemp)
   GIVN_FEATURES=givn/specs GIVN_RESULT_FILE="$result" <scope command>
   cat "$result"
   ```

2. Run `givn lint` and the regular suite normally once to confirm the runner
   still works outside Givn.
3. Complete the maintenance change through the normal workflow and archive it.
   The receipt must show both scopes `passed` (or e2e `not_applicable` when
   the merged corpus has no `@e2e` scenario) and `status: "proven"`.
4. Record before/after commands and the sample result JSON in
   `runner-result-contract.md`.

## Blocking checks

- Both configured scopes write a valid result under `GIVN_RESULT_FILE`.
- No scenario count is hardcoded or inferred from the exit code.
- The project is a Git worktree whose root is the project root, and the
  migration work is committed.
- No specification or product behavior changed in this migration.
- The first archive after the migration is proven. If it fails after the
  candidate commit, the change is already in `givn/archive/<id>/` with a
  `pending` receipt: repair the committed state, then rerun
  `givn archive --change <id>` (do not recreate the merge). `givn status`
  lists the unproven candidate and routes to the resume command.

## Evidence files

Retain in the maintenance change:

```text
runner-result-contract.md
```


## Completion boundary

This generated design does not contain `DESIGN-REVIEW: PASS`, `REVIEW: PASS`, or implementation evidence. The project LLM must derive tasks and complete the normal Givn gates after the ordered bundle phases are understood.
