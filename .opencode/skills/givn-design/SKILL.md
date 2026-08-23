---
name: givn-design
description: Write the technical design for a givn change — the HOW layer with technology decisions, step definitions, and architecture impact.
---

# givn-design

Write the design for change `<change-id>`.

## Context

- Design file: `givn/changes/<change-id>/design.md`
- Instructions: run `givn instructions design --change <change-id>`
- Spec (WHAT): `givn/changes/<change-id>/specs/`
- Test runner: `./run-tests.sh`

## This is the HOW layer

Everything technical belongs here — do not add implementation details to the spec.

- Technology decisions: language, framework, libraries, patterns.
- Architecture impact: which modules/components are affected, what is new.
- Data model changes: schema, structs, database changes.
- Step definition locations: where Cucumber/Gherkin step defs live. **One
  file per capability** — never a single file for the whole change.
- Runner command: confirm `verify.command` in `givn/config.yaml`.
- **Strict-mode config**: the exact flag/mechanism that makes the runner
  fail on undefined/pending steps (see "Strict mode" below).
- **Single-scenario run command**: the exact invocation to run one named
  scenario, used by every RED/GREEN check in tasks.md.
- Justify non-obvious technical choices.

## Version freshness (mandatory)

Never write a version number for a language, runtime, framework, library,
database, or container image from memory. Training data has an unknown
cutoff — a remembered "current" version can now be outdated or
end-of-life. For every versioned choice, either state "latest" or the
ecosystem's LTS explicitly (e.g. "Node.js: latest LTS"), or look up a
current version now and record it with what was checked (e.g. "Playwright
1.49.x — checked npmjs.com/package/playwright"). A bare specific version
with no lookup note and no "latest/LTS" designation is an unverified guess.

## Strict mode (mandatory)

Most runners treat undefined/pending steps as neither pass nor hard failure
by default; some (notably Java/cucumber-jvm with an empty step body) report
an outright PASS. This has checked off whole changes complete with nothing
implemented. Document explicitly:

- The strict-mode flag/config for the chosen runner (e.g. `cucumber-rs`:
  `.fail_on_skipped()`; `cucumber-js`: `--strict`; `cucumber-jvm` CLI:
  `--strict`; `cucumber-jvm` JUnit Platform Suite: no native flag — enforced
  by never leaving a step body empty, plus a plugin/listener that fails on
  UNDEFINED/PENDING results; `behave`: steps must `raise` explicitly).
- The not-implemented stub pattern for this language (e.g. Java:
  `throw new io.cucumber.java.PendingException(...)`; Python:
  `raise NotImplementedError(...)`; Rust: `unimplemented!()`).

If the runner cannot fail on undefined/pending steps via configuration
alone, state that step-body discipline is the enforcement mechanism and
name a mechanical check (grep-based lint, review-time audit) that will
catch empty bodies before archive.

## Local runnability & digital twins (mandatory)

The system MUST be runnable and testable locally, fully, with one command.
"The interface had a technical problem so we tested the database instead"
is not an acceptable outcome — it is a design gap.

- **Local run command**: the single command that starts the entire system
  (app + every dependency) in an isolated network (e.g. `docker-compose up`).
- **Digital twin per external/third-party dependency**: every service this
  system talks to that is not part of this codebase (email, payments,
  third-party APIs, cloud services) gets a fake/stub/emulator running in
  the same isolated network. No scenario may depend on a live third-party
  service. State explicitly if there are none.
- **Anticipated interface obstacles get a named fix**: if you can foresee a
  technical obstacle to driving the real interface (session/cookie
  persistence across steps, auth redirects, websocket handshakes), name the
  concrete fix here. A scenario blocked on an undesigned interface problem
  during implementation is a design gap, not a reason to test something weaker.

## Canonical E2E policy

Read `givn instructions specs --change <change-id>` before designing E2E
coverage. That instruction owns inventory normalization, action scope,
real-interface assertions, and driver fidelity. This skill owns only the
technical design fields and the coverage matrix below.

## Black-Box-First policy

Prefer the real interface. For every internal test retained, answer the
question: **which case this test covers that the E2E does not**? Without a
concrete answer, use the E2E scenario instead of adding a weaker duplicate.

## Coverage process boundaries (when enabled)

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|

Include the runner and every production process started by tests. For each:

- Launch the instrumented artifact, not the normal build.
- Use collision-safe profile output.
- Flush data before shutdown.
- Merge its profile before export.
- Name one exercised production path expected to be non-zero.

### Interaction Coverage Matrix (mandatory)

Map every normalized entry from the canonical specs policy to its matching
`@e2e` scenario and driving mechanism. One row per entry:

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| (e.g. "submit passcode form on login page") | (e.g. "Family member enters the correct passcode") | (e.g. "Web UI") | (e.g. "Playwright: fill input, click submit button") |
| (e.g. "click 'Add' button on list page") | (e.g. "Family member adds an item") | (e.g. "Web UI") | (e.g. "Playwright: fill form inputs, click 'Add' button") |
| (e.g. "click 'Check' button on a list item") | (e.g. "Family member marks item as purchased") | (e.g. "Web UI") | (e.g. "Playwright: click 'Check' button on list row") |

**Every row must have a non-empty driving mechanism.** For a Web UI, the
driving mechanism MUST name a specific browser driver (Playwright, Selenium,
WebDriverIO) and describe what action it performs (click button X, fill
input Y, navigate to Z). An HTTP client (`reqwest`, `curl`, `fetch()`,
etc.) is NOT a valid driving mechanism for a Web UI — it cannot click
buttons or read the DOM. Specifying "HTTP client via unit steps" in a Web
UI matrix row is a design deficiency that will be caught by design-review's
E2E fidelity branch. If the interface type is HTTP/REST API (no browser
involved), an HTTP client IS the real interface and that is the correct
driving mechanism.

Every row must name a concrete interface and driving mechanism. The matrix is
proven at design time and verified at review time; policy semantics come from
the resolved specs instruction.

## Verify command

Unit/integration:
```
./run-tests.sh
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
