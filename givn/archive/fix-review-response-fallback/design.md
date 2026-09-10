# Design: fix-review-response-fallback

## Use-Case Traceability

| Contract | Design decision | Evidence |
|---|---|---|
| The review surface shows a candidate the provider wrote | Recovery extracts only provider-written text; Watn never authors command or purpose text | Recovery contract below; fenced and invalid-response scenarios |
| An invalid structured response keeps the Candidate reviewable | When strict validation fails but a JSON-shaped payload contains a non-empty `command`, the candidate is that command with `purpose-unavailable` | Invalid-response scenario |
| A payload without a usable command releases nothing | No command text means `Unavailable`: the surface does not open and nothing reaches the shell | No-command scenario |
| Every rendered value stays within the bounded inline panel | Rendered candidate, intent, stage, purpose, and context values are flattened to a single row | Multiline-payload scenario; row-bound assertion |
| Disabled and non-review behavior is unchanged | Changes are confined to review-mode response handling and review-surface rendering | Routing untouched; full suites remain GREEN |

The confirmed Persona `terminal-developer--interactive` remains a review lens.
Actors stay `Terminal developer` and `Shell line editor`.

## Provider Response Recovery

The buffered provider payload is classified in order:

1. **Locate a JSON object.** If the payload contains a fenced block (```...
   ```), use its content; otherwise use the substring from the first `{` to
   the last `}`.
2. **Strict structured response.** Parse and validate the existing versioned
   contract: version, non-empty command, exact locally derived stage text,
   and known purpose status. On success the Candidate keeps model-written
   stage purposes.
3. **Command recovery.** If strict validation fails but the payload is a JSON
   object with a non-empty string `command`, the Candidate is that command
   with `purpose-unavailable` and no stage purposes. Watn does not invent
   purpose text.
4. **Command-only payload.** If the payload is not JSON-shaped, the Candidate
   is the trimmed payload as before.
5. **Unavailable.** If no non-empty command can be recovered, the review is
   `Unavailable`: the surface does not open, the original input is preserved,
   and nothing is released.

The recovery rule is a pure function of the provider payload:
`candidate_from_provider_response(raw) -> Option<ReviewCandidate>`.
`None` means `Unavailable`.

## Inline Rendering Safety

`sanitize_terminal_text` continues to strip ANSI and OSC escape sequences and
to replace unknown control characters with `?`. It now maps line feed,
carriage return, and tab to a single space. As a result every element returned
by `render_lines` contains no embedded row break, so `wrap_lines` and the
inline row accounting in `ControllingTerminal` stay exact for arbitrary
provider text, including pretty-printed JSON.

## Review Prompt

The review-mode system prompt states that the response must be one JSON object
without a markdown code fence and without surrounding prose. Prompt wording is
defence in depth; the recovery contract above is what guarantees the behaviour.

## Interfaces

- `src/review/response.rs`: tolerant `parse_structured_review_response` and new
  `candidate_from_provider_response`.
- `src/review/panel.rs`: flattening sanitization.
- `src/main.rs`: review path consumes the recovery function; a payload with no
  usable command exits as `Unavailable` without releasing output.
- `tests/steps/interactive_shell_shortcut_steps.rs`: harness uses the same
  recovery function so scenarios and product share one contract.

## Failure Outcomes

| Provider payload | Review outcome |
|---|---|
| Valid structured response | Candidate with model-written purposes |
| Fenced or prose-wrapped valid structured response | Same as valid |
| Invalid structured response with a command | Candidate with that command, `purpose-unavailable` |
| Command-only text | Candidate with that text, `purpose-unavailable` |
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
  "canonical_artifact": "givn/changes/fix-review-response-fallback/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "The payload can be rejected outright, or recovered into a reviewable candidate."
    ],
    "architectural_impact": [
      "The recovery rule and rendering flattening stay inside the reviewed review-surface capability."
    ],
    "durable_consequence": [],
    "lower_level_artifact": [],
    "existing_adr_check": [
      "ADR-0015 owns the stream/completion boundary; this change only chooses what the completed payload becomes inside the review surface."
    ]
  }
}
```

## Architecture Impact

Affected Arc42 chapters: 6 (runtime review flow gains the recovery branch),
8 (crosscutting text sanitization gains row flattening), 10 (quality scenario
QS-072 gains provider-wrapped formatting), 11 (risk R-072 mitigation gains
payload-shape tolerance). No new component, endpoint, or deployment fact.

## Verification Contract

The existing Cucumber runner remains the executable specification. Unit tests
in `src/review` cover fence extraction, command recovery, unavailable payloads,
and sanitization. `./run-tests.sh` and `./run-tests.sh --e2e` remain the verify
commands. No live provider is used.
