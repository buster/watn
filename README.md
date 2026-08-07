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

## License

TBD
