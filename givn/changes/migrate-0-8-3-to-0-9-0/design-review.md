# Design Review: migrate-0-8-3-to-0-9-0

Review kind: self-review
Author: maki
Reviewer context: maki (current session)

## Grilling

Ordered by implementation risk. Every branch is resolved from project evidence; none required a user decision.

### 1. Rule displacement completeness

Question: could project-specific rules have existed only in untracked generated files, making the "none found" inventory unsafe?

Recommended answer: no. The pre-upgrade revision `e12e90d^` tracks no skill or command files, `.gitignore` excludes `.opencode/`, `.agents/`, `.claude/`, `.omp/`, and `/AGENTS.md`, and the config then declared the default `# rules: local-only`. The project owner confirmed no project-specific rules were kept in those files or in the managed `givn-project` block.

Resolution: inventory records "none found" with the checked paths. No `extensions:` entry is added. No finding.

### 2. Installed-file contract

Question: do the generated files carry trigger and direction only, with no rule copy and no extension text?

Recommended answer: yes. Checked all four configured targets: 15 `givn-*` skills each; `.opencode` and `.omp` also carry 13 `givn-*` commands each. A sample skill and command contain only frontmatter metadata plus a pointer to `givn instructions <id>`. Searches for `Project extension` and rule-copy text found nothing.

Resolution: no finding.

### 3. Scope

Question: does the design contain exactly the proposal's work and nothing extra?

Recommended answer: yes. The proposal asks to inventory displaced rules, restore any to `extensions:`, and otherwise record "none found"; the design does exactly that and leaves `givn/config.yaml` unchanged. The completion boundary is the archive of this maintenance change.

Resolution: no finding.

### 4. Tech choices

Question: is any technology choice introduced?

Recommended answer: no. The change edits no source and no configuration; the 0.9.0 managed refresh was already recorded in `e12e90d`. No finding.

### 5. Missing scenarios

Question: does the proposal imply observable behaviours absent from the spec?

Recommended answer: no. `specs` is skipped for this maintenance change, and the proposal declares no product behaviour. No finding.

### 6. Testability

Question: can the planned verification fail?

Recommended answer: N/A for scenarios; specs are skipped, so there is no RED step. The design's completion check is the project's existing suite from `verify.command`, which exercises the unchanged product. No finding.

### 7. E2E fidelity

Question: are the spec instruction's action scope, driver, and assertion rules respected?

Recommended answer: N/A. No spec exists and no `@e2e` scenario is added or removed. No finding.

### 8. Interaction Coverage

Question: does a use-case interaction inventory require rows in a coverage matrix?

Recommended answer: N/A. A maintenance migration has no use case and no interaction inventory. No finding.

### 9. Risk

Question: what is the most likely way this plan fails?

Recommended answer: a displaced rule goes unnoticed because the pre-upgrade files were untracked. Mitigation: the owner confirmation recorded in `design.md`, plus the exhaustive tracked-file and ignore-policy check. No residual risk after that confirmation.

Resolution: no finding.

### 10. Use-case and Persona context

Question: does the change need a use-case contract or Persona disposition?

Recommended answer: no. The change is maintenance with no capability owner and no user-visible surface; no Persona is invented or promoted. No finding.

### 11. Decision ledger

Question: does every proposal question (`D<n>`) resolve in the design?

Recommended answer: N/A. The proposal carries no decision questions. No finding.

### 12. Contracts and components

Question: does the design add a contract, component, or adapter?

Recommended answer: no. It adds no code or interface; no consumer imports anything new. No finding.

### 13. Decision timing

Question: is any foundational question deferred or carried?

Recommended answer: no. There are no open questions and no speculative abstraction. No finding.

### 14. Visual design contract

Question: does the change introduce or visibly change a screen?

Recommended answer: no. No interface is touched. No finding.

### 15. Deferrals

Question: is any mandatory check deferred?

Recommended answer: none. No finding.

### 16. Persona disposition

Question: does the change require a confirmed Persona?

Recommended answer: no. It declares no visual design contract and has no user-visible surface. No finding.

### 17. Interface classification

Question: does a non-browser interface classification need justification?

Recommended answer: N/A. No interface is introduced or changed. No finding.

### 18. ADR qualification

Question: does the recorded assessment correctly route the decision?

Recommended answer: yes. `arc42.md` records architectural commitment, real alternatives, and durable consequence as none. The chapter 09 register (ADR-0001 through ADR-0026) and `docs/adr/` were searched; no decision covers Givn generated-rule placement. Action `NONE` has a canonical home: the inventory disposition in `design.md`. No finding.

### 19. Architecture documentation

Independent re-derivation of all 12 rows against the proposal and design produced "No" for every row, matching `arc42.md` row for row. All twelve chapter files exist with substantial content (3.0 KB to 52.9 KB), not scaffold. No Unicode box-drawing characters appear in any chapter. The pseudo-diagram scan matched only GitHub table separator rows, not ASCII diagrams. Chapter 09 links ADR-0001 through ADR-0026; no new ADR is required. No finding.

### 20. Ubiquitous language

Question: does the change introduce or misuse a domain term?

Recommended answer: no. "Displaced statement" and "extensions" are Givn tooling terms, not project domain vocabulary, and no glossary entry is needed. No finding.

## Hardening

No hardening edits were required. `design.md` already carries the inventory and disposition, `arc42.md` matches the independent assessment, and no scenario or contract was added. `tasks.md` is not written by this review.

## Verdict

DESIGN-REVIEW: PASS
