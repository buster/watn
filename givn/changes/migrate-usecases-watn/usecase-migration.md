# Watn Use-Case Migration Ledger

All operations preserve feature text and scenario tags. No behavior change is
intended.

| Old entry | Old owner | Operation | New owner | New capability | Old @e2e | New @e2e | Behavior changed | Reason |
|---|---|---|---|---|---:|---:|---|---|
| all corpus-infra capabilities | corpus-infra | CLASSIFY_AS_FRAGMENT | fragments/corpus-infra | unchanged capability IDs | preserved | preserved | no | reusable infrastructure |
| all setup capabilities | interactive-setup | MOVE | configure-interactive | unchanged capability IDs | preserved | preserved | no | guided setup goal |
| all model capabilities | model-setup | MOVE | configure-model | unchanged capability IDs | preserved | preserved | no | model selection goal |
| all provider capabilities | provider | MOVE | configure-provider | unchanged capability IDs | preserved | preserved | no | provider configuration goal |
| all shell capabilities | shell | MOVE | use-shell | unchanged capability IDs | preserved | preserved | no | shell integration goal |

## Confirmation

Watn had a clean worktree, no active changes, no ideation topics, and no
Personas. The mapping is applied as a corpus migration only.
