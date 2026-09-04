---
description: Grill the plan or design against the project's own docs and specs — cite the source for every challenge.
---

Grill the current plan or design, grounding every challenge in the project's
existing documentation and specs.

Unlike `/grill-me`, this command **must cite a source** for each challenge: a
line in the proposal, a spec scenario, the design doc, an architecture chapter,
or an ADR. If no source backs a concern, drop it.

## Steps

### 1. Inventory the sources

Gather the authoritative material:

- `givn/changes/<id>/proposal.md`
- `givn/changes/<id>/specs/**/*.feature`
- `givn/changes/<id>/design.md`
- `givn/changes/<id>/tasks.md`
- `docs/arc42/` (if the arc42 addon is enabled)
- Any ADRs under `docs/adr/` or the project's decisions log
- `givn/config.yaml` (verify.command, features_path, addons, extras)

### 2. Cross-check, one question at a time

For each plan element, ask: which source supports it, and which source
contradicts it? Pose one question, cite the source, wait. Examples:

- "tasks.md says the flow is write-config -> set_extras -> sync, but design.md
  pitfall #4 warns about --force wiping the flag. Is the ordering covered?"
- "The spec asserts `extras: true`, but config.yaml.tmpl ships it commented.
  Which step uncomments it?"

### 3. Flag drift

Call out anywhere the plan disagrees with a cited source. Force a resolution:
update the plan, update the source, or record an explicit decision.

### 4. Record

List each challenge with its citation, the resolution, and any open question
that needs a human decision before implementation.

## Output

A sourced list of contradictions and drift findings. Every line must name where
it came from. Unresolved drift blocks implementation.
