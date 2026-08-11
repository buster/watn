# Main-First GitHub Actions Release Flow

Status: planning and handover document
Last reviewed: 2026-08-11
Repository: `buster/watn`
Crate: `watn`
Primary branch: `main`

## Purpose

Define a safe, simple release process for the `watn` Rust crate.

The project is maintained by a sole developer. Development happens directly on
`main`. The project does not use pull requests or squash merges.

The current release target is crates.io only. Do not build or publish GitHub
release binaries, Docker images, packages, installers, or other distribution
targets.

A Git tag is still required as the immutable source marker for a crates.io
publication. The tag is repository metadata, not an additional distribution
target.

## Non-Goals

Do not introduce:

- Pull-request-based release preparation.
- Squash-merge requirements.
- Automatic GitHub Release objects.
- Binary artifact uploads.
- Docker or container publication.
- Cross-platform release matrices.
- Automatic version selection without human review.
- Automatic publication from branch pushes.
- A long-lived crates.io token if Trusted Publishing is available.

## Automation Boundary

The repository implementation can be prepared without access to the maintainer's
GitHub or crates.io accounts. No application API key is needed for CI: the
acceptance tests use mock providers and must not call OpenRouter or another live
provider.

### Repository-Only Work

An agent or contributor with repository write access can complete these tasks:

- Inspect the current branch, package metadata, dependency graph, and published
  version history.
- Correct the CLI version and version-test drift.
- Add `publish = ["crates-io"]` to `Cargo.toml`.
- Add `cliff.toml`, `deny.toml`, `CHANGELOG.md`, and the workflow files.
- Derive the release version from the pushed tag and compare it with
  `Cargo.toml`; do not store the version in a secret or repository variable.
- Pin every action and tool to a reviewed version or full commit SHA.
- Run formatting, compilation, lint, tests, package inspection, `cargo audit`,
  `cargo deny`, `actionlint`, and `zizmor`.
- Exercise tag and manifest mismatch failures without publishing.
- Generate a changelog draft and show the package contents before publication.

These tasks can be prepared in a branch, but the maintainer must still review
the resulting diff and explicitly approve pushing a release tag or publishing a
crate.

### External Account Work

An agent cannot finish these tasks from a repository checkout alone:

- Log in to crates.io and create or confirm the Trusted Publisher.
- Log in to GitHub and create the `crates-io` environment.
- Configure environment reviewers, protected `v*` tags, and repository Actions
  settings.
- Create, copy, rotate, or revoke a crates.io API token.
- Store a fallback token in GitHub Actions secrets.
- Enable maintainer two-factor authentication and decide whether to enable
  dependency-alert notifications.
- Make the final version decision and approve the generated changelog.
- Approve the first real `cargo publish` run.

Trusted Publishing is preferred because it avoids storing a long-lived registry
credential. The publish job receives a short-lived OIDC token only when all
validation checks pass. The GitHub `GITHUB_TOKEN` is supplied automatically by
GitHub and is not a secret that the maintainer needs to create.

### Variables And Secrets

Use these names consistently in commands, workflow expressions, and account
configuration:

| Name | Where it is defined | Required value |
|---|---|---|
| `REPOSITORY` | Documentation or local shell variable | `buster/watn` |
| `CRATE_NAME` | Documentation or local shell variable | `watn` |
| `PRIMARY_BRANCH` | Documentation or local shell variable | `main` |
| `RELEASE_WORKFLOW` | Documentation or Trusted Publisher setting | `.github/workflows/release.yml` |
| `RELEASE_PREPARATION_WORKFLOW` | Documentation or Actions workflow | `.github/workflows/prepare-release.yml` |
| `RELEASE_ENVIRONMENT` | Workflow and GitHub environment | `crates-io` |
| `RELEASE_TAG` | Local release command | `vX.Y.Z`, selected manually |
| `RELEASE_PUSH_TOKEN` | GitHub Actions secret, release preparation only | A fine-grained GitHub token with Contents write; never print it |
| `CARGO_REGISTRY_TOKEN` | GitHub Actions secret, fallback only | A crates.io publish token; never commit it |

`CARGO_REGISTRY_TOKEN` is not required when Trusted Publishing is configured.
If it is used as a fallback, expose it only to the publish step with
`${{ secrets.CARGO_REGISTRY_TOKEN }}`. Never print it, put it in a workflow file,
or use it in CI and validation jobs. Do not use `OPENROUTER_API_KEY`,
`WATN_API_KEY`, or any provider credential in CI or release validation.

## Repository Snapshot

The checked-out worktree at the time of this handover is:

- Worktree branch: `bright-fireant`
- HEAD: `f523548`
- `origin/main`: `f523548`
- Working tree: clean before this document was added
- Local Git tags: none
- GitHub Actions workflows: none
- `CHANGELOG.md`: absent
- `cliff.toml`: absent
- `deny.toml`: absent
- `dependabot.yml`: absent

The repository also has a separate local `main` branch at `ea44c40`, which is
ahead of the checked-out branch. That branch contains commit `5c7b4e8`, which
changes the CLI version to use Cargo package metadata. Confirm the intended
branch lineage before implementing release changes.

Current package metadata:

- Package name: `watn`
- Package version: `0.1.2`
- License: `GPL-3.0-or-later`
- Edition: `2021`
- Repository: `https://github.com/buster/watn`
- Binary target: `watn`
- Library target: `watn`
- Committed lockfile: yes
- Current dependency sources: crates.io registry dependencies
- Current package exclusions: `coverage/`, `givn/`, `docs/`, `.agents/`,
  `.claude/`, `.opencode/`, `.ralph/`

Important current drift:

- `Cargo.toml` declares `0.1.2`.
- The checked-out `src/main.rs` still reports `0.1.0`.
- The checked-out version test still expects `0.1.0`.
- The separate local `main` branch contains the fix, but it is not present in
  the checked-out `bright-fireant` branch.

The version mismatch must be corrected before the release workflow is treated
as complete.

## Existing Test Structure

The repository has a custom acceptance-test runner:

- Main integration target: `tests/features_runner.rs`
- Acceptance scenarios: `givn/specs/`
- Test commands: `givn/commands.yaml`
- README development command: the locked binary-setup command documented in
  `README.md` and `givn/commands.yaml`

The configured Givn commands build separate debug binaries and use environment
variables to distinguish normal and `test-support` binaries. The release
workflow must not silently replace those commands with a simpler command unless
the behavior has been verified.

Every Cargo invocation used by CI or release validation should use `--locked`.

The repository is Linux-oriented for its current acceptance tests because the
test setup uses shell commands, temporary directories, PTY support, and
Linux-specific dependencies. Use an Ubuntu GitHub-hosted runner initially.

## Release Model

The source of truth is:

1. `Cargo.toml` package version.
2. `Cargo.lock` package version and locked dependency graph.
3. `watn --version` runtime output.
4. `CHANGELOG.md` release heading.
5. Annotated Git tag `vX.Y.Z`.

All five must agree.

The release version must be selected manually. Conventional Commit types may
inform the decision, but the version must not be inferred mechanically. This
is especially important while the package is below `1.0.0`.

Recommended version policy:

- `fix:` usually means a patch release.
- `feat:` usually means a minor release.
- A documented breaking change requires an explicit versioning decision.
- Never reuse a version already published to crates.io.

## Direct-to-main Development Flow

Normal work is committed directly to `main`.

Use user-facing Conventional Commit-style messages where practical:

```text
feat(cli): add shell completion support
fix(provider): preserve saved credentials
docs: clarify configuration
test: cover provider cancellation
chore: update development tooling
```

Squash merges are not required. A linear direct-commit history is suitable for
`git-cliff`.

The quality of generated release notes depends on commit messages. Configure
`git-cliff` to include meaningful user-facing changes and exclude internal
noise such as most tests, CI maintenance, release commits, and repository
housekeeping.

Do not rely on local Git hooks for security. Hooks are not copied during clone
and can be bypassed with `--no-verify`. A local `commit-msg` hook may be used as
a convenience for the sole developer, but CI and the release workflow remain
authoritative.

## Generated Changelog

Use `git-cliff` for changelog generation.

The reviewed generator version for this setup is `git-cliff 2.13.1`; verify the
local tool with `git cliff --version` before preparing a release.

`git-cliff`:

- Reads Git history.
- Uses Conventional Commit-style messages.
- Groups entries into release-note sections.
- Uses Git tags as release boundaries.
- Supports a checked-in `cliff.toml`.
- Is a development tool and must not become a runtime dependency.

The changelog must be generated before the release tag is created.

Do not have the final publish workflow generate and commit the changelog. That
would require repository write permissions after tagging and could make the
published source differ from the tagged source.

Recommended release preparation:

1. Start from a clean, current `main`.
2. Review commits since the previous release tag.
3. Select the next version manually.
4. Generate the changelog for that version.
5. Update `Cargo.toml`.
6. Update `Cargo.lock`.
7. Run all tests and security checks.
8. Review the generated changelog.
9. Commit the release preparation directly to `main`.
10. Create the annotated tag on that release commit.
11. Push `main` and the tag.

With no historical `v0.1.2` tag, generate only the commits after the verified
published source into a review file. Do not overwrite the historical sections
in `CHANGELOG.md` with an unbounded no-tag generation:

```text
git cliff d5ddb36..HEAD --tag v0.1.3 -o /tmp/watn-CHANGELOG.md
```

After the historical tag is approved, use `v0.1.2..HEAD` instead of the
commit range. Review the generated release section, replace the matching
`[Unreleased]` section, and preserve the historical baseline before committing.

The `cliff.toml` configuration should:

- Use `vX.Y.Z` tags.
- Recognize `feat`, `fix`, `perf`, and breaking-change commits.
- Exclude or separately classify `test`, `ci`, and release-maintenance
  commits.
- Avoid silently losing non-conforming commits during the initial history
  migration.
- Produce ISO-formatted release dates.
- Include comparison links where practical.
- Exclude the generated release commit itself from future release notes.

A generated changelog still requires human review. Commit messages can contain
inaccurate or awkward text. Since this project does not use pull requests, the
commit message itself is the release-note input.

## Initial Changelog Baseline

The crates.io API shows published versions:

- `0.1.0`
- `0.1.1`
- `0.1.2`

The repository currently has no Git tags.

The existing `0.1.2` publication was manual and has no Trusted Publishing
metadata. Crates.io versions are permanent and cannot be overwritten.

Before implementing the first automated release:

1. Verify whether commit `d5ddb36` corresponds exactly to the published
   `0.1.2`.
2. If verified, create an annotated `v0.1.2` tag on that historical commit.
3. If it cannot be verified, do not fabricate a historical tag.
4. In that case, record the historical release in `CHANGELOG.md` and begin
   automated tagging with the next unpublished version, likely `0.1.3`.

Do not tag the current HEAD as `v0.1.2` solely because `Cargo.toml` still says
`0.1.2`. The current branch contains many commits after the version bump and
publication.

## Local Release Sequence

The preferred release sequence is local preparation followed by a tag push.

Before preparing a release:

```text
git switch main
git pull --ff-only
git status --short
```

The working tree must be clean before release preparation begins.

Release preparation should:

- Confirm the current branch is `main`.
- Confirm local `main` is synchronized with `origin/main`.
- Select the next unpublished version.
- Generate `CHANGELOG.md`.
- Update `Cargo.toml`.
- Update `Cargo.lock`.
- Run formatting, compilation, lint, tests, package, and security checks.
- Verify `watn --version`.
- Commit only the intended release files.

Recommended commit message:

```text
chore(release): prepare v0.1.3
```

Create an annotated tag:

```text
git tag -a v0.1.3 -m "Release v0.1.3"
```

Push the release commit and tag together where supported:

```text
git push --atomic origin main v0.1.3
```

If atomic push is not used, push `main` first and the tag second. The tag
should never point to a commit that is not already present on `origin/main`.

The tag push is the only event that may trigger crates.io publication.

### GitHub Release Preparation Workflow

The repository also provides a manually triggered `Prepare Release` workflow at
`${RELEASE_PREPARATION_WORKFLOW}`. Start it from the `main` branch in the GitHub
Actions tab and provide the next version without the `v` prefix, such as
`0.1.3`. The input is required and is never inferred from commit messages.

The workflow validates that the version is newer than the manifest version and
does not already exist on crates.io. It then updates `Cargo.toml`, refreshes
`Cargo.lock`, generates the versioned changelog section with the pinned
`git-cliff` tool, runs the locked validation and package checks, commits only
the release preparation files, and creates an annotated `vX.Y.Z` tag. It pushes
the release commit and tag atomically with the `RELEASE_PUSH_TOKEN`.

The existing tag-only `${RELEASE_WORKFLOW}` workflow remains responsible for
Trusted Publishing. A failed preparation or dry run does not push the tag.

The preparation workflow requires `${RELEASE_PUSH_TOKEN}` rather than the
default `GITHUB_TOKEN` for its final push. GitHub does not start downstream
workflows for most events created by `GITHUB_TOKEN`; the fine-grained token
allows the pushed release tag to start `${RELEASE_WORKFLOW}` normally.

## CI Workflow

Create `.github/workflows/ci.yml`.

The CI workflow should run on:

- Pushes to `main`.
- Explicit `workflow_dispatch` runs.

It should not publish anything.

Default permissions:

```yaml
permissions:
  contents: read
```

Initial checks:

```text
cargo fmt --all -- --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --lib
```

Run the repository's existing non-E2E and E2E acceptance commands from
`givn/commands.yaml`, adding `--locked` to every Cargo invocation.

The CI job should use:

- An Ubuntu GitHub-hosted runner.
- A pinned Rust toolchain policy.
- `actions/checkout` pinned to a full commit SHA.
- `persist-credentials: false`.

Do not make CI depend on secrets.

## Security Workflow

Create `.github/workflows/security.yml`.

The security workflow should run:

- On pushes to `main`.
- On changes to `Cargo.toml`, `Cargo.lock`, `deny.toml`, or workflows.
- On a daily or weekly schedule.
- On explicit `workflow_dispatch`.

Recommended checks:

```text
cargo audit
cargo deny --locked check advisories bans licenses sources
actionlint
zizmor
```

The workflow must use version-pinned tools or actions. Do not install
unversioned tools from `latest` during a release.

### cargo audit

`cargo audit` checks the committed dependency graph against the RustSec
Advisory Database.

It detects known issues, not novel malicious code.

Run it:

- On regular pushes.
- On a schedule, because advisories can be published after the last commit.
- Again during release validation.

The RustSec `audit-check` action may be used, but pin it to a full commit SHA.
Do not grant issue-writing permissions unless automatic issue creation is
explicitly desired.

### cargo deny

Add `deny.toml`.

Use `cargo-deny` for:

- RustSec advisories.
- License policy.
- Duplicate dependency detection.
- Dependency source restrictions.

At minimum:

- Deny unknown registries.
- Deny unknown Git sources.
- Allow crates.io.
- Fail on known advisories.
- Define an explicit acceptable license policy.
- Decide whether duplicate dependencies are warnings or errors.

`cargo-deny` does not prove that crates.io packages are safe. It enforces
dependency policy and known advisory checks.

### Dependency Updates

Do not introduce Dependabot update pull requests because the project does not
use pull-request-based development.

GitHub's dependency graph and Dependabot alerts may still be enabled for
notifications. Dependency updates should be handled as direct commits to
`main`, with the normal tests and security checks.

Do not automatically merge dependency updates.

Review every `Cargo.lock` change, especially:

- New direct dependencies.
- New transitive dependencies.
- Source changes from registry to Git.
- Large dependency graph changes.
- Changes to build scripts or procedural macros.

## Workflow Security

All GitHub Actions must be pinned to full commit SHAs.

Do not use:

```yaml
uses: some/action@latest
uses: some/action@v1
```

unless the action is deliberately accepted as a temporary exception.

Use comments beside pinned SHAs to record the human-readable release tag.

Use:

```yaml
permissions:
  contents: read
```

as the default.

Use `persist-credentials: false` for checkout.

Do not use privileged workflows that check out untrusted code through
`pull_request_target` or `workflow_run`.

Run:

- `actionlint` to catch workflow syntax, expression, shell, and injection
  problems.
- `zizmor` to inspect workflow security.
- OpenSSF Scorecard periodically to assess repository-level controls.

The project should also enable:

- Two-factor authentication on the maintainer account.
- Protected version tag pattern `v*`.
- Prevention of tag updates and deletion where possible.
- A protected crates.io GitHub environment.
- Required approval for the crates.io environment if a manual publication gate
  is desired.

## Release Workflow

Create `.github/workflows/release.yml`.

Trigger only on version tags:

```yaml
on:
  push:
    tags:
      - "v*.*.*"
      - "!v0.1.2"
```

The repository's existing `v0.1.2` publication is excluded from this trigger;
it may be recorded as an annotated historical tag, but it must never enter the
automated publish path again.

Do not publish on:

- Pushes to `main`.
- `workflow_dispatch`.
- Pull requests.
- Arbitrary branch names.
- Arbitrary user-provided version inputs.

Use a concurrency group:

```yaml
concurrency:
  group: crates-io-watn-release
  cancel-in-progress: false
```

The workflow has two jobs:

1. `validate`
2. `publish`

### Validate Job

The validate job must have no crates.io credentials.

It should:

- Check out the full history.
- Confirm the tag matches `vX.Y.Z`.
- Confirm the tag points to a commit reachable from `main`.
- Compare the tag version with `Cargo.toml`.
- Verify the lockfile is consistent.
- Build and run `watn --version`.
- Run formatting, checks, lint, and tests.
- Run `cargo audit`.
- Run `cargo deny`.
- Review package contents.
- Run `cargo package --locked --list`.
- Run `cargo publish --locked --dry-run`.

Important commands:

```text
cargo fmt --all -- --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --lib
cargo build --locked --release --bin watn
target/release/watn --version
cargo package --locked --list
cargo publish --locked --dry-run
```

Run the repository's existing acceptance test commands with `--locked`.

The package list must be reviewed carefully. `PLAN.md`, `QUESTIONS.md`,
`githubflow.md`, coverage output, internal specifications, and development-only
files should not accidentally enter the published crate.

Consider adding these to the Cargo manifest exclusion list:

```text
PLAN.md
QUESTIONS.md
githubflow.md
```

Only add exclusions after confirming they are not needed by the package build or
README.

### Publish Job

The publish job:

- Depends on successful validation.
- Uses the `crates-io` GitHub environment.
- Has `contents: read`.
- Has `id-token: write`.
- Uses crates.io Trusted Publishing.
- Publishes only the tagged package.

Conceptual permissions:

```yaml
permissions:
  contents: read

jobs:
  publish:
    environment: crates-io
    permissions:
      contents: read
      id-token: write
```

Configure crates.io Trusted Publishing for:

- Repository: `buster/watn`
- Workflow file: `.github/workflows/release.yml`
- Environment: `crates-io`

Use the official:

```text
rust-lang/crates-io-auth-action
```

Pin it to a full commit SHA.

The action produces a temporary token. Pass it only to Cargo:

```yaml
env:
  CARGO_REGISTRY_TOKEN: ${{ steps.auth.outputs.token }}
```

Publish with:

```text
cargo publish --locked
```

Do not use:

```text
cargo publish --allow-dirty
cargo publish --no-verify
cargo publish || true
```

If publication fails after the upload has begun, inspect crates.io before
retrying. A version may already exist even if the workflow reported an error
while waiting for registry propagation.

## Cargo Manifest Policy

Add an explicit crates.io publication restriction:

```toml
publish = ["crates-io"]
```

Keep `Cargo.lock` committed.

All release and CI commands must use `--locked`.

Review `cargo package --locked --list` before the first automated release.

The current manifest has useful metadata:

- Description.
- License.
- Repository.
- Homepage.
- Documentation URL.
- README.
- Keywords.
- Category.

Do not claim that the release binary is statically linked. Existing project
documentation states that the current release binary is dynamically linked and
has target-dependent runtime library requirements.

## Binary Auditing

The current release target is crates.io source publication only. Cargo
publishes a source archive, not a prebuilt executable.

Optional future binary auditing:

```text
cargo auditable build --release
cargo audit bin target/release/watn
```

`cargo-auditable` embeds dependency metadata into a binary. It helps identify
the exact dependency tree used by a binary and allows later vulnerability
scanning.

It does not detect malicious behavior and does not prove a binary is safe.

Do not make `cargo-auditable` a release requirement until prebuilt binaries are
actually distributed.

## cargo-vet

`cargo-vet` is a possible later maturity step.

It helps establish whether dependency versions have been audited by trusted
entities. It requires an audit policy and audit records, so it is more work
than `cargo audit` or `cargo deny`.

Use it later if:

- The project becomes security-sensitive.
- The dependency graph grows substantially.
- Stronger protection against novel malicious dependency code is required.

Do not claim that SBOMs or dependency metadata prevent supply-chain attacks.

## Git Hooks

Git hooks are optional convenience mechanisms only.

Allowed local uses:

- `commit-msg`: check Conventional Commit formatting.
- `pre-commit`: run formatting checks.
- `pre-push`: run fast tests.

Do not put `cargo publish` in a Git hook.

Do not make release correctness depend on hooks. Hooks are not cloned and can
be bypassed with `--no-verify`.

The authoritative controls are:

- Main-branch CI.
- Scheduled security checks.
- The release tag validation job.
- Protected version tags.
- The crates.io environment.
- Trusted Publishing.

## Release Acceptance Criteria

The implementation is complete only when all of these are true:

- The intended `main` lineage is confirmed.
- CLI version comes from Cargo package metadata.
- The version test no longer hard-codes `0.1.0`.
- `CHANGELOG.md` exists and is generated by `git-cliff`.
- `cliff.toml` exists and documents the changelog policy.
- The initial historical changelog baseline is reviewed.
- The existing `0.1.2` publication is not overwritten or reused.
- `publish = ["crates-io"]` is configured.
- Package contents have been inspected.
- CI runs on direct pushes to `main`.
- CI does not have publication credentials.
- Scheduled security checks exist.
- `cargo audit` runs successfully.
- `cargo deny` runs successfully.
- Workflow files pass `actionlint`.
- Workflow files pass `zizmor`.
- Actions are pinned to full commit SHAs.
- The release workflow triggers only from `vX.Y.Z` tags.
- Tag version, manifest version, lockfile, CLI output, and changelog agree.
- The release workflow runs `cargo publish --locked --dry-run`.
- Trusted Publishing is configured.
- Only the publish job receives `id-token: write`.
- No GitHub Release or binary artifact is created.
- A test release validation failure is demonstrated before the first real
  automated publication.
- No commit is pushed and no crate is published without explicit approval.

## Implementation Order

Tasks in the first group do not require maintainer credentials or account
changes. Finish and review them before doing the account setup in the second
group. The version and historical-tag decisions are called out as inputs even
though the repository inspection itself is automatable.

### Repository Tasks

1. Confirm the intended `main` branch lineage by comparing `bright-fireant`,
   local `main`, and `origin/main`.
2. Fix CLI/package version drift and make the version test derive its expected
   value from Cargo package metadata.
3. Add `publish = ["crates-io"]` to `Cargo.toml`.
4. Review package exclusions and add `PLAN.md`, `QUESTIONS.md`, and
   `githubflow.md` only if they are not needed by the package build or README.
5. Add `cliff.toml` with the `vX.Y.Z` tag policy.
6. Prepare the initial `CHANGELOG.md` draft, preserving historical releases
   without inventing a tag.
7. Add `deny.toml` with the registry, source, license, duplicate, and advisory
   policy.
8. Add direct-push CI with no secrets or publication permissions.
9. Add scheduled dependency and workflow security checks.
10. Add manually triggered release preparation plus tag-only release validation
    and crates.io publication workflows, using the variables in this document.
11. Pin actions and tool versions, then run `actionlint` and `zizmor` locally
    against every workflow.
12. Run all `--locked` formatting, build, lint, unit, acceptance, audit, deny,
    package-list, and publish-dry-run checks.
13. Exercise these failure cases without publishing:
    - Mismatched tag and manifest versions.
    - Missing changelog version.
    - Stale lockfile.
    - Failing security check.
    - Invalid package contents.
    - Missing OIDC permissions.
14. Review the complete diff and package list, then commit the repository-only
    changes directly to `main`.

### Maintainer Tasks

15. Decide whether `d5ddb36` exactly represents the published `0.1.2`; if not,
    do not create a historical `v0.1.2` tag.
16. Select the next unpublished `RELEASE_TAG` and approve the initial changelog.
17. Configure the GitHub `RELEASE_ENVIRONMENT` environment and its reviewers.
18. Protect the `v*` tag pattern against deletion and updates.
19. Configure the crates.io Trusted Publisher for:
    - Repository: `${REPOSITORY}` (`buster/watn`).
    - Workflow: `${RELEASE_WORKFLOW}` (`.github/workflows/release.yml`).
    - Environment: `${RELEASE_ENVIRONMENT}` (`crates-io`).
20. If Trusted Publishing cannot be used, create `CARGO_REGISTRY_TOKEN` and
    store it as a GitHub Actions secret instead of putting it in the repository.
21. Create a fine-grained `RELEASE_PUSH_TOKEN` secret with Contents write access
    so the preparation workflow's tag push starts the publish workflow.
22. Approve the first real tag push and publication after a successful dry run.

## Project Process

The repository uses Givn for feature work.

Before editing, run:

```text
givn instructions
```

Follow the repository lifecycle:

```text
new -> propose -> spec -> design -> design-review -> tasks -> implement -> review -> archive
```

Do not amend existing commits.

Do not push unless explicitly requested.

Keep release-process work separate from unrelated behavioral changes.

## Research Sources

Kagi searches were attempted but the Kagi API returned `401 Unauthorized` in
the available environment. The following primary sources were used instead:

- Cargo publishing:
  https://doc.rust-lang.org/cargo/reference/publishing.html
- Cargo publish:
  https://doc.rust-lang.org/cargo/commands/cargo-publish.html
- git-cliff:
  https://git-cliff.org/docs/
- release-plz:
  https://github.com/release-plz/release-plz
- RustSec cargo-audit:
  https://github.com/RustSec/rustsec/tree/main/cargo-audit
- cargo-deny:
  https://embarkstudios.github.io/cargo-deny/
- cargo-vet:
  https://mozilla.github.io/cargo-vet/
- cargo-auditable:
  https://github.com/rust-secure-code/cargo-auditable
- GitHub Dependency Review:
  https://docs.github.com/en/code-security/concepts/supply-chain-security/dependency-review
- OpenSSF Scorecard:
  https://github.com/ossf/scorecard
- actionlint:
  https://github.com/rhysd/actionlint
- zizmor:
  https://zizmor.sh/
- GitHub Actions security:
  https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions
- GitHub Actions OIDC:
  https://docs.github.com/en/actions/concepts/security/openid-connect
- crates.io Trusted Publishing:
  https://crates.io/docs/trusted-publishing
- crates.io auth action:
  https://github.com/rust-lang/crates-io-auth-action

## Maintainer Completion Checklist

The repository-only implementation should be finished and reviewed first. The
remaining release steps require the maintainer's account access or approval:

1. Confirm that the intended release branch is `main`, and decide whether
   commit `d5ddb36` is the exact source for the published `0.1.2` crate.
2. If that historical release cannot be verified, leave `v0.1.2` untagged and
   choose the next unpublished version, for example:

   ```text
   export REPOSITORY="buster/watn"
   export CRATE_NAME="watn"
   export PRIMARY_BRANCH="main"
   export RELEASE_WORKFLOW=".github/workflows/release.yml"
   export RELEASE_PREPARATION_WORKFLOW=".github/workflows/prepare-release.yml"
   export RELEASE_ENVIRONMENT="crates-io"
   export RELEASE_TAG="v0.1.3"
   ```

   Replace `RELEASE_TAG` with the manually approved version; do not reuse a
   version already present on crates.io.
3. Review and approve the generated `CHANGELOG.md`, version change, lockfile,
   and `cargo package --locked --list` output.
4. In GitHub repository settings, create the `crates-io` environment and add
   any required approval rule. Protect the `v*` tag pattern from updates and
   deletion.
5. In GitHub repository secrets, configure the fine-grained
   `RELEASE_PUSH_TOKEN` with Contents write access.
6. In crates.io, configure Trusted Publishing for repository `buster/watn`,
   workflow `.github/workflows/release.yml`, and environment `crates-io`.
7. If Trusted Publishing is unavailable, create a narrowly scoped crates.io
   publish token and add it in GitHub as the Actions secret
   `CARGO_REGISTRY_TOKEN`. Do not create this secret when OIDC is working.
8. Confirm that no provider API keys are configured for CI. The test suite must
   use mocks rather than `OPENROUTER_API_KEY` or `WATN_API_KEY`.
9. Approve the first release only after the tag validation and
   `cargo publish --locked --dry-run` jobs pass.
10. Start `${RELEASE_PREPARATION_WORKFLOW}` from the `main` branch in the GitHub
    Actions tab with the approved version, without the `v` prefix:

   ```text
   version=0.1.3
   ```

11. Confirm on crates.io that the exact intended `RELEASE_TAG` version was
    published, then record the release result and any failure or retry details.
