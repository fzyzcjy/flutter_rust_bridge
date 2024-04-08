/// This file mainly serves as compilation test

#![allow(unused_variables)]

use flutter_rust_bridge::for_generated::*;
use flutter_rust_bridge::*;
use std::future::Future;

pub type MyUnmodifiedHandler = SimpleHandler<
    SimpleExecutor<NoOpErrorListener, SimpleThreadPool, SimpleAsyncRuntime>,
    NoOpErrorListener,
>;

pub struct MyFullyCustomHandler;

impl Handler for MyFullyCustomHandler {
    fn wrap_normal<Rust2DartCodec, PrepareFn, TaskFn>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn,
        TaskFn: FnOnce(
                TaskContext<Rust2DartCodec>,
            ) -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>
            + Send
            + 'static,
        Rust2DartCodec: BaseCodec,
    {
        unimplemented!()
    }

    fn wrap_sync<Rust2DartCodec, SyncTaskFn>(
        &self,
        task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> <Rust2DartCodec::Message as Rust2DartMessageTrait>::WireSyncRust2DartType
    where
        SyncTaskFn: FnOnce() -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>,
        Rust2DartCodec: BaseCodec,
    {
        unimplemented!()
    }

    fn wrap_async<Rust2DartCodec, PrepareFn, TaskFn, TaskRetFut>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn,
        TaskFn: FnOnce(TaskContext<Rust2DartCodec>) -> TaskRetFut
            + Send
            + 'static,
        TaskRetFut: Future<Output = Result<Rust2DartCodec::Message, Rust2DartCodec::Message>>
            + TaskRetFutTrait,
        Rust2DartCodec: BaseCodec,
    {
        unimplemented!()
    }

    fn dart_fn_invoke(
        &self,
        dart_fn: DartOpaque,
        args: Vec<DartAbi>,
    ) -> DartFnFuture<Dart2RustMessageSse> {
        unimplemented!()
    }

    fn dart_fn_handle_output(&self, call_id: i32, message: Dart2RustMessageSse) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_create_custom_handlers() {
        MyUnmodifiedHandler::new_simple(Default::default());
    }
}
