pub trait BaseAsyncRuntime {}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleAsyncRuntime;

impl BaseAsyncRuntime for SimpleAsyncRuntime {}
