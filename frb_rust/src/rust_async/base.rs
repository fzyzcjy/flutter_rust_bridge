pub trait BaseAsyncRuntime {
    type JoinHandle<O>;

    #[cfg(not(target_family = "wasm"))]
    fn spawn<F>(&self, future: F) -> Self::JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static;

    #[cfg(not(target_family = "wasm"))]
    fn spawn_blocking<F>(&self, func: F) -> Self::JoinHandle<F::Output>
    where
        F: FnOnce() + Send + 'static,
        F::Output: Send + 'static;

    #[cfg(not(target_family = "wasm"))]
    fn block_on<F: std::future::Future>(&self, future: F) -> F::Output;

    #[cfg(target_family = "wasm")]
    fn spawn<F>(&self, future: F)
    where
        F: std::future::Future<Output = ()> + 'static;
}
