# Proposal: changelog-release-notes

## Release note

The changelog lists one entry per change, taken from the change's release note, and no longer carries documentation, refactoring, or cleanup noise.

## Use-Case Context

- Use-case ID: corpus-infra (fragment), capability release-truth
- Ideation topic: none
- Seed path: none
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

Every scenario used to write its own revision subject, so one feature became
dozens of changelog lines: release 0.5.0 carries 34 lines for the explain
command, eleven of them exact duplicates. The configuration also emits
Documentation, Refactoring, and Other Changes groups, so a reader wades
through planning notes and "Satisfy rustfmt and clippy gates" to find what
actually changed.

Since every revision of a change now carries the change's release note as its
subject, the same configuration would instead repeat one line once per
revision. Neither shape tells the reader what a release changed.

## Proposed Solution

A release section lists each change exactly once. The entry text is the
change's release note; repeated revisions of the same change collapse into one
line.

Only user-visible change appears: features, bug fixes, performance changes,
reverts, and breaking changes. Documentation, refactoring, tests, chores, and
cleanup do not appear.

The released section keeps its current place: it is inserted into the changelog
file and becomes the GitHub release body.

## Capability Routing

`givn spec route` reports no signal; the decision is recorded with its
rationale.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| release-truth | no signal | `NEW in fragments` | The capability already lives in the `corpus-infra` fragment (`fragment.md` declares it) and the changelog is the release's public statement of what watn does; the checker resolves `EXTEND` only against the use-case corpus, so the fragment form is the declared decision. |

## Out of Scope

- Rewriting release sections that are already published; they shipped and stay
  as they are.
- The rest of the release workflow: version, package, publish, tag, and the
  GitHub release body shape.
- The givn-side rule that authors release notes (delivered separately); watn
  consumes the resulting subjects.
- Givn's own changelog and every other project's changelog.

## Open Questions

### D1 — What happens to a release with no user-visible change?

- Question: When a release range contains no feature, fix, performance change,
  revert, or breaking change, does the release stop or carry an empty section?
- Options:
  1. **Release stops** — the release check that requires a release section
     fails with a clear message; gain: no release ships without notes; cost: a
     maintenance-only release needs a manual note or a wider range; in
     practice: the maintainer sees the missing-section failure and cancels the
     release.
  2. **Empty section allowed** — the release carries a section with no
     entries; gain: maintenance releases still ship; cost: a release body that
     says nothing; in practice: the GitHub release carries only the crates.io
     link.
- Disposition: carried — design decides; owner: design.
