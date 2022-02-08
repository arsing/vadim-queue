use std::sync::Arc;

#[cfg(any(
    feature = "lock_lock_build",
    feature = "lock_swap_build",
))]
pub(super) mod lock;

#[cfg(any(
    feature = "swap_lock_build",
    feature = "swap_swap_build",
))]
pub(super) mod swap;

#[allow(clippy::module_name_repetitions)]
pub trait AtomicNodeCell<T, TInner, WS> {
    fn new() -> Self;
    fn load(&self) -> Arc<TInner>;
    fn swap(&self, new_tail: Arc<TInner>) -> Arc<TInner>;
}
