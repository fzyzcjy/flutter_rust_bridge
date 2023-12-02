use std::future::Future;
use std::panic::UnwindSafe;
use allo_isolate::IntoDart;

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
    fn wrap<PrepareFn, TaskFn, TaskRetDirect, TaskRetData, Er>(&self, wrap_info: WrapInfo, prepare: PrepareFn)
    where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskCallback) -> Result<TaskRetDirect, Er> + Send + UnwindSafe + 'static,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static;

    /// Same as [`wrap`][Handler::wrap], but the Rust function will be called synchronously and
    /// need not implement [Send].
    fn wrap_sync<SyncTaskFn, TaskRetDirect, TaskRetData, Er>(
        &self,
        wrap_info: WrapInfo,
        sync_task: SyncTaskFn,
    ) -> WireSyncReturn
    where
        SyncTaskFn: FnOnce() -> Result<TaskRetDirect, Er> + UnwindSafe,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static;

    #[cfg(feature = "rust-async")]
    fn wrap_async<PrepareFn, TaskFn, TaskRetFut, TaskRetDirect, TaskRetData, Er>(
        &self,
        wrap_info: WrapInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskCallback) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRetFut: Future<Output = Result<TaskRetDirect, Er>> + TaskRetFutTrait + UnwindSafe,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static;
}

/// Supporting information to identify a function's operating mode.
#[derive(Clone)]
pub struct WrapInfo {
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
