# 7. Deployment View

Deployment via `cargo install` or copying the single statically-linked binary to
PATH. No runtime dependencies, no server infrastructure.

The binary is built with `cargo build --release` and produces a standalone
executable with no dynamic library requirements beyond the OS kernel. Users
install it once and run it from any terminal.

## Verification build topology

Transport verification builds explicit binary variants before the Cucumber
runner. Each variant has an isolated target directory and an absolute path
passed to the harness:

| Variant | Feature/profile | Binary path shape | Transport behavior |
|---|---|---|---|
| Default debug | no feature / dev | `<target>/debug/watn` | Configured endpoint |
| Test-support debug | `test-support` / dev | `<target>/debug/watn` | Debug-only non-empty override for outbound requests |
| Default release | no feature / release | `<target>/release/watn` | Configured endpoint |
| Test-support release | `test-support` / release | `<target>/release/watn` | Configured endpoint; override branch is not compiled |

The four target directories are distinct. The harness does not discover or
reuse `target/debug/watn`, and scenarios do not compile binaries.
