use crate::for_generated::{
    box_from_leak_ptr, into_leak_vec_ptr, new_leak_box_ptr, vec_from_leak_ptr,
};
use crate::generalized_isolate::IntoDart;
use crate::platform_types::{
    DartAbi, WireSyncReturnDco, WireSyncReturnSse, WireSyncReturnSseStruct,
};
use crate::rust2dart::action::Rust2DartAction;

// TODO change comments
// TODO mv
/// An object that can be converted into `WireSyncReturn*`
/// This object is safe (no worries about memory leak, etc), while `WireSyncReturn` is not.
/// That is why we have this intermediate object - we can safely play with this one.
pub trait Rust2DartMessageTrait {
    type InnerType: IntoDart;
    type WireSyncType;

    fn new(inner: Self::InnerType) -> Self;

    fn simplest() -> Self;

    unsafe fn from_raw_wire_sync(raw: Self::WireSyncType) -> Self;

    fn into_raw_wire_sync(self) -> Self::WireSyncType;
}

pub struct WireSyncReturnCstWrapper(DartAbi);

impl Rust2DartMessageTrait for WireSyncReturnCstWrapper {
    type InnerType = ();
    type WireSyncType = ();

    fn new(inner: Self::InnerType) -> Self {
        unreachable!()
    }

    fn simplest() -> Self {
        unreachable!()
    }

    unsafe fn from_raw_wire_sync(raw: Self::WireSyncType) -> Self {
        unreachable!()
    }

    fn into_raw_wire_sync(self) -> Self::WireSyncType {
        unreachable!()
    }
}

pub struct WireSyncReturnDcoWrapper(DartAbi);

impl Rust2DartMessageTrait for WireSyncReturnDcoWrapper {
    type InnerType = DartAbi;
    type WireSyncType = WireSyncReturnDco;

    fn new(inner: Self::InnerType) -> Self {
        Self(inner)
    }

    fn simplest() -> Self {
        Self(().into_dart())
    }

    unsafe fn from_raw_wire_sync(raw: Self::WireSyncType) -> Self {
        Self::new(*box_from_leak_ptr(raw))
    }

    fn into_raw_wire_sync(self) -> Self::WireSyncType {
        #[cfg(not(wasm))]
        return new_leak_box_ptr(self.0);

        #[cfg(wasm)]
        return self.0;
    }
}

pub struct WireSyncReturnSseWrapper(Vec<u8>);

impl Rust2DartMessageTrait for WireSyncReturnSseWrapper {
    type InnerType = Vec<u8>;
    type WireSyncType = WireSyncReturnSse;

    fn new(inner: Self::InnerType) -> Self {
        Self(inner)
    }

    fn simplest() -> Self {
        Self(vec![])
    }

    unsafe fn from_raw_wire_sync(raw: Self::WireSyncType) -> Self {
        let WireSyncReturnSseStruct { ptr, len } = raw;
        Self(vec_from_leak_ptr(ptr, len))
    }

    fn into_raw_wire_sync(self) -> Self::WireSyncType {
        #[cfg(not(wasm))]
        {
            let (ptr, len) = into_leak_vec_ptr(self.0);
            return WireSyncReturnSseStruct { ptr, len };
        }

        #[cfg(wasm)]
        return self.0;
    }
}
