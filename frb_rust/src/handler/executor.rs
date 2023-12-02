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
    fn execute<TaskFn, TaskRet, D, Er>(&self, wrap_info: WrapInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskCallback) -> Result<TaskRet, Er> + Send + UnwindSafe + 'static,
        TaskRet: IntoIntoDart<D>,
        D: IntoDart,
        Er: IntoDart + 'static;

    /// Executes a synchronous Rust function
    fn execute_sync<SyncTaskFn, TaskRet, D, Er>(
        &self,
        wrap_info: WrapInfo,
        sync_task: SyncTaskFn,
    ) -> Result<TaskRet, Er>
    where
        SyncTaskFn: FnOnce() -> Result<TaskRet, Er> + UnwindSafe,
        TaskRet: IntoIntoDart<D>,
        D: IntoDart,
        Er: IntoDart + 'static;

    #[cfg(feature = "rust-async")]
    fn execute_async<TaskFn, TaskRet, TaskRetFut, D, Er>(&self, wrap_info: WrapInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskCallback) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRet: IntoIntoDart<D>,
        TaskRetFut: Future<Output = Result<TaskRet, Er>> + TaskRetFutTrait + UnwindSafe,
        D: IntoDart,
        Er: IntoDart + 'static;
}
