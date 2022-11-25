use crate::Channel;
pub use crate::Dart_DeletePersistentHandle_DL_Trampolined;
pub use crate::Dart_HandleFromPersistent_DL_Trampolined;
pub use crate::Dart_NewPersistentHandle_DL_Trampolined;

pub use super::DartAbi;
pub use super::MessagePort;
pub use allo_isolate::*;
pub use dart_sys::Dart_Handle;
pub use dart_sys::Dart_PersistentHandle;
pub use dart_sys::_Dart_Handle;
use std::thread::ThreadId;

#[cfg(feature = "chrono")]
#[inline]
pub fn wire2api_timestamp(ts: i64) -> Timestamp {
    let s = (ts / 1_000_000) as i64;
    let ns = (ts.rem_euclid(1_000_000) * 1_000) as u32;
    Timestamp { s, ns }
}

/// a timestamp with microseconds precision
#[cfg(feature = "chrono")]
pub struct Timestamp {
    /// seconds
    pub s: i64,
    /// nanoseconds
    pub ns: u32,
}

#[cfg(test)]
#[cfg(feature = "chrono")]
mod tests {
    #[test]
    fn wire2api() {
        // input in microseconds
        let input: i64 = 3_496_567_123;
        let super::Timestamp { s, ns } = super::wire2api_timestamp(input);
        assert_eq!(s, 3_496);
        assert_eq!(ns, 567_123_000);
    }
}

#[derive(Debug)]
pub struct DartOpaqueBase {
    inner: Option<Dart_PersistentHandle>,
    port: MessagePort
}

impl DartOpaqueBase {
    pub fn new(handle: Dart_PersistentHandle, port: MessagePort) -> Self {
        Self { inner: Some(handle), port }
    }

    pub fn drop_ptr(&mut self) -> usize {
        self.inner.take().unwrap() as _
    }

    pub fn unwrap(mut self) -> Dart_PersistentHandle {
        self.inner.take().unwrap()
    }

    pub fn channel(&self) -> Channel {
        Channel::new(self.port)
    }
}

impl Drop for DartOpaqueBase {
    fn drop(&mut self) {
        if let Some(inner) = self.inner {
            unsafe { Dart_DeletePersistentHandle_DL_Trampolined(inner) }
        }
    }
}
