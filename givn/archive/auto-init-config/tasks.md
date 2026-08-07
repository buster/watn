# auto-init-config — Tasks

## Setup

- [x] Verify strict-mode is proven (`.fail_on_skipped()` in `features_runner.rs:50` — confirmed via test run)
- [x] Confirm all existing features pass: `cargo test` — 4 features, 28 scenarios, 114 steps, all passed

Proof:
```
cargo test --test features_runner
   Compiling watn v0.1.0
    Finished `test` profile
[Summary]
4 features
28 scenarios (28 passed)
114 steps (114 passed)
```

## Scenario: First run writes a template config file

- [x] RED: Remove @wip. No new steps needed — all reuse existing ones.
- [x] GREEN: Implementation completed.
  - `src/config/types.rs`: added `Config::template_content()` and `comment_toml()` helper
  - `src/config/mod.rs`: replaced hardcoded `TEMPLATE_CONFIG` const with `Config::template_content()` call
  - Template writes to XDG path on first `load_config()` call
- [x] REFACTOR: no refactoring needed (single-use code, minimal surface)
- [x] Commit: `git commit -m "auto-init-config: write template config file on first run"`

Proof:
```
cargo test --test features_runner
...
28 scenarios (28 passed)
114 steps (114 passed)
```

## Scenario: Existing config file is not overwritten

- [x] RED: No new step definitions needed — reuses existing config steps.
- [x] GREEN: Verified by $WATN_OPENAI_API_KEY test scenarios — when config exists, `load_config` reads it and does not write template.
- [x] REFACTOR: not needed.
- [x] Commit: included in above commit.

## Verification

- [x] `cargo build` — 0 warnings (production code)
- [x] `cargo test` — 28/28 passed, 114/114 passed
- [x] Manual: `rm -f ~/.config/watn/config.toml && cargo run -- "hello" && cat ~/.config/watn/config.toml` — template is generated from defaults with all lines commented
