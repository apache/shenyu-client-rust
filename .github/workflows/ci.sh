#!/bin/bash

set -ex

CARGO=cargo
if [[ "${CROSS}" = "1" ]]; then
    export CARGO_NET_RETRY=5
    export CARGO_NET_TIMEOUT=10

    cargo install cross
    CARGO=cross
fi

# If a test crashes, we want to know which one it was.
export RUST_TEST_THREADS=1
export RUST_BACKTRACE=1

"${CARGO}" test --target "${TARGET}"
"${CARGO}" test --target "${TARGET}" --release

"${CARGO}" test --target "${TARGET}" --all-features
"${CARGO}" test --target "${TARGET}" --all-features --release

"${CARGO}" run --package examples --all-features --bin axum_example || true
"${CARGO}" run --package examples --all-features --bin actix_web_example || true
