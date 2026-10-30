# Proposal: normalize-fragment-e2e-inventory

## Release note

Keep one end-to-end fragment scenario per distinct consumer action and run the remaining variants in the regular suite.

## Use-Case Context

- Use-case ID: fragments (fragment: corpus-infra)
- Ideation topic: none
- Seed path: none
- Confirmed Personas: terminal-developer--interactive — the change classifies existing tests only; no product surface changes.

## Review Context

- Review kind: self-review
- Reason: specification-metadata maintenance; no public contract, persistent data, security, concurrency, architecture boundary, or destructive operation changes.

## Problem / Opportunity

The fragment use case (`fragments`) marks 26 scenarios `@e2e` across six capabilities, but those scenarios exercise 11 distinct consumer actions. The ask capability alone runs 10 `@e2e` scenarios for two actions: asking for a command, and executing a generated command after confirmation. Tier flags, confirmation answers, cost and throughput output, cancellation timing, configuration precedence, streaming flags, error cases, and input channels all run under the e2e filter as if each were a distinct real-interface action.

The interaction inventory added in the previous fix records those 26 scenarios one-for-one. It makes lint pass, but it also records the over-production as the intended inventory. The canonical policy says variants, error cases, and output-affecting flag combinations of the same action are regular scenarios, not additional inventory entries or additional `@e2e` scenarios. The e2e suite therefore overstates distinct coverage: 92 scenarios run through the e2e filter where 77 distinct-action scenarios suffice, and the regular suite loses the variant coverage it should own.

## Proposed Solution

Normalize the fragment inventory to 11 distinct actions and retag the remaining 15 scenarios as regular:

- ask keeps 2: asking for a command, and executing a generated command after confirmation. The tier flags, both explicit confirmation answers, cost display, throughput display, and stdin input become regular.
- cancel-running-completion keeps 1: cancelling a running completion. The connection-wait timing variant becomes regular.
- config keeps 2: configuring model selection, and configuring pricing. The environment-variable and CLI precedence variants become regular.
- incremental-sse-rendering keeps 2: reading incrementally streamed output, and confirming on the terminal after the streamed command arrives. The verbose flag, mid-stream failure, and piped confirmation variants become regular.
- release-truth keeps 2: reading the package version, and generating one changelog entry per change. No scenario changes.
- transport keeps 2: normal debug routing, and isolated test-support routing. The empty-override fallback variant becomes regular.

Every retagged scenario keeps its Given/When/Then text and still runs in the regular suite. The e2e suite drops from 92 to 77 scenarios; the regular suite grows from 275 to 290; the 367 total scenarios are unchanged. The fragment interaction inventory is updated to the 11 retained actions.

## Capability Routing

`givn spec route --change normalize-fragment-e2e-inventory` reported no signal. The decision is to extend the existing fragment capabilities; no capability is added or moved.

| Proposed capability | Decision | Rationale |
|---|---|---|
| ask | `EXTEND ask` | Existing capability; only scenario tags and the inventory change. |
| cancel-running-completion | `EXTEND cancel-running-completion` | Existing capability; only scenario tags and the inventory change. |
| config | `EXTEND config` | Existing capability; only scenario tags and the inventory change. |
| incremental-sse-rendering | `EXTEND incremental-sse-rendering` | Existing capability; only scenario tags and the inventory change. |
| release-truth | `EXTEND release-truth` | Existing capability; no scenario or inventory change. |
| transport | `EXTEND transport` | Existing capability; only scenario tags and the inventory change. |

## Out of Scope

- No product behaviour, output, CLI surface, or configuration changes.
- No scenario Given/When/Then text changes.
- No capability added, removed, renamed, or moved.
- No change to the fragment's use-case relationships, goal, or rules.
- No change to other use cases' `@e2e` inventories.

## Open Questions

### D1 — Are input channels distinct actions?

- Question: should "Ask via stdin pipe" and "Piped confirmation remains available after streamed output" keep their own `@e2e` scenarios as distinct input-channel actions, or are they variants of the ask and terminal-confirmation actions?
- Options:
  1. **Keep them as distinct actions** — each input channel gets its own end-to-end entry; gain: channel behaviour has dedicated e2e evidence; cost: the canonical policy treats input variants of the same action as regular, and the e2e count stays at 13 fragment scenarios; in practice `echo "list pdfs" | watn` would run only under the e2e filter.
  2. **Treat them as variants** — the action is asking or confirming and the channel is a variant; gain: matches the canonical normalization and the scenarios still run in the regular suite; cost: channel-specific behaviour is not exercised by the e2e filter; in practice `echo "list pdfs" | watn` runs in the regular suite while `watn "list pdfs"` proves the ask action end to end.
- Disposition: answered — the canonical specs instruction records that input variants, error cases, and flag combinations of the same action are regular scenarios; the action is asking or confirming, and the channel is a variant.
