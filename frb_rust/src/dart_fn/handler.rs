use super::DartFnFuture;
use crate::codec::sse::Dart2RustMessageSse;
use crate::dart_opaque::action::DartHandlerPortAction;
use crate::generalized_isolate::{Channel, IntoDart};
use crate::misc::atomic::{AtomicI32, Ordering};
use crate::misc::logs::log_warn_or_println;
use crate::platform_types::{handle_to_message_port, DartAbi};
use crate::rust2dart::sender::Rust2DartSender;
use crate::DartOpaque;
use futures::channel::oneshot;
use futures::channel::oneshot::Sender;
use futures::FutureExt;
use std::collections::HashMap;
use std::panic;
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
                    if let Err(err) = completer.send(message) {
                        // We do not care about details of this warning
                        // frb-coverage:ignore-start
                        log_warn_or_println(&format!(
                            "Error in dart_fn_handle_output when sending message for call_id {call_id}: {err:?}"
                        ));
                        // frb-coverage:ignore-end
                    }
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

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::DartFnHandler;
    use crate::codec::sse::{Dart2RustMessageSse, SseDeserializer};
    use futures::channel::oneshot;
    use std::sync::MutexGuard;

    fn completers(
        handler: &DartFnHandler,
    ) -> MutexGuard<'_, std::collections::HashMap<i32, oneshot::Sender<Dart2RustMessageSse>>> {
        handler.completers.lock().unwrap()
    }

    fn message(bytes: Vec<u8>) -> Dart2RustMessageSse {
        let (ptr, len) = crate::for_generated::into_leak_vec_ptr(bytes);
        unsafe { Dart2RustMessageSse::from_wire(ptr, len, len) }
    }

    /// Resolves only the completer matching the returned Dart call identifier.
    #[test]
    fn test_handle_output_delivers_matching_message_once() {
        let handler = DartFnHandler::new();
        let (sender, receiver) = oneshot::channel();
        completers(&handler).insert(17, sender);

        handler.handle_output(17, message(vec![3, 4]));

        let received = futures::executor::block_on(receiver).unwrap();
        assert_eq!(
            SseDeserializer::new(received).cursor.into_inner(),
            vec![3, 4]
        );
        assert!(!completers(&handler).contains_key(&17));
    }

    /// Ignores an output whose call identifier has no registered completer.
    #[test]
    fn test_handle_output_ignores_unknown_call_id() {
        let handler = DartFnHandler::new();

        handler.handle_output(99, message(vec![1]));

        assert!(completers(&handler).is_empty());
    }
}
