# ADR-0006: LiteLLM-powered model discovery

- **Status:** superseded by ADR-0021
- **Date:** 2024-12-01
- **Decision-makers:** architect

## Context and Problem Statement

Users who self-host LLMs via LiteLLM proxy have many available models but no
easy way to discover which ones are available and assign them to tiers. How
should model selection work for LiteLLM users?

## Decision Drivers

- Must work without LiteLLM (manual config is always an option)
- Interactive selection is easier than editing TOML for first-time setup
- The tool should not vendor a model list

## Considered Options

- **Hardcoded model list** — ships known model IDs (stale, incomplete)
- **LiteLLM `/models` API** — discovers available models at runtime
- **No discovery** — user always edits config manually

## Decision Outcome

Chosen: **Optional LiteLLM `/models` endpoint**. If configured, `watn models`
fetches the list, lets user select one per tier interactively, and writes the
result to config. If not configured, prints manual setup instructions.

## Consequences

- Good: works with any LiteLLM proxy out of the box
- Good: no stale model list to maintain
- Bad: requires LiteLLM endpoint URL in config (discoverable once, remembered)

## Confirmation

E2E scenario: mock LiteLLM endpoint, assign tiers via `--set-small`/`--set-normal`/`--set-thinking` flags, verify config file written.
