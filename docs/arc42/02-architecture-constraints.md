# 2. Architecture Constraints

<!-- Constraints that must be observed by the architecture and that therefore limit
     design freedom. Distinguish between technical, organisational, and regulatory
     constraints. -->

## Technical constraints

| Constraint | Motivation |
|---|---|
| Rust (latest stable) | Language constraint from project choice |
| OpenAI-compatible chat-completions API shape | Must work with any provider that exposes the `/v1/chat/completions` endpoint and its SSE response framing |
| Complete SSE termination | A successful streaming response must provide a `[DONE]` data event; EOF without it is a truncated network failure even when content was received |
| Single blocking stream consumer | The current CLI consumes provider events through a synchronous content callback; no async runtime, worker channel, or background output path is introduced |
| XDG Base Directory Specification | Watn uses `$XDG_CONFIG_HOME/watn/config.toml` for configuration, defaulting to `~/.config/watn/config.toml`; it does not use an XDG data directory |
| TOML for config files | Rust ecosystem standard; serde + toml crate |
| Provider model endpoint with optional `?search=` query support | Server-side model filtering for catalogs larger than one page; providers that do not support search report a clear error rather than silently filtering only the local page |
| Ratatui/crossterm terminal interaction | Provider onboarding and model selection must work as keyboard-driven terminal flows in the existing single binary; automatic onboarding is TTY-only |
| Credential references must remain environment-resolved | A configured `${VARIABLE}` reference may be persisted, but its resolved value must not be emitted in status output or replace the reference in config |
| Config snapshots use atomic replacement and Unix mode `0600` | A confirmed provider/model snapshot is serialized to a same-directory temporary file, flushed, permissioned, and renamed; a failed write leaves the previous file in place |
| Test transport is debug-only | The endpoint override branch is compiled only under `cfg(all(feature = "test-support", debug_assertions))`; every release-profile build, including one with `test-support`, uses the configured endpoint branch |
| Release evidence is target-specific | The release artifact is inspected with `file` and `ldd` on Linux or `otool -L` on macOS; a static artifact and universal shared-library set are not assumed |
| Completion selector is closed | `watn completions <SHELL>` accepts only the lowercase values `bash`, `elvish`, `fish`, `powershell`, and `zsh`; the CLI does not expose the broader `clap_complete::Shell` value type |
| Completion metadata is authoritative | Generated scripts derive from the same Clap command definition used for parsing and help; separately maintained command lists are not permitted |
| Completion generation is side-effect free | Successful generation writes only the selected script to stdout, writes nothing to stderr, does not load or create config, contacts no provider, and changes no shell configuration |
| Shell startup targets are user-owned | Bash and Zsh use `$HOME/.bashrc` and `$HOME/.zshrc`; Fish uses `$XDG_CONFIG_HOME/fish/config.fish` or `$HOME/.config/fish/config.fish`; only an explicitly selected target may be created or changed |
| Shell widget invocation is non-evaluating | Generated widgets use native line-editor APIs, invoke `command watn -- "$question"` through `PATH`, preserve stderr diagnostics, and never evaluate captured stdout |
| Shortcut writes are atomic and marker-owned | A target must have zero markers or exactly one ordered marker pair; valid replacements use a same-directory temporary file and rename, while malformed targets are unchanged |
| Permanent scenario titles are repository-wide unique | A behavior has one canonical owner in the active Gherkin tree; overlap findings are reviewed before archive rather than silently accumulated |

## Organisational constraints

| Constraint | Motivation |
|---|---|
| Single binary distribution | `cargo build --release` produces one executable for the selected target; its target-dependent runtime libraries must be available at deployment time |

## Conventions

| Convention | Motivation |
|---|---|
| CLI flags use `--kebab-case` | clap convention, POSIX-idiomatic |
| Environment variables use `WATN_` prefix | Namespace to avoid collisions |
| Exit codes: 0=ok, 1=user, 2=API, 3=network, 130=SIGINT | Distinguish error categories for script consumption |
| Provider readiness is local | First-run detection checks config and environment state without probing a live provider or consulting the E2E transport override |
| Explicit provider selection preserves errors | `--provider` and `WATN_PROVIDER` never trigger onboarding; unknown-provider and missing-key errors remain observable |
| Test binaries use explicit paths | The debug verification bootstrap builds the two required feature variants sequentially through Cargo's shared default target cache, copies each executable to a unique temporary path, and passes only those absolute paths to the harness; stale `target/debug/watn` discovery is not permitted |
| Catalog and chat endpoints are provider-local concerns | Model discovery uses the selected provider's saved or derived catalog endpoint and its credential; legacy `[litellm]` data is retained but is not contacted by setup or model discovery |
| Credential source is authoritative | A literal saved key or complete saved `${VARIABLE}` reference cannot be replaced by environment fallback; only an absent source may use fallback discovery |
| Reasoning values are open non-empty strings | `off` is the only omission sentinel; every other non-empty value is persisted and sent verbatim, while whitespace-only custom values are rejected |
| Reserved completion token is explicit | The unquoted first token `completions` dispatches to the completion subcommand; question text beginning with that token must be quoted or passed after `--` |
