pub trait BaseThreadPool {
    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}

#[derive(Debug, Default)]
pub struct SimpleThreadPool(pub threadpool::ThreadPool);

impl BaseThreadPool for SimpleThreadPool {
    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.0.execute(job)
    }
}

#[cfg(test)]
mod tests {
    use super::{BaseThreadPool, SimpleThreadPool};
    use std::sync::mpsc;
    use std::time::Duration;

    /// Executes a submitted job on the native thread pool.
    #[test]
    fn test_execute_runs_submitted_job() {
        let pool = SimpleThreadPool::default();
        let (sender, receiver) = mpsc::channel();

        pool.execute(move || sender.send(()).unwrap());

        assert_eq!(receiver.recv_timeout(Duration::from_secs(1)), Ok(()));
    }
}

// TODO migrate this documentation
// /// Spawn a task using the internal thread pool.
// /// Interprets the parameters as a list of captured transferables to
// /// send to this thread.
// ///
// /// Also see [`transfer`].
// ///
// /// Example:
// /// ```
// /// let (tx, rx) = std::sync::mpsc::channel();
// /// flutter_rust_bridge::spawn!(|| {
// ///     tx.send(true).unwrap();
// /// });
// /// assert_eq!(rx.recv(), Ok(true));
// /// ```
// ///
// /// Sending a JS transferable:
// ///
// /// ```
// /// # #[cfg(target_family = "wasm")] fn main() {
// /// use web_sys::{MessagePort, MessageEvent};
// /// use wasm_bindgen::prelude::*;
// ///
// /// let channel = web_sys::MessageChannel::new().unwrap();
// ///
// /// let onmessage = Closure::new(move |event: MessageEvent| {
// ///     assert!(event.data() == true);
// /// });
// /// channel.port1().set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
// /// onmessage.forget();
// /// let port2 = channel.port2();
// /// // Declare the transferable with the same name and type
// /// flutter_rust_bridge::spawn!(|port2: MessagePort| {
// ///     port2.post_message(&JsValue::from(true)).unwrap();
// /// });
// /// # } #[cfg(not(target_family = "wasm"))] fn main() {}
// /// ```
