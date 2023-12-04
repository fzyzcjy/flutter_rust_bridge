use std::panic::{AssertUnwindSafe, RefUnwindSafe};
pub use threadpool::ThreadPool;

pub trait BaseThreadPool: RefUnwindSafe {
    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}

impl BaseThreadPool for AssertUnwindSafe<ThreadPool> {
    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        ThreadPool::execute(self, job)
    }
}
