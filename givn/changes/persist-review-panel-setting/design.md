# Design: persist-review-panel-setting

## Use-Case Traceability

| Contract | Design decision | Evidence |
|---|---|---|
| The review surface is enabled by default and optional | `[review] panel` stays the persisted source of truth; the surface renders when it is enabled and the request is eligible | Persist scenarios |
| The last setting survives | `--review-panel` persists `panel = true`; `--no-review-panel` persists `panel = false`; the in-panel `d` decision persists `false` | Persist scenarios; permanent-disable scenario |
| Configuration is preserved | The write loads the existing configuration, changes only `review.panel`, and saves through the atomic path | Persistence unit test |
| The current invocation is not held hostage to disk | A failed persist reports a warning and the invocation still follows the requested setting | Failure branch noted in the failure table |

## Resolution

1. `--review-panel` / `--no-review-panel` remain the per-invocation override.
2. A set override persists to `[review] panel` before generation through
   `persist_review_panel(enabled)`.
3. No override reads the persisted setting; the default remains enabled.
4. The in-panel `d` decision calls the same persistence with `false`.

## Interfaces

- `src/config/mod.rs`: generalize `persist_review_disabled_at` to
  `persist_review_panel_at(path, enabled)` and add `persist_review_panel`.
- `src/main.rs`: persist a set override and warn on failure.
- Harness: the permanent-disable step uses the generalized seam.

## Failure Outcomes

| Condition | Outcome |
|---|---|
| Persist succeeds | Setting stored; current invocation follows it |
| Persist fails | Warning printed; current invocation still follows the requested setting |
| No override | Persisted setting decides |

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
  "canonical_artifact": "givn/changes/persist-review-panel-setting/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "CLI flags can stay per-invocation, or record the last choice in configuration."
    ],
    "architectural_impact": [
      "Preference persistence reuses the existing configuration write path; no new boundary."
    ],
    "durable_consequence": [],
    "lower_level_artifact": [],
    "existing_adr_check": [
      "ADR-0015 owns stream completion; configuration preference precedence is already documented."
    ]
  }
}
```

## Architecture Impact

Affected Arc42 chapters: 4 (preference resolution), 6 (override persistence in
the request path), 8 (configuration precedence and the `d` decision). No new
component, endpoint, or deployment fact.

## Verification Contract

The existing Cucumber runner remains the executable specification. Scenarios
drive the real binary with an isolated configuration home. `./run-tests.sh`
remains the verify command.
