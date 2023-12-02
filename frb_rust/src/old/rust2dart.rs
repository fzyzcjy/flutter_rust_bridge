//! Manages receiving and sending values across the FFI boundary.

pub use crate::ffi::*;
pub use crate::into_into_dart::IntoIntoDart;
use std::marker::PhantomData;

/// A handle to a [`web_sys::BroadcastChannel`].
#[derive(Clone)]
pub struct ChannelHandle(pub String);

impl ChannelHandle {
    #[cfg(wasm)]
    pub fn port(&self) -> MessagePort {
        PortLike::broadcast(&self.0)
    }
}

