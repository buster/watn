---
description: Plan a givn change through design — proposal, spec, and design in the order the manifest defines; hands off to /givn-design-review next
---

Plan a new givn change from proposal through design.

**Input**: The argument after `/givn-propose` is the change ID (kebab-case), OR a
plain description of what to build. If nothing is provided, ask.

This command discovers the artifact sequence from `givn` at runtime — it does not
hardcode which steps exist or in what order. If the project has customised its
manifest (added a risk-assessment step, removed review, etc.), this command
automatically follows the configured sequence.

---

## Role: orchestrator

This command is an **orchestrator**. It creates the change scaffold, collects
context, and then delegates production of the planning artifacts through
`design` to a **thinking architect subagent**, in one pass. The orchestrator
does not write artifacts directly. `design-review` and everything after it
(`tasks`, `review`) are separate steps run in their own fresh contexts — see
"When to stop" below.

---

## Steps

### 1. Resolve the change ID

If the user gave a description rather than a kebab-case ID, derive one
(e.g. "add user auth" → `add-user-auth`). Confirm with the user only if
multiple reasonable kebab-case IDs could derive from the description.

### 2. Create the change

```sh
givn new <id>
```

### 3. Discover the artifact sequence

```sh
givn status --change <id> --json
```

Parse the JSON:
- `artifacts`: array in execution order, each with `id`, `status`, `optional`, `gates_archive`
- `next_required`: the first artifact to write
- `verify_command`: the configured test runner

### 4. Collect instructions for every planning artifact up to `design`

For each artifact in the sequence **up to and including `design`** (stop
before `design-review` if it appears in the sequence; otherwise stop before
`tasks`):

```sh
givn instructions <artifact-id> --change <id>
```

Capture the full output for each.

**Do not include `design-review` or anything after it in this pass.**
`design-review` must run as its own step, in a fresh context, after this
architect subagent's work is done — never continued from the same session
that wrote the design (see step 6 below). If the manifest has been
customised to remove `design-review` entirely, the sequence naturally has no
such artifact and this restriction does not apply — in that case, continue
through `tasks` as before.

### 5. Spawn a thinking architect subagent

Use the **Task tool** to spawn a thinking architect subagent (fall back to
describing the pattern in prose if the Task tool is not available to you).

The architect subagent is a **deep-thinking planning agent** — give it room to
reason carefully. It must not rush to produce output; it should think through
requirements, constraints, and decisions before writing each artifact.

Pass a self-contained prompt:

```
You are a thinking architect producing the complete plan for a givn change.

## Change ID
<id>

## What to build
<user's original description or request>

## Artifact sequence (from givn status --json)
<paste the artifacts array>

## Your task
Produce every planning artifact up to and including `design` — stop there.
Do NOT produce `design-review`, `tasks`, or `review`. `design-review` is a
mandatory gate that must run as a separate step, in a fresh context, never
continued from this session — a subagent that just wrote the design cannot
credibly stress-test its own plan. `tasks` and `review` depend on
`design-review` having run first.

For each artifact:
1. Think carefully about the requirements, constraints, and implications.
2. Read all previously written artifacts for this change as inputs.
3. Write the artifact to the path listed in `generates` under givn/changes/<id>/.
4. Follow the instruction for that artifact precisely.

## Instructions per artifact

<for each artifact, paste the full output of: givn instructions <id> --change <id>>

## Artifact-specific rules

**proposal** (proposal.md):
- WHAT and WHY in domain language — no implementation detail.
- Observable behaviour only. No class names, routes, DB schemas, step mechanics.
- Think about: who benefits, what changes from the user's perspective, what
  constraints or risks exist.
- **Capability Routing (mandatory, before specs)**: after drafting the
  proposal, run `givn spec route --change <id>` and fill in the proposal's
  `## Capability Routing` table — one row per capability the change touches,
  recording route's recommendation and the decision (`EXTEND <cap>` or
  `NEW in <group>`). Deviations from route are allowed but must carry a
  rationale in the table. The specs step reads this table, not the proposal's
  prose; `givn check review` compares the recorded decision against the delta
  that was actually authored.

**specs** (specs/**/*.feature):
- One .feature file per capability, grouped under the capability's use-case
  group: `givn/changes/<id>/specs/<group>/<capability>.feature` (mirrors the
  permanent layout `givn/specs/<group>/<capability>.feature`). The group for
  an existing capability is wherever it already lives; a new capability's
  group comes from the Capability Routing decision.
- Tags: `@givn.delta @<capability>` on Feature; `@givn.added` + `@wip` on
  each Scenario (use `@givn.modified` or `@givn.removed` where appropriate).
  Tags go on the line **before** `Scenario:`, never inside the scenario body.
  <example caption="Correct tag placement">
  @givn.added @wip
  Scenario: Observable outcome when something happens
    Given some context
    When an action occurs
    Then an observable result is visible
  </example>
- Scenarios assert observable behaviour in Given/When/Then domain language.
- Read `givn instructions specs --change <id>` for the canonical interaction
  inventory and E2E policy before proposing scenarios. The spec phase owns the
  exact inventory, tags, and real-interface scenarios; this command only
  orchestrates the proposal-to-spec transition.
- After writing, validate: run `givn lint --change <id>`
  Exit 0 or 2 = correct. Exit 1 = parse error — fix before continuing.
- Think about: what are the key observable behaviours? What are the edge cases?
  What existing behaviour might be affected? What does the real user interaction
  look like end-to-end?

**design** (design.md):
- HOW layer: technology decisions, architecture impact, step def locations
  (one file per capability, name the exact files), data model changes.
  Justify non-obvious choices.
- **Version freshness (mandatory)**: never write a version number for a
  language, runtime, framework, library, database, or container image from
  memory — training data has an unknown cutoff and a remembered "current"
  version may now be outdated or end-of-life. For every versioned choice,
  either (a) state "latest" or the ecosystem's LTS explicitly as the
  resolution strategy (tasks.md then pins whatever that resolves to at
  setup time), or (b) if you have a web search or similar tool available,
  use it now to check the current version and record it with a note of
  what was checked (e.g. "checked npmjs.com/package/playwright"). Do not
  write a bare specific version number with neither a lookup note nor a
  "latest/LTS" designation — that is an unverified guess, not a decision.
- **Test runner**: pick one fitting the tech stack, document the command. If
  `verify.command` is still `"givn missing-testrunner"`, set it now.
- **Strict mode (mandatory)**: most runners treat undefined/pending steps as
  neither pass nor hard failure by default; some (notably cucumber-jvm with
  an empty step body) report an outright PASS for a step that does nothing.
  Document explicitly: the strict-mode flag/config for the chosen runner
  (`cucumber-rs`: `.fail_on_skipped()`; `cucumber-js`/`cucumber-jvm` CLI:
  `--strict`; `cucumber-jvm` JUnit Platform Suite: no native flag — enforce
  via never-empty bodies plus a plugin/listener failing on UNDEFINED/
  PENDING; `behave`: steps must `raise`), the not-implemented stub for the
  language (Java `PendingException`, Python `NotImplementedError`, Rust
  `unimplemented!()`, JS `return "pending"` or `throw new Error(...)`), and
  the single-scenario run command.
- **Local runnability & digital twins (mandatory)**: a local run command
  that starts the entire system + every dependency in one step (e.g.
  `docker-compose up`), in an isolated network; a named digital twin
  (fake/stub/emulator, same isolated network) for every external/
  third-party dependency — no scenario may depend on a live third-party
  service; and any foreseeable real-interface obstacle (browser session/
  cookie persistence, auth redirects) named with its concrete fix.
  "The interface had a technical problem so we tested the database
  instead" is never acceptable — that obstacle must be designed away here.
- **E2E smoke test infrastructure**: read the canonical specs policy and
  document its technical consequences in design.md (runner, step location,
  infrastructure, driver, and matrix). Do not restate the policy here.
- Think about: simplest architecture that delivers the proposal, tradeoffs,
  affected existing code, whether the runner can actually be forced to fail
  on an unimplemented step or whether step-body discipline must carry that
  weight, and what could go wrong when actually driving the real interface.

**Any custom artifact between here and `design-review`/`tasks` (whichever
comes first in the sequence)**:
- Follow the instruction returned by `givn instructions <id>`.
- Write to the path in `generates`.
- Think before writing.

## Stop after `design`

Do not write `tasks.md`. Do not run `givn status --change`. Those happen after
`design-review` has run as a separate, fresh-context step — see the
orchestrator's next steps.

## Report back with
- A summary of each artifact written (path + one-sentence summary)
- Any significant decisions or tradeoffs made
- Any ambiguities that required a judgment call (flag these clearly)
- Lint result for specs
- Final givn status --change <id> output
```

### 6. Wait for the architect subagent to complete

The architect must produce all planning artifacts through `design` (and any
custom artifact preceding `design-review`/`tasks`) before the orchestrator
proceeds. It must NOT produce `design-review`, `tasks`, or `review`.

If the subagent flags ambiguities that require user input, surface them and
wait before spawning again (or re-prompting the subagent with the answers).

### 6b. Hand off to design-review

If `design-review` appears in the artifact sequence (the default), tell the
user:

```
Design written. Design-review is mandatory and must run in a fresh context —
run /givn-design-review <id> now before tasks are written.
```

Do not spawn `/givn-design-review`'s subagents yourself from this
orchestrator's context — the user (or their agent harness) invokes
`/givn-design-review` as a new, separate command so the grilling subagent
starts genuinely fresh, not as a continuation of this conversation. Stop
here.

If `design-review` has been removed from the manifest (project override),
tell the user to run `/givn-tasks <id>` next instead. Do not write
`tasks.md` from this orchestrator's own architect subagent — `/givn-tasks`
is its own command with its own fresh invocation, kept consistent regardless
of whether design-review ran.

### 7. Show final status

```sh
givn status --change <id>
```

---

## When to stop

Stop after the architect subagent has written `design` (and any custom
artifact preceding `design-review`/`tasks` in the sequence). Never let this
orchestrator's architect subagent produce `design-review`, `tasks`, or
`review` — each is a separate command run in its own fresh context:
`/givn-design-review`, then `/givn-tasks`, then (after implementation)
`/givn-review`.

---

## Output

```
## Change planned so far: <id>

**Artifacts written** (in manifest order):
- proposal:  givn/changes/<id>/proposal.md
  <one-sentence summary>
- specs:     givn/changes/<id>/specs/<group>/<cap>.feature  (N scenarios, all @wip)
- design:    givn/changes/<id>/design.md
  Key decisions: <brief summary>
[... any additional artifacts preceding design-review/tasks ...]

**Decisions / tradeoffs:** <any flagged by the architect>

**Next:** Run `/givn-design-review <id>` now, in a fresh context, before
tasks are written. (Or `/givn-tasks <id>` if design-review has been removed
from this project's manifest.)
```

---

## Guardrails

- The orchestrator delegates proposal/spec/design writing to the architect
  subagent — never `design-review`, `tasks`, or `review`.
- Always run `givn status --change <id> --json` for current state — never
  hardcode the sequence.
- No implementation detail (HOW) in proposal or specs.
- All scenarios `@wip` — step definitions are not written during planning.
- `givn lint --change <id>` after writing specs; fix parse errors (exit 1).
- design.md must name a concrete strict-mode mechanism and stub pattern —
  this is what prevents the change from being fabricated with empty stubs
  during implementation. Do not accept a plan that omits it.
- design.md must name a local run command, a digital twin for every
  external/third-party dependency, and a fix for any foreseeable
  real-interface obstacle. Do not accept a plan that defers this to
  "figure it out during implementation."
- design.md must not contain a version number written from memory. Every
  versioned technology choice is either "latest"/"LTS" or a looked-up
  version with a note of what was checked. Reject a plan with bare version
  numbers and no lookup note or "latest/LTS" designation.
- **Never run `/givn-design-review`'s subagents from within this
  orchestrator's context.** Design-review must start genuinely fresh — tell
  the user to invoke `/givn-design-review <id>` as its own command.
- Critical ambiguities go to the user — the architect does not guess.
