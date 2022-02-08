use std::sync::Arc;

// use arc_swap::ArcSwap; // 5x slower than the RwLock strategy
type ArcSwap<T> = arc_swap::ArcSwapAny<Arc<T>, std::sync::RwLock<()>>;

use crate::{WakerStrategy, queue::strategic::node};
use super::AtomicNodeCell;

pub struct AtomicNodeCellImpl<TInner>(ArcSwap<TInner>);

impl<T, TInner, WS> AtomicNodeCell<T, TInner, WS> for AtomicNodeCellImpl<TInner> where TInner: node::AtomicNode<T, WS>, WS: WakerStrategy {
    fn new() -> Self {
        AtomicNodeCellImpl(ArcSwap::new(TInner::pending()))
    }

    fn load(&self) -> Arc<TInner> {
        self.0.load_full()
    }

    fn swap(&self, new_node: Arc<TInner>) -> Arc<TInner> {
        self.0.swap(new_node)
    }
}
