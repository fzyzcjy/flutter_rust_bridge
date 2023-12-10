use super::DartFnFuture;
use crate::generalized_isolate::Channel;
use crate::platform_types::{DartAbi, SendableMessagePortHandle};
use crate::rust2dart::sender::Rust2DartSender;
use futures::channel::oneshot;
use futures::channel::oneshot::Sender;
use log::warn;
use std::collections::HashMap;
use std::panic;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

pub(crate) struct DartFnHandler {
    completers: Mutex<HashMap<i32, Sender<()>>>,
    next_call_id: AtomicI32,
}

impl DartFnHandler {
    pub(crate) fn new() -> Self {
        Self {
            completers: Mutex::new(HashMap::new()),
            next_call_id: AtomicI32::new(1),
        }
    }

    pub(crate) fn invoke<Ret>(
        &self,
        dart_fn_and_args: Vec<DartAbi>,
        invoke_port: SendableMessagePortHandle,
    ) -> DartFnFuture<Ret> {
        let call_id = self.next_call_id.fetch_add(1, Ordering::Relaxed);
        let (sender, receiver) = oneshot::channel::<()>();
        (self.completers.lock().unwrap()).insert(call_id, sender);

        let sender = Rust2DartSender::new(Channel::new(invoke_port));
        sender.send(dart_fn_and_args);

        Box::new(receiver)
    }

    pub(crate) fn handle_output(&self, call_id: i32) {
        // NOTE This [catch_unwind] should also be put outside **ALL** code, see comments above for reasonk
        panic::catch_unwind(move || {
            let catch_unwind_result = panic::catch_unwind(move || {
                if let Some(completer) = (self.completers.lock().unwrap()).remove(&call_id) {
                    completer.send(());
                }
            });
            if let Err(err) = catch_unwind_result {
                warn!("Error when dart_fn_handle_output: {err:?}");
            }
        });
    }
}
