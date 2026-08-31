# 9. Architecture Decisions

Architecture decisions are recorded as standalone Markdown Architecture Decision
Records (MADRs) under `docs/adr/`. Each ADR includes context, considered
options, the decision outcome, and consequences.

| ID | Title | File |
|---|---|---|
| ADR-0001 | Provider-agnostic via OpenAI-compatible API | [docs/adr/0001-provider-agnostic-via-openai-compatible-api.md](../adr/0001-provider-agnostic-via-openai-compatible-api.md) |
| ADR-0002 | Streaming-first response | [docs/adr/0002-streaming-first-response.md](../adr/0002-streaming-first-response.md) |
| ADR-0003 | Layered XDG configuration | [docs/adr/0003-layered-xdg-configuration.md](../adr/0003-layered-xdg-configuration.md) |
| ADR-0004 | Model tier dispatch | [docs/adr/0004-model-tier-dispatch.md](../adr/0004-model-tier-dispatch.md) |
| ADR-0005 | Execution mode with confirmation | [docs/adr/0005-execution-with-confirmation.md](../adr/0005-execution-with-confirmation.md) |
| ADR-0006 | LiteLLM-powered model discovery (superseded) | [docs/adr/0006-litellm-model-discovery.md](../adr/0006-litellm-model-discovery.md) |
| ADR-0007 | Reasoning support via reasoning_effort parameter | [docs/adr/0007-reasoning-support.md](../adr/0007-reasoning-support.md) |
| ADR-0008 | Template config generated from code | [docs/adr/0008-template-generated-from-code.md](../adr/0008-template-generated-from-code.md) |
| ADR-0009 | Hybrid filtering for complete and paginated model catalogs | [docs/adr/0009-server-side-filtering-model-catalogs.md](../adr/0009-server-side-filtering-model-catalogs.md) |
| ADR-0010 | SetupWizard model picker and reasoning selection | [docs/adr/0010-ratatui-model-picker.md](../adr/0010-ratatui-model-picker.md) |
| ADR-0011 | Interactive provider onboarding with environment-backed credentials (superseded for setup persistence) | [docs/adr/0011-interactive-provider-onboarding.md](../adr/0011-interactive-provider-onboarding.md) |
| ADR-0012 | Structured widget composition for terminal setup views | [docs/adr/0012-structured-widget-composition-for-terminal-setup-views.md](../adr/0012-structured-widget-composition-for-terminal-setup-views.md) |
| ADR-0013 | Shared five-page setup wizard (superseded) | [docs/adr/0013-shared-five-page-setup-wizard.md](../adr/0013-shared-five-page-setup-wizard.md) |
| ADR-0014 | Independent catalog source and provider confirmation boundary (superseded) | [docs/adr/0014-independent-catalog-source-and-provider-confirmation.md](../adr/0014-independent-catalog-source-and-provider-confirmation.md) |
| ADR-0015 | Synchronous stream callback and completion boundary | [docs/adr/0015-synchronous-stream-callback-and-completion-boundary.md](../adr/0015-synchronous-stream-callback-and-completion-boundary.md) |
| ADR-0016 | Release truth and target-dependent runtime requirements | [docs/adr/0016-release-truth-and-target-dependent-runtime-requirements.md](../adr/0016-release-truth-and-target-dependent-runtime-requirements.md) |
| ADR-0017 | Completion generation from authoritative command definition | [docs/adr/0017-completion-generation-from-authoritative-command-definition.md](../adr/0017-completion-generation-from-authoritative-command-definition.md) |
| ADR-0018 | Safe shell shortcut installation and native widgets | [docs/adr/0018-safe-shell-shortcut-installation-and-native-widgets.md](../adr/0018-safe-shell-shortcut-installation-and-native-widgets.md) |
| ADR-0019 | Interruptible completion via worker thread and bounded grace | [docs/adr/0019-interruptible-completion-via-worker-thread.md](../adr/0019-interruptible-completion-via-worker-thread.md) |
| ADR-0020 | Final-confirmation setup snapshots | [docs/adr/0020-final-confirmation-setup-snapshots.md](../adr/0020-final-confirmation-setup-snapshots.md) |
| ADR-0021 | Provider-local catalog discovery | [docs/adr/0021-provider-local-catalog-discovery.md](../adr/0021-provider-local-catalog-discovery.md) |
| ADR-0022 | Verbatim reasoning values | [docs/adr/0022-verbatim-reasoning-values.md](../adr/0022-verbatim-reasoning-values.md) |
| ADR-0023 | Canonical provider migration | [docs/adr/0023-canonical-provider-migration.md](../adr/0023-canonical-provider-migration.md) |
| ADR-0024 | Atomic configuration replacement | [docs/adr/0024-atomic-config-replacement.md](../adr/0024-atomic-config-replacement.md) |

## ADR-0011 summary (superseded where noted)

ADR-0011 chooses a TTY-gated ratatui/crossterm provider setup state machine
with typed provider/model results and environment-backed credential references.
The considered linear-prompt alternative is intentionally recorded: it is
adequate for simple input, but does not provide the same renderer boundary,
inline validation, masking, terminal restoration, and review-state contract.

The onboarding names remain fixed: normalized OpenRouter uses `openrouter`, and
every other endpoint uses `custom`. The current change supersedes the old
arbitrary-provider preservation rule: when the selected baseline provider has an
arbitrary key, it migrates to `custom` on successful confirmation and the old
selected key is removed. ADR-0023 defines collision/default-model behavior;
unrelated providers, tiers, pricing, legacy LiteLLM settings, and other config
remain unchanged.

The same ADR records the TTY/non-TTY boundary, explicit-provider error
preservation, saved credential precedence, typed cancellation, no-resume
automatic completion, and the ephemeral E2E HTTP construction override for both
`/models` and `/chat/completions`. The transport refinement makes that override
available only under `cfg(all(feature = "test-support", debug_assertions))`,
requires pure URL builders after resolution, and uses two copied debug paths
from Cargo's shared target cache. A release binary with the feature enabled
still uses the configured endpoint by source guard; current release verification
inspects the exact release artifact and its target runtime libraries.

## ADR-0012 summary

ADR-0012 chooses native Ratatui widget composition for the existing provider and
model setup flows. Borders establish the screen boundary, lists and tabs expose
selection context, tables align provider/model metadata, paragraphs carry
 guidance and status, and a scrollbar makes long model catalogs navigable. The
decision preserves the existing event loop and state transitions while removing
direct raw cursor output from these renderers.

## ADR-0014 summary (superseded)

ADR-0014 is retained as historical context. Its independent LiteLLM source and
provider-first persistence boundary are superseded by ADR-0020 and ADR-0021.

## ADR-0020 to ADR-0024 summaries

ADR-0020 makes coordinated setup a final-confirmation snapshot and keeps focused
commands domain-scoped. ADR-0021 makes the selected provider the only catalog
source while retaining legacy LiteLLM data as unrelated configuration.
ADR-0022 accepts arbitrary non-empty reasoning values verbatim and reserves
`off` for omission. ADR-0023 migrates a selected arbitrary provider key to
canonical `custom` with deterministic collision/default-model rules. ADR-0024
uses same-directory atomic replacement for one configuration file and keeps
shell target operations independent.

## ADR-0015 summary

ADR-0015 refines the existing streaming-first decision for the one-consumer CLI.
The provider uses a buffered blocking reader and a synchronous content callback,
with no worker channel or incremental reasoning output. Command chunks are
flushed once, reasoning is buffered until successful `[DONE]` completion and is
printed only under `-v`, and the final aggregate is not printed again. `[DONE]`
is mandatory; truncation and read failures preserve visible content but skip
metadata and execution, while output write/flush failures use the existing I/O
status. Completion timing begins at the first non-DONE data event and does not
wait for a post-DONE connection close.

## ADR-0016 summary

ADR-0016 derives the CLI version from Cargo package metadata and makes release
deployment claims target-specific. Release evidence uses `file` and `ldd` on
Linux or `otool -L` on macOS to identify the dynamic runtime libraries of the
exact artifact. The decision deliberately does not add or promise a universal
static artifact.

## ADR-0017 summary

ADR-0017 chooses a local closed `CompletionShell` selector and renders from
`Cli::command()` rather than exposing `clap_complete::Shell` or maintaining a
second command tree. Bash, Elvish, Fish, PowerShell, and Zsh are the only accepted lowercase values.
Generation happens before configuration and provider setup, writes only
deterministic script bytes to stdout, and leaves stderr and the filesystem
unchanged. The parser error keeps the literal contract
`unsupported shell '<value>'; choose bash, elvish, fish, powershell, or zsh`. The `completions`
subcommand intentionally reserves that unquoted first token, so question text
beginning with it must be quoted or passed after `--`.

## ADR-0018 summary

ADR-0018 chooses an opt-in, post-setup shell shortcut that writes exact
marker-owned Bash, Zsh, and Fish widgets using atomic same-directory replacement.
The widgets invoke `command watn -- "$question"` through `PATH`, capture only
stdout, preserve stderr, trim trailing CR/LF characters, strip one leading `# `
comment prefix, record the flattened request as a `#`-prefixed comment in the
shell history (Bash `history -s`, Zsh `print -s`, Fish `builtin history
append`), and replace the buffer with only the generated command, without
evaluation. The comment is recorded before execution, so the request stays
recallable from the shell history even when the command is never run. Pressing
Enter executes only the generated command as its own history entry. The installer
attempts selected targets independently and reports every success or failure;
it does not roll back a successful target when a later target fails. The
optional interaction is also available in implicit first-use setup, while the
default Enter path preserves the existing five-tab flow.

## ADR-0019 summary

ADR-0019 makes a single Ctrl+C stop an in-flight completion in every phase. The
reqwest blocking client cannot split connect and read timeouts, so the
streaming call runs on a worker thread and the SSE parser checks a shared
interrupt flag at every line; the main thread bounds the unreachable phases
(stalled stream, connection pending) with a 500 ms grace before detaching the
worker and exiting 130. `Interrupted` is a new error variant that suppresses the
error message but finishes the spinner and partial output. The tradeoff is a
fixed grace heuristic and possible truncation of the final buffered bytes in the
hard-exit case.

<!--
MADR template for future decisions:

# ADR-NNNN: Decision title

- **Status:** proposed | accepted | deprecated | superseded
- **Date:** YYYY-MM-DD
- **Decision-makers:** role or name

## Context and Problem Statement

What problem requires a decision?

## Decision Drivers

- Driver one
- Driver two

## Considered Options

- **Option one** - tradeoff
- **Option two** - tradeoff

## Decision Outcome

Chosen option and the governing details.

## Consequences

### Good

- Positive consequence

### Bad

- Negative consequence

## Confirmation

How the decision is verified.
-->
