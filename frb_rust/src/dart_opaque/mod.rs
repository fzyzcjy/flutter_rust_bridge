use crate::generalized_isolate::{Channel, IntoDart};
use crate::platform_types::{handle_to_message_port, DartAbi, SendableMessagePortHandle};
use dart_sys::Dart_Handle;
use dart_sys::Dart_NewPersistentHandle_DL;
use log::warn;
use std::thread::ThreadId;

#[cfg(wasm)]
mod web;
#[cfg(wasm)]
pub use web::*;

#[cfg(not(wasm))]
mod io;
use crate::dart_opaque::thread_box::ThreadBox;
#[cfg(not(wasm))]
pub use io::*;

#[cfg(not(wasm))]
mod auto_drop_dart_persistent_handle;

mod thread_box;

/// Arbitrary Dart object, whose type can be even non-encodable and non-transferable.
#[derive(Debug)]
pub struct DartOpaque {
    /// The internal persistent handle
    persistent_handle: ThreadBox<GeneralizedAutoDropDartPersistentHandle>,

    /// The port to drop object (when we cannot drop in current thread)
    drop_port: SendableMessagePortHandle,
}

impl DartOpaque {
    pub fn new(handle: Dart_Handle, drop_port: SendableMessagePortHandle) -> Self {
        let auto_drop_persistent_handle =
            GeneralizedAutoDropDartPersistentHandle::new_from_non_persistent_handle(handle);
        Self {
            persistent_handle: ThreadBox::new(auto_drop_persistent_handle),
            drop_port,
        }
    }
}

// TODO the api2wire side: just send the object itself, nothing more
pub unsafe fn wire2api_dart_opaque(
    raw: Dart_Handle,
    drop_port: SendableMessagePortHandle,
) -> DartOpaque {
    DartOpaque::new(raw, drop_port)
}

// TODO things below not migrated yet --------------------------------------------------------

// TODO rm
// impl DartOpaque {
//     /// Creates a new [DartOpaque].
//     ///
//     /// # Safety
//     ///
//     /// The [GeneralizedDartPersistentHandle] must be created on the current thread.
//     pub unsafe fn new(
//         handle: GeneralizedDartPersistentHandle,
//         drop_port: SendableMessagePortHandle,
//     ) -> Self {
//         Self {
//             handle: Some(DartOpaqueBase::new(handle)),
//             drop_port,
//         }
//     }
// }

impl From<DartOpaque> for DartAbi {
    fn from(mut data: DartOpaque) -> Self {
        data.persistent_handle
            .take()
            .unwrap()
            .into_raw()
            .into_dart()
    }
}

impl Drop for DartOpaque {
    fn drop(&mut self) {
        // TODO about thread
        if let Some(inner) = self.persistent_handle.take() {
            if std::thread::current().id() != self.thread_id {
                let channel = Channel::new(handle_to_message_port(&self.drop_port));
                let ptr = inner.into_raw();

                if !channel.post(ptr) {
                    warn!("Drop DartOpaque after closing the port, thus the object will be leaked forever.");
                    // In case logs are disabled
                    println!("Drop DartOpaque after closing the port, thus the object will be leaked forever.");
                };
            }
        }
    }
}

#[cfg(not(wasm))]
pub type DartOpaqueWireType = *const std::ffi::c_void;
#[cfg(wasm)]
pub type DartOpaqueWireType = wasm_bindgen::JsValue;

// TODO rm
// pub unsafe fn wire2api_dart_opaque(
//     raw: DartOpaqueWireType,
//     drop_port: SendableMessagePortHandle,
// ) -> DartOpaque {
//     DartOpaque::new(raw as _, drop_port)
// }
