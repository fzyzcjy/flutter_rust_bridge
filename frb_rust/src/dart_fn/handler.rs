use super::DartFnFuture;
use crate::codec::sse::Dart2RustMessageSse;
use crate::dart_opaque::action::DartHandlerPortAction;
use crate::generalized_isolate::{Channel, IntoDart};
use crate::misc::logs::log_warn_or_println;
use crate::platform_types::{handle_to_message_port, DartAbi};
use crate::rust2dart::sender::Rust2DartSender;
use crate::DartOpaque;
use futures::channel::oneshot;
use futures::channel::oneshot::Sender;
use futures::FutureExt;
use std::collections::HashMap;
use std::panic;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

pub(crate) struct DartFnHandler {
    completers: Mutex<HashMap<i32, Sender<Dart2RustMessageSse>>>,
    next_call_id: AtomicI32,
}

impl DartFnHandler {
    pub(crate) fn new() -> Self {
        Self {
            completers: Mutex::new(HashMap::new()),
            next_call_id: AtomicI32::new(1),
        }
    }

    pub(crate) fn invoke(
        &self,
        dart_fn: DartOpaque,
        args: Vec<DartAbi>,
    ) -> DartFnFuture<Dart2RustMessageSse> {
        let dart_handler_port = dart_fn.dart_handler_port();

        let call_id = self.next_call_id.fetch_add(1, Ordering::Relaxed);
        let (sender, receiver) = oneshot::channel::<Dart2RustMessageSse>();
        (self.completers.lock().unwrap()).insert(call_id, sender);

        let sender = Rust2DartSender::new(Channel::new(handle_to_message_port(dart_handler_port)));
        let msg = {
            let mut ans = vec![
                DartHandlerPortAction::DartFnInvoke.into_dart(),
                dart_fn.into_dart(),
                call_id.into_dart(),
            ];
            ans.extend(args);
            ans
        };
        sender.send_or_warn(msg);

        Box::pin(receiver.then(|x| async move { x.unwrap() }))
    }

    pub(crate) fn handle_output(&self, call_id: i32, message: Dart2RustMessageSse) {
        // NOTE This [catch_unwind] should also be put outside **ALL** code, see comments above for reasonk
        let _ = panic::catch_unwind(move || {
            let catch_unwind_result = panic::catch_unwind(move || {
                if let Some(completer) = (self.completers.lock().unwrap()).remove(&call_id) {
                    completer.send(message).unwrap();
                }
            });
            if let Err(err) = catch_unwind_result {
                // We do not care about details of this warning
                // frb-coverage:ignore-start
                log_warn_or_println(&format!("Error when dart_fn_handle_output: {err:?}"));
                // frb-coverage:ignore-end
            }
        });
    }
}
