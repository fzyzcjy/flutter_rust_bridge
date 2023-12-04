use std::panic::RefUnwindSafe;
pub use threadpool::ThreadPool;

pub trait BaseThreadPool: RefUnwindSafe {
    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}

impl BaseThreadPool for ThreadPool {
    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.execute(job)
    }
}
