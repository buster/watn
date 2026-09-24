# Tasks: changelog-release-notes

## Domain constraints

Owning contract: fragment `corpus-infra`, capability `release-truth`
(`givn/specs/fragments/fragment.md` records `Interactions: none`). The
fragment has no guarantees or actor goals; the changelog is repository output.

Persona lens: `terminal-developer--interactive` reads the published release
notes; it is a review lens, never a Gherkin actor.

Design constraints: `cliff.toml` keeps only `Breaking Changes`, `Features`,
`Bug Fixes`, `Performance`, and `Reverts`; the release note is the shared
subject of every revision of a change; the release guard is D1 in `design.md`.

Signature per revision:

```
feat(release-truth): The changelog lists one entry per change, taken from the change's release note, and no longer carries documentation, refactoring, or cleanup noise.
```

with the scenario or task title in the message body.

## T1 — Setup: runner and strict mode

The runner is already configured: `verify.command` is `./run-tests.sh`, the
e2e command is `./run-tests.sh --e2e`, and cucumber-rs strict mode is
`.fail_on_skipped()` at `tests/features_runner.rs:210`. No runner change is
needed.

Proof for this change: the first scenario's RED run, where a new step using
`unimplemented!()` must exit non-zero instead of being skipped. Record the
output under S1 RED.

- [x] Confirm `.fail_on_skipped()` is still present and `givn/commands.yaml`
      still names both commands; record the evidence.
- [x] Evidence:
      `rg -n 'fail_on_skipped' tests/features_runner.rs` →
      `210:        .fail_on_skipped()`.
      `rg -n 'command:|e2e_command:' givn/commands.yaml` →
      `command: "./run-tests.sh"` and `e2e_command: "./run-tests.sh --e2e"`.
      Strict-mode proof against this change's scenarios:
      `./run-tests.sh --e2e --name "Repeated revisions of one change yield one changelog entry"`
      → `1 scenario (1 failed)`, `Step doesn't match any function`,
      `exit status: 1`. An undefined step fails instead of being skipped.

## T2 — Release guard (design D1)

Add the user-visible-group guard to `.github/workflows/release.yml`: the
prepare job's `Generate reviewed changelog` step and the publish job's
`Validate release identity` step both extract the release section and require
at least one `### ` group before the version commit, tag, or `cargo publish`.

- [x] Add the guard to both steps; keep the `github_release` check as a
      backstop.
- [x] Manual evidence: the guard's `awk | grep` against the existing 0.5.0
      section prints `0.5.0 guard: PASS`; against a section with no `### `
      group it prints `empty-section guard: FAIL (expected)`.
- [x] COMMIT: `ef6f197` — release note subject, body
      `Task: release guard (D1) before the version commit, tag, and publish.`

## S1 — A changelog section keeps only user-visible groups

(`@givn.added`, regular, `release-truth`)

- [x] **RED** — remove `@wip` from this scenario only. Add the five new
      steps to `tests/steps/release_truth_steps.rs` with `unimplemented!()`
      bodies. Run
      `./run-tests.sh --name "A changelog section keeps only user-visible groups"`.
  - Evidence: exit=1; `1 scenario (1 failed)`; `Step panicked. Captured
    output: not implemented`; `Step doesn't match` was not needed because the
    step existed as a stub.
- [x] **GREEN** — implement the fixture, the git-cliff invocation, and the
      assertions; rewrite the `commit_parsers` in `cliff.toml` to the five
      user-visible groups plus the skip-all fallback (keep `^fix\(e2e\)`);
      add the pinned git-cliff install to the CI acceptance job.
  - Production files: `cliff.toml`, `.github/workflows/ci.yml`,
    `tests/steps/release_truth_steps.rs`.
  - Evidence: same command → exit=0, `1 scenario (1 passed)`, `5 steps
    (5 passed)`.
- [x] **REFACTOR** — `cargo fmt --all`; `cargo clippy --locked
      --all-targets -- -D warnings` → `Finished`; same command → exit=0,
      `1 scenario (1 passed)`.
- [x] COMMIT: `9bcec21` — release note subject, body
      `Scenario: A changelog section keeps only user-visible groups`.

## S2 — Repeated revisions of one change yield one changelog entry

(`@givn.added` `@e2e`, `release-truth`)

- [x] **RED** — remove `@wip` from this scenario only. Add the e2e fixture
      and assertion steps to `tests/steps/release_truth_e2e_steps.rs` with
      `unimplemented!()` bodies (the generation step is shared from
      `release_truth_steps.rs`). Run
      `./run-tests.sh --e2e --name "Repeated revisions of one change yield one changelog entry"`.
  - Evidence: exit=1; `1 scenario (1 failed)`; `Step panicked. Captured
    output: not implemented`.
- [x] **GREEN** — implement the fixture commits, the shared generation step,
      the once-only count, and the documentation-absence assertion; add
      `unique(attribute="message")` to the `cliff.toml` body template.
  - Production files: `cliff.toml`,
    `tests/steps/release_truth_e2e_steps.rs`.
  - Evidence: same command → exit=0, `1 scenario (1 passed)`, `5 steps
    (5 passed)`.
- [x] **REFACTOR** — `cargo fmt --all`; `cargo clippy --locked
      --all-targets -- -D warnings` → `Finished`; same command → exit=0,
      `1 scenario (1 passed)`.
- [ ] COMMIT: one atomic revision for RED+GREEN+REFACTOR; subject is the
      release note, body `Scenario: Repeated revisions of one change yield
      one changelog entry`. Record the revision hash in `tasks.md`.

## Evidence of the generated section

- [ ] Re-run both scopes (`./run-tests.sh`, `./run-tests.sh --e2e`) and
      regenerate a preview of a real range (for example `v0.5.1..HEAD` with
      the new configuration) as the change's visual evidence; commit the
      preview under `evidence/` if it adds information beyond the scenarios.
  - Evidence: commands and outputs; revision hash.
