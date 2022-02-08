#!/bin/bash

set -euo pipefail

queues='lock_lock lock_swap swap_lock swap_swap'
waker_strategies='event_listener mutex_vec_waker'

for queue in $queues; do
    for waker_strategy in $waker_strategies; do
        make -s "QUEUE=$queue" "WAKER_STRATEGY=$waker_strategy" build
    done
done

for queue in $queues; do
    for waker_strategy in $waker_strategies; do
        (
            printf '%s:%s : ' "$queue" "$waker_strategy"

            read -r line
            <<< "$line" awk '{ printf("%s ", $3) }'

            cat
        ) < <(
            /usr/bin/time --format '%P:%M:%S:%U' make -s BENCH=1 "QUEUE=$queue" "WAKER_STRATEGY=$waker_strategy" run |&
            grep -E 'd:|%:'
        )
    done
done
