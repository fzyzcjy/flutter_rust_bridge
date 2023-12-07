use crate::codec::BaseCodec;
use crate::generalized_isolate::IntoDart;
use crate::handler::handler::{TaskContext, TaskInfo, TaskRetFutTrait};
use crate::misc::into_into_dart::IntoIntoDart;
use crate::rust2dart::wire_sync_return_src::WireSyncReturnSrc;
use std::future::Future;
use std::panic::{RefUnwindSafe, UnwindSafe};

/// An executor model for Rust functions.
///
/// For example, the default model is [SimpleExecutor]
/// which runs each function in a separate thread.
pub trait Executor: RefUnwindSafe {
    /// Executes a Rust function and transforms its return value into a Dart-compatible
    /// value, i.e. types that implement [`IntoDart`].
    fn execute_normal<Rust2DartCodec, TaskFn, TaskRetDirect, TaskRetData, Er>(
        &self,
        task_info: TaskInfo,
        task: TaskFn,
    ) where
        TaskFn: FnOnce(TaskContext<Rust2DartCodec>) -> Result<TaskRetDirect, Er>
            + Send
            + UnwindSafe
            + 'static,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static,
        Rust2DartCodec: BaseCodec;

    /// Executes a synchronous Rust function
    fn execute_sync<Rust2DartCodec, SyncTaskFn, TaskRetDirect, TaskRetData, Er>(
        &self,
        task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> Result<WireSyncReturnSrc, Er>
    where
        SyncTaskFn: FnOnce() -> Result<TaskRetDirect, Er> + UnwindSafe,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static,
        Rust2DartCodec: BaseCodec;

    #[cfg(feature = "rust-async")]
    fn execute_async<Rust2DartCodec, TaskFn, TaskRetFut, TaskRetDirect, TaskRetData, Er>(
        &self,
        task_info: TaskInfo,
        task: TaskFn,
    ) where
        TaskFn: FnOnce(TaskContext<Rust2DartCodec>) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRetFut: Future<Output = Result<TaskRetDirect, Er>> + TaskRetFutTrait + UnwindSafe,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static,
        Rust2DartCodec: BaseCodec;
}
