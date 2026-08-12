#!/bin/sh
set -eu

root=$(mktemp -d /tmp/watn-transport.XXXXXX)
trap 'rm -rf "$root"' EXIT

cargo build --locked --bin watn
cp target/debug/watn "$root/default-debug"
cargo build --locked --features test-support --bin watn
cp target/debug/watn "$root/test-support-debug"

if [ "${1:-}" = "--e2e" ]; then
    tags='@e2e and not @wip'
else
    tags='not @wip and not @e2e'
fi

WATN_DEFAULT_DEBUG_BIN="$root/default-debug" \
WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" \
    cargo test --locked --test features_runner --features test-support -- --tags "$tags"
