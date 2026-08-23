---
name: givn-propose
description: Write the proposal for a givn change — describe WHAT and WHY in domain language, without implementation detail.
---

# givn-propose

Write the proposal for change `<change-id>`.

## Context

- Proposal file: `givn/changes/<change-id>/proposal.md`
- Instructions: run `givn instructions proposal --change <change-id>`
- Spec will follow: `givn/changes/<change-id>/specs/`

## Rules

- Write from the user's perspective. Use domain language (not technical terms).
- Describe observable behaviour only. No class names, functions, routes, schemas,
  framework details, or step-definition mechanics.
- The only exception: observable requirements that are themselves externally
  measurable (e.g. "response is valid JSON", "persists across restart").
- If implementation details surface, move them to design.md.

## Structure

Fill in the proposal template sections:
1. **Problem / Opportunity** — what is wrong or missing?
2. **Proposed Solution** — what should the system do?
3. **Out of Scope** — what is not changing?
4. **Open Questions** — unresolved decisions before writing specs.

## Verify command

```
./run-tests.sh
```
