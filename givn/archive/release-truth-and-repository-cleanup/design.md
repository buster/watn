# Design: release-truth-and-repository-cleanup

## Scope And Decisions

- Use Cargo package metadata for the CLI version. Do not change the package
  version in `Cargo.toml`.
- Verify the release artifact with the host `file` and `ldd` commands. Record
  dynamic-linking truth; do not add a musl/static build or claim portability
  beyond the verified target.
- Update only active README and Arc42 claims that contradict the current
  implementation. Archived snapshots remain in `givn/archive/` and are labeled
  historical by their archive location and documentation index.
- Remove confirmed dead code and obsolete names only after repository-wide usage
  search. Keep public modules and result types when their external-consumer
  status cannot be established from this binary repository.
- Do not run a repository-wide formatting rewrite as part of this change.

## Architecture Impact

### Version

Change the Clap package version declaration in `src/main.rs` to use Cargo's
compile-time package metadata. The existing `--version` output remains the same
format; only the source of the version value changes. The modified Gherkin
scenario asserts the exact package version currently declared in `Cargo.toml`.

### Release verification

The release verification step builds `cargo build --release`, then invokes
host-appropriate inspection commands. On Linux it invokes:

```text
file target/release/watn
ldd target/release/watn
```

On macOS it invokes `otool -L target/release/watn` instead of `ldd`. The `file`
result must identify a dynamically linked executable for the current host, and
the host library inspection must succeed with at least one shared library entry.
The test does not assert a universal library set because library names vary by
target. The deployment chapter states target-dependent dynamic runtime
requirements and explicitly avoids a static-deployment claim.

### Documentation reconciliation

Update these active claims where stale:

- README: incremental command streaming, buffered verbose reasoning, XDG config
  storage, and the current completion/confirmation behavior.
- `docs/arc42/01-introduction-and-goals.md`, `02-architecture-constraints.md`,
  `03-context-and-scope.md`, `04-solution-strategy.md`,
  `05-building-block-view.md`, `06-runtime-view.md`,
  `07-deployment-view.md`, `08-crosscutting-concepts.md`,
  `09-architecture-decisions.md`, `10-quality-requirements.md`,
  `11-risks-and-technical-debt.md`, and `12-glossary.md`: remove stale static,
  data-directory, dialoguer/helper-name, plain-`r`, deferred-verification, and
  output-channel claims and record the current facts.
- `docs/arc42/README.md` and the documentation index: archived Arc42 snapshots
  are historical records, not current architecture, with an explicit archive
  status section.
- Remove obsolete helper names and claims only after exact-tree search proves
  they are not active interfaces.

### Hygiene decisions

Search results determine each cleanup:

- Remove the unused `_config` parameter from `build_registry()`.
- Retain `ProviderRegistry`: it remains a public library module and provides the
  stable lookup boundary even though the binary currently registers one active
  provider.
- Retain `ProviderSetupResult` and its public wrapper functions because current
  feature steps consume them and external library consumers cannot be ruled out.
- Remove write-only fields from `WatnWorld` only when no feature step reads or
  writes them after the active archived-spec migration.
- Remove only fields proven write-only by repository-wide search, recording the
  exact field list in tasks and preserving all fields used by permanent steps.
- Remove stale helper names and documentation claims after exact grep confirms
  they are obsolete.

No persisted configuration shape, provider behavior, model discovery, setup
interaction, or streaming output behavior changes.

## Test Infrastructure

### Step definitions

- `tests/steps/release_truth_steps.rs`: regular release artifact, version, and
  documentation assertions. It uses temporary copies or repository paths and
  invokes only local commands/files.
- `tests/steps/release_truth_e2e_steps.rs`: the real subprocess invocation for
  the modified `--version` scenario, using the unique wording `run the release
  binary with --version` so it does not collide with the existing version step.
  It is separate from regular steps and
  asserts captured CLI output first.
- `tests/features_runner.rs`: register the two modules and add only the small
  release-specific state needed for captured output.

Reuse existing subprocess/environment helpers only where their expressions do
not collide with the release-specific wording. New RED bodies use
`unimplemented!()`; the runner's `.fail_on_skipped()` makes them fail.

### Local runnability and digital twins

This is a single binary CLI. The full local verification command is the current
`verify.command` from `givn/commands.yaml`:

```text
./run-tests.sh
```

The E2E command is:

```text
./run-tests.sh --e2e
```

The scripts build explicit debug binaries and run the Cucumber feature runner.
No application server, database, container, or external service is required.
The release artifact is inspected locally; its target-dependent runtime
libraries are part of the release evidence, and the version scenario invokes the
real binary.

The anticipated interface obstacle is binary selection: step definitions use
the explicit binary path exported by the runner rather than discovering a
possibly stale `target/debug/watn`. Release inspection uses the exact
`target/release/watn` produced by the scenario's build command.

### Strict mode and E2E fidelity

The Cucumber runner is `cucumber-rs` and calls `.fail_on_skipped()` in
`tests/features_runner.rs`. New Rust RED steps use `unimplemented!()` and are
never left empty. The capability interface is CLI-only. The E2E scenario drives
the built binary as a real subprocess and asserts its stdout/stderr and exit
status; repository/file assertions are secondary.

## Coverage Process Boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Cucumber runner and child debug binaries | `measure-coverage.sh` | `cargo llvm-cov` test runner and copied instrumented `watn` binaries | `coverage/profraw/%p-%m.profraw` | `merge-coverages.sh` per-file/per-line union | `watn --version` and release documentation checks |

Branch coverage is not claimed on the stable toolchain because the installed
cargo-llvm-cov branch mode requires nightly. Line coverage remains measured and
the report explicitly records branch coverage as `n/a`.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| run `watn --version` and inspect the reported package version | Version flag reports the package version | CLI | A real built `watn --version` subprocess is invoked with the runner's explicit binary path; stdout and exit status are asserted. |

The release-artifact and documentation scenarios are regular maintainer-facing
repository checks, not additional user interactions, so they do not receive
duplicate E2E tags.

## Single-Scenario Commands

Use the explicit binary bootstrap and Cucumber name filter for one scenario:

```text
root=$(mktemp -d /tmp/watn-release.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --test features_runner --features test-support -- --name "<scenario title>"
```

This command executes the Cucumber feature files under both `givn/specs/` and
the active change directory through `tests/features_runner.rs`; it is not a
bare unit-test command.

## Implementation Order

1. Configure the package-derived version scenario and prove strict step
   execution.
2. Build and inspect the release artifact, then correct deployment claims.
3. Reconcile active README/Arc42 claims and archive-status wording.
4. Search and remove only confirmed dead code and obsolete helper names.
5. Run full non-E2E/E2E verification, hygiene checks, and review before archive.

Each scenario follows RED, GREEN, REFACTOR, one atomic commit, and immediate
task evidence recording.
