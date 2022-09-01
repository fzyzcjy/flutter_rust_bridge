use super::DartAbi;
use super::IntoDart;
use super::MessagePort;
pub use js_sys;
pub use js_sys::Array as JsArray;
use js_sys::*;
use std::iter::FromIterator;
pub use wasm_bindgen;
pub use wasm_bindgen::closure::Closure;
pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen::JsCast;
use web_sys::BroadcastChannel;
use web_sys::{DedicatedWorkerGlobalScope, Worker};

pub trait IntoDartExceptPrimitive: IntoDart {}
impl IntoDartExceptPrimitive for JsValue {}
impl IntoDartExceptPrimitive for String {}
impl<T: IntoDart> IntoDartExceptPrimitive for Option<T> {}

impl IntoDart for () {
    #[inline]
    fn into_dart(self) -> DartAbi {
        JsValue::undefined()
    }
}
macro_rules! delegate {
    ($( $ty:ty )*) => {$(
        impl IntoDart for $ty {
            #[inline]
            fn into_dart(self) -> DartAbi {
                DartAbi::from(self)
            }
        }
    )*};
}
macro_rules! delegate_buffer {
    ($( $ty:ty => $buffer:ty )*) => {$(
        impl IntoDart for $ty {
            #[inline]
            fn into_dart(self) -> DartAbi {
                <$buffer>::from(self.as_slice()).into()
            }
        }
    )*};
}
// Orphan rules disallow blanket implementations, so we have to manually delegate here.
delegate! {
    bool
    i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize
    f32 f64
    &str String JsValue
}
delegate_buffer! {
    Vec<i8> => js_sys::Int8Array
    Vec<u8> => js_sys::Uint8Array
    Vec<i16> => js_sys::Int16Array
    Vec<u16> => js_sys::Uint16Array
    Vec<i32> => js_sys::Int32Array
    Vec<u32> => js_sys::Uint32Array
    Vec<f32> => js_sys::Float32Array
    Vec<f64> => js_sys::Float64Array
    ZeroCopyBuffer<Vec<i8>> => js_sys::Int8Array
    ZeroCopyBuffer<Vec<u8>> => js_sys::Uint8Array
    ZeroCopyBuffer<Vec<i16>> => js_sys::Int16Array
    ZeroCopyBuffer<Vec<u16>> => js_sys::Uint16Array
    ZeroCopyBuffer<Vec<i32>> => js_sys::Int32Array
    ZeroCopyBuffer<Vec<u32>> => js_sys::Uint32Array
    ZeroCopyBuffer<Vec<f32>> => js_sys::Float32Array
    ZeroCopyBuffer<Vec<f64>> => js_sys::Float64Array
}

impl<T: IntoDartExceptPrimitive> IntoDart for Vec<T> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        Array::from_iter(self.into_iter().map(IntoDart::into_dart)).into()
    }
}

impl<T: IntoDart> IntoDart for Option<T> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.map(T::into_dart).unwrap_or_else(JsValue::null)
    }
}
impl<T> IntoDart for *const T {
    #[inline]
    fn into_dart(self) -> DartAbi {
        (self as usize).into_dart()
    }
}
impl<T> IntoDart for *mut T {
    #[inline]
    fn into_dart(self) -> DartAbi {
        (self as usize).into_dart()
    }
}

impl<const N: usize, T: IntoDartExceptPrimitive> IntoDart for [T; N] {
    #[inline]
    fn into_dart(self) -> DartAbi {
        let boxed: Box<[T]> = Box::new(self);
        boxed.into_vec().into_dart()
    }
}

impl<const N: usize> IntoDart for [u8; N] {
    #[inline]
    fn into_dart(self) -> DartAbi {
        Vec::from(self).into_dart()
    }
}

macro_rules! delegate_big_buffers {
    ($($buf:ty => $buffer:ty)*) => {$(
        impl IntoDart for $buf {
            fn into_dart(self) -> DartAbi {
                let buf: &[i32] = bytemuck::cast_slice(&self[..]);
                let buf = Int32Array::from(buf);
                <$buffer>::new(&buf.buffer()).into()
            }
        }
    )*};
}
delegate_big_buffers! {
    Vec<i64> => BigInt64Array
    Vec<u64> => BigUint64Array
}
impl IntoDart for ZeroCopyBuffer<Vec<i64>> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.0.into_dart()
    }
}
impl IntoDart for ZeroCopyBuffer<Vec<u64>> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.0.into_dart()
    }
}

#[derive(Clone)]
pub struct Channel {
    port: MessagePort,
}

impl Channel {
    pub fn new(port: MessagePort) -> Self {
        Self { port }
    }
    pub fn post(&self, msg: impl IntoDart) -> bool {
        self.port
            .post_message(&msg.into_dart())
            .map_err(|err| {
                crate::console_error!("post: {:?}", err);
            })
            .is_ok()
    }
    pub(crate) fn broadcast_name(&self) -> Option<String> {
        self.port
            .dyn_ref::<BroadcastChannel>()
            .map(|channel| channel.name())
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "error")]
    pub fn error(msg: &str);
}

type RawClosure<T> = Box<dyn FnOnce(&[T]) + Send + 'static>;

pub struct TransferClosure<T> {
    transfer: Vec<T>,
    closure: RawClosure<T>,
}

pub struct TransferClosurePayload<T> {
    func: RawClosure<T>,
}

impl TransferClosure<JsValue> {
    pub fn new(transfer: Vec<JsValue>, closure: impl FnOnce(&[JsValue]) + Send + 'static) -> Self {
        let closure = Box::new(closure);
        Self { transfer, closure }
    }
    /// Posts a <code>[*mut [TransferClosurePayload], ...[JsValue]]</code> message to this worker.
    ///
    /// The worker's `onmessage` should run the corresponding [`receive_transfer_closure`]
    /// to receive the message.
    pub fn apply(self, worker: &Worker) -> Result<(), JsValue> {
        let transfer = Array::from_iter(self.transfer);
        let data = Array::from(&transfer);
        // The worker is responsible for cleaning up the leak here.
        let payload = Box::into_raw(Box::new(TransferClosurePayload { func: self.closure }));
        data.unshift(&JsValue::from(payload as i32));
        // Remove untransferables
        let transfer = transfer.filter(&mut |val, _, _| val.is_truthy());
        worker
            .post_message_with_transfer(&data, &transfer)
            .map_err(|err| {
                // post message failed, ownership remains with us.
                drop(unsafe { Box::from_raw(payload) });
                err
            })
    }
}

#[derive(Debug)]
pub struct ZeroCopyBuffer<T>(pub T);

impl<T> ZeroCopyBuffer<Vec<T>> {
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}

/// Internal implementations for transferables on WASM platforms.
pub trait FromDart {
    fn from_dart(value: &JsValue) -> Self;
}

impl<T: FromDart> FromDart for Option<T> {
    #[inline]
    fn from_dart(value: &JsValue) -> Self {
        (!value.is_null() && !value.is_undefined()).then(|| T::from_dart(value))
    }
}

macro_rules! delegate_from_dart {
    ($($ty:ty)*) => {$(
        impl FromDart for $ty {
            #[inline]
            fn from_dart(value: &JsValue) -> Self {
                value.clone().unchecked_into()
            }
        }
    )*};
}

delegate_from_dart! {
    PortLike ArrayBuffer
}

/// ## Safety
/// This function reclaims a raw pointer created by [`TransferClosure`], and therefore
/// should **only** be used in conjunction with it.
/// Furthermore, the WASM module in the worker must have been initialized with the shared
/// memory from the host JS scope.
// wasm_bindgen cannot work with unsafe functions, hence the clippy ignore.
#[wasm_bindgen]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn receive_transfer_closure(
    payload: *mut TransferClosurePayload<JsValue>,
    transfer: Box<[JsValue]>,
) -> Result<(), JsValue> {
    let closure = unsafe { Box::from_raw(payload) };
    (closure.func)(&transfer);
    // Once we're done here, notify the host scope so that it can reclaim this worker.
    global()
        .unchecked_into::<DedicatedWorkerGlobalScope>()
        .post_message(&JsValue::UNDEFINED)
}

#[wasm_bindgen]
extern "C" {
    /// Any object implementing the interface of [`web_sys::MessagePort`].
    ///
    /// Attempts to coerce [`JsValue`]s into this interface using [`dyn_into`][JsCast::dyn_into]
    /// or [`dyn_ref`][JsCast::dyn_ref] will fail at runtime.
    #[derive(Clone)]
    pub type PortLike;
    #[wasm_bindgen(method, catch, js_name = "postMessage")]
    pub fn post_message(this: &PortLike, value: &JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(method, catch)]
    pub fn close(this: &PortLike) -> Result<(), JsValue>;
}

impl PortLike {
    /// Create a [`BroadcastChannel`] with the specified name.
    pub fn broadcast(name: &str) -> Self {
        BroadcastChannel::new(name)
            .expect("Failed to create broadcast channel")
            .unchecked_into()
    }
}

/// Copied from https://github.com/chemicstry/wasm_thread/blob/main/src/script_path.js
pub fn script_path() -> Option<String> {
    js_sys::eval(
        r#"
(() => {
    try {
        throw new Error();
    } catch (e) {
        let parts = e.stack.match(/(?:\(|@)(\S+):\d+:\d+/);
        return parts[1];
    }
})()"#,
    )
    .ok()?
    .as_string()
}
