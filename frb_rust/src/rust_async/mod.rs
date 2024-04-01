#[cfg(feature = "rust-async")]
#[cfg(not(target_family = "wasm"))]
mod io;
use futures::Future;
#[cfg(feature = "rust-async")]
#[cfg(not(target_family = "wasm"))]
pub use io::*;

#[cfg(feature = "rust-async")]
#[cfg(target_family = "wasm")]
mod web;
#[cfg(feature = "rust-async")]
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(feature = "rust-async"))]
mod stub;
#[cfg(not(feature = "rust-async"))]
pub use stub::*;

pub trait BaseAsyncRuntime {
    type JoinHandle<O>;

    #[cfg(feature = "rust-async")]
    #[cfg(not(target_family = "wasm"))]
    fn spawn<F>(&self, future: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;

    #[cfg(feature = "rust-async")]
    #[cfg(not(target_family = "wasm"))]
    fn spawn_blocking<F>(&self, func: F) -> Self::JoinHandle<F::Output>
    where
        F: FnOnce() + Send + 'static,
        F::Output: Send + 'static;

    #[cfg(feature = "rust-async")]
    #[cfg(not(target_family = "wasm"))]
    fn block_on<F: Future>(&self, future: F) -> F::Output;

    #[cfg(feature = "rust-async")]
    #[cfg(target_family = "wasm")]
    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + 'static;
}
