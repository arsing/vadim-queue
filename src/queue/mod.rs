use std::task::Poll;

use futures_core::Stream;

mod strategic;
pub use strategic::*;

pub trait Queue<T>: Default {
    type Reader: Reader<T>;

    fn new() -> Self;
    fn reader(&self) -> Self::Reader;
    fn append(&self, value: T);
}

pub trait Reader<T>: Stream<Item = T> {
    fn read(&mut self) -> Poll<T>;
}
