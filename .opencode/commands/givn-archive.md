---
description: Archive a completed givn change — runs gate hooks, merges delta specs, moves to archive/
---

Archive a completed givn change.

**Input**: Optionally specify a change ID after `/givn-archive`. If omitted, infer
from conversation context, or auto-select if only one active change exists. If
ambiguous, list changes and ask.

---

## Steps

### 1. Resolve the change and run pre-flight checks

```sh
givn status --change <id> --json
```

Identify the active change. Announce: "Using change: <id>".

From the JSON, check:
- `artifacts` where `id == "tasks"`: if `status != "done"`, warn and confirm.
- `artifacts` where `id == "review"`: if `status != "done"`, stop — direct
  the user to `/givn-review` first.
- `all_required_done`: if false, list what is missing and stop.

**Review:**
Check that `givn/changes/<id>/review.md` exists and contains `REVIEW: PASS`.
If absent, stop and direct the user to `/givn-review` first.

**Spec lint:**
```sh
givn lint
```
- Exit 1 (parse error): STOP — fix syntax errors before archiving.
- Exit 2 (@wip remaining): STOP — all `@wip` tags must be removed before
  archiving. The spec must be fully implemented.
- Exit 0: proceed.

### 2. Run the archive command

```sh
givn archive --change <id>
```

`givn archive` automatically enforces the gate:
1. All `apply.requires` artifacts exist (tasks.md, review.md by default).
2. All tasks checked (`[x]`).
3. `review.md` contains `REVIEW: PASS`.
4. Post hooks of all `apply.requires` artifacts pass — this runs the test suite.
   The suite **must be GREEN** for the archive to proceed.

If any gate check fails, `givn archive` exits non-zero with a clear error.
Report the error to the user and stop.

### 3. Verify the result

On success, `givn archive` will have:
- Run the test suite via the configured hooks (final GREEN confirmation).
- Merged delta `.feature` files into `givn/specs/` (stripped `@givn.*` tags).
- Moved `givn/changes/<id>/` to `givn/archive/<id>/`.

Run a regression check against the permanent specs:
```sh
givn lint
```
Exit 0 = permanent specs are clean. Exit non-zero = report to user (the archive
completed, but a problem in the permanent specs needs attention).

### 4. Show summary

```sh
givn status
```

---

## Output on success

```
## Archive complete: <id>

**Archived to:** givn/archive/<id>/

### Delta specs merged into givn/specs/
  + <capability>: N scenarios added
  ~ <capability>: N scenarios modified
  - <capability>: N scenarios removed

### Gate result
All hooks passed — test suite GREEN.

### Regression check
givn lint → clean (permanent specs well-formed).
```

## Output on gate failure

```
## Archive blocked: <id>

**Gate failed:** <error from givn archive>

**Action required:**
<specific guidance based on the failure>
```

---

## Guardrails

- NEVER archive with `@wip` scenarios remaining — the spec must be fully
  implemented and GREEN.
- NEVER archive without `REVIEW: PASS` in review.md.
- Do not bypass `givn archive` by moving files manually — the `.feature` merge
  pipeline must run.
- If the archive command fails, fix the gate failure rather than working around it.
- Report the regression check result even if the archive succeeded.
