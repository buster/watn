██╗    ██╗ █████╗ ████████╗███╗   ██╗██████╗
██║    ██║██╔══██╗╚══██╔══╝████╗  ██║╚════██╗
██║ █╗ ██║███████║   ██║   ██╔██╗ ██║  ▄███╔╝
██║███╗██║██╔══██║   ██║   ██║╚██╗██║  ▀▀══╝
╚███╔███╔╝██║  ██║   ██║   ██║ ╚████║  ██╗
 ╚══╝╚══╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═══╝  ╚═╝

You know what you want. watn knows how to type it.
Ask in plain language. Get one command. Run it with `-x`, or pipe it anywhere.

## Demo

```text
┌──────────────────────────────────────────────┐
│ $ watn "files changed in the last day"       │
│                                              │
│   find . -type f -mtime -1                   │
│                                              │
│   gpt-4o-mini · 142 tok/s · $0.0003 · 0.6s   │
│   Execute now? [Y/n]                         │
└──────────────────────────────────────────────┘
```

## Quick start

```
git clone <repo>
cd <project>
make setup
```

## Prerequisites

- Rust (latest stable)
- An OpenAI-compatible API endpoint

## Usage

```text
watn "find all files modified in the last day"
```

Pass a question as a positional argument or via stdin. watn streams the
generated command to stdout and metadata (model, tokens/s, cost) to stderr.

| Flag | Description |
|---|---|
| `-1` | Small/fast model tier (default) |
| `-2` | Balanced model tier |
| `-3` | Thinking/reasoning model tier |
| `--model <NAME>` | Explicit model override |
| `-x` | Execute mode: prompt before running |
| `--version` | Print version and exit |

## Configuration

Layered config stack: built-in defaults → XDG config file → environment
variables → CLI flags. Config file is TOML.

## Development

```
cargo test --test features_runner
```

## Coverage

[![Coverage: 77.8%](https://img.shields.io/badge/coverage-77.8%25-brightgreen)](coverage/non-e2e-cobertura.xml)

Coverage runs the acceptance scenarios against the instrumented `watn` executable. The latest non-E2E run covers `583/749` lines (`77.8%`); the E2E subset covers `465/749` lines (`62.1%`).

```sh
mkdir -p coverage
cargo llvm-cov clean --workspace
cargo llvm-cov run --bin watn --no-report -- --version
cargo llvm-cov test --no-clean --test features_runner \
  --cobertura --output-path coverage/non-e2e-cobertura.xml \
  -- --tags 'not @wip'
```

## License

GPL-3.0-or-later
