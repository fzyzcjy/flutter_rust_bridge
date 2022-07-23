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
#[allow(clippy::type_complexity)]
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

    pub trait IntoDartExceptPrimitive {}

    impl IntoDart for () {
        fn into_dart(self) -> DartCObject {
            JsValue::undefined()
        }
    }
    macro_rules! delegate {
        ($( $ty:ty )*) => {$(
            impl IntoDart for $ty {
                #[inline] fn into_dart(self) -> DartCObject {
                    DartCObject::from(self)
                }
            }
        )*};
    }
    // Orphan rules do not benefit our implementation, so we manually delegate From conversions here.
    delegate! {
        bool i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize
        &str String JsValue
    }

    impl<T: IntoDart> IntoDart for Vec<T> {
        fn into_dart(self) -> DartCObject {
            <Array as std::iter::FromIterator<DartCObject>>::from_iter(
                self.into_iter().map(IntoDart::into_dart),
            )
            .into()
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
        fn eval(script: &str);
    }

    pub struct TransferClosure<'a, T> {
        transfer: &'a [T],
        closure: Box<dyn for<'any> FnOnce(&'any [T]) + Send + 'static>,
    }

    pub struct TransferClosurePayload {
        func: Box<dyn FnOnce(&[JsValue]) + Send + 'static>,
    }

    impl<'a> TransferClosure<'a, JsValue> {
        pub fn new(
            transfer: &'a [JsValue],
            closure: impl for<'any> FnOnce(&'any [JsValue]) + Send + 'static,
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
}
