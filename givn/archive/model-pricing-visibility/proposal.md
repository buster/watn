# Proposal: model-pricing-visibility

## Use-Case Context

- Use-case ID: configure-model
- Ideation topic: none
- Confirmed Personas: none

## Problem / Opportunity

Provider catalogs publish a price for each model, but watn makes that price
invisible in two ways:

- Model lists and the interactive model table render every published price as
  `$0.00`, regardless of the model's real cost. The user cannot compare models
  by price at selection time.
- Choosing models persists their tiers and reasoning but drops their published
  price. The pricing configuration stays empty, so request output never
  contains a cost estimate even though the provider published usable prices.

## Proposed Solution

- Present every published catalog price in USD per one million tokens, the same
  unit that pricing configuration and cost estimates already use, so a list
  entry for a real model shows its real price.
- When the user chooses models from a catalog that publishes prices, record
  each chosen model's input and output price in pricing configuration together
  with the tier and reasoning choices.
- Once a chosen model has a recorded price, the existing cost estimate appears
  for requests that use it.
- A model chosen without published price metadata records nothing and leaves any
  existing pricing entry untouched. Pricing entries for models that were not
  chosen are preserved.
- A model has no published price when either price component is missing or not
  a non-negative amount: it is neither shown as a price nor recorded.
- A chosen model with published price metadata replaces its previous pricing
  entry with the published values.
- The plain model list and the interactive model table present the same
  per-million price for the same catalog metadata.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| models | EXTEND catalog-source (3.00, all candidates tied) | `EXTEND models` | Owns the plain model-list presentation and the non-terminal model assignment behavior this change corrects. |
| ratatui-model-picker | EXTEND catalog-source (3.00, all candidates tied) | `EXTEND ratatui-model-picker` | Owns the interactive model table whose price column is corrected. |
| streamlined-setup | EXTEND catalog-source (3.00, all candidates tied) | `EXTEND streamlined-setup` | Owns the model-choice persistence boundary where selected catalog prices are captured. |

## Out of Scope

- Guessing prices for catalogs that publish none.
- Changing the cost formula or the per-million unit of pricing configuration.
- Currencies other than USD, cache read/write price components, request-time
  price overrides, and historical usage accounting.
- Quick setup's manual model entry, which does not contact a catalog.
- Provider or catalog selection behavior that is not model choice.

## Open Questions

- None blocking.
