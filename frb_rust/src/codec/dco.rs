use super::{BaseCodec, Rust2DartMessageTrait};
use crate::for_generated::{box_from_leak_ptr, new_leak_box_ptr};
use crate::generalized_isolate::IntoDart;
use crate::platform_types::{DartAbi, WireSyncReturnDco};
use crate::rust2dart::action::Rust2DartAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DcoCodec;

impl BaseCodec for DcoCodec {
    type Message = Rust2DartMessageDco;

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
    type WireSyncType = WireSyncReturnDco;

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
