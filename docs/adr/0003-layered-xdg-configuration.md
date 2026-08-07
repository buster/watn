# ADR-0003: Layered XDG configuration

- **Status:** accepted
- **Date:** 2024-12-01
- **Decision-makers:** architect

## Context and Problem Statement

Users need to configure default provider, API keys, model tiers, and per-provider
settings. Configuration should be persistent, overridable per invocation (CLI
flags), and settable in CI/CD pipelines (env vars).

## Decision Drivers

- XDG Base Directory compliance
- Clear precedence rules
- API keys must be settable via env vars (not just config files)

## Considered Options

- **Layered merge with TOML** — CLI flags > env vars > user config > system config > built-in defaults
- **Single config file only** — no env var or CLI override path
- **JSON config** — more verbose, less Rust-ecosystem-idiomatic

## Decision Outcome

Chosen: **Layered merge with TOML**. Precedence: CLI flags > env vars > user
config > system config > built-in defaults.

## Consequences

- Good: works in CI without config files (env vars only)
- Good: system administrators can set defaults via `/etc/watn/config.toml`
- Bad: merge logic is more complex than a single file

## Confirmation

E2E scenarios verify precedence: env var overrides config, CLI flag overrides env.
