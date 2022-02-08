use std::marker::PhantomData;

use crate::{Queue, WakerStrategy};

mod node;

mod node_cell;

#[allow(clippy::module_name_repetitions)]
pub struct StrategicQueue<T, TInner, TOuter, WS> {
    element: PhantomData<T>,
    inner: PhantomData<TInner>,
    outer: TOuter,
    waker_strategy: PhantomData<WS>,
}

impl<T, TInner, TOuter, WS> Queue<T> for StrategicQueue<T, TInner, TOuter, WS>
where
    TInner: node::AtomicNode<T, WS>,
    TOuter: node_cell::AtomicNodeCell<T, TInner, WS>,
    WS: WakerStrategy,
{
    type Reader = node::ReaderImpl<T, TInner, WS>;

    fn new() -> Self {
        StrategicQueue {
            element: Default::default(),
            inner: Default::default(),
            outer: TOuter::new(),
            waker_strategy: Default::default(),
        }
    }

    fn reader(&self) -> Self::Reader {
        node::ReaderImpl::new(self.outer.load())
    }

    fn append(&self, value: T) {
        let new_tail = TInner::pending();
        let previous_tail = self.outer.swap(new_tail.clone());
        previous_tail.store(value, new_tail);
    }
}

impl<T, TInner, TOuter, WS> Default for StrategicQueue<T, TInner, TOuter, WS>
where
    TInner: node::AtomicNode<T, WS>,
    TOuter: node_cell::AtomicNodeCell<T, TInner, WS>,
    WS: WakerStrategy,
{
    fn default() -> Self {
        StrategicQueue::new()
    }
}

#[cfg(feature = "lock_lock_build")]
pub type LockLockQueueImpl<T, WS> = StrategicQueue<T, node::lock::AtomicNodeImpl<T, WS>, node_cell::lock::AtomicNodeCellImpl<node::lock::AtomicNodeImpl<T, WS>>, WS>;

#[cfg(feature = "lock_swap_build")]
pub type LockSwapQueueImpl<T, WS> = StrategicQueue<T, node::swap::AtomicNodeImpl<T, WS>, node_cell::lock::AtomicNodeCellImpl<node::swap::AtomicNodeImpl<T, WS>>, WS>;

#[cfg(feature = "swap_lock_build")]
pub type SwapLockQueueImpl<T, WS> = StrategicQueue<T, node::lock::AtomicNodeImpl<T, WS>, node_cell::swap::AtomicNodeCellImpl<node::lock::AtomicNodeImpl<T, WS>>, WS>;

#[cfg(feature = "swap_swap_build")]
pub type SwapSwapQueueImpl<T, WS> = StrategicQueue<T, node::swap::AtomicNodeImpl<T, WS>, node_cell::swap::AtomicNodeCellImpl<node::swap::AtomicNodeImpl<T, WS>>, WS>;
