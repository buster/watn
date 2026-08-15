#!/bin/sh
set -eu

root=$(mktemp -d /tmp/watn-transport.XXXXXX)
trap 'rm -rf "$root"' EXIT

e2e=0
if [ "${1:-}" = "--e2e" ]; then
    e2e=1
    shift
fi

scenario_name=""
if [ "${1:-}" = "--name" ]; then
    scenario_name=${2:?"--name requires a scenario name"}
    shift 2
fi

cargo build --locked --bin watn
cp target/debug/watn "$root/default-debug"
cargo build --locked --features test-support --bin watn
cp target/debug/watn "$root/test-support-debug"

if [ "$e2e" -eq 1 ]; then
    tags='@e2e and not @wip'
else
    tags='not @wip and not @e2e'
fi

if [ -n "$scenario_name" ]; then
    WATN_DEFAULT_DEBUG_BIN="$root/default-debug" \
    WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" \
        cargo test --locked --test features_runner --features test-support -- --name "$scenario_name" "$@"
else
    WATN_DEFAULT_DEBUG_BIN="$root/default-debug" \
    WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" \
        cargo test --locked --test features_runner --features test-support -- --tags "$tags" "$@"
fi
