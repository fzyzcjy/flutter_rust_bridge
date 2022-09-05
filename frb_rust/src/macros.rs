/// On WASM, [JsValue][wasm_bindgen::JsValue]s cannot be shared between scopes but instead can be
/// ["transferred"]. Rust however is not aware of transferables and therefore cannot
/// capture these values. This macro wraps a closure and returns a [TransferClosure][crate::ffi::TransferClosure] on WASM platforms
/// which will capture these special values, or a normal [FnOnce] on other platforms.
/// Note that the parameter names must match available variables/bindings from the outer scope.
///
/// ["transferred"]: https://developer.mozilla.org/en-US/docs/Glossary/Transferable_objects
#[macro_export]
macro_rules! transfer {
    (|| $block:block) => {{
        #[cfg(not(target_family = "wasm"))]
        { move || $block }

        #[cfg(target_family = "wasm")]
        {
            $crate::ffi::TransferClosure::new(vec![], vec![], move |_: &[JsValue]| $block)
        }
    }};
    (|$($param:ident: $ty:ty),*| $block:block) => {{
        #[cfg(not(target_family = "wasm"))]
        {
            move || $block
        }

        #[cfg(target_family = "wasm")]
        {
            use wasm_bindgen::JsValue;
            use $crate::ffi::Transfer;
            #[allow(unused_variables)]
            let worker = move |transfer: &[JsValue]| {
                let idx = 0;
                $(
                    let $param = <$ty>::deserialize(&transfer[idx]);
                    let idx = idx + 1;
                )*
                $block
            };
            let transferables = [$($param.transferables()),*].concat();
            $crate::ffi::TransferClosure::new(vec![$($param.serialize()),*], transferables, worker)
        }
    }};
}

/// Spawn a task using the internal thread pool.
/// Interprets the parameters as a list of captured transferables to
/// send to this thread.
///
/// Also see [`transfer`].
///
/// Example:
/// ```
/// let (tx, rx) = std::sync::mpsc::channel();
/// flutter_rust_bridge::spawn!(|| {
///     tx.send(true).unwrap();
/// });
/// assert_eq!(rx.recv(), Ok(true));
/// ```
///
/// Sending a JS transferable:
///
/// ```
/// # #[cfg(target_family = "wasm")] fn main() {
/// use web_sys::{MessagePort, MessageEvent};
/// use wasm_bindgen::prelude::*;
///
/// let channel = web_sys::MessageChannel::new().unwrap();
///
/// let onmessage = Closure::new(move |event: MessageEvent| {
///     assert!(event.data() == true);
/// });
/// channel.port1().set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
/// onmessage.forget();
/// let port2 = channel.port2();
/// // Declare the transferable with the same name and type
/// flutter_rust_bridge::spawn!(|port2: MessagePort| {
///     port2.post_message(&JsValue::from(true)).unwrap();
/// });
/// # } #[cfg(not(target_family = "wasm"))] fn main() {}
/// ```
#[macro_export]
macro_rules! spawn {
    ($($tt:tt)*) => {{
        let worker = $crate::transfer!($($tt)*);
        #[cfg(not(target_family = "wasm"))]
        {
            $crate::thread::THREAD_POOL.lock().execute(worker)
        }

        #[cfg(target_family = "wasm")]
        {
            use anyhow::anyhow;
            let res = $crate::thread::WORKER_POOL.with(|pool| {
                if let Some(pool) = pool.as_ref() {
                    pool.run(worker).map_err(|err| anyhow!("worker error: {:?}", err))
                } else {
                    Err(anyhow!("Worker was not initialized."))
                }
            });
            if let Err(err) = res {
                $crate::console_error!("worker error: {:?}", err);
            }
        }
    }};
}

#[macro_export]
macro_rules! console_error {
    ($lit:literal) => {
        $crate::error($lit)
    };
    ($($tt:tt)*) => {
        $crate::error(&format!($($tt)*))
    };
}
