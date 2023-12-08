use crate::codec::BaseCodec;
use crate::generalized_isolate::IntoDart;
use crate::handler::handler::{TaskContext, TaskInfo, TaskRetFutTrait};
use crate::misc::into_into_dart::IntoIntoDart;
use crate::rust2dart::wire_sync_return_src::Rust2DartMessageTrait;
use allo_isolate::ffi::DartCObject;
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
        TaskFn: FnOnce(TaskContext<Rust2DartCodec>) -> Result<DartCObject, DartCObject>
            + Send
            + UnwindSafe
            + 'static,
        Rust2DartCodec: BaseCodec;

    /// Executes a synchronous Rust function
    fn execute_sync<Rust2DartCodec, SyncTaskFn>(
        &self,
        task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> Result<Rust2DartCodec::Rust2DartMessage, Rust2DartCodec::Rust2DartMessage>
    where
        SyncTaskFn: FnOnce() -> Result<Rust2DartCodec::Rust2DartMessage, Rust2DartCodec::Rust2DartMessage>
            + UnwindSafe,
        Rust2DartCodec: BaseCodec;

    #[cfg(feature = "rust-async")]
    fn execute_async<Rust2DartCodec, TaskFn, TaskRetFut>(&self, task_info: TaskInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskContext<Rust2DartCodec>) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRetFut:
            Future<Output = Result<DartCObject, DartCObject>> + TaskRetFutTrait + UnwindSafe,
        Rust2DartCodec: BaseCodec;
}
