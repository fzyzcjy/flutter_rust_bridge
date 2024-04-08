use flutter_rust_bridge::for_generated::*;

pub type MyUnmodifiedHandler = SimpleHandler<
    SimpleExecutor<NoOpErrorListener, SimpleThreadPool, SimpleAsyncRuntime>,
    NoOpErrorListener,
>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_create_custom_handlers() {
        MyUnmodifiedHandler::new_simple(Default::default());
    }
}
