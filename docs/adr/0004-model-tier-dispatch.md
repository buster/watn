# ADR-0004: Model tier dispatch

- **Status:** accepted
- **Date:** 2024-12-01
- **Decision-makers:** architect

## Context and Problem Statement

Users need different models for different task complexity. A shell command needs
a fast/cheap model; a design question needs a reasoning model. How should the
tool expose model selection?

## Decision Drivers

- Simple and fast for the common case (shell commands)
- Power users can upgrade for harder questions
- No need to remember model names for routine use

## Considered Options

- **Tier flags (`-1`/`-2`/`-3`)** — fixed tiers, user assigns models to them in config
- **`--model` always explicit** — user must always specify which model
- **Smart auto-selection** — heuristic chooses model based on question complexity

## Decision Outcome

Chosen: **Tier flags**. Configurable mapping from tier name to model ID. Default
is `-1` (small/fast). `-2` for normal, `-3` for thinking. `--model <NAME>` bypasses
tiers for one-off overrides.

## Consequences

- Good: common case is a single word (`watn "command"`)
- Good: `-3` for hard questions requires only one extra flag
- Bad: user must configure tiers (manual or via `watn models`)

## Confirmation

E2E scenarios verify that `-1`, `-2`, `-3` each select the correct model from config.
