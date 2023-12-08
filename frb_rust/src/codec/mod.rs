use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::wire_sync_return_src::WireSyncReturnSrcTrait;

pub(crate) mod cst;
pub(crate) mod dco;
pub(crate) mod sse;

pub(crate) trait BaseCodec: Clone + Copy {
    type WireSyncReturnSrc: WireSyncReturnSrcTrait;

    fn encode<T: IntoDart>(data: T, result_code: Rust2DartAction) -> DartAbi;
}
