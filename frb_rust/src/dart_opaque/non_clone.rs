use super::{
    GeneralizedAutoDropDartPersistentHandle, GeneralizedDartHandle, GeneralizedDartHandleBox,
};
use crate::dart_opaque::action::DartHandlerPortAction;
use crate::for_generated::{box_from_leak_ptr, new_leak_box_ptr};
use crate::generalized_isolate::DartSendPort;
use crate::generalized_isolate::IntoDart;
use crate::misc::logs::log_warn_or_println;
use crate::platform_types::DartNativeSendPort;
#[cfg(wasm)]
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub(super) struct DartOpaqueNonClone {
    /// The internal persistent handle
    // `Option` is used for correct drop.
    persistent_handle: Option<GeneralizedDartHandleBox<GeneralizedAutoDropDartPersistentHandle>>,

    /// The port to drop object (when we cannot drop in current thread)
    dart_handler_port: DartNativeSendPort,
}

impl DartOpaqueNonClone {
    pub(super) fn new(
        handle: GeneralizedDartHandle,
        dart_handler_port: DartNativeSendPort,
    ) -> Self {
        let auto_drop_persistent_handle =
            GeneralizedAutoDropDartPersistentHandle::new_from_non_persistent_handle(handle);
        Self {
            persistent_handle: Some(GeneralizedDartHandleBox::new(auto_drop_persistent_handle)),
            dart_handler_port,
        }
    }

    pub(super) fn into_inner(mut self) -> GeneralizedAutoDropDartPersistentHandle {
        // Though inner GeneralizedDartHandleBox has a check, we still check here
        // to avoid (auto) invoking GeneralizedDartHandleBox.drop during its panicking,
        // which causes either leak or abort.
        // In addition, here we have more user friendly error message.
        if !(self.persistent_handle.as_ref().unwrap()).check_context() {
            panic!("DartOpaque can only be used on the creation thread");
        }

        self.persistent_handle.take().unwrap().into_inner()
    }

    pub(super) fn create_dart_handle(&self) -> GeneralizedDartHandle {
        (self.persistent_handle.as_ref().unwrap().as_ref()).create_dart_handle()
    }

    pub(crate) fn dart_handler_port(&self) -> &DartNativeSendPort {
        &self.dart_handler_port
    }
}

impl Drop for DartOpaqueNonClone {
    fn drop(&mut self) {
        if let Some(persistent_handle) = self.persistent_handle.take() {
            // If we forget to do so, GeneralizedDartHandleBox will panic because it requires things to be dropped on creation thread
            if !persistent_handle.check_context() {
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
    persistent_handle: GeneralizedDartHandleBox<GeneralizedAutoDropDartPersistentHandle>,
    dart_handler_port: &DartNativeSendPort,
) {
    let port = DartSendPort::new(dart_handler_port.to_owned());
    let ptr = new_leak_box_ptr(persistent_handle) as usize;

    let msg_creator = move || {
        [
            DartHandlerPortAction::DartOpaqueDrop.into_dart(),
            ptr.into_dart(),
        ]
    };

    if !port.post(msg_creator) {
        // We do not care about the detailed error message
        // frb-coverage:ignore-start
        log_warn_or_println(
            "Drop DartOpaque after closing the port, thus the object will be leaked forever.",
        );
        // frb-coverage:ignore-end
    };
}

#[cfg(not(wasm))]
#[no_mangle]
pub unsafe extern "C" fn dart_opaque_drop_thread_box_persistent_handle(ptr: usize) {
    dart_opaque_drop_thread_box_persistent_handle_inner(ptr)
}

#[cfg(wasm)]
#[wasm_bindgen]
pub unsafe extern "C" fn dart_opaque_drop_thread_box_persistent_handle(ptr: usize) {
    dart_opaque_drop_thread_box_persistent_handle_inner(ptr)
}

unsafe fn dart_opaque_drop_thread_box_persistent_handle_inner(ptr: usize) {
    let value: GeneralizedDartHandleBox<GeneralizedAutoDropDartPersistentHandle> =
        *box_from_leak_ptr(ptr as _);
    drop(value);
}
