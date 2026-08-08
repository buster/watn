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
