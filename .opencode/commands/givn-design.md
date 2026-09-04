---
description: (Re-)write the technical design for a givn change — HOW layer, technology decisions, architecture impact
---

Write (or re-write) the technical design for a givn change.

For starting a new change, use `/givn-propose` instead — it writes the
proposal, spec, design, and tasks in one pass.
Use this command to redo or extend the design for an existing change.

**Input**: Optionally specify a change ID after `/givn-design`. If omitted, infer
from conversation context, or auto-select if only one active change exists. If
ambiguous, list changes and ask.

---

## Steps

### 1. Resolve the change

```sh
givn status
```

Identify the active change. Announce: "Using change: <id>".

### 2. Get design instructions

```sh
givn instructions design --change <id>
```

When `--json` is supplied, parse the documented object:
- `id`: effective artifact id
- `generates`: output paths or globs for the artifact
- `requires`: prerequisite artifact ids
- `instruction`: resolved writing guidance; do not copy it into the file

Without `--json`, the command emits the resolved instruction text only.

### 3. Read the inputs

Read:
- `givn/changes/<id>/proposal.md` — WHAT and WHY
- `givn/changes/<id>/specs/<group>/<capability>.feature` — observable behaviours
- Relevant existing code (architecture, patterns, integration points)

### 4. Write the design

Write `givn/changes/<id>/design.md` covering:

- **Technology decisions** — language, framework, libraries, patterns chosen and why.
  **Version freshness (mandatory):** never write a version number from
  memory — training data has an unknown cutoff and a remembered "current"
  version can now be outdated or end-of-life. For every versioned choice,
  either state "latest" or the ecosystem's LTS explicitly as the resolution
  strategy (e.g. "Node.js: latest LTS"), or look it up now (web search, the
  package registry, or an existing lockfile) and record the version plus
  how/when it was checked (e.g. "Playwright 1.49.x — checked
  npmjs.com/package/playwright"). A bare specific version number with
  neither a lookup note nor a "latest/LTS" designation is an unverified
  guess — do not write one.
- **Architecture impact** — which modules/components are affected; what is new.
- **Data model changes** — schema, structs, database changes if any.
- **Step definition locations** — where Cucumber/Gherkin step defs will live.
- **Runner command** — choose the test runner that fits the tech stack
  (e.g. `cargo test`, `npm test`, `pytest`). Document it in design.md.
  If `verify.command` in `givn/config.yaml` is still `"givn missing-testrunner"`,
  update it now to the chosen runner. The tasks artifact will include a
  setup task to install and configure the runner.
- **Local runnability & digital twins (mandatory)** — the system must be
  runnable and testable locally with one command:
  - Local run command that starts the entire system + every dependency
    (e.g. `docker-compose up`), in an isolated network.
  - A digital twin (fake/stub/emulator, running in the same isolated
    network) for every external/third-party dependency — email provider,
    payment processor, third-party API, cloud service. No scenario may
    depend on a live third-party service. If there are none, say so explicitly.
  - Any foreseeable obstacle to driving the real interface (browser
    session/cookie persistence, auth redirects, websocket handshakes) named
    with its concrete fix. "The browser had a cookie problem so we checked
    the database instead" is not an acceptable outcome — that obstacle
    belongs here, fixed, before tasks.md is written.
- **E2E smoke test infrastructure** — read the canonical policy with
  `givn instructions specs --change <id>`, then document the exact runner,
  step location, infrastructure, interface type, driver, and coverage matrix
  needed for this design. Do not restate the policy here.
- **Black-Box-First** — for every internal test retained, answer "which case
  this test covers that the E2E does not". Without a concrete answer, use the
  real-interface scenario instead.
- **Non-obvious choices** — justify them; obvious choices need no justification.

This is the HOW layer. Everything that was kept out of the proposal and spec
belongs here: class names, routes, DB schemas, function signatures,
framework details, step-definition mechanics.

### 5. Show next step

```sh
givn status --change <id> --json
```

Read `next_required.id` from the output and report it as the next step
(e.g. `/givn-tasks`, or whatever the manifest defines after `design`).

---

## Output

```
## Design written: <id>

**Design written:** givn/changes/<id>/design.md

### Key decisions
- <decision 1>
- <decision 2>

**Next:** Run `/<next-artifact-id>` to continue (from `givn status --change <id> --json` → `next_required.id`).
```

---

## Guardrails

- Keep implementation detail OUT of proposal.md and the .feature files — it
  belongs here in design.md.
- Read the proposal and spec before writing.
- Justify non-obvious technical choices; skip self-evident ones.
- Local runnability + digital twins are mandatory, not optional — a design
  with no answer for "how does someone run and test this locally, fully
  isolated" is incomplete. Do not accept a design that defers this to tasks.md.
- Every external/third-party dependency needs a named digital twin. No
  design may leave a scenario dependent on a live third-party service.
- Never write a version number from memory. Use "latest"/"LTS", or actually
  look one up (search, registry, existing lockfile) and note how it was
  checked. Reject a design with bare version numbers and no lookup note.
- Verify the file exists after writing.
