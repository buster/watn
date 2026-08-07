# auto-init-config — Design

## Technology decisions

- **Runtime**: Rust (latest stable via `rust-toolchain.toml` or `cargo`).
- **Config format**: TOML via `toml` crate (already a dependency, v0.8).
- **Serialization**: `serde` with `Serialize`/`Deserialize` derives (already
  a dependency).

## Architecture impact

**Affected module**: `src/config/` only.

- `src/config/types.rs`: add `Config::template_content()` which constructs a
  `Config` with example values, serializes it to TOML, and comments every
  line.
- `src/config/mod.rs`: replace the hardcoded `TEMPLATE_CONFIG` constant with
  a call to `Config::template_content()` in `write_template_config()`. The
  function is called from `load_config()` when the config file does not exist.

No new modules. No public API changes.

## Step definitions

No new Gherkin steps are needed. The two new scenarios reuse existing steps:

- "First run writes a template config file" reuses: `no config file exists`,
  `I run `watn "hello"``.
- "Existing config file is not overwritten" reuses: existing config steps.

One new step is needed for the "config file contains a commented-out defaults
section" assertion. It will live in `tests/steps/ask_steps.rs` (the existing
monolithic step file for this project).

## Strict-mode config

Already configured: `cucumber-rs` with `.fail_on_skipped()` on the
`Cucumber` builder (see `tests/features_runner.rs:50`).

Not-implemented step stub: `unimplemented!()`.

## Local runnability

No new dependencies. Tests run via `cargo test`. No external services needed.

## Design decisions

The template is generated from code rather than a hardcoded string so that
adding a new config field automatically adds it to the template (no manual
sync step).
