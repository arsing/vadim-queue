use std::task::{Context, Waker};

use parking_lot::Mutex;

use crate::WakerStrategy;
use super::{WakerStrategyWakerSet, WakerStrategyWakerProxy};

pub struct WakerStrategyImpl;

impl WakerStrategy for WakerStrategyImpl {
    type WakerSet = WakerSetImpl;
    type WakerProxy = Option<usize>;
}

impl WakerStrategyWakerSet<WakerStrategyImpl> for WakerSetImpl {
    fn register(&self, cx: &mut Context<'_>, proxy: &mut Option<usize>) {
        let mut guard = self.0.lock();
        if let Some(waker_index) = *proxy {
            guard[waker_index] = cx.waker().clone();
        }
        else {
            guard.push(cx.waker().clone());
            *proxy = Some(guard.len());
        }
    }
}

impl WakerStrategyWakerProxy<WakerStrategyImpl> for Option<usize> {}

#[derive(Default)]
pub struct WakerSetImpl(Mutex<Vec<Waker>>);

impl Drop for WakerSetImpl {
    fn drop(&mut self) {
        for waker in std::mem::take(self.0.get_mut()) {
            waker.wake();
        }
    }
}
