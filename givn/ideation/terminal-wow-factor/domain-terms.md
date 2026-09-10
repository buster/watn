# Domain Terms: terminal-wow-factor

| Term | Working definition | Anti-terms / collision notes |
|---|---|---|
| Intent | The terminal developer's natural-language request for a terminal task | Avoid generic `input` when referring to the domain request |
| Proposal | A generated shell command offered for review | Do not use `answer` when the artifact is intended for terminal action |
| Command flow | The visible syntactic structure of a proposal, including stages and operators | Not a semantic safety verdict; confirmed term |
| Review surface | The interactive terminal presentation where a proposal and its command flow are examined | Kitty overlay is a presentation form, not the domain boundary |
| Review decision | The explicit developer choice to accept, edit, reject, cancel, rephrase, or request a higher tier | Keep separate from command execution |
| Candidate | One generated or directly edited command that may be selected and finally accepted during the current review | Confirmed canonical term; avoid generic `result` or `proposal version` |
| Shell buffer | The current editable command line owned by the shell's line editor | Only Ctrl-W has a replacement buffer |
| Presentation adapter | The terminal-specific mechanism that renders the review surface and returns its decision | Design term; not a user-facing domain event |

## Terms confirmed during Event Storming

- `command flow` is the confirmed domain term for the visual syntactic
  structure of a candidate.
- `candidate` replaces `proposal version` as the canonical term for each
  selectable generated or edited command.
