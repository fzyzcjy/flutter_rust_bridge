use super::{BaseCodec, Rust2DartMessageTrait};
use crate::generalized_isolate::IntoDart;
use crate::handler::error::error_to_string;
use crate::misc::into_into_dart::IntoIntoDart;
use crate::platform_types::{DartAbi, WireSyncRust2DartDco};
use crate::rust2dart::action::Rust2DartAction;
use std::any::Any;
use std::backtrace::Backtrace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DcoCodec;

impl BaseCodec for DcoCodec {
    type Message = Rust2DartMessageDco;

    fn encode_panic(error: &Box<dyn Any + Send>, backtrace: &Option<Backtrace>) -> Self::Message {
        Self::encode(Rust2DartAction::Panic, error_to_string(error, backtrace))
    }

    fn encode_close_stream() -> Self::Message {
        Rust2DartMessageDco(vec![Rust2DartAction::CloseStream.into_dart()].into_dart())
    }
}

impl DcoCodec {
    pub fn encode<T: IntoDart>(result_code: Rust2DartAction, data: T) -> Rust2DartMessageDco {
        Rust2DartMessageDco(vec![result_code.into_dart(), data.into_dart()].into_dart())
    }
}

pub struct Rust2DartMessageDco(DartAbi);

impl Rust2DartMessageTrait for Rust2DartMessageDco {
    type WireSyncRust2DartType = WireSyncRust2DartDco;

    fn simplest() -> Self {
        Self(().into_dart())
    }

    fn into_dart_abi(self) -> DartAbi {
        self.0
    }

    unsafe fn from_raw_wire_sync(raw: Self::WireSyncRust2DartType) -> Self {
        #[cfg(not(wasm))]
        return Self(*crate::for_generated::box_from_leak_ptr(raw));

        #[cfg(wasm)]
        return Self(raw);
    }

    fn into_raw_wire_sync(self) -> Self::WireSyncRust2DartType {
        #[cfg(not(wasm))]
        return crate::for_generated::new_leak_box_ptr(self.0);

        #[cfg(wasm)]
        return self.0;
    }
}

pub fn transform_result_dco<T, T2, E>(
    raw: Result<T, E>,
) -> Result<Rust2DartMessageDco, Rust2DartMessageDco>
where
    T: IntoIntoDart<T2>,
    T2: IntoDart,
    E: IntoDart,
{
    match raw {
        Ok(raw) => Ok(DcoCodec::encode(
            Rust2DartAction::Success,
            raw.into_into_dart(),
        )),
        Err(raw) => Err(DcoCodec::encode(Rust2DartAction::Error, raw)),
    }
}

#[cfg(test)]
mod tests {

    #[cfg(not(wasm))]
    #[test]
    fn test_simplest() {
        use crate::codec::Rust2DartMessageTrait;
        use crate::for_generated::Rust2DartMessageDco;
        use allo_isolate::ffi::DartCObjectType;
        assert_eq!(
            Rust2DartMessageDco::simplest().0.ty,
            DartCObjectType::DartNull
        );
    }
}
