# Changelog

All notable changes to watn are documented in this file.

The release sections are generated with [git-cliff](https://git-cliff.org/).
Versions are selected manually and use annotated `vX.Y.Z` Git tags.
## [Unreleased]

## [0.1.4] - 2026-08-11

### Bug Fixes

- **release:** Preserve annotated tags during validation

## [0.1.3] - 2026-08-11

### Bug Fixes

- Cancel execute prompt with Esc and Ctrl-C

- **provider-setup:** Harden baseline fixtures and coverage

- **provider-setup:** Remediate review findings

- **provider-setup:** Finalize review traceability

- **provider-setup:** Complete production result seam

- **unified-setup-wizard:** Remove obsolete model dialog path

- **unified-setup-wizard:** Invalidate searches when discarding setup

- **unified-setup-wizard:** Remove obsolete provider dialog path

- **unified-setup-wizard:** Align modified feature with permanent spec

- **transport:** Keep debug transport verification clippy-clean

- **output:** Remove extra response blank line

- **reasoning:** Validate unknown persisted request effort


### Documentation

- Rewrite README usage section, add examples, license badge


### Features

- Add pulsing request spinner

- **provider-setup:** Configure a custom endpoint with a pasted credential

- **provider-setup:** Configure a custom provider with the generic environment variable

- **provider-setup:** A recognized environment credential skips automatic provider setup

- **provider-setup:** A saved provider with a default model skips automatic provider setup

- **provider-setup:** Invalid endpoint remains in provider setup for correction

- **provider-setup:** Empty credential remains in provider setup for correction

- **provider-setup:** A missing saved environment reference fails authentication without a request

- **provider-setup:** A saved OpenRouter endpoint takes precedence over the built-in endpoint

- **provider-setup:** An explicitly named environment variable is persisted and expanded at use time

- **provider-setup:** Trailing slashes are normalized before persistence and requests

- **provider-setup:** Rerunning provider setup preserves unrelated configuration

- **provider-setup:** Escape cancellation preserves the existing provider configuration

- **provider-setup:** Ctrl-C cancellation preserves the existing provider configuration

- **provider-setup:** Model catalog failure after provider setup preserves the provider and sends no request

- **provider-setup:** The explicit provider command ends without model setup

- **provider-setup:** Non-TTY first use prints setup guidance instead of starting ratatui

- **provider-setup:** A literal saved credential is authoritative over environment fallback

- **provider-setup:** Explicit provider selection from the environment preserves missing-key errors

- **provider-setup:** Saving provider configuration secures a world-readable file

- **provider-setup-widget-layout:** Provider setup separates choices, details, and guidance

- **provider-setup-widget-layout:** Model picker makes tiers and long model lists easy to scan

- **unified-setup-wizard:** Provider setup separates choices, details, and guidance

- **unified-setup-wizard:** Setup wizard guides provider and model configuration page by page

- **unified-setup-wizard:** Models command opens the shared wizard on Small Model

- **unified-setup-wizard:** Escape asks whether to save or discard current setup

- **credentials:** A missing saved environment credential fails before discovery

- **credentials:** Provider-specific environment fallback precedes generic fallback

- **catalog:** LiteLLM discovery without a key sends no authorization header

- **reasoning:** A disabled model default selects off even when a default effort is present

- **catalog:** Provider discovery is used when LiteLLM is absent

- **reasoning:** Mandatory reasoning excludes off

- **catalog:** Catalog pagination and search use the configured catalog source

- **reasoning:** Unknown persisted reasoning sends no reasoning request

- **reasoning:** Non-TTY model assignment never persists empty reasoning values

- **reasoning:** Existing reasoning survives selection without a valid replacement

- **search:** The newest search result stays visible when an older result arrives later


### Other Changes

- Merge pull request #1 from buster/bright-fireant

Bright fireant


### Refactoring

- Remove obsolete code paths


## [0.1.2] - 2026-08-09

- Published to crates.io from the verified source commit `d5ddb36`.
- This already-published version is excluded from automated publishing.

## [0.1.1] - 2026-08-08

- Historical crates.io publication before the tagged release workflow existed.
- No Git tag was recorded for this publication.

## [0.1.0] - 2026-08-08

- Initial crates.io publication before the tagged release workflow existed.
- No Git tag was recorded for this publication.
