# Design Review: Preserve Ctrl-W Requests In Shell Config

## Grilling Findings

### Scope

The proposal asks for a preserved request comment above the generated command,
commit-time isolation so only the generated command runs, comment flattening of
metacharacters and embedded newlines, unchanged failure/empty behavior, and no
change to `watn` itself. The spec covers each with five added scenarios and
three modified scenarios that keep the existing permanent contracts aligned.

### Technical Choices

Changing only the three generated widget blocks in `src/shell_shortcut.rs` is
the smallest correct implementation. Bash, Zsh, and Fish each replace their
native buffer with `# flattened-request` + newline + generated text and place
the cursor at the end. Flattening replaces CR/LF/TAB with spaces so the comment
is exactly one line. Reusing the existing fake-`watn` Bash harness avoids a
parallel unit-test path while exercising the real generated block.

### Missing Scenarios And Boundaries

- Success preserves the request as a comment (Bash).
- Commit-time execution runs only the generated command; a command embedded in
  the comment does not run.
- Metacharacters and embedded newlines flatten to one comment line.
- Failure/empty output preserves the original buffer (unchanged permanent
  behavior, explicitly re-asserted).
- Zsh and Fish generate comment-plus-command blocks and pass syntax checks.
- Modified permanent scenarios keep the existing E2E Bash path contract.

### Testability

Every then-step asserts a concrete editable-buffer string, cursor position,
file existence for commit-time side effects, or generated-block content. The
real Bash subprocess executes the produced buffer, so commit-time isolation is
proven rather than inferred. Zsh/Fish interactive redraw is not measurable in a
non-interactive runner and is recorded as legitimately hard to test.

### E2E Fidelity And Interaction Coverage

The capability is a generated shell widget. The E2E scenario executes the
installed Bash block in a real Bash subprocess with a fake `watn`, and asserts
the returned buffer plus the absence of evaluation. This is the real interface;
it is not a repository or HTTP assertion. The inventory has one interaction and
one matching `@e2e` scenario.

### Risk

The most likely failure is a comment line that is not recognized as a comment
or that executes unintended text. The mitigation is the real-Bash commit-time
execution scenario plus the no-evaluation sentinel, and flattening only CR/LF/TAB.

## Arc42 Independent Cross-Check

All twelve chapter rows were assessed independently and match `arc42.md`:

| # | Chapter | Expected impact | `arc42.md` | Match |
|---|---|---|---|---|
| 1 | Introduction and goals | Yes: request-preservation goal | Yes | Yes |
| 2 | Architecture constraints | No: existing constraints remain | No | Yes |
| 3 | Context and scope | Yes: line-editor boundary output changes | Yes | Yes |
| 4 | Solution strategy | Yes: widget strategy extends ADR-0018 | Yes | Yes |
| 5 | Building block view | Yes: Shell Shortcut / line-editor responsibilities change | Yes | Yes |
| 6 | Runtime view | Yes: Ctrl-W buffer flow changes | Yes | Yes |
| 7 | Deployment view | No: no artifact or topology change | No | Yes |
| 8 | Cross-cutting concepts | Yes: flattening and no-evaluation guarantees | Yes | Yes |
| 9 | Architecture decisions | Yes: ADR-0018 updated | Yes | Yes |
| 10 | Quality requirements | Yes: QS-055 | Yes | Yes |
| 11 | Risks and technical debt | Yes: R-055 and ADR-0018 consequences | Yes | Yes |
| 12 | Glossary | Yes: request comment, request flattening | Yes | Yes |

ADR-0018 and the durable chapters now describe the comment-plus-command buffer.

## Hardening Applied

- Added a commit-time execution scenario proving only the generated command runs.
- Added comment-flattening coverage for metacharacters and embedded newlines.
- Added Zsh/Fish content and syntax assertions.
- Added `@givn.modified` entries for the three behaviourally-changed permanent
  scenarios.
- Updated ADR-0018 and Arc42 chapters 01, 03, 04, 05, 06, 08, 09, 10, 11, 12.
- Kept exactly one `@e2e` scenario for the single inventory entry.
- Ran `givn lint --change preserve-ctrl-w-requests-in-shell-config`; the only
  findings are the expected `@wip` scenario markers.

## Open Questions

None.

DESIGN-REVIEW: PASS
