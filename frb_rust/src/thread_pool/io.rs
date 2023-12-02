use std::panic::AssertUnwindSafe;

pub use threadpool::ThreadPool;

// TODO remove `AssertUnwindSafe` after the Rust bug is fixed:
// https://github.com/rust-lang/rust/issues/118009
pub type ThreadPoolWrapped = AssertUnwindSafe<ThreadPool>;
