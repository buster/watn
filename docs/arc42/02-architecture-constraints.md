# 2. Architecture Constraints

<!-- Constraints that must be observed by the architecture and that therefore limit
     design freedom. Distinguish between technical, organisational, and regulatory
     constraints. -->

## Technical constraints

| Constraint | Motivation |
|---|---|
| Rust (latest stable) | Language constraint from project choice |
| OpenAI-compatible chat-completions API shape | Must work with any provider that exposes the `/v1/chat/completions` endpoint |
| XDG Base Directory Specification | Config at `~/.config/watn/`, data at `~/.local/share/watn/` |
| TOML for config files | Rust ecosystem standard; serde + toml crate |
| Provider model endpoint with optional `?search=` query support | Server-side model filtering for catalogs larger than one page; providers that do not support search report a clear error rather than silently filtering only the local page |
| Ratatui/crossterm terminal interaction | Provider onboarding and model selection must work as keyboard-driven terminal flows in the existing single binary; automatic onboarding is TTY-only |
| Credential references must remain environment-resolved | A configured `${VARIABLE}` reference may be persisted, but its resolved value must not be emitted in status output or replace the reference in config |
| Direct config writes enforce Unix mode `0600` | Every template, provider, and model save uses the existing direct-write mechanism and repairs file permissions after writing; atomic rename is not promised |
| Test transport is debug-only | The endpoint override branch is compiled only under `cfg(all(feature = "test-support", debug_assertions))`; every release-profile build, including one with `test-support`, uses the configured endpoint branch |

## Organisational constraints

| Constraint | Motivation |
|---|---|
| Single binary distribution | No runtime dependencies; `cargo build --release` produces a standalone binary |

## Conventions

| Convention | Motivation |
|---|---|
| CLI flags use `--kebab-case` | clap convention, POSIX-idiomatic |
| Environment variables use `WATN_` prefix | Namespace to avoid collisions |
| Exit codes: 0=ok, 1=user, 2=API, 3=network, 130=SIGINT | Distinguish error categories for script consumption |
| Provider readiness is local | First-run detection checks config and environment state without probing a live provider or consulting the E2E transport override |
| Explicit provider selection preserves errors | `--provider` and `WATN_PROVIDER` never trigger onboarding; unknown-provider and missing-key errors remain observable |
| Test binaries use explicit paths | The debug verification bootstrap builds the two required feature variants sequentially through Cargo's shared default target cache, copies each executable to a unique temporary path, and passes only those absolute paths to the harness; stale `target/debug/watn` discovery is not permitted |
