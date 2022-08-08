use super::DartCObject;
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
use web_sys::{DedicatedWorkerGlobalScope, Worker};

pub trait IntoDartExceptPrimitive: IntoDart {}
impl IntoDartExceptPrimitive for JsValue {}
impl IntoDartExceptPrimitive for String {}
impl<T: IntoDart> IntoDartExceptPrimitive for Option<T> {}

mod pointer {
    use std::marker::PhantomData;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::*;

    /// A non-nullable pointer wrapping a [u32].
    ///
    /// Designed as a replacement for [`Option<*mut T>`].
    ///
    /// Usage | Parameter | Return
    /// ----- | --------- | ------
    /// `T` | Yes | Yes
    /// `Option<T>` | Yes | Yes
    /// `&T` | No | No
    /// `&mut T` | No | No
    pub struct Pointer<T>(u32, PhantomData<*mut T>);
    impl<T> Pointer<T> {
        pub const fn as_mut(self) -> *mut T {
            self.0 as _
        }
    }

    impl<T> WasmDescribe for Pointer<T> {
        fn describe() {
            // <*const T>::describe();
            inform(RUST_STRUCT);
            inform(7);
            inform(b'P' as _);
            inform(b'o' as _);
            inform(b'i' as _);
            inform(b'n' as _);
            inform(b't' as _);
            inform(b'e' as _);
            inform(b'r' as _);
        }
    }

    impl<T> FromWasmAbi for Pointer<T> {
        type Abi = u32;

        unsafe fn from_abi(js: Self::Abi) -> Self {
            debug_assert!(js != 0, "Attempted to retrieve a nullptr");
            Self(js as _, PhantomData)
        }
    }

    impl<T> OptionFromWasmAbi for Pointer<T> {
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }

    impl<T> IntoWasmAbi for Pointer<T> {
        type Abi = u32;

        fn into_abi(self) -> Self::Abi {
            debug_assert!(self.0 != 0, "Attempted to return a nullptr.");
            self.0 as _
        }
    }

    impl<T> OptionIntoWasmAbi for Pointer<T> {
        fn none() -> Self::Abi {
            0
        }
    }
}
pub use pointer::Pointer;

impl IntoDart for () {
    #[inline]
    fn into_dart(self) -> DartCObject {
        JsValue::undefined()
    }
}
macro_rules! delegate {
    ($( $ty:ty )*) => {$(
        impl IntoDart for $ty {
            #[inline]
            fn into_dart(self) -> DartCObject {
                DartCObject::from(self)
            }
        }
    )*};
}
macro_rules! delegate_buffer {
    ($( $ty:ty => $buffer:ty )*) => {$(
        impl IntoDart for $ty {
            #[inline]
            fn into_dart(self) -> DartCObject {
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
    fn into_dart(self) -> DartCObject {
        Array::from_iter(self.into_iter().map(IntoDart::into_dart)).into()
    }
}

impl<T: IntoDart> IntoDart for Option<T> {
    #[inline]
    fn into_dart(self) -> DartCObject {
        self.map(T::into_dart).unwrap_or_else(JsValue::null)
    }
}

impl<const N: usize, T: IntoDartExceptPrimitive> IntoDart for [T; N] {
    #[inline]
    fn into_dart(self) -> DartCObject {
        let boxed: Box<[T]> = Box::new(self);
        boxed.into_vec().into_dart()
    }
}

impl<const N: usize> IntoDart for [u8; N] {
    #[inline]
    fn into_dart(self) -> DartCObject {
        Vec::from(self).into_dart()
    }
}

macro_rules! delegate_big_buffers {
    ($buf:ident, $prim:ty => $buffer:ty) => {
        const RATIO: usize = core::mem::size_of::<$prim>() / core::mem::size_of::<i32>();
        // Multiply the length by the difference in byte length
        let len = $buf.len() * RATIO;
        // Turn Vec into a Box, dropping excess capacity.
        let ptr = Box::into_raw($buf.into_boxed_slice()) as *mut i32;
        // Reassemble into a new slice, now with more length and shorter byte length
        let buf = unsafe { Box::from_raw(core::slice::from_raw_parts_mut(ptr, len)) };
        // Turn into an Int32Array
        let out = Int32Array::from(&buf[..]);
        // And transfer its buffer to output.
        return <$buffer>::new(&out.buffer()).into();
        // buf is dropped here, safely.
    };
}

impl IntoDart for Vec<i64> {
    fn into_dart(self) -> DartCObject {
        delegate_big_buffers!(self, i64 => BigInt64Array);
    }
}
impl IntoDart for Vec<u64> {
    fn into_dart(self) -> DartCObject {
        delegate_big_buffers!(self, u64 => BigUint64Array);
    }
}
impl IntoDart for ZeroCopyBuffer<Vec<i64>> {
    fn into_dart(self) -> DartCObject {
        self.0.into_dart()
    }
}
impl IntoDart for ZeroCopyBuffer<Vec<u64>> {
    fn into_dart(self) -> DartCObject {
        self.0.into_dart()
    }
}

#[derive(Clone)]
pub struct Isolate {
    port: MessagePort,
}

impl Isolate {
    pub fn new(port: MessagePort) -> Self {
        Self { port }
    }
    pub fn post(&self, msg: impl IntoDart) -> bool {
        self.port
            .post_message(&msg.into_dart())
            .map_err(|err| {
                crate::console_error!("post: {:?}", err);
                err
            })
            .is_ok()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(msg: &str);
    // fn eval(script: &str);
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
        // The worker is responsible for cleaning up the leak here.
        let payload = Box::into_raw(Box::new(TransferClosurePayload { func: self.closure }));
        let data = Array::from(&transfer);
        data.unshift(&JsValue::from(payload as i32));
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

impl<T: JsCast + Clone> FromDart for Option<T> {
    #[inline]
    fn from_dart(value: &JsValue) -> Self {
        value.dyn_ref().cloned()
    }
}

macro_rules! delegate_from_dart {
    ($($ty:ty)*) => {$(
        impl FromDart for $ty {
            #[inline]
            fn from_dart(value: &JsValue) -> Self {
                value.unchecked_ref::<Self>().clone()
            }
        }
    )*};
}

delegate_from_dart! {
    MessagePort ArrayBuffer
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
