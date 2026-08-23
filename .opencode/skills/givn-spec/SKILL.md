---
name: givn-spec
description: Write Gherkin .feature delta files for a givn change — observable behaviour in domain language, no implementation detail.
---

# givn-spec

Write the Gherkin spec delta for change `<change-id>`.

## Context

- Delta spec location: `givn/changes/<change-id>/specs/<capability>/<capability>.feature`
- Instructions: run `givn instructions specs --change <change-id>`
- Proposal (WHY): `givn/changes/<change-id>/proposal.md`
- Permanent specs: `givn/specs/`

## Delta tags

| Tag | Scope | Meaning |
|---|---|---|
| `@givn.delta` | Feature | Marks this as a delta document |
| `@<capability>` | Feature | Capability identifier (kebab-case) |
| `@givn.added` | Scenario | Append to permanent spec (default) |
| `@givn.modified` | Scenario | Replace scenario by title |
| `@givn.removed` | Scenario | Delete scenario by title |
| `@wip` | Scenario | Step definitions not yet implemented |

## Rules

- Scenarios assert observable behaviour in domain language (Given/When/Then).
  No class names, function names, routes, DB schemas, or step mechanics.
- One scenario = one observable behaviour.
- `@givn.removed` scenarios include exactly one placeholder step.
- On archive, all `@givn.*` tags are stripped automatically.
  The `@e2e` tag is preserved (it is not a `@givn.*` tag).

## Canonical interaction policy

Before writing any scenario, run:

```sh
givn instructions specs --change <change-id>
```

That instruction is the normative source for the user interaction inventory,
real-interface assertions, flag-variant classification, and one-E2E-per-action
scope. This skill only choreographs when to read it and how to write the delta;
it does not restate the policy.

## Retrieval-aware authoring

The normal Cargo installation includes every retrieval feature. In a
retrieval-capable build, use `givn spec index` for the permanent index and
`givn spec search` or `givn spec explore` for advisory authoring evidence. The
blocking review path is `givn check review --change <change-id>`. In a
feature-free build, preserve the explicit `retrieval-unavailable` result;
never treat unavailable retrieval as a clean blocking review.

E5, BGE, and NLI use the same complete deterministic Gherkin serialization.
Actual model tokenizers check complete inputs before model construction or
inference, without truncation or padding; exactly 512 tokens fits and 513 is
over. `BGE_TOKEN_CAP` and `NLI_TOKEN_CAP` are per-candidate, visible,
unresolved, non-filtered evidence. Run-level `BGE_UNAVAILABLE` or
`NLI_UNAVAILABLE` is used only when that layer can score no pair. BGE and NLI
score the same E5 pool independently, and combined recommendations intersect
their recommendations. Tell the author to shorten or split the scenarios and
rerun review; do not chunk or edit scenarios automatically.

## Verify command

Unit/integration:
```
./run-tests.sh
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
