use crate::codec::CodecTrait;
use crate::handler::handler::{TaskContext, TaskInfo, TaskRetFutTrait};
use std::future::Future;
use std::panic::{RefUnwindSafe, UnwindSafe};

/// An executor model for Rust functions.
///
/// For example, the default model is [SimpleExecutor]
/// which runs each function in a separate thread.
pub trait Executor: RefUnwindSafe {
    /// Executes a Rust function and transforms its return value into a Dart-compatible
    /// value, i.e. types that implement [`IntoDart`].
    fn execute_normal<Rust2DartCodec, TaskFn>(&self, task_info: TaskInfo, task: TaskFn)
    where
        TaskFn: FnOnce(
                TaskContext<Rust2DartCodec>,
            ) -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>
            + Send
            + UnwindSafe
            + 'static,
        Rust2DartCodec: CodecTrait;

    /// Executes a synchronous Rust function
    fn execute_sync<Rust2DartCodec, SyncTaskFn>(
        &self,
        task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> Rust2DartCodec::Message
    where
        SyncTaskFn:
            FnOnce() -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message> + UnwindSafe,
        Rust2DartCodec: CodecTrait;

    #[cfg(feature = "rust-async")]
    fn execute_async<Rust2DartCodec, TaskFn, TaskRetFut>(&self, task_info: TaskInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskContext<Rust2DartCodec>) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRetFut: Future<Output = Result<Rust2DartCodec::Message, Rust2DartCodec::Message>>
            + TaskRetFutTrait
            + UnwindSafe,
        Rust2DartCodec: CodecTrait;
}
