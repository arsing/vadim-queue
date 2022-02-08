.PHONY: build perf run test


# 1 => true, _ => false
BENCH = 0

# 1 => true, _ => false
RELEASE = 0

ifeq ($(BENCH), 1)
	CARGO_PROFILE = --release
endif

ifeq ($(RELEASE), 1)
	CARGO_PROFILE = --release
endif


# One of:
# - lock_lock
# - lock_swap
# - swap_lock
# - swap_swap
QUEUE =

# One of:
# - event_listener
# - mutex_vec_waker
WAKER_STRATEGY =


build:
	cargo build $(CARGO_PROFILE) --features '$(QUEUE)_default,waker_strategy_$(WAKER_STRATEGY)_default'

perf:
	BENCH=1 cargo flamegraph --features '$(QUEUE)_default,waker_strategy_$(WAKER_STRATEGY)_default'

run:
	cargo run $(CARGO_PROFILE) --features '$(QUEUE)_default,waker_strategy_$(WAKER_STRATEGY)_default'

test:
	cargo clippy --lib \
		--features lock_lock_build \
		--features lock_swap_build \
		--features swap_lock_build \
		--features swap_swap_build \
		--features waker_strategy_event_listener_build \
		--features waker_strategy_mutex_vec_waker_build
