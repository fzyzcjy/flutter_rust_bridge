use crate::for_generated::{
    box_from_leak_ptr, into_leak_vec_ptr, new_leak_box_ptr, vec_from_leak_ptr,
};
use crate::generalized_isolate::IntoDart;
use crate::platform_types::{
    DartAbi, WireSyncReturnDco, WireSyncReturnSse, WireSyncReturnSseStruct,
};
use crate::rust2dart::action::Rust2DartAction;

/// An object that can be converted into `WireSyncReturn*`
/// This object is safe (no worries about memory leak, etc), while `WireSyncReturn` is not.
/// That is why we have this intermediate object - we can safely play with this one.
pub trait WireSyncReturnSrcTrait {
    type Target;

    fn new(inner: DartAbi) -> Self;

    unsafe fn from_raw(raw: Self::Target) -> Self;

    fn into_raw(self) -> Self::Target;
}

pub struct WireSyncReturnDcoSrc(DartAbi);

impl WireSyncReturnSrcTrait for WireSyncReturnDcoSrc {
    type Target = WireSyncReturnDco;

    fn new(inner: DartAbi) -> Self {
        Self(inner)
    }

    unsafe fn from_raw(raw: Self::Target) -> Self {
        Self::new(*box_from_leak_ptr(raw))
    }

    fn into_raw(self) -> Self::Target {
        #[cfg(not(wasm))]
        return new_leak_box_ptr(self.0);

        #[cfg(wasm)]
        return self.0;
    }
}

pub struct WireSyncReturnSseSrc(Vec<u8>);

impl WireSyncReturnSrcTrait for WireSyncReturnSseSrc {
    type Target = WireSyncReturnSse;

    fn new(inner: DartAbi) -> Self {
        todo!()
    }

    unsafe fn from_raw(raw: Self::Target) -> Self {
        let WireSyncReturnSseStruct { ptr, len } = raw;
        Self(vec_from_leak_ptr(ptr, len))
    }

    fn into_raw(self) -> Self::Target {
        #[cfg(not(wasm))]
        {
            let (ptr, len) = into_leak_vec_ptr(self.0);
            return WireSyncReturnSseStruct { ptr, len };
        }

        #[cfg(wasm)]
        return self.0;
    }
}
