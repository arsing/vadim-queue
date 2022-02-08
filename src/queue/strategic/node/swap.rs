use std::sync::Arc;
use std::task::{Context, Poll};

// use arc_swap::ArcSwap; // 5x slower than the RwLock strategy
type ArcSwap<T> = arc_swap::ArcSwapAny<Arc<T>, std::sync::RwLock<()>>;

use crate::WakerStrategy;
use super::{AtomicNode, Node};

pub struct AtomicNodeImpl<T, WS>(ArcSwap<Node<T, Self, WS>>) where WS: WakerStrategy;

impl<T, WS> AtomicNode<T, WS> for AtomicNodeImpl<T, WS> where T: Clone, WS: WakerStrategy {
    fn pending() -> Arc<Self> {
        Arc::new(AtomicNodeImpl(ArcSwap::from_pointee(Node::Pending {
            waker_set: Default::default(),
        })))
    }

    fn load(&self, cx_and_waker_proxy: Option<(&mut Context<'_>, &mut WS::WakerProxy)>) -> Poll<(T, Arc<Self>)> {
        self.0.load().load(cx_and_waker_proxy)
    }

    fn store(&self, value: T, next: Arc<Self>) {
        self.0.store(Arc::new(Node::Ready {
            value,
            next,
        }));
    }
}
