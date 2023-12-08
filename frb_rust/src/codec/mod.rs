use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::wire_sync_return_src::Rust2DartMessageTrait;

pub(crate) mod cst;
pub(crate) mod dco;
pub(crate) mod sse;

pub trait BaseCodec: Clone + Copy {
    type Rust2DartMessage: Rust2DartMessageTrait;

    fn encode<T: IntoDart>(
        data: T,
        result_code: Rust2DartAction,
    ) -> <Self::Rust2DartMessage as Rust2DartMessageTrait>::InnerType;
}

/// An encoded message
pub trait Rust2DartMessageTrait {
    type InnerType: IntoDart;
    type WireSyncType;

    fn new(inner: Self::InnerType) -> Self;

    fn simplest() -> Self;

    unsafe fn from_raw_wire_sync(raw: Self::WireSyncType) -> Self;

    fn into_raw_wire_sync(self) -> Self::WireSyncType;
}
