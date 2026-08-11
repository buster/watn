# Design Review: shell-completions

## Grilling Results

### Scope

Proposal, specification, and design agree on a closed selector aligned to all
five native `clap_complete 4.6.9` generators: Bash, Elvish, Fish, PowerShell,
and Zsh. They also agree on authoritative command-tree generation,
deterministic stdout-only scripts, no configuration/provider/filesystem side
effects, explicit unsupported-shell guidance, and the intentional reservation
of `completions` as a command name.

### Technology And Testability

The existing Clap tree remains the sole metadata source. A local
`CompletionShell` parser maps the complete native library set without exposing
the library enum as the public argument type. Regular scenarios cover all five
generators, short and long root options, subcommands, renderer-emitted selector
suggestions, help, output purity, determinism, shell parser checks, the
reserved-token escape hatch, and the no-config/provider sentinel. The free-form
`question` positional is preserved through `Cli::command()` metadata; renderers
are not required to emit a literal placeholder for it. The E2E scenario uses a
real built subprocess and is uniquely bound.

### Arc42

Arc42 is enabled. The change-local marker contains all twelve rows and
`STATUS: DONE`. Chapters 1-6 and 8-12 are affected; chapter 7 is explicitly
unaffected because no deployment topology changes. Durable chapters, the
README index, and ADR-0017 describe the new boundary and use Mermaid-only
diagrams.

### Resolved Risks

- Unsupported values use a literal `unsupported shell '<value>'; choose bash,
  elvish, fish, powershell, or zsh` parser contract.
- The no-config scenario snapshots the isolated XDG directory and a provider
  request sentinel before and after execution.
- Successful stderr emptiness, stdout purity, deterministic repeated output,
  and shell-native parser/source checks are explicit requirements.
- The `completions` token reservation is documented and the `--` escape hatch is
  exercised by a regular subprocess scenario.
- Missing local shell executables emit an explicit environment-limitation
  message; generation remains tested, but syntax acceptance is not claimed for
  an unavailable parser.
- The interaction matrix has one CLI happy-path E2E row and explicitly records
  Elvish, Fish, PowerShell, and Zsh as regular subprocess variants rather than
  multiplying E2E tags for enum values.
- Dependency compatibility is locked through Cargo.lock and any Clap or
  completion-generator upgrade requires rerunning the output and shell checks.

## Review Outcome

All required design-review branches were resolved by repository inspection or
artifact hardening. No user decision remains before implementation resumes.

DESIGN-REVIEW: PASS
