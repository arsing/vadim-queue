use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures_core::Stream;

use crate::{Reader, WakerStrategy, waker_strategy::WakerStrategyWakerSet as _};

#[cfg(any(
    feature = "lock_lock_build",
    feature = "swap_lock_build",
))]
pub(super) mod lock;

#[cfg(any(
    feature = "lock_swap_build",
    feature = "swap_swap_build",
))]
pub(super) mod swap;

#[allow(clippy::module_name_repetitions)]
pub trait AtomicNode<T, WS> where WS: WakerStrategy {
    fn pending() -> Arc<Self>;
    fn load(&self, cx_and_waker_proxy: Option<(&mut Context<'_>, &mut WS::WakerProxy)>) -> Poll<(T, Arc<Self>)>;
    fn store(&self, value: T, next: Arc<Self>);
}

enum Node<T, TInner, WS> where WS: WakerStrategy {
    Ready {
        value: T,
        next: Arc<TInner>,
    },
    Pending {
        waker_set: WS::WakerSet,
    },
}

impl<T, TInner, WS> Node<T, TInner, WS> where T: Clone, WS: WakerStrategy {
    fn load(&self, cx_and_waker_proxy: Option<(&mut Context<'_>, &mut WS::WakerProxy)>) -> Poll<(T, Arc<TInner>)> {
        match self {
            Node::Ready { value, next } => Poll::Ready((value.clone(), next.clone())),
            Node::Pending { waker_set } => {
                if let Some((cx, waker_proxy)) = cx_and_waker_proxy {
                    waker_set.register(cx, waker_proxy);
                }
                Poll::Pending
            },
        }
    }
}

pub struct ReaderImpl<T, TInner, WS> where WS: WakerStrategy {
    element: PhantomData<fn() -> T>,
    head: Arc<TInner>,
    waker_proxy: WS::WakerProxy,
}

impl<T, TInner, WS> ReaderImpl<T, TInner, WS> where WS: WakerStrategy {
    pub(super) fn new(head: Arc<TInner>) -> Self {
        ReaderImpl {
            element: Default::default(),
            head,
            waker_proxy: Default::default(),
        }
    }
}

impl<T, TInner, WS> ReaderImpl<T, TInner, WS> where TInner: AtomicNode<T, WS>, WS: WakerStrategy {
    fn read_inner(&mut self, cx: Option<&mut Context<'_>>) -> Poll<T> {
        self.head.load(cx.map(|cx| (cx, &mut self.waker_proxy))).map(|(value, next)| {
            self.head = next;
            self.waker_proxy = Default::default();
            value
        })
    }
}

impl<T, TInner, WS> Reader<T> for ReaderImpl<T, TInner, WS> where TInner: AtomicNode<T, WS>, WS: WakerStrategy {
    fn read(&mut self) -> Poll<T> {
        self.read_inner(None)
    }
}

impl<T, TInner, WS> Stream for ReaderImpl<T, TInner, WS> where TInner: AtomicNode<T, WS>, WS: WakerStrategy {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.read_inner(Some(cx)).map(Some)
    }
}
