# Domain Terms: terminal-wow-factor

| Term | Working definition | Anti-terms / collision notes | Collision check |
|---|---|---|---|
| Intent | The terminal developer's natural-language request for a terminal task | Avoid generic `input` when referring to the domain request | - |
| Proposal | A generated shell command offered for review | Do not use `answer` when the artifact is intended for terminal action | - |
| Command flow | The visible syntactic structure of a proposal, including stages and operators | Not a semantic safety verdict; confirmed term | Re-run 2026-09-17: `rg -i -c 'command flow' --glob '!target' --glob '!.git' --glob '!givn/archive' .` -> 71 lines across 25 files; anti-term `pipeline map` -> 2 hits, both anti-term declarations (`docs/ideation-terminal-wow-factor.md:76`, `docs/arc42/12-glossary.md:105`); `flow map` -> 0 hits. No competing usage. |
| Review surface | The interactive terminal presentation where a proposal and its command flow are examined | Kitty overlay is a presentation form, not the domain boundary | - |
| Review decision | The explicit developer choice to accept, edit, reject, cancel, rephrase, or request a higher tier | Keep separate from command execution | - |
| Candidate | One generated or directly edited command that may be selected and finally accepted during the current review | Confirmed canonical term; avoid generic `result` or `proposal version` | Re-run 2026-09-17: `rg -i -c 'candidate' --glob '!target' --glob '!.git' --glob '!givn/archive' .` -> 1468 lines across 47 files (canonical usage); anti-term `proposal version` -> 2 hits, both anti-term declarations in this file (lines 10, 18); generic `result` not searched (too common to be a discriminating signal). No competing usage. |
| Shell buffer | The current editable command line owned by the shell's line editor | Only Ctrl-W has a replacement buffer | - |
| Presentation adapter | The terminal-specific mechanism that renders the review surface and returns its decision | Design term; not a user-facing domain event | - |

## Terms confirmed during Event Storming

- `command flow` is the confirmed domain term for the visual syntactic
  structure of a candidate.
- `candidate` replaces `proposal version` as the canonical term for each
  selectable generated or edited command.
