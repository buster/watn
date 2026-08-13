# ADR-0024: Atomic configuration replacement

- **Status:** accepted
- **Date:** 2026-08-13
- **Decision-makers:** Watn maintainers

## Context and Problem Statement

Writing a complete setup result directly over the configuration file can leave
malformed or truncated TOML when the process or filesystem fails during a save.

## Decision Drivers

- Preserve the previous valid configuration on a failed save.
- Keep the write local to the configured file's directory.
- Preserve the existing private Unix permission requirement.

## Considered Options

- **Direct replacement** - simpler, but failure can damage the only config copy.
- **Same-directory temporary write and rename** - gives a one-file atomic
  replacement boundary and preserves the previous target until commit.

## Decision Outcome

Confirmed provider/model configuration is serialized to a same-directory
temporary sibling, flushed, permissioned `0600` on Unix, and renamed over the
destination only after all preparation succeeds. A failed write leaves the
previous target untouched and prevents shell operations from starting.

## Consequences

### Good

- A failed configuration save does not expose a partially serialized file.
- Existing permissions and unrelated config values can be preserved deliberately.

### Bad

- Config and shell target changes are not one multi-file transaction.
- Temporary-file cleanup and rename behavior require platform-specific error
  handling.

## Confirmation

Feature scenarios force a final-write failure and assert unchanged config bytes
and zero shell operations.
