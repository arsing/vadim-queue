use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use event_listener::{Event, EventListener};

use crate::WakerStrategy;
use super::{WakerStrategyWakerSet, WakerStrategyWakerProxy};

pub struct WakerStrategyImpl;

impl WakerStrategy for WakerStrategyImpl {
    type WakerSet = WakerSetImpl;
    type WakerProxy = Option<EventListener>;
}

impl WakerStrategyWakerSet<WakerStrategyImpl> for WakerSetImpl {
    fn register(&self, cx: &mut Context<'_>, proxy: &mut Option<EventListener>) {
        let listener = proxy.get_or_insert_with(|| self.0.listen());
        // Since the WakerSet only gets signaled when it's drop()'ped, and we have the WakerSet here,
        // it can't possibly be signaled right now. So we need to call `poll()` to register the waker with the set,
        // but we know that it's going to return Pending.
        match Pin::new(listener).poll(cx) {
            Poll::Ready(()) => unsafe { std::hint::unreachable_unchecked(); },
            Poll::Pending => (),
        }
    }
}

impl WakerStrategyWakerProxy<WakerStrategyImpl> for Option<EventListener> {}

#[derive(Default)]
pub struct WakerSetImpl(Event);

impl Drop for WakerSetImpl {
    fn drop(&mut self) {
        self.0.notify(usize::max_value());
    }
}
