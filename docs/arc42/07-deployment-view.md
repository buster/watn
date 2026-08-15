# 7. Deployment View

Deployment via `cargo install` or copying the release executable to PATH. The
artifact is one binary for the selected target, but its runtime library
requirements are target-dependent; no server infrastructure is required.

The optional shell shortcut is a per-user deployment integration, not a second
binary or service. The installed `watn` executable must be resolvable through the
user's `PATH`; setup writes only the selected marked block to the user's Bash,
Zsh, or Fish startup target. The user reloads the reported file or starts a new
shell. Completion installation and shortcut installation remain separate paths.

The binary is built with `cargo build --release`. On the verified Linux host,
`file target/release/watn` identifies a dynamically linked executable and
`ldd target/release/watn` lists the shared libraries it requires. On macOS,
`otool -L target/release/watn` provides the equivalent library inspection. The
project does not claim a universal static deployment artifact.

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
`target/debug/watn`. Product release deployment remains the single release
executable described above, with the target-library inspection as its release
truth check.

The incremental SSE change does not add a production service, sidecar, or
deployment artifact. Verification runs the child binary against a loopback
streaming twin that can flush content, hold a later event, send `[DONE]` without
closing immediately, close without `[DONE]`, or reset the connection. The twin
exists only in the test process; the installed production binary continues to
use the configured provider endpoint.

Shortcut verification uses temporary HOME/XDG trees and isolated startup files.
It does not modify the developer's shell configuration. The regular runner
checks generated Bash, Zsh, and Fish text; the E2E runner executes `bash -n`,
`fish -n`, a non-interactive Bash process, and an interactive Fish process under
a pseudo-terminal with a temporary startup file and fake `watn` on PATH. No
terminal emulator is required for this capability.

The specification consolidation changes no Watn deployment artifact, runtime
service, or installed configuration. Its review/archive commands operate on
repository files before the unchanged executable is released.
