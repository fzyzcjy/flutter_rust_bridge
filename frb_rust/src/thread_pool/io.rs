use std::panic::{AssertUnwindSafe, RefUnwindSafe};

pub trait BaseThreadPool: RefUnwindSafe {
    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}

#[derive(Debug, Default)]
pub struct SimpleThreadPool(pub AssertUnwindSafe<threadpool::ThreadPool>);

impl BaseThreadPool for SimpleThreadPool {
    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.0.execute(job)
    }
}
