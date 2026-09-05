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
        #[cfg(not(target_family = "wasm"))]
        return Self(*crate::for_generated::box_from_leak_ptr(raw));

        #[cfg(target_family = "wasm")]
        return Self(raw);
    }

    fn into_raw_wire_sync(self) -> Self::WireSyncRust2DartType {
        #[cfg(not(target_family = "wasm"))]
        return crate::for_generated::new_leak_box_ptr(self.0);

        #[cfg(target_family = "wasm")]
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
    use super::{transform_result_dco, DcoCodec, Rust2DartMessageDco};
    use crate::codec::{BaseCodec, Rust2DartMessageTrait};
    use crate::for_generated::{Rust2DartAction, WireSyncRust2DartDco};

    fn assert_array_action_and_i32_payload(
        message: &Rust2DartMessageDco,
        expected_action: Rust2DartAction,
        expected_payload: i32,
    ) {
        use allo_isolate::ffi::DartCObjectType;

        assert_eq!(message.0.ty, DartCObjectType::DartArray);
        let array = unsafe { message.0.value.as_array };
        assert_eq!(array.length, 2);
        let values = unsafe { std::slice::from_raw_parts(array.values, array.length as usize) };
        let action = unsafe { &*values[0] };
        let payload = unsafe { &*values[1] };

        assert_eq!(action.ty, DartCObjectType::DartInt32);
        assert_eq!(unsafe { action.value.as_int32 }, expected_action as i32);
        assert_eq!(payload.ty, DartCObjectType::DartInt32);
        assert_eq!(unsafe { payload.value.as_int32 }, expected_payload);
    }

    #[cfg(not(target_family = "wasm"))]
    /// Encodes the minimal DCO payload as Dart null.
    #[test]
    fn test_simplest() {
        use allo_isolate::ffi::DartCObjectType;
        assert_eq!(
            Rust2DartMessageDco::simplest().0.ty,
            DartCObjectType::DartNull
        );
    }

    #[cfg(not(target_family = "wasm"))]
    /// Preserves a DCO action and payload through synchronous ownership transfer.
    #[test]
    fn test_wire_sync_round_trip_preserves_payload_type() {
        let raw: WireSyncRust2DartDco =
            DcoCodec::encode(Rust2DartAction::Success, 42_i32).into_raw_wire_sync();
        let message = unsafe { Rust2DartMessageDco::from_raw_wire_sync(raw) };

        assert_array_action_and_i32_payload(&message, Rust2DartAction::Success, 42);
    }

    #[cfg(not(target_family = "wasm"))]
    /// Maps successful and failed results to distinct DCO message arrays.
    #[test]
    fn test_transform_result_encodes_both_result_variants() {
        let success = match transform_result_dco::<_, i32, _>(Ok::<i32, i32>(7)) {
            Ok(message) => message,
            Err(_) => panic!("success result must encode as success"),
        };
        let failure = match transform_result_dco::<_, i32, _>(Err::<i32, i32>(9)) {
            Ok(_) => panic!("error result must encode as error"),
            Err(message) => message,
        };

        assert_array_action_and_i32_payload(&success, Rust2DartAction::Success, 7);
        assert_array_action_and_i32_payload(&failure, Rust2DartAction::Error, 9);
    }

    #[cfg(not(target_family = "wasm"))]
    /// Encodes stream closure as an action-only DCO array.
    #[test]
    fn test_close_stream_encodes_array() {
        use allo_isolate::ffi::DartCObjectType;

        let message = DcoCodec::encode_close_stream();
        assert_eq!(message.0.ty, DartCObjectType::DartArray);
        let array = unsafe { message.0.value.as_array };
        assert_eq!(array.length, 1);
        let action = unsafe { &**array.values };
        assert_eq!(action.ty, DartCObjectType::DartInt32);
        assert_eq!(
            unsafe { action.value.as_int32 },
            Rust2DartAction::CloseStream as i32
        );
    }
}
