use std::future::Future;
use std::panic::{RefUnwindSafe, UnwindSafe};
use allo_isolate::IntoDart;

/// An executor model for Rust functions.
///
/// For example, the default model is [ThreadPoolExecutor]
/// which runs each function in a separate thread.
pub trait Executor: RefUnwindSafe {
    /// Executes a Rust function and transforms its return value into a Dart-compatible
    /// value, i.e. types that implement [`IntoDart`].
    fn execute<TaskFn, TaskRetDirect, TaskRetData, Er>(&self, wrap_info: WrapInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskCallback) -> Result<TaskRetDirect, Er> + Send + UnwindSafe + 'static,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static;

    /// Executes a synchronous Rust function
    fn execute_sync<SyncTaskFn, TaskRetDirect, TaskRetData, Er>(
        &self,
        wrap_info: WrapInfo,
        sync_task: SyncTaskFn,
    ) -> Result<TaskRetDirect, Er>
    where
        SyncTaskFn: FnOnce() -> Result<TaskRetDirect, Er> + UnwindSafe,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static;

    #[cfg(feature = "rust-async")]
    fn execute_async<TaskFn, TaskRetFut, TaskRetDirect, TaskRetData, Er>(&self, wrap_info: WrapInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskCallback) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRetFut: Future<Output = Result<TaskRetDirect, Er>> + TaskRetFutTrait + UnwindSafe,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static;
}
