use super::{BaseCodec, Rust2DartMessageTrait};
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use crate::rust2dart::action::Rust2DartAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CstCodec;

impl BaseCodec for CstCodec {
    type Rust2DartMessage = Rust2DartMessageCst;

    fn encode<T: IntoDart>(data: T, result_code: Rust2DartAction) -> () {
        unreachable!()
    }
}

pub struct Rust2DartMessageCst(DartAbi);

impl Rust2DartMessageTrait for Rust2DartMessageCst {
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
