---
name: givn-archive
description: Archive a completed givn change — runs gate hooks, merges delta specs into permanent specs, and moves the change to archive/.
---

# givn-archive

Archive change `<change-id>`.

## Context

- Change: `givn/changes/<change-id>/`
- Archive target: `givn/archive/<change-id>/`
- Permanent specs: `givn/specs/`
- Test runner: `./run-tests.sh`

## Archive gate

`givn archive` automatically checks:

1. All `apply.requires` artifacts exist (tasks.md, review.md by default).
2. All tasks checked (`[x]`) — each with evidence and commit hash
   filled in. A mass check-in via `sed` or other text replacement is
   not completion; the review fabrication audit and component diff will
   catch it and block archive anyway.
3. `review.md` contains `REVIEW: PASS`.
4. Hooks of all `apply.requires` artifacts pass (runs `./run-tests.sh`).
5. **README completeness gate**: with the readme addon enabled, the README
   must carry no `<!-- givn:todo: ... -->` open placeholders, no scaffold
   remnants, no damaged managed block, and must not be missing or
   marker-less. A blocked archive names every unfinished section — fill each
   one with real content before re-attempting. `givn readme check` reports
   the same findings any time.

If any gate fails, the archive is refused with a clear error. Do **not**
bypass a gate failure by mass-checking boxes — go back, genuinely
complete each task (RED→GREEN→REFACTOR→COMMIT with evidence), then
re-attempt archive.

## Steps

1. Confirm all tasks are done:
   ```
   givn status --change <change-id>
   ```

2. Confirm review is signed off:
   ```
   givn check review --change <change-id>
   ```

3. Archive:
   ```
   givn archive --change <change-id>
   ```

This will:
- Run the gate hooks (test suite must be GREEN).
- Merge delta `.feature` files into `givn/specs/` (strips `@givn.*` tags;
  preserves `@e2e` tags — they are not `@givn.*` tags).
- Move `givn/changes/<change-id>/` to `givn/archive/<change-id>/`.

## Post-archive: README update

After archive succeeds, if the project has a README, update it to reflect
the current project state:

1. **Project name**: If the README still contains `{{PROJECT_NAME}}`,
   replace it with the actual project name (from `Cargo.toml`,
   `package.json`, or the directory name).
2. **Coverage badges**: The archive gate runs the configured
   `coverage.measure_command`, then `coverage.merge_command`. Place the two
   generated badges directly after the README title. Both badges must link to
   the merged `coverage.output_path`, never only to the non-E2E report.
3. **Coverage section**: Below the badges, keep the `## Coverage` section
   with the merged line/branch totals, merged report link, and the exact
   measurement and merge commands. Use
   `<!-- givn:begin:coverage-badge -->` /
   `<!-- givn:end:coverage-badge -->` for the top badges and
   `<!-- givn:begin:coverage -->` / `<!-- givn:end:coverage -->` for the
   generated summary. Do not hand-edit either managed block.

**Example** — replace the values and paths with the output of the configured
project scripts:

The two badges, directly under the README title:

```markdown
# <project name>

<!-- givn:begin:coverage-badge -->
[![Line Coverage: 77.8%](https://img.shields.io/badge/line%20coverage-77.8%25-brightgreen)](coverage/cobertura-coverage.xml)
[![Branch Coverage: 64.2%](https://img.shields.io/badge/branch%20coverage-64.2%25-brightgreen)](coverage/cobertura-coverage.xml)
<!-- givn:end:coverage-badge -->
```

The coverage section, further down:

```markdown
## Coverage

The merged coverage report combines the non-E2E and E2E runs. It covers
`583/749` lines (`77.8%`) and `321/500` branches (`64.2%`).

Merged report: [coverage/cobertura-coverage.xml](coverage/cobertura-coverage.xml)

```sh
./measure-coverage.sh
./merge-coverages.sh
```
```

The managed coverage marker blocks are generated automatically during archive
and should not be manually edited. The examples above are a starting point;
adapt the scripts to your project's test runner and coverage tool. The final
report emitted by `merge_command` is the only source for the badges and totals.

## Verify command

Unit/integration:
```
./run-tests.sh
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
