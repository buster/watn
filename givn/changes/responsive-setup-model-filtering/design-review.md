# Design Review: Responsive Setup Model Filtering

## Grilling Findings

### Scope

The proposal asks for a visible query, responsive debounced filtering, local
filtering when the complete catalog is available, provider-backed filtering for
incomplete catalogs, newest-query authority, and unchanged setup behavior. The
spec covers each behavior with three regular scenarios and one terminal E2E
scenario. Shell integration, persistence, provider credentials, and reasoning
policy remain out of scope.

The initial draft of the E2E scenario paired a complete catalog with a delayed
provider search, which was internally contradictory. It was hardened to use an
incomplete catalog for the delayed-search path. The E2E scenario also now makes
a third filter change after the delayed response window, proving that input
remains available rather than merely asserting a static screen.

### Technical Choices

Reusing the existing SetupWizard event loop, model matching rule, generation
counter, `portable-pty` harness, and `httpmock` twin is the smallest correct
approach. A new async runtime, search service, or picker framework would add
state without improving the observable terminal behavior.

Catalog completeness is intentionally conservative: a response shorter than
the configured page limit is complete; a full page is treated as incomplete
because the provider may have more results. A full page therefore uses remote
search even when the provider happens to have exactly that many models. This
avoids false local completeness and preserves the paginated-catalog contract.

Remote workers retain their generation and are joined at wizard shutdown. The
generation guard prevents stale application; joining prevents detached work
from surviving the wizard. Complete catalogs do not create a worker at all.

### Missing Scenarios And Boundaries

- Complete local filtering asserts matching and non-matching rows plus zero
  provider search requests.
- Incomplete-catalog filtering asserts the visible query, provider search, and
  matching result.
- Newest-query authority asserts that a late older response cannot replace the
  newer result. Existing permanent search-concurrency scenarios continue to
  cover the baseline generation contract.
- The E2E scenario drives a delayed terminal search, checks the query and
  result on the rendered PTY, and performs another filter change.
- Existing permanent scenarios cover empty results, unsupported provider search,
  local fallback after a search error, selection, and reasoning behavior.
- Provider authentication, persistence, shell integration, and deployment are
  unchanged and require no new scenario in this change.

No unresolved observable boundary remains.

### Testability

Every new Then-step asserts a concrete visible query, suggestion, provider
request count, or result-order outcome. The local path can fail RED if remote
search is attempted unnecessarily. The incomplete path can fail RED if the
provider request is skipped or the query/result is hidden. The E2E path can fail
RED if the terminal blocks during the delayed response, loses the current query,
or ignores a subsequent keystroke.

The worker-join invariant is verified by the implementation's retained-handle
shutdown path and the process-boundary E2E exit. It is not claimed from a
repository-only assertion.

### E2E Fidelity And Interaction Coverage

The capability is a CLI terminal UI. `portable-pty` starts the real
`watn setup` subprocess and sends user keystrokes. The primary E2E assertions
inspect the rendered terminal query and model rows. `httpmock` is only the
provider digital twin; it is not substituted for the PTY interaction.

The inventory has one interaction and the design matrix has exactly one row and
one matching `@e2e` scenario. Regular scenarios cover catalog and error variants
without spending additional E2E budget.

### Risk

The most likely failure is a stale worker updating the screen after a newer
query or surviving after the wizard exits. The mitigation is one generation
authority checked before request, before publish, and before apply, plus retained
worker handles joined during shutdown. The E2E scenario deliberately types a
replacement query while a provider response is delayed.

## Arc42 Independent Cross-Check

All twelve chapter rows were assessed independently against the proposal,
specification, and design:

| # | Chapter | Expected impact | `arc42.md` | Match |
|---|---|---|---|---|
| 1 | Introduction and goals | Yes: responsive setup filtering goal | Yes | Yes |
| 2 | Architecture constraints | No: existing terminal/provider constraints remain | No | Yes |
| 3 | Context and scope | Yes: visible filter and provider-search interaction | Yes | Yes |
| 4 | Solution strategy | Yes: hybrid local/remote filtering and lifecycle | Yes | Yes |
| 5 | Building block view | Yes: existing SetupWizard/Models responsibilities change | Yes | Yes |
| 6 | Runtime view | Yes: query, debounce, stale-result, and shutdown flow | Yes | Yes |
| 7 | Deployment view | No: no deployment artifact or topology changes | No | Yes |
| 8 | Cross-cutting concepts | Yes: terminal responsiveness and concurrency | Yes | Yes |
| 9 | Architecture decisions | Yes: ADR-0009 evolves to hybrid filtering | Yes | Yes |
| 10 | Quality requirements | Yes: QS-054 | Yes | Yes |
| 11 | Risks and technical debt | Yes: R-020 is strengthened | Yes | Yes |
| 12 | Glossary | Yes: catalog completeness, local filter, search worker | Yes | Yes |

The durable chapters contain substantive content. ADR-0009 now records the
accepted hybrid decision and its consequences; R-020 records the worker and
generation risks. No new ADR is required.

## Hardening Applied

- Corrected the delayed E2E scenario to use an incomplete catalog.
- Added a post-delay filter change to prove continued input responsiveness.
- Documented the conservative full-page completeness rule.
- Updated ADR-0009 and the affected Arc42 chapters 01, 03, 04, 05, 06, 08,
  09, 10, 11, and 12.
- Kept exactly one E2E scenario for the single interaction-inventory entry.
- Ran `givn lint --change responsive-setup-model-filtering`; the only findings
  are the four expected `@wip` scenario markers.

## Open Questions

None.

DESIGN-REVIEW: PASS
