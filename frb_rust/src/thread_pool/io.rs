pub use threadpool::ThreadPool;

pub trait BaseThreadPool {
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
