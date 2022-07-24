//! Stubs for allo_isolate on the web.

#[cfg(not(target_family = "wasm"))]
pub use allo_isolate::{ffi::DartCObject, *};
#[cfg(not(target_family = "wasm"))]
pub type MessagePort = i64;

#[macro_export]
macro_rules! console_log {
    ($lit:literal) => {
        #[allow(unused_unsafe)]
        unsafe { $crate::ffi::log($lit) }
    };
    ($( $tt:tt )*) => {
        #[allow(unused_unsafe)]
        unsafe { $crate::ffi::log(&format!($( $tt )*)) }
    };
}

#[macro_export]
macro_rules! console_error {
    ($lit:literal) => {
        #[allow(unused_unsafe)]
        unsafe { $crate::ffi::error($lit) }
    };
    ($( $tt:tt )*) => {
        #[allow(unused_unsafe)]
        unsafe { $crate::ffi::error(&format!($( $tt )*)) }
    };
}

#[cfg(target_family = "wasm")]
pub use web::*;
#[cfg(target_family = "wasm")]
mod web {
    use js_sys::{global, Array};
    use std::iter::FromIterator;
    pub use wasm_bindgen::closure::Closure;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{DedicatedWorkerGlobalScope, Worker};
    pub type MessagePort = web_sys::MessagePort;

    pub type DartCObject = JsValue;

    pub trait IntoDart {
        fn into_dart(self) -> DartCObject;
    }

    pub trait IntoDartExceptPrimitive: IntoDart {}
    impl IntoDartExceptPrimitive for JsValue {}

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
    // Orphan rules do not benefit our implementation, so we manually delegate From conversions here.
    delegate! {
        bool i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize
        f32 f64 &str String JsValue
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

    impl<T: IntoDart + Clone> IntoDart for &T {
        #[inline]
        fn into_dart(self) -> DartCObject {
            self.clone().into_dart()
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
            let err = self.port.post_message(&msg.into_dart());
            if let Err(err) = &err {
                #[cfg(target_family = "wasm")]
                crate::console_error!("post: {:?}", err);
            }
            err.is_ok()
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

    pub struct TransferClosure<'a, T> {
        transfer: &'a [T],
        closure: RawClosure<T>,
    }

    pub struct TransferClosurePayload {
        func: RawClosure<JsValue>,
    }

    impl<'a> TransferClosure<'a, JsValue> {
        pub fn new(
            transfer: &'a [JsValue],
            closure: impl FnOnce(&[JsValue]) + Send + 'static,
        ) -> Self {
            Self {
                transfer,
                closure: Box::new(closure),
            }
        }
        /// Posts a <code>[*mut [TransferClosurePayload], ...[JsValue]]</code> message to this worker.
        ///
        /// The worker's onmessage should run the corresponding [receive_transfer_closure]
        /// to receive the message.
        pub fn apply(self, worker: &Worker) -> Result<(), JsValue> {
            let transfer = Array::from_iter(self.transfer);
            // The worker is responsible for cleaning up the leak here.
            let payload = Box::leak(Box::new(TransferClosurePayload { func: self.closure }));
            let data = Array::from(&transfer);
            data.unshift(&JsValue::from(payload as *mut _ as u32));
            worker
                .post_message_with_transfer(&data, &transfer)
                .map_err(|err| {
                    // post message failed, ownership remains with us.
                    drop(unsafe { Box::from_raw(payload) });
                    err
                })
        }
    }

    #[wasm_bindgen]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn receive_transfer_closure(
        payload: *mut TransferClosurePayload,
        transfer: Box<[JsValue]>,
    ) -> Result<(), JsValue> {
        let TransferClosurePayload { func } = unsafe { *Box::from_raw(payload) };
        func(&transfer);
        global()
            .unchecked_into::<DedicatedWorkerGlobalScope>()
            .post_message(&JsValue::undefined())
    }

    pub struct ZeroCopyBuffer<T>(pub T);

    impl<T> ZeroCopyBuffer<Vec<T>> {
        #[inline]
        pub fn as_slice(&self) -> &[T] {
            self.0.as_slice()
        }
    }
}
