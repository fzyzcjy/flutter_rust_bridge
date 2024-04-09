// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// This file mainly serves as compilation test

#![allow(unused_variables)]
#![allow(dead_code)]

use flutter_rust_bridge::for_generated::*;
use flutter_rust_bridge::*;
use std::future::Future;

pub type MyUnmodifiedHandler = SimpleHandler<
    SimpleExecutor<NoOpErrorListener, SimpleThreadPool, SimpleAsyncRuntime>,
    NoOpErrorListener,
>;

#[derive(Clone, Copy)]
pub struct MyCustomErrorListener;

impl ErrorListener for MyCustomErrorListener {
    fn on_error(&self, error: HandlerError) {
        unimplemented!()
    }
}

pub struct MyCustomThreadPool;

impl BaseThreadPool for MyCustomThreadPool {
    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unimplemented!()
    }
}

pub struct MyCustomAsyncRuntime;

impl BaseAsyncRuntime for MyCustomAsyncRuntime {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        unimplemented!()
    }
}

pub type MyCustomSimpleHandler = SimpleHandler<
    SimpleExecutor<MyCustomErrorListener, MyCustomThreadPool, MyCustomAsyncRuntime>,
    MyCustomErrorListener,
>;

pub struct MyCustomExecutor;

impl Executor for MyCustomExecutor {
    fn execute_normal<Rust2DartCodec, TaskFn>(&self, task_info: TaskInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskContext) -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>
            + Send
            + 'static,
        Rust2DartCodec: BaseCodec,
    {
        unimplemented!()
    }

    fn execute_sync<Rust2DartCodec, SyncTaskFn>(
        &self,
        task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> Rust2DartCodec::Message
    where
        SyncTaskFn: FnOnce() -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>,
        Rust2DartCodec: BaseCodec,
    {
        unimplemented!()
    }

    fn execute_async<Rust2DartCodec, TaskFn, TaskRetFut>(&self, task_info: TaskInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskContext) -> TaskRetFut + Send + 'static,
        TaskRetFut: Future<Output = Result<Rust2DartCodec::Message, Rust2DartCodec::Message>>
            + TaskRetFutTrait,
        Rust2DartCodec: BaseCodec,
    {
        unimplemented!()
    }
}

pub type MyCustomHandlerWithCustomExecutor = SimpleHandler<MyCustomExecutor, MyCustomErrorListener>;

pub struct MyFullyCustomHandler;

impl Handler for MyFullyCustomHandler {
    fn wrap_normal<Rust2DartCodec, PrepareFn, TaskFn>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn,
        TaskFn: FnOnce(TaskContext) -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>
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
        TaskFn: FnOnce(TaskContext) -> TaskRetFut + Send + 'static,
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
        MyCustomSimpleHandler::new(
            SimpleExecutor::new(
                MyCustomErrorListener,
                MyCustomThreadPool,
                MyCustomAsyncRuntime,
            ),
            MyCustomErrorListener,
        );
        MyCustomHandlerWithCustomExecutor::new(MyCustomExecutor, MyCustomErrorListener);
    }
}
