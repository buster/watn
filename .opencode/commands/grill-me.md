---
description: Socratically grill the current plan or design — one hard question at a time — before implementation begins.
---

Grill the current plan or design before implementation.

This command is a **stress test**: it interrogates the proposal, specs, design,
and tasks for a givn change one question at a time, refusing to let weak
assumptions hide behind "looks reasonable."

## Steps

### 1. Read the change

```sh
givn status --change <id>
```

Read in order: `proposal.md`, the `.feature` specs, `design.md`, `tasks.md`.

### 2. Ask one question at a time

Pose a single sharp question, wait for the answer, then follow up. Do not dump
a checklist. Good grills:

- What is the single most likely way this breaks in production?
- Which step's assertion is fake — a no-op disguised as a check?
- What does this change assume that the proposal never actually states?
- If you deleted this scenario, which behaviour would regress silently?
- Where does the design duplicate an existing convention instead of reusing it?
- Which task's "Design constraints" block is missing or copy-pasted?

### 3. Push until concrete

Reject answers that stay at the architecture level. Push for the specific file,
function, branch, or edge value. If the answer is "it should be fine," ask what
"fine" means observably.

### 4. Record outcomes

After each resolved question, note it as a bullet: the assumption, the risk, and
the decision. Surface any that need a human call before implementation starts.

## Output

A short list of hardened decisions and the unresolved questions that block
implementation. Do not proceed to implement until the blocking questions are
answered.
