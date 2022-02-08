use std::sync::Arc;

use parking_lot::RwLock;

use crate::{WakerStrategy, queue::strategic::node};
use super::AtomicNodeCell;

pub struct AtomicNodeCellImpl<TInner>(RwLock<Arc<TInner>>);

impl<T, TInner, WS> AtomicNodeCell<T, TInner, WS> for AtomicNodeCellImpl<TInner> where TInner: node::AtomicNode<T, WS>, WS: WakerStrategy {
    fn new() -> Self {
        AtomicNodeCellImpl(RwLock::new(TInner::pending()))
    }

    fn load(&self) -> Arc<TInner> {
        self.0.read().clone()
    }

    fn swap(&self, new_node: Arc<TInner>) -> Arc<TInner> {
        std::mem::replace(&mut *self.0.write(), new_node)
    }
}
