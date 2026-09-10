# Design: accept-provider-stage-split

## Use-Case Traceability

| Contract | Design decision | Evidence |
|---|---|---|
| Model-written stage purposes remain visible | A provider split that provably covers the command becomes the reviewed Command flow with its purposes | Covered-split scenario |
| Watn never invents stage or purpose text | Every accepted stage text must appear verbatim in the provider command, in order, without overlap; purposes are provider strings only | Guard scenario; unit tests |
| Unusable responses keep the candidate reviewable | An untrusted split falls back to the recovered command with `purpose-unavailable` | Guard scenario |

## Provider Split Contract

Given the normalized command and the response `stages` array:

1. Every stage has a non-empty `stage_text` and a non-empty `purpose`.
2. Scanning the command left to right, each `stage_text` is found at or after
   the current cursor; the gap before it contains only whitespace, `|`, `&`,
   or `;`; the stage text is copied verbatim.
3. After the last stage, only whitespace remains.
4. Non-overlap follows from advancing the cursor past each accepted stage.

When all conditions hold, the accepted stages become the Candidate's
`Command flow` (with local unsupported-syntax marking) and their purposes
become the displayed `Stage purpose` values. Otherwise the existing fallbacks
apply: exact match against the locally derived stages, then command-only with
`purpose-unavailable`, then `Unavailable`.

## Interfaces

- `src/review/flow.rs`: expose `command_stage(command, start, end)` so a
  validated range produces a stage with local support marking.
- `src/review/response.rs`: `provider_stage_split(command, value)` used by
  `candidate_from_provider_response` before the derived-stage match.

## Failure Outcomes

| Provider payload | Review outcome |
|---|---|
| Provider split covers the command with non-empty purposes | Provider stages and purposes shown |
| Split not trusted, but derived-stage exact match holds | Derived stages with provider purposes |
| Split not trusted and no exact match | Recovered command with `purpose-unavailable` |
| No usable command | `Unavailable`; preserve input; release nothing |

## ADR Qualification And Routing

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/accept-provider-stage-split/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Only the local split may be trusted, or a provider split that provably covers the command may be trusted."
    ],
    "architectural_impact": [
      "Trust stays inside the review-surface capability; the accepted text is verified verbatim against the provider command."
    ],
    "durable_consequence": [],
    "lower_level_artifact": [],
    "existing_adr_check": [
      "ADR-0015 owns stream completion; this only refines stage acceptance inside the review surface."
    ]
  }
}
```

## Architecture Impact

Affected Arc42 chapters: 6 (stage acceptance in the review flow), 8 (verbatim
stage proof and no-invention rule), 10 (QS-072 gains compound provider splits),
11 (R-072 mitigation gains provider-split trust rules). No new component,
endpoint, or deployment fact.

## Verification Contract

The existing Cucumber runner remains the executable specification. Unit tests
cover the real compound split, fabricated, out-of-order, overlapping,
non-covering, and missing-purpose payloads. `./run-tests.sh` and
`./run-tests.sh --e2e` remain the verify commands.
