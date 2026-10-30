# Tasks: normalize-fragment-e2e-inventory

This change carries no new product behaviour and no production code. It retags 15 existing fragment scenarios through `@givn.modified` deltas and reduces the fragment inventory to the 11 retained actions. The design constraints for every batch are the reclassification table and the interaction matrix in `design.md`; no scenario text changes.

## Batch: delta reclassification and inventory

- [x] **Evidence:** Five delta files under `givn/changes/normalize-fragment-e2e-inventory/specs/fragments/` carry 15 `@givn.modified` scenarios without the `@e2e` tag. A parse comparison against the permanent features found 0 title or step-text errors. `givn/specs/fragments/fragment.md` lists 11 actions; the retained-tag set matched exactly (11 retained, 11 declared, 0 missing, 0 extra). The reclassification table records each moved scenario as a variant, error case, output-affecting flag, input-channel variant (D1), timing, precedence, or fallback. No production file or step definition changed.
- [x] **Verification:** `givn lint --change normalize-fragment-e2e-inventory` exits 0 with advisories only; the 15 delta titles exist in the permanent features with identical Given/When/Then lines.
- [x] COMMIT: 9c6f633842fe48bc10bb866553c3843f78a8b3f0 -- Batch: delta reclassification and inventory

## Batch: scoped verification and review

- [ ] **Evidence:** `./run-tests.sh` runs 28 features, 290 scenarios, and 1756 steps, all passing in two consecutive confirmation runs. The first run of this batch reported one transient failure that did not reproduce; `review.md` records it. `./run-tests.sh --e2e` runs 26 features, 92 scenarios, and 699 steps, all passing; the e2e scope stays at 92 before the archive merge. `givn check review --change normalize-fragment-e2e-inventory` passes every configured gate.
- [ ] **Verification:** `review.md` records the counts, the transient failure, and the expected post-merge result (26 features, 77 e2e scenarios; 290 regular scenarios); the archive receipt proves the post-merge counts and `givn lint` cleanliness.
- [ ] COMMIT: pending -- Batch: scoped verification and review
