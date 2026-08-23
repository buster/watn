# Design Review: Givn 0.2.0 to 0.3.0 Migration

## Review basis

The review covered the complete `proposal.md`, `design.md`,
`migration.yaml`, and `.givn-skip`. The migration is an aggregate maintenance
change with no product feature specification. The machine-safe preparation is
recorded by Commit A `d319e3e6aa04884fa53864d7548408bebc9e7c5e` and Commit B
`5cf7bd1631f155a98b8e60485dd9ac10a4e9e983`.

## Grilling outcomes

| Branch | Question | Finding and disposition |
|---|---|---|
| Scope and completion | Does the plan cover the complete migration without extra or missing product work? | Yes. The single bundle is ordered in `migration.yaml`; the design contains inventory, managed-upgrade, guidance, project-review, and verification/archive prompts. Completion remains gated by project-specific tasks and the normal Givn artifacts. |
| Ownership boundary | Could the migration invent domain behavior, tests, ADRs, or Arc42 facts? | No. The proposal explicitly forbids those changes. Evidence-based reconciliation is required and `.givn-skip` contains only `specs`, so no synthetic migration feature is allowed. |
| Technical approach | Is the prompt-driven maintenance approach justified over a config-only refresh? | Yes. A config marker cannot determine which project-owned specifications, generated guidance, or architecture records require review. The ordered prompt pack is the smallest approach that preserves those ownership boundaries. |
| Managed paths | Does the machine-safe step constrain staging to the generated allowlist? | Yes. Commit A was inspected and contains `givn/config.yaml`, generated `.opencode` guidance, and `AGENTS.md` only. Commit B contains only the generated migration plan. No whole-worktree staging was used. |
| Missing scenarios and errors | Are observable migration failures covered? | Yes. The design covers dirty/non-Git boundaries, ignored managed paths, stale generated guidance, unrepresented project overrides, missing architecture evidence, failed lint/verification, and archive gating. These are operational evidence concerns, not product scenarios. |
| Testability | Can the migration be tested RED/GREEN without inventing production behavior? | No product RED/GREEN scenario applies. The change has no `.feature` files by design. Its testability is through commit allowlists, artifact status, lint, configured verification commands, and review/archive gates. No fabricated test was added. |
| E2E fidelity | Does the change require a browser, HTTP, or CLI product E2E scenario? | No. The migration does not change a user-facing interface or runtime path. The configured project E2E command remains a final verification obligation for the existing product tree. |
| Interaction coverage | Does a feature User Interaction Inventory require a design matrix? | No. The migration has no product `.feature` file and no new interaction. No inventory entry is present or missing. |
| Risk | What is the most likely failure during implementation? | The primary risk is declaring the migration complete after only refreshing managed files. The mitigation is to derive tasks from the inventory, require current evidence for each concern, run lint and both configured runners, and retain the review/archive gates. |
| ADR qualification | Does the migration introduce a qualified architectural decision? | No. The migration changes process and authoring guidance, not a durable Watn boundary or contract. Existing ADR indexes were searched; no new, amended, or superseding ADR is justified. |

No user decision was required: every review branch was resolved from the
proposal, design, repository state, and existing architecture records. No
hardening edit to `design.md` or a product specification is required.

## Arc42 independent check

The Arc42 addon is enabled. The twelve chapter-impact rows were independently
re-derived before comparing `arc42.md`:

| Row | Independent result | Evidence |
|---|---|---|
| 1. Goals, stakeholders, quality attributes | No | No Watn goal, stakeholder expectation, or product quality scenario changes. |
| 2. Constraints | No | The Givn authoring contract is process guidance, not a Watn constraint. |
| 3. External systems, interfaces, context, user-facing surface | No | No Watn command, provider interface, external system, or boundary changes. |
| 4. Major technical strategy | No | No runtime implementation strategy changes. |
| 5. Building blocks | No | No production component, module, or connector changes. |
| 6. Runtime flows | No | Ordered migration phases are not Watn runtime sequences. |
| 7. Deployment | No | No executable, service, target, library, or installation change. |
| 8. Cross-cutting concepts | No | No Watn error, security, configuration, persistence, or transport behavior changes. |
| 9. Architecture decisions | No | No decision candidate passes the ADR qualification gate. |
| 10. Quality scenarios | No | The migration explicitly has no product feature specification. |
| 11. Risks and technical debt | No | No new durable Watn risk or debt is evidenced; migration risk is tracked here and in the task/review gates. |
| 12. Glossary | No | No new Watn domain term is introduced. |

The independent table matches `arc42.md` exactly. All twelve files under
`docs/arc42/` exist and contain project-specific content. The chapter scan
found Mermaid diagrams and no prohibited Unicode box-drawing or ASCII-art
diagrams. Existing ADR indexes and records were checked; no qualified
candidate is missing and no non-qualified decision was duplicated.

## Verification

- `git show --stat d319e3e6aa04884fa53864d7548408bebc9e7c5e` confirms the managed
  allowlist boundary.
- `git show --stat 5cf7bd1631f155a98b8e60485dd9ac10a4e9e983` confirms that the
  aggregate commit contains only the migration plan.
- `givn addons list` confirms the enabled Arc42, guidance, and coverage addons.
- `givn graph` confirms that `design-review` requires `design` and `arc42-docs`
  and that `tasks` follows this review.
- `givn lint --change migrate-0-2-0-to-0-3-0` reports no feature files, as
  required by `.givn-skip`.

DESIGN-REVIEW: PASS
