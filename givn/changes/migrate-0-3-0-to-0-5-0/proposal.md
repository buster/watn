# Proposal: .migrate-0-3-0-to-0-5-0.tmp-1045447

## Aggregate migration

This maintenance change migrates a project from Givn 0.3.0 to theinstalled Givn 0.5.0 contract. Managed preparation was recorded inCommit A `8cd0d581f25b1f55f4975e8e6040139a11b2b235`. The migration phases below are one ordered plan; completeeach phase before continuing to the next.

## Scope

The LLM must inspect project evidence and derive project-specific tasks. It mustpreserve project-owned overrides and must not invent domain behavior, tests,architecture decisions, or provenance.

## Bundle proposals

### Phase 1: migrate-0-3-0-to-0-4-0 (0.3.0 -> 0.4.0)

# Migration Proposal: 0.3.0 to 0.4.0

## Problem / Opportunity

Givn 0.4.0 adds the automated delivery pipeline (CI, security, prepare-release, and release workflows), an auto-generated changelog, and corrected repository metadata. It changes no Givn-authored file formats, no Cargo feature contract, and no project-facing workflow. A project created with Givn 0.3.x therefore needs no domain or configuration migration to run the 0.4.0 contract — the machine-safe managed refresh already performed by `givn upgrade` is the complete operation.

## Proposed Solution

Verify that the consumer project is running the Givn 0.4.0 managed contract, that the managed upgrade commit (Commit A) contains only the expected allowlisted paths, and that no project-owned specification, architecture, or configuration surface was silently changed. Complete the normal Givn review and archive gates for this maintenance change.

This bundle is one phase in an aggregate migration plan when a project skips additional versions.

## Out of Scope

- Inventing or changing domain behavior without project evidence.
- Automatically editing project specifications, source code, tests, ADRs, Arc42 facts, ejected facets, or user-authored documentation.
- Adding a synthetic migration feature to the permanent specification tree.

## Completion

The LLM must verify the 0.4.0 managed contract and confirm no project migration is required, then complete this phase before the next bundle phase in an aggregate plan.


### Phase 2: migrate-0-4-0-to-0-5-0 (0.4.0 -> 0.5.0)

# Migration Proposal: 0.4.0 to 0.5.0

## Problem / Opportunity

Givn 0.5.0 is a major behavioural release. It retires the embedding-based
semantic similarity gate from the default build, reorganizes the permanent
specification corpus into use-case groups (`givn/specs/<group>/<capability>.feature`
plus one `group.md` per group), adds deterministic discovery
(`givn spec tree`, `spec find`, `spec route`, `spec regroup`, and a
deterministic `spec duplicates`), introduces capability retirement
(`@givn.retired`), and tightens several gates. A project created with
Givn 0.4.x has a flat specification corpus, may still carry
semantic-review guidance in ejected overrides, and may have active
changes whose disposition tables use free-form decision words. Running
the 0.5.0 binary against such a project without a migration leaves the
project on a contract it no longer matches: discovery tools see no
capabilities, stale semantic instructions keep steering authors, and
review.md disposition tables silently stop clearing findings.

## Proposed Solution

The machine-safe part (managed config scaffold, generated skill/command
targets, `givn_config_version` bump) is performed automatically by
`givn upgrade` and recorded in Commit A. This bundle then walks the
project LLM through the project-specific decisions that must never be
automatic: reconciling ejected guidance with the 0.5.0 instructions,
deciding whether the specification corpus migrates to the grouped layout
(using `givn spec regroup` as a proposal, confirmed by a human), deciding
whether the semantic features remain wanted as an explicit opt-in,
converting existing disposition tables to the enforced decision
vocabulary, moving interaction inventories into `group.md`, and fixing
any overlay that removes the now-protected `review` artifact.

This bundle is one phase in an aggregate migration plan when a project
skips additional versions.

## Out of Scope

- Automatically moving or rewriting project specification files. The
  grouped-layout migration applies only after a human confirms the
  proposed grouping; all moves are explicit `git mv` renames.
- Choosing group names or writing group narratives automatically. The
  proposal output is advisory; a human names the groups and writes real
  Actor/Goal/Interactions prose.
- Deleting project-authored content, ADRs, Arc42 facts, ejected facets,
  or user documentation.
- Adding a synthetic migration feature to the permanent specification
  tree.

## Completion

The LLM completes this phase when the project's specification layout
decision is applied or explicitly deferred, all ejected guidance matches
the 0.5.0 contract, every active change's review evidence satisfies the
deterministic gates, and this maintenance change passes its own review
and archive gates.


