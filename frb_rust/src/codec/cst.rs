use super::{BaseCodec, Rust2DartMessageTrait};
use crate::platform_types::DartAbi;
use std::any::Any;

#[cfg_attr(coverage_nightly, coverage(off))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CstCodec;

impl BaseCodec for CstCodec {
    type Message = Rust2DartMessageCst;

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn encode_panic(_error: &Box<dyn Any + Send>) -> Self::Message {
        unreachable!()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn encode_close_stream() -> Self::Message {
        unreachable!()
    }
}

pub struct Rust2DartMessageCst(DartAbi);

impl Rust2DartMessageTrait for Rust2DartMessageCst {
    type WireSyncRust2DartType = ();

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn simplest() -> Self {
        unreachable!()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn into_dart_abi(self) -> DartAbi {
        unreachable!()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    unsafe fn from_raw_wire_sync(_raw: Self::WireSyncRust2DartType) -> Self {
        unreachable!()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn into_raw_wire_sync(self) -> Self::WireSyncRust2DartType {
        unreachable!()
    }
}
