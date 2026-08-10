# Proposal: release-truth-and-repository-cleanup

## Problem / Opportunity

Watn reports a version that can differ from the installed package, and some
release and architecture documentation describes behavior or deployment
properties that the delivered binary does not provide. This makes `--version`,
deployment decisions, troubleshooting, and repository maintenance unreliable.
The repository also carries unused or misleading implementation names and
parameters that obscure the actual application boundary.

## Proposed Solution

The version shown by `watn --version` shall be the package version used to build
the binary. Release documentation shall state the target-dependent runtime
library requirements and shall not claim universal static deployment without a
verified static artifact. Active README and architecture documentation shall
describe the current streaming behavior, terminal helper names, reasoning
shortcut, configuration storage, and the historical status of archived
snapshots.

Repository cleanup shall remove only confirmed dead code and obsolete names,
preserve public behavior that has a concrete consumer, and leave command
output, diagnostics, provider selection, credential authority, and persisted
configuration unchanged. Release verification shall provide repeatable evidence
for the version and dynamic-linking claims.

## Out of Scope

The package version is not being bumped. Static deployment is not being added.
No provider protocol, model discovery behavior, setup flow, shell integration,
stream rendering, output-channel contract, or credential resolution rule is
changing. Repository-wide formatting is out of scope. Historical archived
documentation remains historical rather than being rewritten as current
behavior.

## Open Questions

No unresolved product decisions remain. The observable release and repository
truth requirements are ready for executable scenarios.
