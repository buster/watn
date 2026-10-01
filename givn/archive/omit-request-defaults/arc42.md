# arc42 impact: omit-request-defaults

## ADR qualification

Existing-ADR check: the register in `docs/arc42/09-architecture-decisions.md`
lists ADR-0001…ADR-0026 and `docs/adr/*.md` holds the bodies. Searching them for
a decision about the provider request parameter set finds ADR-0007 (reasoning
support via `reasoning_effort`), which owns only the reasoning field. No ADR
owns the sampling-temperature or output-token-limit parameters.

Candidate — removing Watn-invented sampling temperature and output-token limit
from the request.

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "the ask capability's Gherkin scenario",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": ["omit both parameters and rely on provider defaults vs keep a 4096 output limit with a model-aware field name vs retry once without a rejected parameter"],
    "architectural_impact": ["the request body of one existing component loses two optional keys; no component, ownership, dependency, deployment, or data boundary moves"],
    "durable_consequence": ["fully reversible within one change: re-adding a field is a local edit, with no migration or cross-command coordination"],
    "lower_level_artifact": ["observable request behaviour owned by the capability spec and design.md; no architectural trade-off remains"],
    "existing_adr_check": ["docs/arc42/09-architecture-decisions.md register and docs/adr/*.md search for temperature, max_tokens, request parameters: only ADR-0007 owns reasoning_effort"]
  }
}
```

No ADR is created by this change.

## 12-row chapter assessment

| # | Chapter | Affected | Reason / what changed |
|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The product goal and stakeholders are unchanged. |
| 2 | 02 architecture-constraints | No | The OpenAI-compatible constraint stands; the change reduces the request surface rather than adding a constraint. |
| 3 | 03 context-and-scope | No | The external interface is still `POST /v1/chat/completions`; no new external interface or data exchange is introduced. |
| 4 | 04 solution-strategy | No | No change of technical strategy; the request keeps the streaming-first, single-wire-protocol approach. |
| 5 | 05 building-block-view | Yes | The `OpenAICompatibleProvider` responsibility now names that only model, messages, stream, and a configured reasoning field are sent; sampling temperature and output-token limits are never sent. |
| 6 | 06 runtime-view | No | The request-body sequence diagrams already name only the model and reasoning options; no flow changes. |
| 7 | 07 deployment-view | No | Same binary, same install path, no new service. |
| 8 | 08 crosscutting-concepts | Yes | Added the provider request-defaults rule: omitted parameters take the provider's own default; only explicitly required fields are sent. |
| 9 | 09 architecture-decisions | No | No ADR qualifies; the candidate routes to canonical artifacts and the register stands unchanged. |
| 10 | 10 quality-requirements | No | QS-060 concerns the reasoning field, which is unchanged; no new quality scenario is introduced. |
| 11 | 11 risks-and-technical-debt | Yes | R-091's mitigation (a scoped 4096 cap) no longer applies and is amended; R-097 records the provider-default truncation risk carried as out of scope in the proposal. |
| 12 | 12 glossary | Yes | `Provider default` was added when the proposal was written. |

## Files touched

- `docs/arc42/05-building-block-view.md`
- `docs/arc42/08-crosscutting-concepts.md`
- `docs/arc42/11-risks-and-technical-debt.md`
- `docs/arc42/12-glossary.md` (the `Provider default` term was added earlier in this change)

`docs/arc42/README.md` and the chapter-09 register need no update: no chapter
is added or removed and no ADR is created.

## Status

STATUS: DONE