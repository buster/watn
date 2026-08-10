# 7. Deployment View

Deployment via `cargo install` or copying the single statically-linked binary to
PATH. No runtime dependencies, no server infrastructure.

The binary is built with `cargo build --release` and produces a standalone
executable with no dynamic library requirements beyond the OS kernel. Users
install it once and run it from any terminal.

## Verification build topology

Debug transport verification builds the two required binary variants before the
Cucumber runner through Cargo's shared default target cache, then copies each
result to a unique temporary path:

| Variant | Feature/profile | Copied path shape | Transport behavior |
|---|---|---|---|
| Default debug | no feature / dev | `<root>/default-debug` | Configured endpoint |
| Test-support debug | `test-support` / dev | `<root>/test-support-debug` | Debug-only non-empty override for outbound requests |

The builds run sequentially so the second feature build can reuse Cargo's
dependency cache without overwriting the first copied executable. The harness
receives only these two absolute paths and never discovers or reuses
`target/debug/watn`. Release-profile runtime verification is deferred to
`release-truth-and-repository-cleanup`; product release deployment remains the
single release binary described above.
