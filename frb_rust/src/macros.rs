/// On WASM, [JsValue][wasm_bindgen::JsValue]s cannot be shared between scopes but instead can be
/// ["transferred"]. Rust however is not aware of transferables and therefore cannot
/// capture these values. This macro wraps a closure and returns a [TransferClosure] on WASM platforms
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
            $crate::ffi::TransferClosure::new(vec![], move |_: &[JsValue]| $block)
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
            #[allow(unused_variables)]
            let worker = move |transfer: &[JsValue]| {
                let idx = 0;
                $(
                    let $param: $ty = $crate::ffi::FromDart::from_dart(&transfer[idx]);
                    let idx = idx + 1;
                )*
                $block
            };
            $crate::ffi::TransferClosure::new(vec![$(<$ty>::into($param)),*], worker)
        }
    }};
}

/// Spawn a task using the internal thread pool.
/// Interprets the parameters as a list of captured transferables to
/// send to this thread.
/// 
/// Also see [`transfer`].
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
                $crate::console_error(&format!("{}", err));
            }
        }
    }};
}
