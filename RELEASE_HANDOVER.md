# Release Handover

Status: repository-only release implementation is complete. Maintainer account
configuration, release approval, and the first automated publication remain.

Repository: `buster/watn`
Crate: `watn`
Primary branch: `main`
Release workflow: `.github/workflows/release.yml`
Release preparation workflow: `.github/workflows/prepare-release.yml`
Release environment: `crates-io`

## Implemented

- CLI and acceptance tests derive the version from Cargo package metadata.
- Cargo publication is restricted to crates.io.
- Development, planning, coverage, and workflow files are excluded from the
  published package.
- `CHANGELOG.md` and `cliff.toml` define the tagged release-note flow.
- `deny.toml` defines dependency advisory, license, duplicate, and source
  policies.
- CI runs locked formatting, compilation, lint, unit, non-E2E, and E2E checks on
  `main` pushes and manual dispatches.
- Security checks run locked dependency audits, `cargo audit`, `cargo deny`,
  `actionlint`, and `zizmor` on `main` pushes, weekly schedule, and manual
  dispatches.
- Release validation runs only for annotated version tags other than the
  already-published `v0.1.2` tag.
- The `Prepare Release` workflow can be started from the Actions tab with an
  explicit version. It updates the manifest, lockfile, and changelog, validates
  the package, commits the preparation, and pushes the annotated tag.
- Version and lockfile updates use pinned `cargo-release`; changelog generation
  uses pinned `git-cliff`.
- CI and preparation cache Cargo/build state and versioned tools; the
  tag-triggered publish workflow remains cache-free for hermetic validation.
- Only the publish job receives `id-token: write` and uses crates.io Trusted
  Publishing.

## Maintainer Actions

1. Review and land the repository changes on the intended `main` branch.
2. Create the GitHub environment `crates-io` and configure any required
   reviewers.
3. Protect the `v*` tag pattern against updates and deletion.
4. Configure crates.io Trusted Publishing for repository `buster/watn`,
   workflow `.github/workflows/release.yml`, and environment `crates-io`.
5. Decide whether to create the historical `v0.1.2` tag. The published crate
   was verified against commit `d5ddb36`; if accepted, tag that commit rather
   than the current release-flow branch.
6. Select the next unpublished version, likely `v0.1.3`, and start `Prepare
   Release` from the Actions tab with that version as input.
7. Approve the first real release only after the preparation workflow's tag
   validation and `cargo publish --locked --dry-run` pass.
8. Review the generated changelog, manifest version, lockfile, and package
   contents before allowing the tag-triggered publish workflow to complete.

The historical `0.1.0`, `0.1.1`, and `0.1.2` crates already exist on crates.io.
Do not reuse any of those versions or tag the current commit as `v0.1.2`.

## Release Sequence

After the repository changes are on `origin/main`:

```sh
# In GitHub: Actions -> Prepare Release -> Run workflow
# Enter the approved version without the `v` prefix, for example `0.1.3`.
```

The preparation workflow creates the release commit and annotated tag only
after validation and a clean `cargo publish --locked --dry-run`. The tag push is
the only event that can publish a crate. The workflow creates no GitHub Release
object and uploads no binary artifacts.

## Secrets And Credentials

No provider API key is needed by CI or release validation. Acceptance tests use
mock providers.

Do not add `CARGO_REGISTRY_TOKEN` when Trusted Publishing is configured. If
Trusted Publishing is unavailable, the workflow must be deliberately changed
to support a narrowly scoped fallback token before adding that secret.

Configure a fine-grained GitHub Actions secret named `RELEASE_PUSH_TOKEN` with
Contents write access. The preparation workflow uses it for the final commit
and tag push because pushes made with the default `GITHUB_TOKEN` do not trigger
the downstream tag-publish workflow.

## Verification Evidence

- `cargo fmt --all -- --check` passed.
- `cargo check --locked --all-targets` passed.
- `cargo clippy --locked --all-targets -- -D warnings` passed.
- `cargo test --locked --lib` passed.
- 56 non-E2E acceptance scenarios passed.
- 52 E2E acceptance scenarios passed.
- The release binary reported `watn 0.1.2`.
- Package contents excluded development-only files.
- `cargo audit` passed with an existing transitive `async-std` unmaintained
  warning.
- `cargo deny --locked check advisories bans licenses sources` passed with
  duplicate-dependency warnings.
- `actionlint` passed.
- `zizmor` reported no findings.

No release tag or crate publication was performed before this handover.
