use super::thread_box::ThreadBox;
use super::{GeneralizedAutoDropDartPersistentHandle, GeneralizedDartHandle};
use crate::dart_opaque::action::DartHandlerPortAction;
use crate::for_generated::{box_from_leak_ptr, new_leak_box_ptr};
use crate::generalized_isolate::Channel;
use crate::platform_types::{handle_to_message_port, SendableMessagePortHandle};
use allo_isolate::IntoDart;
use log::warn;

#[derive(Debug)]
pub(super) struct DartOpaqueNonClone {
    /// The internal persistent handle
    // `Option` is used for correct drop.
    persistent_handle: Option<ThreadBox<GeneralizedAutoDropDartPersistentHandle>>,

    /// The port to drop object (when we cannot drop in current thread)
    dart_handler_port: SendableMessagePortHandle,
}

impl DartOpaqueNonClone {
    pub(super) fn new(
        handle: GeneralizedDartHandle,
        dart_handler_port: SendableMessagePortHandle,
    ) -> Self {
        let auto_drop_persistent_handle =
            GeneralizedAutoDropDartPersistentHandle::new_from_non_persistent_handle(handle);
        Self {
            persistent_handle: Some(ThreadBox::new(auto_drop_persistent_handle)),
            dart_handler_port,
        }
    }

    pub(super) fn into_inner(mut self) -> GeneralizedAutoDropDartPersistentHandle {
        // Though inner ThreadBox has a check, we still check here
        // to avoid (auto) invoking ThreadBox.drop during its panicking,
        // which causes either leak or abort.
        // In addition, here we have more user friendly error message.
        if !(self.persistent_handle.as_ref().unwrap()).is_on_creation_thread() {
            panic!("DartOpaque can only be used on the creation thread");
        }

        self.persistent_handle.take().unwrap().into_inner()
    }

    pub(super) fn create_dart_handle(&self) -> GeneralizedDartHandle {
        (self.persistent_handle.as_ref().unwrap().as_ref()).create_dart_handle()
    }

    pub(crate) fn dart_handler_port(&self) -> &SendableMessagePortHandle {
        &self.dart_handler_port
    }
}

impl Drop for DartOpaqueNonClone {
    fn drop(&mut self) {
        if let Some(persistent_handle) = self.persistent_handle.take() {
            // If we forget to do so, ThreadBox will panic because it requires things to be dropped on creation thread
            if !persistent_handle.is_on_creation_thread() {
                drop_thread_box_persistent_handle_via_port(
                    persistent_handle,
                    &self.dart_handler_port,
                )
            }
        }
    }
}

/// Drop by sending to a Dart port and let the handler there call [dart_opaque_drop_thread_box_persistent_handle]
fn drop_thread_box_persistent_handle_via_port(
    persistent_handle: ThreadBox<GeneralizedAutoDropDartPersistentHandle>,
    drop_port: &SendableMessagePortHandle,
) {
    let channel = Channel::new(handle_to_message_port(drop_port));
    let ptr = new_leak_box_ptr(persistent_handle) as usize;

    let msg = [DartHandlerPortAction::DartOpaqueDrop, ptr.into_dart()];

    if !channel.post(msg) {
        warn!("Drop DartOpaque after closing the port, thus the object will be leaked forever.");
        // In case logs are disabled
        println!("Drop DartOpaque after closing the port, thus the object will be leaked forever.");
    };
}

#[no_mangle]
pub unsafe extern "C" fn dart_opaque_drop_thread_box_persistent_handle(ptr: usize) {
    let value: ThreadBox<GeneralizedAutoDropDartPersistentHandle> = *box_from_leak_ptr(ptr as _);
    drop(value);
}
