use super::{BaseCodec, Rust2DartMessageTrait};
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use crate::rust2dart::action::Rust2DartAction;
use std::any::Any;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CstCodec;

impl BaseCodec for CstCodec {
    type Message = Rust2DartMessageCst;

    fn encode<T: IntoDart>(data: T, result_code: Rust2DartAction) -> Self::Message {
        unreachable!()
    }

    fn encode_panic(error: &Box<dyn Any + Send>) -> Self::Message {
        unreachable!()
    }
}

pub struct Rust2DartMessageCst(DartAbi);

impl Rust2DartMessageTrait for Rust2DartMessageCst {
    type WireSyncType = ();

    fn simplest() -> Self {
        unreachable!()
    }

    fn into_dart_abi(self) -> DartAbi {
        unreachable!()
    }

    unsafe fn from_raw_wire_sync(raw: Self::WireSyncType) -> Self {
        unreachable!()
    }

    fn into_raw_wire_sync(self) -> Self::WireSyncType {
        unreachable!()
    }
}
