use super::BaseCodec;
use crate::generalized_isolate::IntoDart;
use crate::platform_types::{DartAbi, WireSyncReturnDco};
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::wire_sync_return_src::Rust2DartMessageDco;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DcoCodec;

impl BaseCodec for DcoCodec {
    type Rust2DartMessage = Rust2DartMessageDco;

    fn encode<T: IntoDart>(data: T, result_code: Rust2DartAction) -> DartAbi {
        if result_code == Rust2DartAction::CloseStream {
            vec![result_code.into_dart()].into_dart()
        } else {
            vec![result_code.into_dart(), data.into_dart()].into_dart()
        }
    }
}

pub struct Rust2DartMessageDco(DartAbi);

impl Rust2DartMessageTrait for Rust2DartMessageDco {
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
