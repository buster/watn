# Design: recover-review-stage-purposes

## Use-Case Traceability

| Contract | Design decision | Evidence |
|---|---|---|
| Model-written stage purposes remain visible | A non-canonical response keeps its purposes when stage text matches the locally derived stages and every purpose is non-empty; otherwise `purpose-unavailable` | Unknown-status and multiline scenarios; mismatch guard scenario |
| Watn never invents purpose text | Recovery copies provider purpose strings only, and only after stage-text agreement | Mismatch guard scenario; unit tests |
| The review command is understandable and shell-ready | Provider-written commands are normalized to one line before the Candidate and Command flow are shown | Multiline scenario; single-line assertion |

The confirmed Persona `terminal-developer--interactive` remains a review lens.
Actors stay `Terminal developer` and `Shell line editor`. No interaction is
added.

## Recovery Order

1. Locate the JSON object (fence/prose tolerant, as reviewed).
2. Strict structured response: existing contract; on success the Candidate
   keeps model-written purposes.
3. **Recovered purposes:** normalize the command to a single line; derive the
   Command flow; if the response carries a `stages` array whose trimmed
   `stage_text` values equal the derived stage text in order and every stage
   has a non-empty `purpose`, the Candidate uses those provider purposes with
   `purpose-status: ready`. An unrecognized purpose-status word does not block
   this path.
4. **Command only:** a complete command without matching stages becomes a
   Candidate with `purpose-unavailable`.
5. `Unavailable` when no non-empty command exists.

## Command Normalization

Carriage return, line feed, and tab runs in a provider-written JSON `command`
become a single space; the result is trimmed. Stage text is compared after
trimming. This keeps the Command flow stable for commands that a provider
formats across lines without authoring command text locally.

## Interfaces

- `src/review/response.rs`: normalization and the recovered-purpose path inside
  `candidate_from_provider_response`.
- `src/main.rs`: review prompt lists the accepted purpose-status words and
  forbids line breaks inside `command`.
- No other production surface changes.

## Failure Outcomes

| Provider payload | Review outcome |
|---|---|
| Canonical structured response | Candidate with model-written purposes |
| Matching stages and purposes, unknown status word, or multiline command | Candidate with model-written purposes |
| Command with mismatched stage text or a missing purpose | Candidate with that command, `purpose-unavailable` |
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
  "canonical_artifact": "givn/changes/recover-review-stage-purposes/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Purposes can be hidden on any non-canonical response, or recovered when the provider's stage text agrees."
    ],
    "architectural_impact": [
      "Recovery stays inside the reviewed review-surface capability; no provider, channel, or deployment boundary changes."
    ],
    "durable_consequence": [],
    "lower_level_artifact": [],
    "existing_adr_check": [
      "ADR-0015 owns stream completion; this change only refines what the completed payload contributes to the review surface."
    ]
  }
}
```

## Architecture Impact

Affected Arc42 chapters: 6 (recovery order and normalization), 8 (stage-text
agreement and no-invented-purpose rule), 10 (QS-072 gains unknown-status and
multiline cases), 11 (R-072 mitigation gains status-vocabulary and formatting
drift). No new component, endpoint, or deployment fact.

## Verification Contract

The existing Cucumber runner remains the executable specification. Unit tests
cover normalization, recovered purposes, mismatch, and missing-purpose cases.
`./run-tests.sh` and `./run-tests.sh --e2e` remain the verify commands.
