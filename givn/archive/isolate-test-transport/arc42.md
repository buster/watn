# Arc42 Impact Assessment: Isolate Test Transport

This assessment was independently derived from `proposal.md`, the complete
delta feature file, and `design.md` before comparing it with the durable arc42
chapters. The loopback provider twins and transport harness are test
architecture; they do not create a product-facing provider or user interface.

## Independent Chapter Selection

| # | Chapter | Affected? | Reason | Required architecture consequence |
|---|---|---|---|---|
| 1 | Introduction and Goals | Yes | The change adds security and test-isolation goals for release binaries, configured endpoints, readiness, and persisted configuration. | Record the release-redirection prohibition and the deterministic test-maintainer quality goal. |
| 2 | Architecture Constraints | Yes | Cargo feature/profile compilation, the shared debug target cache, and explicit copied paths constrain implementation and verification. | Require `cfg(all(feature = "test-support", debug_assertions))` and the two named debug binary paths. |
| 3 | Context and Scope | No | No product-facing user surface, provider partner, protocol, or system boundary changes; loopback twins are test fixtures only. | Leave the product context and external-interface chapter unchanged. |
| 4 | Solution Strategy | Yes | Compile-time isolation, pure URL construction, and the efficient two-copy debug bootstrap are material technical strategy decisions. | Document the debug-only seam and the shared-cache copy sequence; defer release runtime proof. |
| 5 | Building Block View | Yes | The transport resolution boundary and transport-specific harness state are explicit building blocks. | Describe the sole endpoint-resolution seam and its configured/readiness boundary. |
| 6 | Runtime View | Yes | Configured, isolated, fallback, source-level release guard, and readiness flows are new or clarified behavior. | Document exact debug request routing and zero-network readiness flows; record release runtime verification as deferred. |
| 7 | Deployment View | Yes | Two copied debug binaries change the verification topology; product release deployment is unchanged. | Document the two explicit debug paths and shared Cargo target cache. |
| 8 | Cross-cutting Concepts | Yes | Environment access, credential headers, configuration persistence, readiness, and transport security are cross-cutting concerns. | Define the compile-time guard and exact transport assertion contract. |
| 9 | Architecture Decisions | Yes | The debug-plus-feature boundary and efficient debug bootstrap refine the existing E2E transport decision. | Update ADR-0011 with release source behavior, pure builders, shared-cache copies, and deferred runtime proof. |
| 10 | Quality Requirements | Yes | Exact endpoint, path, count, Authorization, competing-server, persistence, fallback, readiness, and source-level release-safety criteria are new measurable requirements. | Add debug transport criteria and retain release runtime proof for the later change. |
| 11 | Risks and Technical Debt | Yes | Release verification deferral, stale binary selection, broad mocks, and incomplete per-child evidence are concrete risks. | Record the later verification owner, two-copy bootstrap mitigation, exact assertions, and per-child accounting requirement. |
| 12 | Glossary | Yes | `test-support binary`, `release-profile binary`, configured endpoint, and competing provider twin become required terms. | Add the transport terms and refine the override definition. |

## Required Architecture Consequences

- The override lookup exists only in the debug `test-support` compilation
  branch. A release build with `test-support` enabled uses the configured
  endpoint by source guard; release-profile runtime verification is owned by
  `release-truth-and-repository-cleanup`.
- URL builders are pure and receive an already resolved endpoint. Readiness,
  persistence, display, and configuration loading never resolve the override.
- The harness creates reachable loopback configured, competing, and isolated
  provider twins, writes a default model, and passes two absolute copied debug
  paths from Cargo's shared default target cache instead of discovering a stale
  executable.
- Every transport assertion identifies the full endpoint, method/path, exact
  request count, exact Authorization header, competing-server zero hits,
  response source, and unchanged persisted endpoint.
- The normal debug scenario invokes exactly one `WATN_DEFAULT_DEBUG_BIN` child
  with a non-empty override and requires one configured-server hit and zero
  competing-server hits. The `test-support` debug copy is not invoked in this
  scenario; its override-honoring behavior is covered by isolated routing.
- Missing and whitespace overrides have one E2E scenario with two explicit child
  invocations of `WATN_TEST_SUPPORT_DEBUG_BIN`; each child is checked and the
  aggregate configured count is two. Readiness has a focused non-E2E contract
  scenario and no network request.
- The stale-search false-green defect is explicitly deferred to
  `model-discovery-and-setup-correctness` and is not an implementation
  obligation of this transport change.

## Durable Documentation Applied

The affected durable chapters, index, and existing ADR were updated surgically:

- `docs/arc42/README.md`
- `docs/arc42/01-introduction-and-goals.md`
- `docs/arc42/02-architecture-constraints.md`
- `docs/arc42/04-solution-strategy.md`
- `docs/arc42/05-building-block-view.md`
- `docs/arc42/06-runtime-view.md`
- `docs/arc42/07-deployment-view.md`
- `docs/arc42/08-crosscutting-concepts.md`
- `docs/arc42/09-architecture-decisions.md`
- `docs/arc42/10-quality-requirements.md`
- `docs/arc42/11-risks-and-technical-debt.md`
- `docs/arc42/12-glossary.md`
- `docs/adr/0011-interactive-provider-onboarding.md`

Chapter 03 was assessed as unaffected and was not edited. All diagrams in the
updated arc42 chapters remain Mermaid diagrams. Release runtime smoke execution
is not claimed by these durable updates.

## Status

STATUS: DONE
