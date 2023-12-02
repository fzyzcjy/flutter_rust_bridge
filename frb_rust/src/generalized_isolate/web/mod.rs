mod into_dart;
mod port_like;
mod channel;

pub use into_dart::*;
pub use port_like::*;
pub use channel::*;

#[derive(Debug)]
pub struct ZeroCopyBuffer<T>(pub T);

impl<T> ZeroCopyBuffer<Vec<T>> {
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}
