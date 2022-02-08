#!/bin/bash

# Ref: https://doc.rust-lang.org/rustc/profile-guided-optimization.html

set -euo pipefail

queue_feature="$1"
waker_strategy_feature="$2"

rm -rf ./target/{release,x86_64-unknown-linux-gnu}

profdata="$(mktemp --directory --tmpdir 'vadim-queue-pgo.XXXXXXXXXX')"
profdata_merged="$(mktemp --tmpdir 'vadim-queue-pgo-merged.XXXXXXXXXX')"
trap "rm -rf '$profdata' '$profdata_merged'" EXIT

RUSTFLAGS="-Cprofile-generate=${profdata}" cargo build \
    --release --target x86_64-unknown-linux-gnu \
    --features "${queue_feature}_default,${waker_strategy_feature}_default" \
    --bin vadim-queue

BENCH=1 ./target/x86_64-unknown-linux-gnu/release/vadim-queue

"$(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata" merge -o "$profdata_merged" "$profdata"

RUSTFLAGS="-Cprofile-use=${profdata_merged} -Cllvm-args=-pgo-warn-missing-function" cargo build \
    --release --target x86_64-unknown-linux-gnu \
    --features "${queue_feature}_default,${waker_strategy_feature}_default" \
    --bin vadim-queue

cp ./target{/x86_64-unknown-linux-gnu,}/release/vadim-queue
