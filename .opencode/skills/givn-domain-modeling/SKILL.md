---
name: givn-domain-modeling
description: >-
  Establish and enforce one domain vocabulary for a givn change. Active from
  the first domain noun: challenge generic or overloaded terms (e.g. "list",
  "item"), name every aggregate with a domain noun, and record the agreed
  vocabulary in the project's arc42 glossary so specs, design, and code never
  drift. Load this during every workflow step of a change
  — whenever you are naming or discussing domain concepts, or when spec and code terms disagree.
---

# givn-domain-modeling

Establish and enforce the domain language for change `<change-id>`. This is an
active discipline, not a one-shot step: challenge fuzzy terms the moment they
appear and keep the project's glossary current as the language crystallises.
The design and the specs must use exactly the vocabulary in the glossary.

## Context

- Proposal: `givn/changes/<change-id>/proposal.md`
- Specs: `givn/changes/<change-id>/specs/`
- Glossary: `docs/arc42/12-glossary.md` (maintained here)
- Design (next): `givn/changes/<change-id>/design.md`

## Active discipline

- **Challenge generic or overloaded terms.** When conversation or the specs use
  a word that could mean many things ("list", "item", "user", "trip"), force a
  precise domain noun. Propose it and justify it. Example utterance: "You
  wrote 'list' - do you mean the shared shopping list, or the array backing
  the store?"
- **Name aggregates with domain nouns.** Every consistency boundary gets a
  domain noun as its root (`ShoppingList`, not `List`). If the natural name is
  a framework word, the naming is wrong.
- **Probe edge-case scenarios.** Stress-test a term by inventing a concrete
  scenario that forces the boundary explicit (offline add, two phones, trip
  reset, reconnect).
- **Cross-reference with code.** Where code names exist, check they match the
  agreed terms. On disagreement, flag and resolve it here - do not leave spec
  and code diverging.
- **Update the glossary inline.** Capture a resolved term immediately, before
  moving on. Do not batch it to the end.

## Requirements

Requires the arc42 addon - enable it first when the glossary is missing
(`givn addons enable arc42`) and make sure `docs/arc42/12-glossary.md` exists
before recording terms. Domain modeling is not pointed at projects that skip
the glossary.

## Glossary maintenance

The ubiquitous language lives in the durable arc42 glossary
(`docs/arc42/12-glossary.md`), not in a per-change file: maintaining the project's arc42 glossary inline is the enforcement surface, and specs, design, and code must reuse its exact terms.

Record in the glossary:

- **Ubiquitous language** - the agreed terms, each with its definition, the
  spec wording that uses it, the code symbol, and the Anti-terms (rejected
  synonyms).
- **Aggregates / boundaries** - which entities form a consistency boundary,
  the root of each, and which are independent.
- **Invariants** - rules that must always hold, each marked with the
  enforcement site (data model, command, gate).
- **Lifecycle events** - the *domain's own* state transitions, what the user
  observes. (Shopping list example: open / item added / item marked bought /
  item hidden for this trip / trip reset.)

## Rules

- Stay in domain language. No class names, DB schemas, or framework types
  (except inside the `Code symbol` column).
- Derive everything from the proposal and specs; do not invent new behaviour.
- If a term is ambiguous, pick the interpretation the proposal supports and
  flag it. Do not leave it open for the design to guess.
- No generic nouns where a domain noun exists. `List` is a framework word;
  `ShoppingList`, `Entry` are domain words. Rename.
- One verb per action. List the rejected synonyms explicitly in the
  anti-terms column.
- Keep it short - one page that the design, the lint gate, and the review all
  build on.

## Default anti-term list

`list`, `item`, `data`, `record`, `entity`, `node`, `object`, `manager`,
`service`, `handler`, `container`. Project can extend via the glossary.
