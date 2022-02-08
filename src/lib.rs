#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::default_trait_access,
    clippy::must_use_candidate,
    clippy::too_many_lines,
    clippy::type_complexity,
)]

mod queue;
pub use queue::{Queue, Reader};
#[cfg(feature = "lock_lock_build")]
pub use queue::LockLockQueueImpl;
#[cfg(feature = "lock_lock_default")]
pub use LockLockQueueImpl as QueueImpl;
#[cfg(feature = "lock_swap_build")]
pub use queue::LockSwapQueueImpl;
#[cfg(feature = "lock_swap_default")]
pub use LockSwapQueueImpl as QueueImpl;
#[cfg(feature = "swap_lock_build")]
pub use queue::SwapLockQueueImpl;
#[cfg(feature = "swap_lock_default")]
pub use SwapLockQueueImpl as QueueImpl;
#[cfg(feature = "swap_swap_build")]
pub use queue::SwapSwapQueueImpl;
#[cfg(feature = "swap_swap_default")]
pub use SwapSwapQueueImpl as QueueImpl;

mod waker_strategy;
pub use waker_strategy::WakerStrategy;
#[cfg(any(
    feature = "waker_strategy_event_listener_default",
    feature = "waker_strategy_mutex_vec_waker_default",
))]
pub use waker_strategy::WakerStrategyImpl;
