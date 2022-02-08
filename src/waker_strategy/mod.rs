#[cfg(feature = "waker_strategy_event_listener_build")]
mod event_listener;
#[cfg(feature = "waker_strategy_event_listener_default")]
pub use self::event_listener::WakerStrategyImpl;

#[cfg(feature = "waker_strategy_mutex_vec_waker_build")]
mod mutex_vec_waker;
#[cfg(feature = "waker_strategy_mutex_vec_waker_default")]
pub use mutex_vec_waker::WakerStrategyImpl;

pub trait WakerStrategy {
    type WakerSet: WakerStrategyWakerSet<Self>;
    type WakerProxy: WakerStrategyWakerProxy<Self>;
}

mod private {
    use std::task::Context;

    use crate::WakerStrategy;

    pub trait WakerStrategyWakerSet<WS>: Default where WS: WakerStrategy + ?Sized {
        fn register(&self, cx: &mut Context<'_>, proxy: &mut WS::WakerProxy);
    }

    pub trait WakerStrategyWakerProxy<WS>: Default + Unpin where WS: WakerStrategy + ?Sized {}
}
pub(crate) use private::{WakerStrategyWakerSet, WakerStrategyWakerProxy};
