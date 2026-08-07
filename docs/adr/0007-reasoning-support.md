# ADR-0007: Reasoning support via reasoning_effort parameter

- **Status:** accepted
- **Date:** 2025-08-07
- **Decision-makers:** architect

## Context and Problem Statement

The tool supports a "thinking" tier (`-3` / `--thinking`) that routes questions
to a more capable model. However, it never signals to the API that the user
wants reasoning/chain-of-thought output. Many OpenRouter-compatible models can
expose their internal reasoning when the client sends a `reasoning` parameter.
Without this, the thinking tier is just a different model name — the user gets a
more expensive model but no actual reasoning behavior. Additionally, users who
want to see the model's internal reasoning have no way to request it.

How should the tool signal reasoning effort to the API, and how should it
present reasoning output to the user?

## Decision Drivers

- Must be compatible with OpenRouter's reasoning-effort API
- Must not change the command suggestion on stdout (reasoning is diagnostic)
- Must be easy to add additional effort levels (`medium`, `low`) in the future
- Prefer zero new dependencies

## Considered Options

### Wire format: boolean flag vs. enum string

- **Boolean flag** (`reasoning: true`) — simple but locks the API to
  on/off. OpenRouter supports `high`, `medium`, `low` levels — a boolean
  cannot express this.
- **Nested object** (`{"reasoning": {"effort": "high"}}`) — matches some
  provider schemas but is redundant wrapping.
- **Enum string** (`reasoning_effort: "high"`) — top-level string field,
  directly expresses the effort level. OpenRouter accepts this as a
  top-level parameter alongside `model`, `messages`, etc.

### Reasoning output: stdout vs. stderr

- **stdout** — reasoning would interleave with or pollute the command
  suggestion. The command output is consumed by piping (`watn "..." | xargs`).
- **stderr** — reasoning is diagnostic metadata, like model name and tok/s.

### Parsing strategy: short-circuit vs. always-parse

- **Short-circuit** — skip parsing `delta["reasoning"]` from SSE when
  verbose is not set. Saves CPU but means `StreamingResponse` may lack
  reasoning content when the user didn't pass `-v`. This makes reasoning
  unavailable for future features (logging, alternate output formats).
- **Always-parse** — parse reasoning from every SSE chunk regardless of
  verbose flag, accumulate into `StreamingResponse.reasoning_content`.
  Only the print-to-stderr is gated on verbose. Zero cost difference
  (parsing is a JSON field lookup — same cost regardless).

## Decision Outcome

Chosen options:

1. **Wire format:** `"reasoning_effort": "high"` as a top-level string in
   the request body. Set when tier is `-3` (thinking). Not set for tiers 1/2.
2. **Output stream:** stderr. Reasoning is printed on its own line prefixed
   with `reasoning:` after the response completes, alongside existing
   metadata (model, tok/s, cost).
3. **Parsing:** Always-parse. Reasoning content is extracted from every SSE
   delta and accumulated into `StreamingResponse.reasoning_content`.
   The print is gated on `-v`/`--verbose` — without the flag, reasoning is
   accumulated but not printed.

## Consequences

- Good: easy to add `medium`/`low` effort levels in the future by changing
  the constant from `"high"` to a configurable value.
- Good: reasoning content is always available in the response struct for
  future features (logging, alternate output formats).
- Good: no new dependencies required — `serde_json` already handles the
  request body and SSE parsing.
- Bad: models that do not support reasoning will ignore the parameter
  silently (no error diagnostic).
- Bad: clients that pipe stderr (e.g., `2>&1`) will see reasoning output
  interleaved with metadata. This is acceptable — reasoning lines are
  clearly prefixed.

## Confirmation

E2E scenarios in `givn/changes/reasoning-support/specs/reasoning.feature`
verify:
- Thinking tier sends `reasoning_effort: "high"` in request body
- Verbose flag (`-v`) prints reasoning to stderr
- Without verbose, reasoning content is not printed
- Verbose flag appears in `--help` output
