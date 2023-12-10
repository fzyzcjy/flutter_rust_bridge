use std::collections::HashMap;
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

    pub(crate) fn invoke<Ret>(&self, dart_fn_and_args: Vec<DartAbi>) -> DartFnFuture<Ret> {
        let call_id = self.next_call_id.fetch_add(1, Ordering::Relaxed);
        let (sender, receiver) = futures::channel::oneshot::channel::<()>();
        (self.completers.lock().unwrap()).insert(call_id, sender);

        let sender = Rust2DartSender::new(Channel::new(self.dart_fn_invoke_port()));
        sender.send(dart_fn_and_args);

        receiver
    }

    pub(crate) fn handle_output(&self, call_id: i32) {
        // NOTE This [catch_unwind] should also be put outside **ALL** code, see comments above for reasonk
        panic::catch_unwind(move || {
            let catch_unwind_result = panic::catch_unwind(move || {
                if let Some(completer) = (self.completers.lock().unwrap()).remove(call_id) {
                    completer.send(());
                }
            });
            if let Err(err) = catch_unwind_result {
                warn!("Error when dart_fn_handle_output: {err:?}");
            }
        });
    }
}
