# arc42 Documentation Update: robust-review-response-recovery

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Requirement 4's verbose scope and the review/explanation response contract change. | Extended requirement 4; added requirement 41. |
| 2 | 02 architecture-constraints | Yes | The XDG constraint now includes the state directory, and the structured-response constraint gains tolerant reading. | Updated the XDG row and the "Structured review response is validated" row. |
| 3 | 03 context-and-scope | Yes | A new filesystem artifact (the unusable-response state file) crosses the system boundary, and the explanation entry point names the reason and capture. | Added the state-file node and interface row; extended the explanation entry point row. |
| 4 | 04 solution-strategy | Yes | The solution strategy gains tolerant response reading and response diagnostics. | Extended one key-decision bullet and the review-response technology row. |
| 5 | 05 building-block-view | Yes | The CLI, structured-review-response, and review-diagnostics responsibilities change. | Extended the CLI and structured-response rows; added a review-diagnostics row. |
| 6 | 06 runtime-view | Yes | The review and explain flows gain repair, recovery, capture, and verbose steps. | Updated both sequence diagrams and their prose. |
| 7 | 07 deployment-view | No | No deployment topology, artifact, dependency, or runtime-library change; the state file is user-home data, not a deployment fact. | |
| 8 | 08 crosscutting-concepts | Yes | Response reading and response diagnostics are cross-cutting error-handling behavior; verbose mode gains the raw response. | Extended inline review safety and explanation delivery; added a "Provider response diagnostics" section; extended verbose mode. |
| 9 | 09 architecture-decisions | No | The candidate decision is NOT_QUALIFIED; its complete rationale is owned by `design.md` and the Gherkin contract. No ADR file or register change. | |
| 10 | 10 quality-requirements | Yes | QS-072 and QS-077 change, and response diagnostics is a new quality scenario. | Updated QS-072 and QS-077; added QS-080. |
| 11 | 11 risks-and-technical-debt | Yes | Tolerant recovery, the state-file capture, and the raised completion cap introduce new consequences. | Updated R-072; added R-089 to R-091; extended the review-mode and command-explanation consequence coverage. |
| 12 | 12 glossary | Yes | Six new domain terms and three refined terms describe response reading and diagnostics. | Added Provider response, Unusable provider response, Purpose reason, Unusable-response capture, Response repair, Response recovery; updated XDG, Purpose status, and Structured review response. |

## ADR qualification

Candidate decision: review and explanation provider responses are read tolerantly
(repair literal control characters, recover a complete provider-written command
from a malformed or truncated payload, never display a review-shaped payload as
the command), the card names the Purpose reason, every unusable response is
captured in an overwritten state file, and `-v` prints the raw response.

Structured verdict:

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
  "canonical_artifact": "givn/changes/robust-review-response-recovery/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Keep strict reading only: a malformed or truncated response degrades to unavailable.",
      "Read tolerantly (repair and recovery) and add named reasons plus a state-file capture for bug reports."
    ],
    "architectural_impact": [
      "No component, ownership, dependency, or deployment boundary changes: the same review-response and review-surface modules change local behavior inside the already-owned use-shell capability."
    ],
    "durable_consequence": [
      "Reversal is local code plus Gherkin and arc42 text edits; no migration, interface break, or cross-command coordination is required."
    ],
    "lower_level_artifact": [
      "The complete rationale (repair rules, recovery scanner, reason wording, capture path, verbose timing) is owned by design.md.",
      "The observable contract is owned by the interactive-shell-shortcut and explain-command Gherkin scenarios."
    ],
    "existing_adr_check": [
      "Searched the chapter-09 register and docs/adr/: 26 records; none covers tolerant provider-response reading or response diagnostics.",
      "ADR-0015 owns the stream/completion boundary; this change only chooses what the completed payload becomes.",
      "ADR-0011, ADR-0017, and ADR-0018 are adjacent to onboarding, completion, and widget behavior but not to this boundary."
    ]
  }
}
```

MADR checklist evidence:

- Hard to reverse: checked as supporting evidence - the reason wording, the
  capture path, and the recovery behavior are documented and tested, and users
  may rely on them in bug reports.
- Shapes quality attributes: checked as supporting evidence - observability
  (named reasons, raw response, capture) and correctness (tolerant reading of
  the same trusted data).
- Structural: unchecked - no module ownership or dependency direction changes.
- Must be shared: supporting only - contributors must respect the
  never-display-a-review-shaped-payload and never-invent-purpose-text rules,
  but the Gherkin spec and design already state them.

Routing: `CANONICAL_ARTIFACT` to
`givn/changes/robust-review-response-recovery/design.md` (with the observable
contract in the new `interactive-shell-shortcut` and `explain-command` Gherkin
scenarios). No ADR was created, so no MADR file is added and neither the
chapter-09 register nor `docs/adr/README.md` changes.

## Status

STATUS: DONE
