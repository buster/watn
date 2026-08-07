# Proposal: watn-cli

## Problem / Opportunity

Developers regularly ask LLMs for terminal commands — shell incantations,
one-liners, awk/sed pipelines, git commands. The output is a command to copy
and paste (or execute directly). Existing tools either vendor a single
provider, offer no model-tier selection (fast/small vs. powerful vs.
thinking), or lack shell integration like execution confirmation.

A developer who wants quick shell help should be able to ask, get a command,
and be back at their prompt in seconds — with full control over how much
thinking the LLM does, which model handles the request, and whether the
response is copied or executed.

## Proposed Solution

A CLI tool called `watn` oriented around shell-command generation:

- **Ask a question** by passing it as a positional argument or via stdin.
  `watn "find all files modified in the last day"` returns a copy-pasteable
  command.

- **Three model tiers** mapped to user-selected models:
  - small/fast (default, `-1`): cheapest, fastest model for simple tasks
  - normal (`-2`): balanced quality/speed for most tasks
  - thinking/reasoning (`-3`): deep model for complex multi-step reasoning

- **Model explorer (`watn models`):** queries an optional LiteLLM endpoint to
  discover available models, then lets the user interactively select which
  model fills each tier. Selection is persisted in config.

- **No model explorer?** The user configures models directly in config files
  or env vars.

- **Execution mode (`-x`):** prints the command, then prompts the user
  `Execute now? [Y/n]`. A plain Enter (no explicit 'y') confirms execution.
  Runs the command in the current shell.

- **Output metadata:** every response includes the model name, tokens per
  second, and cost (if available) alongside the command text.

- **Streaming output:** shows tokens as they generate.

- **Pipe-friendly:** when stdout is piped, output is raw text without ANSI
  codes. Suitable for `watn "compress this" | pbcopy`.

- **Any OpenAI-compatible API** as a provider — endpoint URL, API key, model
  name. No hardcoded provider list.

- **Configuration** via layered stack: built-in defaults, XDG config file,
  environment variables, CLI flags. Later sources override earlier ones.

## Out of Scope

- **Conversation sessions.** Context persistence, history, and session management are not included.
- **Non-OpenAI-compatible APIs.** Anthropic, Google Gemini, etc. require an
  adapter — future work.
- **Function calling / tool use.** Pure chat-completions only.
- **GUI / TUI.** Beyond readline-style input, no curses or graphical
  interface.
- **Model download or local inference.** Remote APIs only.
- **LLM evaluation or benchmarking.** Quality assessment is not a goal.
- **Customizable --version output.** The logo is fixed. No user-overridable
  banner.

## Branding

- **Full banner (Unicode):** 6-line box-drawing logo — used in `--version` when
  TERM is not `linux` and stdout is a TTY.
- **ASCII-safe fallback:** Used in `--version` when TERM=linux, stdout is piped,
  or the environment lacks Unicode support:

```
__      __ __ _ | |_  _ __   ___
\ \ /\ / // _` || __|| '_ \ |__ \
 \ V  V /| (_| || |_ | | | |  / /
  \_/\_/  \__,_| \__||_| |_| |_|
                              (_)
```

- **One-liner mark:** `watn? ¯\_(ツ)_/¯` — used as a spinner/thinking indicator
  while streaming; replaces the shrug with the command text when the first
  tokens arrive. Also in prompt/status-line contexts and in metadata output:
  `gpt-4o-mini · 142 tok/s · $0.0003 · 0.6s ¯\_(ツ)_/¯`
- **`--version` output:** Prints the Unicode banner (or ASCII fallback), then
  the version number on a new line.
- **Tagline:** "You know what you want. watn knows how to type it. Ask in plain
  language. Get one command. Run it with `-x`, or pipe it anywhere."

## Open Questions

1. **Cost estimation accuracy.** Tracking token counts from the API response
   is trivial. Real cost requires the user to supply per-model pricing
   (e.g. per-million-token rates) in config. Should cost display degrade
   gracefully when pricing is not configured (show token count only)?
