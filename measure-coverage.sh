#!/bin/sh
set -eu

root=$(mktemp -d /tmp/watn-transport-cov.XXXXXX)
trap 'rm -rf "$root"' EXIT
non_e2e_path=${GIVN_COVERAGE_NON_E2E_PATH:-coverage/non-e2e-cobertura.xml}
e2e_path=${GIVN_COVERAGE_E2E_PATH:-coverage/e2e-cobertura.xml}

measure() {
    output_path=$1
    tags=$2
    rm -rf coverage/profraw
    mkdir -p coverage/profraw "$(dirname "$output_path")"
    cargo llvm-cov clean --workspace
    LLVM_PROFILE_FILE=coverage/profraw/%p-%m.profraw \
        cargo llvm-cov run --bin watn --no-report -- --version >/dev/null
    cp target/llvm-cov-target/debug/watn "$root/default-debug"
    LLVM_PROFILE_FILE=coverage/profraw/%p-%m.profraw \
        cargo llvm-cov run --features test-support --bin watn --no-report -- --version >/dev/null
    cp target/llvm-cov-target/debug/watn "$root/test-support-debug"
    LLVM_PROFILE_FILE=coverage/profraw/%p-%m.profraw \
        cargo llvm-cov test --no-clean --lib --features test-support -- --test-threads=1
    LLVM_PROFILE_FILE=coverage/profraw/%p-%m.profraw \
        WATN_DEFAULT_DEBUG_BIN="$root/default-debug" \
        WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" \
        cargo llvm-cov test --no-clean --test features_runner --features test-support \
        --no-default-ignore-filename-regex \
        --ignore-filename-regex '(/\.cargo/registry/|/rustc/|/target/)' \
        --cobertura --output-path "$output_path" -- --tags "$tags"
}

measure "$non_e2e_path" 'not @wip and not @e2e and not @givn.removed'
measure "$e2e_path" '@e2e and not @wip and not @givn.removed'
