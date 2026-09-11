# Arc42 Impact: setup-shell-and-style

| # | Chapter | Affected? | Reason / what changed |
|---:|---|---|---|
| 1 | 01 introduction-and-goals | No | Goals and stakeholders are unchanged; plain-terminal usability remains covered by the existing observability goals. |
| 2 | 02 architecture-constraints | No | No new technical, legal, or organisational constraint; no dependency added. |
| 3 | 03 context-and-scope | No | No new external system or command; the same setup entry points are used. |
| 4 | 04 solution strategy | No | The wizard strategy and shell-integration boundary are unchanged; only page interaction and presentation change. |
| 5 | 05 building-block-view | Yes | `Setup Wizard` now owns the shared visual language and PATH/managed-block shell preselection. |
| 6 | 06 runtime-view | Yes | The optional-shell-shortcut flow is list-first with preselection and an enabled rule that preserves the no-write decline. |
| 7 | 07 deployment-view | No | No deployment or packaging change. |
| 8 | 08 crosscutting-concepts | Yes | Added the setup visual language section; corrected shell preselection and focused-setup wording. |
| 9 | 09 architecture-decisions | Yes | ADR-0018 is amended: its opt-in question, Enter-as-decline default, and `$SHELL`-basename preselection are superseded; installer safety decisions stand. |
| 10 | 10 quality-requirements | No | No new measurable quality scenario; the existing focus and wizard scenarios still hold. |
| 11 | 11 risks-and-technical-debt | Yes | R-048 updated: preselection accepted with Enter must stay visible, bounded, and non-mutating on Escape/empty detection; ADR-0018 consequence coverage corrected. |
| 12 | 12 glossary | Yes | Added `Shell preselection` and `Setup visual language`. |

The arc42 README index summary was also updated to record the shared setup
visual language and shell preselection; it is not a numbered chapter.

## ADR Qualification

| Candidate | Verdict | Route |
|---|---|---|
| Shell preselection from PATH binaries plus managed blocks and the list-first enabled rule | QUALIFIED refinement of ADR-0018 | `AMEND_ADR` `docs/adr/0018-safe-shell-shortcut-installation-and-native-widgets.md` |
| Setup adopts the review visual language | NOT_QUALIFIED — presentation convention without a durable boundary | `design.md`; chapter 8 |
| Shared color-capability fallback | NOT_QUALIFIED — refinement of the existing review color rule; no ADR boundary | `design.md`; chapter 8 |

Existing ADR check: ADR-0012 (structured widget composition) and ADR-0026
(plain-line quick setup) reviewed; neither is amended.

## Status

STATUS: DONE
