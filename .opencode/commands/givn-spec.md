---
description: (Re-)write the Gherkin .feature spec for a givn change
---

Write (or re-write) the Gherkin spec for a givn change.

For starting a new change, use `/givn-propose` instead — it writes the
proposal, spec, design, and tasks in one pass.
Use this command to redo or extend the spec for an existing change.

**Input**: Optionally specify a change ID after `/givn-spec`. If omitted, infer
from conversation context, or auto-select if only one active change exists. If
ambiguous, list changes and ask.

---

## Steps

### 1. Resolve the change

```sh
givn status
```

Identify the active change. Announce: "Using change: <id>".

### 2. Get spec instructions

```sh
givn instructions specs --change <id>
```

When `--json` is supplied, parse the documented object:
- `id`: effective artifact id
- `generates`: output paths or globs for the artifact
- `requires`: prerequisite artifact ids
- `instruction`: resolved writing guidance; do not copy it into the file

Without `--json`, the command emits the resolved instruction text only.

### 3. Read the proposal's Capability Routing table

Read `givn/changes/<id>/proposal.md`'s `## Capability Routing` table. Each
row's **Decision** column, not the proposal's free-text description,
determines what to write:
- `EXTEND <cap>`: add scenarios into the existing capability's delta (a
  `@givn.modified` or `@givn.added` delta targeting that capability, in the
  group where the capability already lives).
- `NEW in <group>`: create a new capability file inside that group:
  `givn/changes/<id>/specs/<group>/<capability>.feature`.

If the table is missing rows for a capability you're about to write specs
for, run `givn spec route` now and fill it in before proceeding — do not
invent a capability name from the proposal's prose without going through
routing first.

Also check permanent specs at `givn/specs/<group>/` — existing scenarios for a
capability are the baseline; delta scenarios extend or modify them. Delta
files mirror the grouped layout: `givn/changes/<id>/specs/<group>/<cap>.feature`.

### 4. Write delta .feature files

For each capability, create:
`givn/changes/<id>/specs/<group>/<capability>.feature`

**Required tags:**
- `@givn.delta` on the Feature line (marks this as a delta document)
- `@<capability>` on the Feature line (capability identifier, kebab-case)
- One of `@givn.added` / `@givn.modified` / `@givn.removed` on each Scenario
  (default is `@givn.added` if omitted)
- `@wip` on every Scenario (steps not yet implemented)

Tags go on the line **before** `Scenario:`, never inside the scenario body.

<example caption="Correct tag placement">
@givn.delta @my-capability

Feature: My Capability

  @givn.added @wip
  Scenario: Observable outcome when something happens
    Given some context
    When an action occurs
    Then an observable result is visible
</example>

**Delta tag meanings:**
| Tag | Meaning |
|---|---|
| `@givn.added` | Append to permanent spec on archive |
| `@givn.modified` | Replace scenario by title on archive |
| `@givn.removed` | Delete scenario by title on archive |

**Rules:**
- Scenarios assert observable behaviour in domain language (Given/When/Then).
- No class names, function names, routes, DB schemas, or step mechanics.
- One scenario = one observable behaviour.
- `@givn.removed` scenarios: include exactly one placeholder step.
- All `@givn.*` tags are stripped automatically on archive.

**Canonical E2E policy:** before writing scenarios, run
`givn instructions specs --change <id>`. That resolved instruction defines
the inventory, real-interface, and one-E2E-per-distinct-action rules. This
command only choreographs the spec-writing steps and must not restate them.

Retrieval-aware authoring is feature-dependent. The normal Cargo installation
includes every retrieval feature; use `givn spec index` to refresh permanent
scenarios and `givn spec search` or `givn spec explore` for advisory authoring
evidence. The blocking path is `givn check review --change <id>`. In a
feature-free build, preserve the explicit `retrieval-unavailable` failure
rather than passing silently.

E5, BGE, and NLI use the same complete deterministic Gherkin serialization.
Actual model tokenizers check complete inputs before model construction or
inference, without truncation or padding; exactly 512 tokens fits and 513 is
over. `BGE_TOKEN_CAP` and `NLI_TOKEN_CAP` are per-candidate, visible,
unresolved, non-filtered evidence. Run-level `BGE_UNAVAILABLE` or
`NLI_UNAVAILABLE` is used only when that layer can score no pair. BGE and NLI
score the same E5 pool independently, and combined recommendations intersect
their recommendations. Tell the author to shorten or split the scenarios and
rerun review; do not chunk or edit scenarios automatically.

### 5. Validate syntax

```sh
givn lint
```

- Exit 0 = clean syntax, `@wip` scenarios present (expected at this stage)
- Exit 1 = parse error (fix before proceeding)
- Exit 2 = `@wip` or PENDING (expected; does NOT indicate test failure)

### 6. Show next step

```sh
givn status --change <id> --json
```

Read `next_required.id` from the output and report it as the next step
(e.g. `/givn-design`, or whatever the manifest defines after `specs`).

---

## Output

```
## Spec written: <id>

**Files created:**
- givn/changes/<id>/specs/<group>/<capability>.feature  (N scenarios)

**Lint:** clean (N @wip scenarios — expected)

**Next:** Run `/<next-artifact-id>` to continue (from `givn status --change <id> --json` → `next_required.id`).
```

---

## Guardrails

- Keep all scenarios `@wip` — step definitions are not written yet.
- Never add implementation detail to the spec (belongs in design.md).
- Run `givn lint` and fix parse errors before finishing.
- One `.feature` file per capability, under the capability's group
  directory (`givn/changes/<id>/specs/<group>/<cap>.feature`). Do not put
  all scenarios in one file.
- `givn lint` exit 2 (@wip present) is expected and correct at this stage.
