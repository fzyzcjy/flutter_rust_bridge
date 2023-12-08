use crate::for_generated::new_leak_box_ptr;
use crate::generalized_isolate::IntoDart;
use crate::platform_types::{DartAbi, WireSyncReturnDco};
use crate::rust2dart::action::Rust2DartAction;

/// An object that can be converted into `WireSyncReturn*`
/// This object is safe (no worries about memory leak, etc), while `WireSyncReturn` is not.
/// That is why we have this intermediate object - we can safely play with this one.
pub trait WireSyncReturnSrc {
    fn new(inner: DartAbi) -> Self;

    fn into_raw(self) -> WireSyncReturnDco;
}

pub struct WireSyncReturnDcoSrc(DartAbi);

impl WireSyncReturnSrc for WireSyncReturnDcoSrc {
    fn new(inner: DartAbi) -> Self {
        Self(inner)
    }

    fn into_raw(self) -> WireSyncReturnDco {
        #[cfg(not(wasm))]
        return new_leak_box_ptr(self.0);

        #[cfg(wasm)]
        return self.0;
    }
}
