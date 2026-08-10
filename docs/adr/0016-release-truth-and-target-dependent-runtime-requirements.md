# ADR-0016: Release truth and target-dependent runtime requirements

- **Status:** accepted
- **Date:** 2026-08-10
- **Decision-makers:** architect

## Context and Problem Statement

The CLI version is currently independent from the Cargo package version, and
active deployment documentation describes a universally static executable even
though the release artifact is dynamically linked on the verified host. Those
claims make release troubleshooting and installation decisions unreliable.

How should the project derive the CLI version and describe and verify the
runtime requirements of a release artifact without adding a static build target?

## Decision Drivers

- Make `watn --version` identify the package used to build the binary.
- Describe deployment from evidence for the target being released.
- Keep release verification local and repeatable.
- Avoid promising a universal shared-library set or static portability that has
  not been produced and verified.
- Preserve the current single-binary distribution and wrapper commands.

## Considered Options

- **Keep a separately maintained CLI version literal** - minimal source change,
  but it can drift from Cargo package metadata.
- **Derive the CLI version from Cargo metadata and inspect each release target**
  - keeps the version authoritative and records the runtime libraries actually
  required by the built artifact.
- **Add a static or musl release artifact** - could reduce target library
  requirements, but expands build, TLS, compression, and CI policy beyond this
  change and would still require artifact verification.

## Decision Outcome

Choose package-derived CLI versioning and target-specific release verification.
The CLI uses Cargo's compile-time package version metadata, so the existing
`--version` output format changes only in its source of truth.

Release verification builds the release artifact and inspects the exact output
for the host. On Linux, `file target/release/watn` must identify a dynamically
linked executable and `ldd target/release/watn` must succeed with at least one
shared library entry. On macOS, `otool -L target/release/watn` is used for the
library inspection. Documentation describes the resulting target-dependent
runtime requirements and makes no universal static-deployment claim.

## Consequences

### Good

- The package metadata is the single source of truth for the CLI version.
- Release evidence names the target and the runtime libraries required by the
  artifact instead of implying unsupported portability.
- Verification uses local standard host tools and does not require a provider,
  service, or deployment environment.
- The current `./run-tests.sh` and `./run-tests.sh --e2e` wrappers remain the
  documented acceptance commands.

### Bad

- A release artifact requires compatible target runtime libraries; copying it
  to a different target is not guaranteed to work.
- The exact shared-library set varies by target, so documentation cannot state
  one universal list.
- A static artifact is not available as a fallback. Adding one requires a
  separate decision and verification policy.
- Version truth depends on rebuilding the binary after package metadata changes.

## Confirmation

The release-truth feature verifies the exact package version through a real
release-binary invocation and checks the release artifact with the host file and
library-inspection commands. The active deployment and quality chapters are
reviewed against those results before the change is archived.
