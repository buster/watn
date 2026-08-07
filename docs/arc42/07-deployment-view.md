# 7. Deployment View

Deployment via `cargo install` or copying the single statically-linked binary to
PATH. No runtime dependencies, no server infrastructure.

The binary is built with `cargo build --release` and produces a standalone
executable with no dynamic library requirements beyond the OS kernel. Users
install it once and run it from any terminal.
