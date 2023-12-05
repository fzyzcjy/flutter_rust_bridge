use crate::generalized_isolate::IntoDart;
use crate::misc::into_into_dart::IntoIntoDart;
use crate::platform_types::{MessagePort, WireSyncReturn};
use crate::rust2dart::context::TaskRust2DartContext;
use std::future::Future;
use std::panic::UnwindSafe;

/// Provide your own handler to customize how to execute your function calls, etc.
pub trait Handler {
    /// Prepares the arguments, executes a Rust function and sets up its return value.
    ///
    /// Why separate `PrepareFn` and `TaskFn`: because some things cannot be [`Send`] (e.g. raw
    /// pointers), so those can be done in `PrepareFn`, while the real work is done in `TaskFn` with [`Send`].
    ///
    /// The generated code depends on the fact that `PrepareFn` is synchronous to maintain
    /// correctness, therefore implementors of [`Handler`] must also uphold this property.
    ///
    /// If a Rust function is marked `sync`, it must be called with
    /// [`wrap_sync`](Handler::wrap_sync) instead.
    fn wrap_normal<PrepareFn, TaskFn, TaskRetDirect, TaskRetData, Er>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskContext) -> Result<TaskRetDirect, Er> + Send + UnwindSafe + 'static,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static;

    /// Same as [`wrap`][Handler::wrap], but the Rust function will be called synchronously and
    /// need not implement [Send].
    fn wrap_sync<SyncTaskFn, TaskRetDirect, TaskRetData, Er>(
        &self,
        task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> WireSyncReturn
    where
        SyncTaskFn: FnOnce() -> Result<TaskRetDirect, Er> + UnwindSafe,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static;

    /// Same as [`wrap`][Handler::wrap], but for async Rust.
    #[cfg(feature = "rust-async")]
    fn wrap_async<PrepareFn, TaskFn, TaskRetFut, TaskRetDirect, TaskRetData, Er>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskContext) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRetFut: Future<Output = Result<TaskRetDirect, Er>> + TaskRetFutTrait + UnwindSafe,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static;

    fn dart_fn_invoke<Ret>(&self, dart_fn_and_args: Vec<DartAbi>) -> DartFnFuture<Ret>;
}

/// Supporting information for a task
#[derive(Clone)]
pub struct TaskInfo {
    /// A Dart `SendPort`. [None] if the mode is [FfiCallMode::Sync].
    pub port: Option<MessagePort>,
    /// Usually the name of the function.
    pub debug_name: &'static str,
    /// The call mode of this function.
    pub mode: FfiCallMode,
}

/// The types of return values for a particular Rust function.
#[derive(Copy, Clone)]
pub enum FfiCallMode {
    /// The default mode, returns a Dart `Future<T>`.
    Normal,
    /// Used by `SyncReturn<T>` to skip spawning workers.
    Sync,
    /// Returns a Dart `Stream<T>`.
    Stream,
}

#[cfg(not(wasm))]
pub trait TaskRetFutTrait: Send {}
#[cfg(not(wasm))]
impl<T: Send> TaskRetFutTrait for T {}

#[cfg(wasm)]
pub trait TaskRetFutTrait {}
#[cfg(wasm)]
impl<T> TaskRetFutTrait for T {}

/// A context for task execution
pub struct TaskContext {
    rust2dart_context: TaskRust2DartContext,
}

impl TaskContext {
    pub fn new(rust2dart_context: TaskRust2DartContext) -> Self {
        Self { rust2dart_context }
    }

    pub fn rust2dart_context(&self) -> &TaskRust2DartContext {
        &self.rust2dart_context
    }
}
