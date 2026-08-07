# ADR-0005: Execution mode with confirmation

- **Status:** accepted
- **Date:** 2024-12-01
- **Decision-makers:** architect

## Context and Problem Statement

The tool's primary output is shell commands. Users copy-paste them to execute.
Can we eliminate the copy-paste step safely?

## Decision Drivers

- Must not execute commands without explicit user consent
- Plain Enter should confirm (muscle memory: `Enter` = "do it")
- Non-interactive usage must not prompt

## Considered Options

- **Direct execution** — runs command immediately, no prompt (dangerous)
- **Clipboard copy** — copies to system clipboard (not universally available)
- **`-x` flag with confirmation** — prints command, prompts, executes on confirm

## Decision Outcome

Chosen: **`-x` flag with confirmation prompt**. Prompt: "Execute now? [Y/n]".
Enter or `y` confirms, `n` declines. No prompt when `-x` is not passed.

## Consequences

- Good: safe by default (must opt-in via `-x`)
- Good: Enter confirms (minimal friction for willing users)
- Bad: command printed first, then prompt — user cannot pre-approve

## Confirmation

E2E scenarios: `-x` with Enter executes, `-x` with `n` skips, command output is visible.
