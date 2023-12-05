use crate::generalized_isolate::IntoDart;
use crate::platform_types::{DartAbi, SendableMessagePortHandle};
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
    /// The [DartObject] must be created on the current thread.
    pub unsafe fn new(handle: DartObject, drop_port: SendableMessagePortHandle) -> Self {
        Self {
            handle: Some(DartOpaqueBase::new(handle)),
            thread_id: std::thread::current().id(),
            drop_port,
        }
    }

    /// Creates a [DartOpaque] for sending to dart.
    ///
    /// # Safety
    ///
    /// The [DartObject] must be created on the current thread.
    ///
    /// The [DartOpaque] created by this method must not be dropped
    /// on a non-parent [DartObject] thread.
    pub unsafe fn new_non_droppable(handle: DartObject) -> Self {
        todo!("new_non_droppable")
        // Self {
        //     // TODO originally this was "dropport=none" while `new` was `dropport=some...`, but now no port at all
        //     // handle: Some(DartOpaqueBase::new(handle, None)),
        //     handle: Some(DartOpaqueBase::new(handle)),
        //     thread_id: std::thread::current().id(),
        // }
    }

    /// Tries to get a Dart [DartObject].
    /// Returns the [DartObject] if the [DartOpaque] was created on the current thread.
    pub fn try_unwrap(mut self) -> Result<DartWrapObject, Self> {
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
        if let Some(inner) = self.handle.take() {
            if std::thread::current().id() != self.thread_id {
                let channel = inner.channel();
                let ptr = inner.into_raw();

                if !channel.post(ptr) {
                    warn!("Drop DartOpaque after closing the port.");
                };
            }
        }
    }
}

#[cfg(not(wasm))]
pub type DartOpaqueWireType = *const std::ffi::c_void;
#[cfg(wasm)]
pub type DartOpaqueWireType = wasm_bindgen::JsValue;

// TODO improve
pub unsafe fn wire2api_dart_opaque(raw: DartOpaqueWireType) -> DartOpaque {
    DartOpaque::new(raw as _)
}
