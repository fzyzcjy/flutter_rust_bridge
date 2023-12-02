pub use threadpool::ThreadPool;

/// A normal thread pool, except that, only a single thread will touch its methods like `execute`.
/// For example, if thread pool's `execute` is only executed from Dart's synchronous call
/// (thus the main thread), then this holds.
pub(crate) struct SingleThreadAccessorThreadPool(threadpool::ThreadPool);
