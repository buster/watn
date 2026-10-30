# Design Review: normalize-fragment-e2e-inventory

Review kind: self-review
Author: maki
Reviewer context: maki

## Grilling

Ordered by implementation risk. Every branch was resolved from the artifacts; no user question remained open.

### 1. Merge identity and scenario text

Question: will each `@givn.modified` delta replace exactly the intended permanent scenario without altering its steps?

Recommended answer: yes. A mechanical comparison parsed all six permanent fragment features and the five delta files: 15 modified scenarios, 0 errors. Every delta title exists in the permanent feature, carries `@givn.modified`, does not carry `@e2e`, and its Given/When/Then lines are byte-identical to the permanent scenario.

Resolution: no finding.

### 2. E2E scope and reclassification justification

Question: is every retagged scenario genuinely a variant, error case, or output-affecting flag combination of an action that keeps an `@e2e` scenario?

Recommended answer: yes. The design's reclassification table classifies each row: tier flags, confirmation answers, cost and throughput output, cancellation timing, configuration precedence, verbose streaming, mid-stream failure, transport fallback, and the two input-channel variants recorded under D1. Each capability keeps at least one real-interface action: ask keeps ask and execute; cancel keeps cancellation; config keeps tier configuration and pricing; incremental keeps streamed rendering and terminal confirmation; release-truth keeps version and changelog; transport keeps both routing modes.

Resolution: no finding. The retained incremental confirmation is a distinct action from the ask execution scenario: it asserts the streamed command is visible before confirmation and executed after it, which the ask execution scenario does not cover.

### 3. Interaction inventory completeness

Question: does the 11-row fragment inventory match the retained `@e2e` set exactly?

Recommended answer: yes. A set comparison of the retained permanent `@e2e` titles against the inventory's E2E column returned 11 retained, 11 declared, zero missing, zero extra.

Resolution: no finding.

### 4. Scope

Question: does the change do exactly what the proposal asks and nothing more?

Recommended answer: yes. Fifteen scenarios are retagged through five delta files; release-truth needs no delta because it keeps both scenarios; the inventory is reduced to the 11 retained actions. No scenario text, capability, relationship, production file, or other use case changes.

Resolution: no finding.

### 5. Tech choices

Question: is any technology choice introduced?

Recommended answer: no. The change uses the existing delta mechanism (`@givn.modified` replacing by title) and the existing `cucumber-rs` runner. No dependency, driver, or configuration is added.

Resolution: no finding.

### 6. Missing scenarios

Question: does the change remove an observable behaviour from the suite?

Recommended answer: no. All 15 retagged scenarios keep their text and remain executed in the regular scope. The 367-scenario total is unchanged.

Resolution: no finding.

### 7. Testability

Question: can the retagged scenarios still fail meaningfully, and is strict mode intact?

Recommended answer: yes. They are unchanged scenarios bound to unchanged step definitions; strict mode remains `.fail_on_skipped()` at `tests/features_runner.rs:211`. The design names the single-scenario command `./run-tests.sh --name "<scenario title>"`.

Resolution: no finding.

### 8. E2E fidelity

Question: do the retained `@e2e` scenarios drive the real CLI interface, and is the matrix complete?

Recommended answer: yes. The matrix has 11 rows for 11 inventory entries, each naming the CLI interface and the concrete driving mechanism: subprocess assertions, PTY input, or a local provider twin. No HTTP client substitutes for a terminal interaction.

Resolution: no finding.

### 9. Interaction Coverage Matrix

Question: does every inventory entry map to one retained scenario and one driving mechanism?

Recommended answer: yes. The matrix and the inventory cover the same 11 actions; the three capabilities with no row (search-concurrency has no `@e2e`) have no inventory entry.

Resolution: no finding.

### 10. Risk

Question: what is the most likely way this plan fails?

Recommended answer: a title mismatch or a step-text drift during archive merge, or a count assumption that is not re-verified. Mitigation: the mechanical comparison above, plus post-archive checks for `givn lint` clean, e2e 77, and regular 290.

Resolution: no finding.

### 11. Use-case and Persona context

Question: does the change preserve the fragment contract and avoid invented Personas?

Recommended answer: yes. The fragment document keeps its capabilities, interactions, and diagram; no capability or relationship changes. `terminal-developer--interactive` remains a review lens; no Persona becomes a Gherkin actor.

Resolution: no finding.

### 12. Decision ledger

Question: does every proposal question resolve in the design?

Recommended answer: yes. `Decides D1` records that piped ask and piped confirmation are input-channel variants, realized in the ask and incremental deltas and the inventory. No carried question exists.

Resolution: no finding.

### 13. Contracts and components

Question: does the change add or change a contract or component?

Recommended answer: no. It edits specification tags and the inventory only; no consumer imports anything new. The Contracts and Components subsection is correctly absent.

Resolution: no finding.

### 14. Decision timing

Question: is a foundational question deferred or carried?

Recommended answer: no. D1 is answered at design time; no speculative abstraction exists.

Resolution: no finding.

### 15. Visual design contract

Question: does the change introduce or visibly change a screen?

Recommended answer: no. No interface is touched; the design correctly declares no visual contract.

Resolution: no finding.

### 16. Deferrals

Question: is a mandatory check deferred?

Recommended answer: none. Both suites and lint remain runnable in this change.

Resolution: no finding.

### 17. Persona disposition

Question: does the change require a confirmed Persona?

Recommended answer: no visual contract and no user-visible surface change; the confirmed `terminal-developer--interactive` remains the review lens. No operator confirmation is needed for a `none` disposition because no visual contract is declared.

Resolution: no finding.

### 18. Interface classification

Question: is the CLI classification correct for the retained scenarios?

Recommended answer: yes. Each scenarios invokes the built `watn` binary as a real subprocess or through a PTY; none renders a browser page. The opposite classification (Web UI) does not hold because no HTML/CSS/JavaScript is rendered.

Resolution: no finding.

### 19. ADR qualification

Question: does the recorded assessment route the decision correctly?

Recommended answer: yes. `arc42.md` records no architectural commitment, no real alternatives, and no durable consequence. The chapter 09 register (ADR-0001 through ADR-0026) and `docs/adr/` were searched; no decision covers test-scope classification. Action `NONE` has a canonical home in `design.md`.

Resolution: no finding.

### 20. Architecture documentation

Independent re-derivation of all 12 rows produced `No` for every row, matching `arc42.md` row for row. All twelve chapter files exist with substantial content; the earlier box-drawing scan remains valid because no chapter was edited. No chapter or ADR file changed.

Resolution: no finding.

### 21. Ubiquitous language

Question: does the change introduce or misuse a domain term?

Recommended answer: no. "E2E", "regular scope", and "variant" are workflow vocabulary; no project domain term is added or redefined.

Resolution: no finding.

## Hardening

No hardening edits were required. The delta files, inventory, and design already match. No scenario was added or removed, and `tasks.md` is not written by this review.

## Re-review: change verification adapter

The design changed after implementation exposed a verification-adapter gap: `givn check tasks` supplies `GIVN_FEATURES` and the `change` scope, but the runner ignored the path and `run-tests.sh` overwrote the scope with `regular`.

Re-walked branches:

- **Tech choices:** no new dependency; the existing runner gains one environment check, and the shell adapter gains one scope branch. The archive path is unchanged because archive supplies `GIVN_FEATURES=givn/specs`.
- **Testability:** the change scope executes exactly the 15 delta scenarios with the tag filter `not @wip`, including any delta `@e2e` scenario; the default regular and e2e filters are untouched.
- **Risk:** a default-path regression would change the 290/92 counts; both suites are re-run after the adapter change and the results are recorded in the review.
- **E2E fidelity and interaction coverage:** unchanged; the adapter does not alter scenario selection for the regular or e2e scopes.
- **Contracts, visual contract, decisions:** unchanged; D1 remains answered.

No finding. The design now names the adapter behavior, its environment inputs, and its verification.

## Verdict

DESIGN-REVIEW: PASS
