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
| ADR-0006 | LiteLLM-powered model discovery | [docs/adr/0006-litellm-model-discovery.md](../adr/0006-litellm-model-discovery.md) |
| ADR-0007 | Reasoning support via reasoning_effort parameter | [docs/adr/0007-reasoning-support.md](../adr/0007-reasoning-support.md) |
| ADR-0008 | Template config generated from code | [docs/adr/0008-template-generated-from-code.md](../adr/0008-template-generated-from-code.md) |
| ADR-0009 | Server-side filtering for paginated model catalogs | [docs/adr/0009-server-side-filtering-model-catalogs.md](../adr/0009-server-side-filtering-model-catalogs.md) |
| ADR-0010 | Keyboard-driven dialog for model and reasoning selection | [docs/adr/0010-ratatui-model-picker.md](../adr/0010-ratatui-model-picker.md) |
| ADR-0011 | Interactive provider onboarding with environment-backed credentials | [docs/adr/0011-interactive-provider-onboarding.md](../adr/0011-interactive-provider-onboarding.md) |
| ADR-0012 | Structured widget composition for terminal setup views | [docs/adr/0012-structured-widget-composition-for-terminal-setup-views.md](../adr/0012-structured-widget-composition-for-terminal-setup-views.md) |
| ADR-0013 | Shared five-page setup wizard | [docs/adr/0013-shared-five-page-setup-wizard.md](../adr/0013-shared-five-page-setup-wizard.md) |

## ADR-0011 summary

ADR-0011 chooses a TTY-gated ratatui/crossterm provider setup state machine
with typed provider/model results and environment-backed credential references.
The considered dialoguer alternative is intentionally recorded: dialoguer is
adequate for linear prompts, but does not provide the same renderer boundary,
inline validation, masking, terminal restoration, and review-state contract.

The onboarding names are fixed: normalized OpenRouter uses `openrouter`, and
every other endpoint uses `custom`. A rerun replaces the selected fixed entry,
so an existing manually maintained `openrouter` or `custom` entry is an
intentional collision. The default-provider field and that one entry change;
unrelated providers, tiers, pricing, LiteLLM settings, and other config remain
unchanged. This fixed-name consequence is part of the decision, not an
implementation detail.

The same ADR records the TTY/non-TTY boundary, explicit-provider error
preservation, saved credential precedence, direct-write `0600` enforcement
without an atomic rename promise, typed cancellation, no-resume automatic
completion, and the ephemeral E2E HTTP construction override for both
`/models` and `/chat/completions`.

## ADR-0012 summary

ADR-0012 chooses native Ratatui widget composition for the existing provider and
model setup flows. Borders establish the screen boundary, lists and tabs expose
selection context, tables align provider/model metadata, paragraphs carry
guidance and status, and a scrollbar makes long model catalogs navigable. The
decision preserves the existing event loop and state transitions while removing
direct raw cursor output from these renderers.
