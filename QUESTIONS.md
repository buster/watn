# Questions

## givn archive merge strips docstrings from .feature files

The `givn archive --change` merge process replaces `@givn.modified` scenarios
by title but strips docstrings (`"""..."""` blocks) from Given/When steps.
This corrupts the permanent spec files, making scenarios non-functional
(parameterized steps lose their docstring arguments).

**Workaround**: Restore the permanent spec files from git (`git checkout HEAD --
givn/specs/...`) immediately after the archive succeeds. The merge only needs
the docstrings to be there for the verify step, but the verify is the gate
that blocks the archive — the docstrings must be present for scenarios to
pass. This is a catch-22: the verify gate requires docstrings, but the merge
strips them.

**Root cause**: The merge algorithm replaces the scenario block using a text
template that doesn't preserve docstrings. The parser extracts the scenario
header (Scenario: title) and step lines (Given/When/Then) but drops the
docstring content during reconstruction.

**Mitigation in test runner**: Added `GIVN_ARCHIVE_ONLY` env var to
`tests/features_runner.rs` to prevent loading change-spec files during
archive verify. This avoids duplicate-scenario failures when both the
merged permanent spec and the still-present change spec are loaded.

**Status**: Unresolved. The givn CLI should be fixed to preserve docstrings
during archive merge, or the merge should operate at the scenario-node level
(using the AST) rather than text-level replacement.
