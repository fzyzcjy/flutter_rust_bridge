use crate::codec::BaseCodec;
use crate::dart_fn::DartFnFuture;
use crate::generalized_isolate::{channel_to_handle, IntoDart};
use crate::misc::into_into_dart::IntoIntoDart;
use crate::platform_types::MessagePort;
use crate::platform_types::SendableMessagePortHandle;
use crate::platform_types::{message_port_to_handle, DartAbi};
use crate::rust2dart::context::TaskRust2DartContext;
use crate::rust2dart::wire_sync_return_src::WireSyncReturnWrapperTrait;
use crate::DartOpaque;
use allo_isolate::ffi::DartCObject;
use std::future::Future;
use std::panic::UnwindSafe;

/// Provide your own handler to customize how to execute your function calls, etc.
pub trait Handler {
    fn initialize(&self, config: HandlerConfig);

    fn dart_opaque_drop_port(&self) -> SendableMessagePortHandle;

    fn dart_fn_invoke_port(&self) -> SendableMessagePortHandle;

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
    fn wrap_normal<Rust2DartCodec, PrepareFn, TaskFn>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskContext<Rust2DartCodec>) -> Result<DartCObject, DartCObject>
            + Send
            + UnwindSafe
            + 'static,
        Rust2DartCodec: BaseCodec;

    /// Same as [`wrap`][Handler::wrap], but the Rust function will be called synchronously and
    /// need not implement [Send].
    fn wrap_sync<Rust2DartCodec, SyncTaskFn>(
        &self,
        task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> <Rust2DartCodec::WireSyncReturnWrapper as WireSyncReturnWrapperTrait>::WireType
    where
        SyncTaskFn: FnOnce() -> Result<DartCObject, DartCObject> + UnwindSafe,
        Rust2DartCodec: BaseCodec;

    /// Same as [`wrap`][Handler::wrap], but for async Rust.
    #[cfg(feature = "rust-async")]
    fn wrap_async<Rust2DartCodec, PrepareFn, TaskFn, TaskRetFut>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskContext<Rust2DartCodec>) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRetFut:
            Future<Output = Result<DartCObject, DartCObject>> + TaskRetFutTrait + UnwindSafe,
        Rust2DartCodec: BaseCodec;

    fn dart_fn_invoke<Ret>(&self, dart_fn_and_args: Vec<DartAbi>) -> DartFnFuture<Ret>;
}

#[derive(Clone, Debug)]
pub struct HandlerConfig {
    pub dart_opaque_drop_port: SendableMessagePortHandle,
    pub dart_fn_invoke_port: SendableMessagePortHandle,
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
pub struct TaskContext<Rust2DartCodec: BaseCodec> {
    rust2dart_context: TaskRust2DartContext<Rust2DartCodec>,
}

impl<Rust2DartCodec: BaseCodec> TaskContext<Rust2DartCodec> {
    pub fn new(rust2dart_context: TaskRust2DartContext<Rust2DartCodec>) -> Self {
        Self { rust2dart_context }
    }

    pub fn rust2dart_context(&self) -> &TaskRust2DartContext<Rust2DartCodec> {
        &self.rust2dart_context
    }
}

pub fn handler_initialize<H: Handler>(
    handler: &H,
    dart_opaque_drop_port: MessagePort,
    dart_fn_invoke_port: MessagePort,
) {
    handler.initialize(HandlerConfig {
        dart_opaque_drop_port: message_port_to_handle(&dart_opaque_drop_port),
        dart_fn_invoke_port: message_port_to_handle(&dart_fn_invoke_port),
    })
}
