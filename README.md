# Watn? ¯\\\_(ツ)\_/¯

You know what you want. watn knows how to type it.
Ask in plain language. Get one command. Run it with `-x`, or pipe it anywhere.

## Demo

```text
┌──────────────────────────────────────────────┐
│ $ watn "files changed in the last day"       │
│                                              │
│   find . -type f -mtime -1                   │
│                                              │
│   deepseek · 100 tok/s · 5.3s · ¯\_(ツ)_/¯   │
└──────────────────────────────────────────────┘
```

## Prerequisites

- Rust (latest stable)
- An OpenAI-compatible API endpoint

## Usage

Pass a question as a positional argument or via stdin. watn streams the
generated command to stdout and metadata (model, tokens/s, cost) to stderr.

### Positional argument

```text
$ watn find all files modified in the last day
find . -type f -mtime -1
```

### stdin

```text
$ printf "find all rust source files" | watn
find . -type f -name "*.rs"
```

### Model tiers

```text
$ watn -2 "list pods with the most memory usage"
$ watn -3 "debug this strace"
$ watn --model anthropic/claude-sonnet-7 "convert flac to mp3 recursively"
```

### Execute mode

```text
$ watn -x "remove all .bak files"
  rm *.bak
  Execute? [y/N]
```

### Flags

| Flag | Description |
|---|---|
| `-1` | Small/fast model tier (default) |
| `-2` | Balanced model tier |
| `-3` | Thinking/reasoning model tier |
| `--model <NAME>` | Explicit model override |
| `-x` | Prompt for confirmation before executing the command |
| `--version` | Print version and exit |

## Configuration

Layered config stack: built-in defaults → XDG config file → environment
variables → CLI flags. Config file is TOML.

Run `watn provider` in a terminal to configure an OpenAI-compatible endpoint.
OpenRouter is the default endpoint. Use `OPENROUTER_API_KEY` or choose another
environment variable to keep the credential out of the config file; the saved
config stores a reference such as `${OPENROUTER_API_KEY}`.

## Development

```
cargo test --test features_runner
```

## Coverage

[![Coverage: 47.2%](https://img.shields.io/badge/coverage-47.2%25-brightgreen)](coverage/non-e2e-cobertura.xml)

Coverage runs the acceptance scenarios against the instrumented `watn` executable. The latest non-E2E run covers `1095/2320` lines (`47.2%`); the E2E subset covers `1804/2320` lines (`77.8%`).

```sh
mkdir -p coverage
cargo llvm-cov clean --workspace
cargo llvm-cov run --bin watn --no-report -- --version
cargo llvm-cov test --no-clean --test features_runner \
  --cobertura --output-path coverage/non-e2e-cobertura.xml \
  -- --tags 'not @wip and not @e2e'
```

## License

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue)](LICENSE)

GPL-3.0-or-later

<!-- givn:begin:coverage -->
Line coverage: 63% (2964/4738)
Branch coverage: 0% (0/0)
<!-- givn:end:coverage -->
