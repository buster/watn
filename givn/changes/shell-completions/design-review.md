# Design Review: shell-completions

## Grilling Results

### Scope

Proposal, specification, and design agree on a closed Bash/Zsh/Fish selector,
authoritative command-tree generation, deterministic stdout-only scripts, no
configuration/provider/filesystem side effects, explicit unsupported-shell
guidance, and the intentional reservation of `completions` as a command name.

### Technology And Testability

The existing Clap tree remains the sole metadata source. A local
`CompletionShell` parser prevents the broader completion-library enum from
accepting unsupported values. Regular scenarios cover all root options,
positional arguments, subcommands, selector suggestions, help, output purity,
determinism, shell parser checks, and the no-config/provider sentinel. The E2E
scenario uses a real built subprocess and is uniquely bound.

### Arc42

Arc42 is enabled. The change-local marker contains all twelve rows and
`STATUS: DONE`. Chapters 1-6 and 8-12 are affected; chapter 7 is explicitly
unaffected because no deployment topology changes. Durable chapters, the
README index, and ADR-0017 describe the new boundary and use Mermaid-only
diagrams.

### Resolved Risks

- Unsupported values use a literal `unsupported shell '<value>'; choose bash,
  zsh, or fish` parser contract.
- The no-config scenario snapshots the isolated XDG directory and a provider
  request sentinel before and after execution.
- Successful stderr emptiness, stdout purity, deterministic repeated output,
  and shell-native parser/source checks are explicit requirements.
- The `completions` token reservation is documented rather than silently
  treated as backwards-compatible question text.
- Dependency compatibility is locked through Cargo.lock and any Clap or
  completion-generator upgrade requires rerunning the output and shell checks.

## Review Outcome

All required design-review branches were resolved by repository inspection or
artifact hardening. No user decision remains before tasks.

DESIGN-REVIEW: PASS
