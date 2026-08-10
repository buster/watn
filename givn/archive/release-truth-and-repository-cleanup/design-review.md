# Design Review: release-truth-and-repository-cleanup

## Grilling Results

### Scope

The proposal, specification, and design agree on package-derived version
output, target-specific release truth, active-document reconciliation, explicit
historical archive labeling, and conservative cleanup. Package version bump,
static deployment, behavior changes, shell integration, and repository-wide
formatting are out of scope.

### Technology And Verification

Cargo package metadata is the smallest authoritative version source. Release
inspection is host-aware: Linux uses `file` and `ldd`; macOS uses `file` and
`otool -L`. The E2E version step has unique wording and does not collide with
the existing global `watn --version` step. Verification uses the configured
wrapper scripts and the strict Cucumber runner.

### Missing Scenarios And Testability

The version interaction is the sole CLI inventory entry and maps to one E2E
scenario. Release artifact and documentation checks are maintainer repository
checks, not duplicate user interactions. Each regular scenario has concrete
file, command, or documentation assertions and can fail against the current
repository before implementation.

### Cleanup Decisions

- Remove the unused local `_config` parameter after compile verification.
- Retain public `ProviderRegistry`, `ProviderSetupResult`, and wrapper
  functions because current feature consumers exist and external consumers
  cannot be ruled out from this binary repository.
- Remove only `WatnWorld` fields proven write-only by exact repository search.
- Remove obsolete documentation/helper names only after active-tree searches.

### Arc42

Arc42 is enabled. The current-change `arc42.md` contains all twelve rows and
`STATUS: DONE`. All twelve durable chapters are substantive and Mermaid-only.
The active documentation now records target-dependent dynamic libraries,
config-only XDG storage, actual SetupWizard/model-picker terminology, Ctrl-R,
current output channels, and historical archive status. ADR-0016 is indexed and
its consequences are recorded in chapter 11.

### E2E And Interaction Coverage

The capability is CLI-only. The version E2E scenario invokes a real built
subprocess and asserts stdout and exit status. Its one inventory entry maps to
one matrix row and one scenario. No browser or HTTP shortcut is involved.

## Review Outcome

All design-review findings were resolved without a product decision request:
release inspection is host-aware and requires dynamic linking plus successful
shared-library output, duplicate step binding is avoided, all stale active
documentation claims are in scope, archive status is explicit, and cleanup
public-consumer boundaries are fixed.

DESIGN-REVIEW: PASS
