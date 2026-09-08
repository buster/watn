# Use-Case Migration Ledger

The 0.5.0-to-0.6.0 preflight found the target corpus already applied by the
archived `migrate-usecases-watn` change. This ledger records verification rather
than repeating file moves. Active feature text, step bindings, product source,
and historical archive files remain unchanged.

| Old entry | Old owner | Operation | New owner | New capability | Old @e2e | New @e2e | Behavior changed | Reason |
|---|---|---|---|---|---:|---:|---|---|
| active specification corpus | archived `migrate-usecases-watn` migration | VERIFY_ALREADY_MIGRATED | four use-case roots and `fragments` | all current capabilities | preserved | preserved | no | target use-case/fragment layout, typed ownership, interaction mappings, and prior evidence are present; no legacy active owner remains |

## Verification record

- Active roots: `configure-interactive`, `configure-model`,
  `configure-provider`, `use-shell`, and `fragments`.
- Legacy active roots: none; no tracked `group.md` file or active `group.md`
  reference exists.
- Ideation topics: none.
- Confirmed Personas: none.
- Handoff or Persona promotion: not applicable and not performed.
- Historical archive: unchanged and excluded from active-path cleanup.
- Inventory/lint: `givn lint` passed with 26 active feature files checked.
