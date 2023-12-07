use crate::for_generated::new_leak_box_ptr;
use crate::generalized_isolate::IntoDart;
use crate::platform_types::{DartAbi, WireSyncReturn};
use crate::rust2dart::action::Rust2DartAction;

/// An object that can be converted into `WireSyncReturn`
/// This object is safe (no worries about memory leak, etc), while `WireSyncReturn` is not.
/// That is why we have this intermediate object - we can safely play with this one.
pub struct WireSyncReturnSrc(DartAbi);

impl WireSyncReturnSrc {
    pub fn new(inner: DartAbi) -> Self {
        Self(inner)
    }

    pub fn leak(self) -> WireSyncReturn {
        #[cfg(not(wasm))]
        return new_leak_box_ptr(self.0);

        #[cfg(wasm)]
        return self.0;
    }
}
