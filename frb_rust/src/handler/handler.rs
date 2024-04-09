use crate::codec::sse::Dart2RustMessageSse;
use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::platform_types::DartAbi;
use crate::platform_types::MessagePort;
use std::future::Future;

/// Provide your own handler to customize how to execute your function calls, etc.
///
/// This API is not guaranteed to be stable following semver (since things are going to be
/// added, and for every addition/change, it is a breaking change for this trait).
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
    #[cfg(feature = "thread-pool")]
    fn wrap_normal<Rust2DartCodec, PrepareFn, TaskFn>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn,
        TaskFn: FnOnce(TaskContext) -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>
            + Send
            + 'static,
        Rust2DartCodec: BaseCodec;

    /// Same as [`wrap`][Handler::wrap], but the Rust function will be called synchronously and
    /// need not implement [Send].
    fn wrap_sync<Rust2DartCodec, SyncTaskFn>(
        &self,
        task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> <Rust2DartCodec::Message as Rust2DartMessageTrait>::WireSyncRust2DartType
    where
        SyncTaskFn: FnOnce() -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>,
        Rust2DartCodec: BaseCodec;

    /// Same as [`wrap`][Handler::wrap], but for async Rust.
    #[cfg(feature = "rust-async")]
    fn wrap_async<Rust2DartCodec, PrepareFn, TaskFn, TaskRetFut>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn,
        TaskFn: FnOnce(TaskContext) -> TaskRetFut + Send + 'static,
        TaskRetFut: Future<Output = Result<Rust2DartCodec::Message, Rust2DartCodec::Message>>
            + TaskRetFutTrait,
        Rust2DartCodec: BaseCodec;

    #[cfg(all(feature = "rust-async", feature = "dart-opaque"))]
    fn dart_fn_invoke(
        &self,
        dart_fn: crate::dart_opaque::DartOpaque,
        args: Vec<DartAbi>,
    ) -> crate::dart_fn::DartFnFuture<Dart2RustMessageSse>;

    #[cfg(all(feature = "rust-async", feature = "dart-opaque"))]
    fn dart_fn_handle_output(&self, call_id: i32, message: Dart2RustMessageSse);
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FfiCallMode {
    /// The default mode, returns a Dart `Future<T>`.
    Normal,
    /// Used by `SyncReturn<T>` to skip spawning workers.
    Sync,
}

#[cfg(not(wasm))]
pub trait TaskRetFutTrait: Send {}
#[cfg(not(wasm))]
impl<T: Send> TaskRetFutTrait for T {}

#[cfg(wasm)]
pub trait TaskRetFutTrait {}
#[cfg(wasm)]
impl<T> TaskRetFutTrait for T {}

// Originally there were things for StreamSink, but it was moved, so now it is empty
/// A context for task execution
pub struct TaskContext {}

// frb-coverage:ignore-start
impl Default for TaskContext {
    fn default() -> Self {
        Self::new()
    }
}
// frb-coverage:ignore-end

impl TaskContext {
    pub fn new() -> Self {
        Self {}
    }
}
