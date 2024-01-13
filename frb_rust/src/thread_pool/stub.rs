pub trait BaseThreadPool {}

#[derive(Debug, Default)]
pub struct SimpleThreadPool;

impl BaseThreadPool for SimpleThreadPool {}
