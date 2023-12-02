// TODO more about this
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
    ($thread_pool:ident, $($tt:tt)*) => {{
        let worker = $crate::transfer!($($tt)*);

        #[cfg(not(target_family = "wasm"))]
        {
            $thread_pool.execute(worker)
        }

        #[cfg(target_family = "wasm")]
        {
            use anyhow::anyhow;
            let res = $thread_pool.with(|pool| {
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
