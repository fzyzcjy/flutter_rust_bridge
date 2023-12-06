use crate::generalized_isolate::{Channel, IntoDart};
use crate::platform_types::{handle_to_message_port, DartAbi, SendableMessagePortHandle};
use log::warn;
use std::thread::ThreadId;

#[cfg(wasm)]
mod web;
#[cfg(wasm)]
pub use web::*;

#[cfg(not(wasm))]
mod io;
#[cfg(not(wasm))]
pub use io::*;

#[cfg(not(wasm))]
mod dart_persistent_handle_auto_drop;

/// Arbitrary Dart object, whose type can be even non-encodable and non-transferable.
#[derive(Debug)]
pub struct DartOpaque {
    /// Dart object
    handle: Option<DartOpaqueBase>,

    /// The ID of the thread on which the Dart Object was created.
    thread_id: ThreadId,
    /// The port to drop object (when we cannot drop in current thread)
    drop_port: SendableMessagePortHandle,
}

/// # Safety
///
/// The implementation checks the current thread
/// and delegates it to the Dart thread when it is drops.
unsafe impl Send for DartOpaque {}
unsafe impl Sync for DartOpaque {}

impl DartOpaque {
    /// Creates a new [DartOpaque].
    ///
    /// # Safety
    ///
    /// The [GeneralizedDartPersistentHandle] must be created on the current thread.
    pub unsafe fn new(
        handle: GeneralizedDartPersistentHandle,
        drop_port: SendableMessagePortHandle,
    ) -> Self {
        Self {
            handle: Some(DartOpaqueBase::new(handle)),
            thread_id: std::thread::current().id(),
            drop_port,
        }
    }

    /// Tries to get a Dart [GeneralizedDartPersistentHandle].
    /// Returns the [GeneralizedDartPersistentHandle] if the [DartOpaque] was created on the current thread.
    pub fn try_unwrap(mut self) -> Result<GeneralizedDartPersistentHandleWrapper, Self> {
        if std::thread::current().id() == self.thread_id {
            Ok(self.handle.take().unwrap().unwrap())
        } else {
            Err(self)
        }
    }
}

impl From<DartOpaque> for DartAbi {
    fn from(mut data: DartOpaque) -> Self {
        data.handle.take().unwrap().into_raw().into_dart()
    }
}

impl Drop for DartOpaque {
    fn drop(&mut self) {
        println!("hi DartOpaque.drop START self={self:?}");
        if let Some(inner) = self.handle.take() {
            println!("hi DartOpaque.drop has inner");
            if std::thread::current().id() != self.thread_id {
                println!("hi DartOpaque.drop not same thread");
                let channel = Channel::new(handle_to_message_port(&self.drop_port));
                let ptr = inner.into_raw();

                if !channel.post(ptr) {
                    warn!("Drop DartOpaque after closing the port, thus the object will be leaked forever.");
                    // In case logs are disabled
                    println!("Drop DartOpaque after closing the port, thus the object will be leaked forever.");
                };
            }
        }
        println!("hi DartOpaque.drop END");
    }
}

#[cfg(not(wasm))]
pub type DartOpaqueWireType = *const std::ffi::c_void;
#[cfg(wasm)]
pub type DartOpaqueWireType = wasm_bindgen::JsValue;

// TODO improve
pub unsafe fn wire2api_dart_opaque(
    raw: DartOpaqueWireType,
    drop_port: SendableMessagePortHandle,
) -> DartOpaque {
    DartOpaque::new(raw as _, drop_port)
}
