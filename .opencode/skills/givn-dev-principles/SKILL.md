---
name: givn-dev-principles
description: Apply the team's engineering principles when implementing a givn change — correctness first, boring when called for, no unnecessary abstractions.
---

# givn-dev-principles

Apply the team's engineering principles while implementing change `<change-id>`.

## Context

- Tasks: `givn/changes/<change-id>/tasks.md`
- Design: `givn/changes/<change-id>/design.md`
- Verify command: `./run-tests.sh`

## Principles

- **Correctness first, then the next maintainer.** Optimise for the reader six
  months out, not for cleverness today.
- **Match the TDD loop.** RED (real failing test) -> GREEN (minimum code) ->
  REFACTOR (no behaviour change). Never skip RED; never suppress a test to pass.
- **Boring when called for.** Prefer established patterns in the repo over new
  abstractions. A second convention beside an existing one is prohibited.
- **No avoidable allocation or computation.** Consider what code compiles to.
- **Fix problems at the source.** Remove obsolete code — no leftover aliases,
  re-exports, or commented-out scaffolding.
- **You are not alone.** Treat unexpected changes as someone else's work and
  adapt; coordinate before editing a file a peer may own.

## Anti-patterns

- Inventing a generic abstraction to serve one call site.
- Suppressing a warning or special-casing an input instead of fixing the cause.
- Expanding scope "while you're at it" — retries, telemetry, or validation the
  change does not require.
