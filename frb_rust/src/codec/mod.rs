use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use crate::rust2dart::action::Rust2DartAction;

pub(crate) mod cst;
pub(crate) mod dco;
pub(crate) mod sse;

pub(crate) trait BaseCodec: Clone + Copy {
    fn encode<T: IntoDart>(data: T, result_code: Rust2DartAction) -> DartAbi;
}
