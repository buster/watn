# Watn? ¯\\\_(ツ)\_/¯

![Crates.io Version](https://img.shields.io/crates/v/watn)
<!-- givn:begin:coverage-badge -->
[![Line Coverage: 90%](https://img.shields.io/badge/line%20coverage-90%25-brightgreen)](coverage/cobertura-coverage.xml)
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

## Prerequisites

- Rust (latest stable)
- An OpenAI-compatible API endpoint

## Usage

```text
watn find all files modified in the last day
```

Pass a question as a positional argument or via stdin. watn prints generated
command to stdout and writes final metadata (model, tokens/s, cost) to stderr.
Reasoning is printed to stderr only after successful completion when `-v` is set.

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

`cargo build --release` produces one executable for the selected target
and are pushed to crates.io.

## Development

The acceptance runner uses separate default and `test-support` binaries so
transport tests cannot affect normal requests. Run the non-E2E suite with the
same setup used by CI:

```sh
root=$(mktemp -d /tmp/watn-transport.XXXXXX)
trap 'rm -rf "$root"' EXIT
cargo build --locked --bin watn
cp target/debug/watn "$root/default-debug"
cargo build --locked --features test-support --bin watn
cp target/debug/watn "$root/test-support-debug"
WATN_DEFAULT_DEBUG_BIN="$root/default-debug" \
  WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" \
  cargo test --locked --test features_runner --features test-support -- \
  --tags 'not @wip and not @e2e'
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

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue)](LICENSE)

GPL-3.0-or-later

<!-- givn:begin:coverage -->
## Coverage

Merged report: [coverage/cobertura-coverage.xml](coverage/cobertura-coverage.xml)

Line coverage: 90% (9321/10302)
Branch coverage: n/a (0/0)

Reproduce the result:

```sh
./measure-coverage.sh
./merge-coverages.sh
```
<!-- givn:end:coverage -->
