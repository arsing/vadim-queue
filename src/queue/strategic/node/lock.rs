use std::sync::Arc;
use std::task::{Context, Poll};

use parking_lot::RwLock;

use crate::WakerStrategy;
use super::{AtomicNode, Node};

pub struct AtomicNodeImpl<T, WS>(RwLock<Node<T, Self, WS>>) where WS: WakerStrategy;

impl<T, WS> AtomicNode<T, WS> for AtomicNodeImpl<T, WS> where T: Clone, WS: WakerStrategy {
    fn pending() -> Arc<Self> {
        Arc::new(AtomicNodeImpl(RwLock::new(Node::Pending {
            waker_set: Default::default(),
        })))
    }

    fn load(&self, cx_and_waker_proxy: Option<(&mut Context<'_>, &mut WS::WakerProxy)>) -> Poll<(T, Arc<Self>)> {
        self.0.read().load(cx_and_waker_proxy)
    }

    fn store(&self, value: T, next: Arc<Self>) {
        *self.0.write() = Node::Ready {
            value,
            next,
        };
    }
}
