# ADR-0026: Plain-line quick setup for first-run configuration

- **Status:** accepted
- **Date:** 2026-09-01
- **Decision-makers:** Watn maintainers

## Context and Problem Statement

The first run without a configuration opens the full-screen ratatui setup
coordinator and walks a new user through provider, catalog, three model and
reasoning questions, and shell choices before anything works. Most new users
only need an endpoint, a credential, and three model identifiers to start.
ADR-0011 recorded the linear-prompt alternative as adequate for simple input
but rejected it for the full flow; it did not define a minimal first-run path.

## Decision Drivers

- Minimize the number of questions between installing watn and the first
  usable configuration.
- Keep the full coordinator unchanged as the refinement and editing surface.
- Never contact the network during onboarding; validate locally only.
- Keep one persistence path: both setup surfaces must produce equivalent
  configuration through the same provider-migration and atomic-save seams.

## Considered Options

- **Plain-line quick setup as the first-run surface** - a small module with
  one question at a time, suggestions, and no probing; the coordinator stays
  for explicit refinement. Two setup surfaces must be maintained.
- **Slim mode inside the coordinator** - one code path, but the entry still
  requires the full ratatui apparatus and the page graph, and cannot beat a
  five-question linear flow on simplicity.
- **Keep the coordinator as the only first-run surface** - single surface,
  but the initial experience stays heavyweight.

## Decision Outcome

Choose the plain-line quick setup. It runs when `watn quicksetup` is invoked
explicitly and automatically on a first run with no configuration file; an
existing-but-incomplete configuration still opens the coordinator. It asks
endpoint, credential, three model strengths, and one shell multiple-choice
question; an empty answer accepts the suggestion; reasoning is never asked
and stays unset. It saves through the same provider migration and atomic
config write and installs both managed shell blocks per selected shell. The
run ends after confirmation without sending an original request.

## Qualification

- Alternatives: PASS — three materially different options above.
- Architectural impact: PASS — fixes the first-run entry contract (dispatch
  by config-file existence) and introduces a second setup surface with its
  own module boundary sharing the persistence seams.
- Durable consequence: PASS — reversal removes the command, restores the
  first-run branch, and migrates docs, specs, and steps; both surfaces must
  stay consistent over time.
- Lower-level artifact classification: PASS — Gherkin owns observable
  behaviour and design.md owns mechanics; the paradigm and entry-boundary
  choice matches the precedent of ADR-0010/0011/0012/0020 and is not fully
  owned by either.
- Existing-ADR check: PASS — ADR-0011 (linear prompt rejected for the full
  flow), ADR-0013 (superseded wizard layout), and ADR-0020 (snapshot
  boundary) cover adjacent boundaries; none records a first-run minimal
  surface. Route: NEW_ADR, related to ADR-0011.

## Consequences

### Good

- First-run time-to-configuration drops to six plain-line questions
  (endpoint, credential, three models, shell selection) with suggestions.
- Onboarding performs no network request; credential suggestions persist the
  `${VARIABLE}` reference instead of a secret.
- The coordinator remains the single place for catalog, reasoning, and review
  refinement.

### Bad

- Two interactive setup surfaces must remain consistent (suggestions,
  persistence semantics, shell behaviour); drift between them is a standing
  risk.
- Hardcoded suggestions (e.g. the default OpenRouter model) can go stale and
  need periodic review.
- Quick-setup configurations carry no catalog endpoint and no reasoning
  settings, so later refinement is likely for reasoning-sensitive users.

## Confirmation

The `quicksetup-first-run` change's Gherkin scenarios assert the entry
contract, the question sequence, suggestion acceptance, overwrite semantics,
and abort side-effect freedom; shell-completions authoritative-tree scenarios
guard the new subcommand in generated completion scripts.

## More Information

Related: ADR-0011 (TTY-gated onboarding and the recorded linear-prompt
tradeoff), ADR-0020 (final-confirmation snapshot boundary reused here),
ADR-0024 (atomic configuration replacement).
