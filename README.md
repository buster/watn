# Watn? ¯\\\_(ツ)\_/¯

<!-- givn:begin:coverage-badge -->
[![Line Coverage: 91%](https://img.shields.io/badge/line%20coverage-91%25-brightgreen)](coverage/cobertura-coverage.xml)
[![Branch Coverage: n/a](https://img.shields.io/badge/branch%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)
<!-- givn:end:coverage-badge -->

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

## Quick start

```
git clone <repo>
cd <project>
cargo run
```

## Prerequisites

- Rust (latest stable)
- An OpenAI-compatible API endpoint

## Usage

```text
watn find all files modified in the last day
```

Pass a question as a positional argument or via stdin. watn streams generated
command content incrementally to stdout and writes final metadata (model,
tokens/s, cost) to stderr. Reasoning is buffered during the stream and is
printed to stderr only after successful completion when `-v` is set.

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
variables → CLI flags. Config is TOML at `$XDG_CONFIG_HOME/watn/config.toml`
(normally `~/.config/watn/config.toml`); watn does not use an XDG data
directory.

Run `watn provider` in a terminal to configure an OpenAI-compatible endpoint.
OpenRouter is the default endpoint. Use `OPENROUTER_API_KEY` or choose another
environment variable to keep the credential out of the config file; the saved
config stores a reference such as `${OPENROUTER_API_KEY}`.

The interactive `watn setup`, `watn provider`, and `watn models` commands share
the five-page SetupWizard. On model pages, `Ctrl-R` toggles focus between the
model table and reasoning strength.

## Release artifacts

`cargo build --release` produces one executable for the selected target. Release
artifacts are dynamically linked on the verified host and require that target's
runtime libraries. On Linux, inspect them with `file target/release/watn` and
`ldd target/release/watn`; on macOS, use `otool -L`. The project does not claim a
universal static deployment artifact.

## Development

```
./run-tests.sh
./run-tests.sh --e2e
./measure-coverage.sh
./merge-coverages.sh
```

## Coverage

Coverage runs both acceptance subsets against instrumented `watn` executables.
`measure-coverage.sh` writes the two source reports and `merge-coverages.sh`
writes the merged report used by the badges and the managed summary below.

```sh
./measure-coverage.sh
./merge-coverages.sh
```

## License

GPL-3.0-or-later

<!-- givn:begin:coverage -->
## Coverage

Merged report: [coverage/cobertura-coverage.xml](coverage/cobertura-coverage.xml)

Line coverage: 91% (8115/8940)
Branch coverage: n/a (0/0)

Reproduce the result:

```sh
./measure-coverage.sh
./merge-coverages.sh
```
<!-- givn:end:coverage -->
