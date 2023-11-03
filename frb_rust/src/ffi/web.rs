use std::iter::FromIterator;

use super::DartAbi;
use super::MessagePort;
use crate::support;
pub use crate::wasm_bindgen_src::transfer::*;
use crate::DartOpaque;
use crate::DartSafe;
use crate::RustOpaque;
pub use js_sys;
pub use js_sys::Array as JsArray;
use js_sys::*;
pub use wasm_bindgen;
pub use wasm_bindgen::closure::Closure;
pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen::JsCast;
use web_sys::BroadcastChannel;

pub use crate::wasm_bindgen_src::transfer::*;
pub trait IntoDart {
    fn into_dart(self) -> DartAbi;
}

pub trait IntoDartExceptPrimitive: IntoDart {}
impl IntoDartExceptPrimitive for JsValue {}
impl<T: DartSafe> IntoDartExceptPrimitive for RustOpaque<T> {}
impl IntoDartExceptPrimitive for DartOpaque {}
impl IntoDartExceptPrimitive for String {}
impl IntoDartExceptPrimitive for bool {}
impl<T: IntoDart> IntoDartExceptPrimitive for Option<T> {}

impl IntoDart for () {
    #[inline]
    fn into_dart(self) -> DartAbi {
        JsValue::undefined()
    }
}
#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> IntoDart for chrono::DateTime<Tz> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.timestamp_millis().into_dart()
    }
}
#[cfg(feature = "chrono")]
impl IntoDart for chrono::NaiveDateTime {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.timestamp_millis().into_dart()
    }
}
#[cfg(feature = "chrono")]
impl IntoDart for chrono::Duration {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.num_milliseconds().into_dart()
    }
}
#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> IntoDart for Vec<chrono::DateTime<Tz>> {
    fn into_dart(self) -> DartAbi {
        self.into_iter()
            .map(IntoDart::into_dart)
            .collect::<Vec<_>>()
            .into_dart()
    }
}
#[cfg(feature = "chrono")]
impl IntoDart for Vec<chrono::NaiveDateTime> {
    fn into_dart(self) -> DartAbi {
        self.into_iter()
            .map(IntoDart::into_dart)
            .collect::<Vec<_>>()
            .into_dart()
    }
}
#[cfg(feature = "chrono")]
impl IntoDart for Vec<chrono::Duration> {
    fn into_dart(self) -> DartAbi {
        self.into_iter()
            .map(IntoDart::into_dart)
            .collect::<Vec<_>>()
            .into_dart()
    }
}

#[cfg(feature = "uuid")]
impl IntoDart for uuid::Uuid {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.as_bytes().to_vec().into_dart()
    }
}

#[cfg(feature = "uuid")]
impl IntoDart for Vec<uuid::Uuid> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        use std::io::Write;
        let mut buffer = Vec::<u8>::with_capacity(self.len() * super::UUID_SIZE_IN_BYTES);
        for id in self {
            let _ = buffer.write(id.as_bytes());
        }
        Uint8Array::from(buffer.as_slice()).into()
    }
}

impl IntoDart for backtrace::Backtrace {
    fn into_dart(self) -> DartAbi {
        format!("{:?}", self).into_dart()
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

impl<T: DartSafe> IntoDart for RustOpaque<T> {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.into()
    }
}

impl IntoDart for DartOpaque {
    #[inline]
    fn into_dart(self) -> DartAbi {
        self.into()
    }
}

impl<const N: usize, T: IntoDartExceptPrimitive> IntoDart for [T; N] {
    #[inline]
    fn into_dart(self) -> DartAbi {
        let boxed: Box<[T]> = Box::new(self);
        boxed.into_vec().into_dart()
    }
}

macro_rules! impl_into_dart_for_primitive {
    ($($prim:ty)*) => {$(
        impl<const N: usize> IntoDart for [$prim; N] {
            #[inline]
            fn into_dart(self) -> DartAbi {
                Vec::from(self).into_dart()
            }
        }
    )*};
}

impl_into_dart_for_primitive!(i8 u8 i16 u16 i32 u32 f32 f64);

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

macro_rules! impl_into_dart_for_tuple {
    ($( ($($T:ident)+) )*) => {$(
        impl<$($T: IntoDart),+> IntoDart for ($($T),+,) {
            #[allow(non_snake_case)]
            fn into_dart(self) -> DartAbi {
                let ($($T),+,) = self;
                vec![$($T.into_dart()),+].into_dart()
            }
        }

        impl<$($T: IntoDart),+> IntoDartExceptPrimitive for ($($T),+,) {}
    )*};
}

impl_into_dart_for_tuple! {
    (A)
    (A B)
    (A B C)
    (A B C D)
    (A B C D E)
    (A B C D E F)
    (A B C D E F G)
    (A B C D E F G H)
    (A B C D E F G H I)
    (A B C D E F G H I J)
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

impl IntoDart for anyhow::Error {
    fn into_dart(self) -> DartAbi {
        format!("{:?}", self).into_dart()
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
    pub(crate) data: Vec<T>,
    pub(crate) transfer: Vec<T>,
    pub(crate) closure: RawClosure<T>,
}

pub struct TransferClosurePayload<T> {
    pub(crate) func: RawClosure<T>,
}

impl TransferClosure<JsValue> {
    pub fn new(
        data: Vec<JsValue>,
        transfer: Vec<JsValue>,
        closure: impl FnOnce(&[JsValue]) + Send + 'static,
    ) -> Self {
        let closure = Box::new(closure);
        Self {
            data,
            transfer,
            closure,
        }
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
pub trait Transfer {
    /// Recover the self value from a [JsValue].
    fn deserialize(value: &JsValue) -> Self;
    /// Transform the self value into a [JsValue].
    fn serialize(self) -> JsValue;
    /// Extract items that are valid to be passed as the "transfer" argument.
    fn transferables(&self) -> Vec<JsValue>;
}

impl<T: Transfer> Transfer for Option<T> {
    fn deserialize(value: &JsValue) -> Self {
        (!value.is_undefined() && !value.is_null()).then(|| T::deserialize(value))
    }
    fn serialize(self) -> JsValue {
        self.map(T::serialize).unwrap_or_default()
    }
    fn transferables(&self) -> Vec<JsValue> {
        self.as_ref().map(T::transferables).unwrap_or_default()
    }
}

impl Transfer for PortLike {
    fn deserialize(value: &JsValue) -> Self {
        if let Some(name) = value.as_string() {
            BroadcastChannel::new(&name).unwrap().unchecked_into()
        } else if value.dyn_ref::<web_sys::MessagePort>().is_some() {
            value.unchecked_ref::<Self>().clone()
        } else {
            panic!("Not a PortLike: {:?}", value)
        }
    }
    fn serialize(self) -> JsValue {
        if let Some(channel) = self.dyn_ref::<BroadcastChannel>() {
            channel.name().into()
        } else {
            self.into()
        }
    }
    fn transferables(&self) -> Vec<JsValue> {
        if let Some(port) = self.dyn_ref::<web_sys::MessagePort>() {
            vec![port.clone().into()]
        } else {
            vec![]
        }
    }
}

impl Transfer for ArrayBuffer {
    fn deserialize(value: &JsValue) -> Self {
        value.dyn_ref().cloned().unwrap()
    }
    fn serialize(self) -> JsValue {
        self.into()
    }
    fn transferables(&self) -> Vec<JsValue> {
        vec![self.into()]
    }
}

#[wasm_bindgen]
extern "C" {
    /// Objects implementing the interface of [`web_sys::MessagePort`].
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
        r"
(() => {
    try {
        throw new Error();
    } catch (e) {
        let parts = e.stack.match(/(?:\(|@)(\S+):\d+:\d+/);
        return parts[1];
    }
})()",
    )
    .ok()?
    .as_string()
}

#[cfg(feature = "chrono")]
#[inline]
pub fn wire2api_timestamp(ts: i64) -> Timestamp {
    let s = ts / 1_000;
    let ns = (ts.rem_euclid(1_000) * 1_000_000) as u32;
    Timestamp { s, ns }
}
/// a timestamp with milliseconds precision
#[cfg(feature = "chrono")]
pub struct Timestamp {
    /// seconds
    pub s: i64,
    /// nanoseconds
    pub ns: u32,
}

#[cfg(test)]
#[cfg(feature = "chrono")]
mod tests {
    #[test]
    fn wire2api() {
        // input in milliseconds
        let input: i64 = 3_496_567;
        let super::Timestamp { s, ns } = super::wire2api_timestamp(input);
        assert_eq!(s, 3_496);
        assert_eq!(ns, 567_000_000);
    }
}

/// # Safety
///
/// TODO: need doc
#[wasm_bindgen]
pub unsafe fn get_dart_object(ptr: usize) -> JsValue {
    *support::box_from_leak_ptr(ptr as _)
}

/// # Safety
///
/// TODO: need doc
#[wasm_bindgen]
pub unsafe fn drop_dart_object(ptr: usize) {
    drop(support::box_from_leak_ptr::<JsValue>(ptr as _));
}

#[derive(Debug)]
pub struct DartOpaqueBase {
    inner: Box<JsValue>,
    drop_port: Option<String>,
}

impl DartOpaqueBase {
    pub fn new(handle: JsValue, port: Option<JsValue>) -> Self {
        Self {
            inner: Box::new(handle),
            drop_port: port.map(|p| p.dyn_ref::<BroadcastChannel>().unwrap().name()),
        }
    }

    pub fn unwrap(self) -> JsValue {
        *self.inner
    }

    pub fn into_raw(self) -> *mut JsValue {
        Box::into_raw(self.inner)
    }

    pub fn channel(&self) -> Option<Channel> {
        Some(Channel::new(PortLike::broadcast(self.drop_port.as_ref()?)))
    }
}
