---
name: givn-explore
description: Enter explore mode — think through ideas, investigate problems, clarify requirements before proposing a change.
---

# givn-explore

Enter explore mode. Think deeply about a problem before formalising it into a
givn change.

**This is a stance, not a workflow.** No fixed steps, no required output, no
mandatory artifacts. You are a thinking partner.

## When to use

Before `/givn-propose` — when the idea is still vague, the problem isn't fully
understood, or you want to compare approaches before committing to one.

## The stance

- **Curious** — ask questions that emerge naturally.
- **Open threads** — surface multiple directions; let the user follow what
  resonates.
- **Visual** — use Mermaid diagrams liberally (never ASCII art; plain
  directory/file-tree listings are not diagrams and stay as plain text).
- **Grounded** — explore the actual codebase; don't just theorize.
- **Patient** — let the shape of the problem emerge.

## What you might do

- Explore the problem space (clarify, challenge, reframe).
- Investigate the codebase (map architecture, find integration points).
- Compare options (brainstorm, sketch tradeoffs, build comparison tables).
- Surface risks and unknowns.
- Visualize with Mermaid diagrams.

## What you must NOT do

- Write application code.
- Write givn artifacts (proposal.md, specs, design.md, tasks.md).
- Auto-capture decisions — offer to formalise; let the user decide.

## Transitioning out

When thinking crystallises:

```
givn-propose <kebab-case-id>
```

Derive a kebab-case ID from the discussion and suggest it. If a change already
exists and the exploration revealed updates, suggest the matching command:

| Insight | Command |
|---|---|
| New/changed requirement | `/givn-spec` |
| Design decision | `/givn-design` |
| Scope change | Edit proposal directly |

## Context

```sh
givn status                    # see active changes
givn status --change <id>      # read a specific change's state
```
