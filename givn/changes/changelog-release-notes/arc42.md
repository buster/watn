# arc42 impact: changelog-release-notes

## ADR qualification

Existing-ADR check: the register in `docs/arc42/09-architecture-decisions.md`
and `docs/adr/` hold no decision about the changelog, release-note authorship,
or release-tooling selection.

Candidate — the changelog selects one entry per change from the change's
release note and stops a release whose range has no user-visible change.

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "the release-truth capability spec (givn/specs/fragments/release-truth.feature)",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": ["one entry per change from the release note vs one entry per revision scenario title; stop a release with no user-visible change vs allow an empty section"],
    "architectural_impact": ["release-tooling configuration and one workflow guard; no component, authority, dependency, or deployment boundary moves"],
    "durable_consequence": ["reversible within one change; no data migration and no interface"],
    "lower_level_artifact": ["the selection behaviour is completely owned by the release-truth capability spec; design.md holds the mechanism as ordinary implementation detail"],
    "existing_adr_check": ["register and docs/adr/ search: no changelog or release-notes decision recorded"]
  }
}
```

No ADR is created: the decision is release process owned by the capability spec.

## 12-row chapter assessment

| # | Chapter | Affected | Reason / what changed |
|---|---|---|---|
| 1 | 01 introduction-and-goals | No | Product goal and stakeholders unchanged. |
| 2 | 02 architecture-constraints | No | No new constraint. |
| 3 | 03 context-and-scope | No | The changelog is repository output, not a Watn interface. |
| 4 | 04 solution-strategy | No | The release tooling approach (git-cliff plus the release workflow) is unchanged. |
| 5 | 05 building-block-view | No | No production building block changes; only CI, the release workflow, and the changelog configuration. |
| 6 | 06 runtime-view | No | No runtime flow; the change is release-time tooling. |
| 7 | 07 deployment-view | No | No deployment change. |
| 8 | 08 crosscutting-concepts | No | The changelog convention is release process, owned by the capability spec and design, not a cross-cutting architecture concept. |
| 9 | 09 architecture-decisions | No | The candidate does not qualify; the register stands unchanged. |
| 10 | 10 quality-requirements | No | No new quality scenario. |
| 11 | 11 risks-and-technical-debt | Yes | Added R-095 (a range without user-visible change stops the release) and R-096 (the acceptance suite depends on the pinned git-cliff binary). |
| 12 | 12 glossary | Yes | Added `Release note` and `Changelog` so the vocabulary matches the amended capability. |

## Files touched

- `docs/arc42/11-risks-and-technical-debt.md`
- `docs/arc42/12-glossary.md`

## Status

STATUS: DONE
