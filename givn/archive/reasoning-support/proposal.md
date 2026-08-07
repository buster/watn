# Proposal: Reasoning Support

## Problem / Opportunity

The tool already supports a "thinking" tier (`-3` / `--thinking`) that routes questions to a more capable model. However, it never signals to the API that the user wants reasoning/chain-of-thought output. Many OpenRouter-compatible models can expose their internal reasoning when the client sends a `reasoning` parameter. Without this, the thinking tier is just a different model name — the user gets a more expensive model but no actual reasoning behavior. Users who want to see the model's internal reasoning have no way to request it.

## Proposed Solution

1. When the user invokes the thinking tier (`-3` or `--thinking`), the tool tells the API to use high reasoning effort. This causes the API to generate reasoning tokens alongside the answer.
2. When the user passes a `-v` or `--verbose` flag, the tool prints the model's reasoning output to the diagnostic stream (stderr) so the user can see how the answer was derived. This works for any tier (tiers 1 and 2 may not produce reasoning, but if they do, the verbose flag prints it).
3. Neither flag changes what appears as the final command suggestion on stdout. The command output remains unchanged.

## Out of Scope

- No interactive reasoning viewer (e.g. expand/collapse, paging).
- No change to formatting of the command suggestion on stdout.
- No change to tier 1 or tier 2 request bodies — they do not send the reasoning parameter.
- No change to the config file format.

## Open Questions

None.
