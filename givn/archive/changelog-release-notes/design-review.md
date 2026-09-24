# Design review: changelog-release-notes

Reviewed plan: `proposal.md`, `specs/fragments/release-truth.feature`,
`design.md`, `arc42.md`, against `cliff.toml`, `.github/workflows/release.yml`,
`.github/workflows/ci.yml`, `tests/features_runner.rs`, and the existing
`tests/steps/release_truth_steps.rs` / `release_truth_e2e_steps.rs`.

Grilling ran as a fresh-context subagent against the plan as it stood after
design. Every branch below was walked; the subagent verified the template
behaviour against the project's pinned git-cliff 2.13.1 and the repository's
real release ranges.

## Findings and dispositions

| # | Finding | Disposition |
|---|---|---|
| 1 | The D1 guard existed only in the prepare job's generation step; a tag-recovery dispatch skips `prepare` and the `publish` job validated only the release heading before `cargo publish`, so a notes-less recovery could publish a crate | Fixed: `design.md` places the guard in the prepare generation step and in the publish job's `Validate release identity` step; the guard is review-verified because workflow YAML is outside the acceptance suite |
| 2 | The `@e2e` scenario's assertion steps sat in the regular step file, against the separate-e2e-steps policy | Fixed: the e2e fixture and assertion steps move to `release_truth_e2e_steps.rs`; only the shared generation step stays in the regular file, justified in `design.md` as the capability's common CLI interface |
| 3 | The skip-all fallback was untested; no fixture commit could reach `Other Changes`, so the group-absence assertion was vacuous for that group | Fixed: the spec's second scenario fixture adds an uncategorised revision, which the old configuration renders under Other Changes and the new one skips |
| 4 | The design listed the dropped skip rules but not `^fix(e2e)`, leaving it ambiguous whether e2e test fixups stay excluded | Fixed: `design.md` states the `^fix(e2e)` skip stays ahead of the generic fix rule |
| 5 | `unique(attribute="message")` compares the subject only; two changes with identical release notes would collapse to one entry | Accepted limitation, recorded in `design.md`: release notes are unique sentences, a cross-change collision is a review finding, and the filter cannot take a composite key |
| 6 | `arc42.md` reported `lower_level_artifact: PASS` while routing to a lower-level artifact; the dimension fails when the lower-level artifact is the proper complete home | Fixed: verdict now reads `FAIL` with one canonical destination, the release-truth capability spec |
| 7 | The named persona's contract is shell-interaction-focused; its relevance to changelog wording is weak | Recorded in `design.md` as a persona disposition: named as the release-note consumer, no contract element contradicted, no persona promoted |
| 8 | Two step-table rows used `<note>` without marking it as a cucumber expression capture | Fixed: the table marks the capture; all other rows match the feature file verbatim |

## Branch dispositions

- **Scope:** spec matches the proposal; no extra or missing observable
  behaviour after fix 3.
- **Tech choices:** the `cliff.toml` change with `unique(attribute="message")`
  is sufficient and simpler than post-processing; verified against
  `v0.5.0..v0.5.1` (only Bug Fixes and Features) and `v0.5.1..HEAD` (only
  Features) with git-cliff 2.13.1.
- **Missing scenarios:** none beyond fix 1's untestable workflow guard, which
  is recorded as review-verified rather than silently deferred.
- **Testability:** both scenarios fail RED today (three duplicate lines plus a
  Documentation line; Documentation, Refactoring, and Other Changes groups);
  Then-steps assert concrete values read from the real CLI output.
- **E2E fidelity:** interface CLI with a real subprocess driver; the fragment
  records no interactions, so the coverage matrix has no inventory rows; the
  mandatory `@e2e` scenario is present and distinct from the regular scenario
  (collapse versus group selection).
- **Visual contract:** no rendered screen; no contract section required.
- **Persona disposition:** see finding 7.
- **Decision ledger:** D1 resolved in `design.md`; no unrecorded question.
- **ADR qualification:** independently re-checked; NOT_QUALIFIED is correct,
  and the canonical destination is now a single artifact.
- **arc42:** all 12 rows independently walked; chapters 11 and 12 are the only
  affected chapters and both are updated; chapters 4-7 correctly unaffected
  because the release workflow is not a Watn component, runtime, or deployment
  artifact.
- **Ubiquitous language:** `Release note` and `Changelog` are recorded in
  `docs/arc42/12-glossary.md` and used consistently in proposal, spec, and
  design.
- **Risk:** the most likely failure is the guard sitting where it never runs
  on a recovery release (finding 1) — mitigated by the second guard; the
  runner-up is an accidentally re-included skip rule, mitigated by the
  uncategorised and duplicate-subject RED fixtures.

DESIGN-REVIEW: PASS
