# Proposal: setup-shell-and-style

## Use-Case Context

- Use-case ID: configure-interactive (presentation), configure-model (shell behavior)
- Ideation topic: none
- Confirmed Personas: none

## Problem / Opportunity

Two problems on the interactive setup surface:

1. The shell integration pages gate the shell list behind a separate yes/no
   question. Only the user's current shell is preselected, not every shell
   installed on the machine. A user who wants to adjust the choices must first
   answer the question with a different key; pressing Enter skips the
   integration entirely instead of accepting the choices that are visible.
2. The setup surface does not share the review panel's visual language. It uses
   plain frame and tab labels, background-filled selection, bracket checkboxes,
   pipe-separated tabs, and undifferentiated guidance text. The review panel's
   symbols, emphasis, color roles, and shortcut hints are missing, so the two
   surfaces feel like different products.

## Proposed Solution

**Shell selection**

- The completion and Ctrl-W pages show the shell list immediately. There is no
  separate opt-in question.
- A shell is preselected when its binary is present on `PATH` or when it
  already contains a watn-managed block for that integration. All detected
  shells are selected at once.
- Up/Down moves, Space toggles the highlighted shell, Enter advances to the
  next page. The shown selection is the desired state.
- Enter applies the desired state: every selected shell is installed. In the
  focused shell flow, a deselected shell has only its watn-managed block
  removed. Coordinated setup keeps its existing install-only behavior
  (`watn shell` is the removal path).
- When no shell is detected and none has a managed block, advancing performs no
  shell target write or creation.
- Escape still discards without touching any target.

**Visual language**

- The setup surface adopts the review panel's markup:
  - `watn · setup` frame label; `◆` for the active page and section context;
  - `▶` for the highlighted list or table row; `●` selected and `○`
    unselected for shell choices;
  - `↳` for guidance tied to the current choice; `⚠` in amber for warnings
    and validation failures;
  - key hints as bold keys with dim labels joined by `·`
    (for example `↑↓ move · ␣ toggle · ⏎ next · esc back`);
  - labels and separators in cyan/dim, values in white, attention in amber,
    selection and the focused input accent in green, secondary text dimmed;
  - inactive panel borders keep a dim treatment with the default foreground,
    and the focused panel border stays green with a bold title.
- The same hierarchy and symbols render without color when the terminal does
  not support it, so the markup stays readable in a plain terminal.
- The green focused-input accent and its existing behavior are preserved.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| streamlined-setup | EXTEND quicksetup (4.00) | `EXTEND streamlined-setup` | Owns the shell completion and Ctrl-W setup pages and their existing scenarios. |
| unified-setup-wizard | EXTEND quicksetup (4.00) | `EXTEND unified-setup-wizard` | Owns the shared wizard page presentation that this change restyles. |
| interactive-shell-shortcut | EXTEND quicksetup (4.00) | `EXTEND interactive-shell-shortcut` | Removes the scenarios describing the opt-in question and `$SHELL`-only preselection; replacements live in `streamlined-setup`. |
| highlight-active-setup-input | EXTEND quicksetup (4.00) | `EXTEND highlight-active-setup-input` | Modifies the shortcut-focus scenario that drove the removed question-to-list transition. |

## Out of Scope

- The plain-line quick setup flow.
- Removing managed blocks through coordinated setup; `watn shell` remains the
  removal path.
- Installing or removing integrations for a shell whose binary is absent and
  whose entry is not managed by watn.
- Shell use after setup completes: the Ctrl-W widget, completions, and review
  panel behavior.
- Changing the review panel itself.
- Replacing the green focus accent.

## Open Questions

- None blocking.
