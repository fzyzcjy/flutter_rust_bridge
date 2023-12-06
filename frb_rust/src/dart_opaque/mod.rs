use crate::generalized_isolate::{Channel, IntoDart};
use crate::platform_types::{handle_to_message_port, DartAbi, SendableMessagePortHandle};
use log::warn;
use std::sync::Arc;
use std::thread::ThreadId;

#[cfg(wasm)]
mod web;
#[cfg(wasm)]
pub use web::*;

#[cfg(not(wasm))]
mod io;
use crate::dart_opaque::thread_box::ThreadBox;
use crate::for_generated::{box_from_leak_ptr, new_leak_box_ptr};
#[cfg(not(wasm))]
pub use io::*;

#[cfg(not(wasm))]
mod auto_drop_dart_persistent_handle;

pub(crate) mod dart2rust;
pub(crate) mod rust2dart;
mod thread_box;

/// Arbitrary Dart object, whose type can be even non-encodable and non-transferable.
// Implementation: Just [DartOpaqueInner] + Arc, in order to support `clone`
#[derive(Debug)]
pub struct DartOpaque {
    // TODO `Arc` is for `DartOpaque` to be clone-able.
    //      When users do not need clone (e.g. NOT used in a DartFn that is called multiple times),
    //      we can generate and use the non-Arc version to speed up.
    arc: Arc<DartOpaqueInner>,
}

impl DartOpaque {
    pub fn new(handle: GeneralizedDartHandle, drop_port: SendableMessagePortHandle) -> Self {
        Self {
            arc: Arc::new(DartOpaqueInner::new(handle, drop_port)),
        }
    }

    pub fn into_inner(mut self) -> Result<GeneralizedAutoDropDartPersistentHandle, Self> {
        let inner = Arc::try_unwrap(self.arc).map_err(|x| Self { arc: x })?;
        Ok(inner.into_inner())
    }

    fn create_dart_handle(&self) -> GeneralizedDartHandle {
        self.arc.create_dart_handle()
    }
}

#[derive(Debug)]
struct DartOpaqueInner {
    /// The internal persistent handle
    // `Option` is used for correct drop.
    persistent_handle: Option<ThreadBox<GeneralizedAutoDropDartPersistentHandle>>,

    /// The port to drop object (when we cannot drop in current thread)
    drop_port: SendableMessagePortHandle,
}

impl DartOpaqueInner {
    fn new(handle: GeneralizedDartHandle, drop_port: SendableMessagePortHandle) -> Self {
        let auto_drop_persistent_handle =
            GeneralizedAutoDropDartPersistentHandle::new_from_non_persistent_handle(handle);
        Self {
            persistent_handle: Some(ThreadBox::new(auto_drop_persistent_handle)),
            drop_port,
        }
    }

    fn into_inner(mut self) -> GeneralizedAutoDropDartPersistentHandle {
        // Though inner ThreadBox has a check, we still check here
        // to avoid (auto) invoking ThreadBox.drop during its panicking,
        // which causes either leak or abort.
        // In addition, here we have more user friendly error message.
        if !(self.persistent_handle.as_ref().unwrap()).is_on_creation_thread() {
            panic!("DartOpaque can only be used on the creation thread");
        }

        self.persistent_handle.take().unwrap().into_inner()
    }

    fn create_dart_handle(&self) -> GeneralizedDartHandle {
        (self.persistent_handle.as_ref().unwrap().as_ref()).create_dart_handle()
    }
}

impl Drop for DartOpaqueInner {
    fn drop(&mut self) {
        if let Some(persistent_handle) = self.persistent_handle.take() {
            // If we forget to do so, ThreadBox will panic because it requires things to be dropped on creation thread
            if !persistent_handle.is_on_creation_thread() {
                drop_thread_box_persistent_handle_via_port(persistent_handle, &self.drop_port)
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

    if !channel.post(ptr) {
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
