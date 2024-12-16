use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::ty::delegate::rust_decode_primitive_enum;
use crate::codegen::generator::misc::is_js_value;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegateArray, MirTypeDelegateMap, MirTypeDelegatePrimitiveEnum,
    MirTypeDelegateSet, MirTypeDelegateTime,
};
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for DelegateWireRustCodecCstGenerator<'a> {
    fn generate_decoder_class(&self) -> Option<WireRustOutputCode> {
        None
    }

    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        match &self.mir {
            MirTypeDelegate::String => {
                Acc {
                    web: Some("self".into()),
                    io: Some("let vec: Vec<u8> = self.cst_decode(); String::from_utf8(vec).unwrap()".into()),
                    ..Default::default()
                }
            },
            MirTypeDelegate::Char => {
                Acc {
                    web: Some("CstDecode::<String>::cst_decode(self).chars().next().unwrap()".into()),
                    io: Some("CstDecode::<String>::cst_decode(self).chars().next().unwrap()".into()),
                    ..Default::default()
                }
            },
            // MirTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
            //     Acc::distribute(Some("flutter_rust_bridge::ZeroCopyBuffer(self.cst_decode())".into()))
            // },
            // MirTypeDelegate::StringList => general_list_impl_decode_body(),
            MirTypeDelegate::PrimitiveEnum (inner) => rust_decode_primitive_enum(inner, self.context.mir_pack, "self").into(),
            MirTypeDelegate::Time(mir) => {
                if mir == &MirTypeDelegateTime::Duration {
                    return Acc {
                        io: Some("chrono::Duration::microseconds(self)".into()),
                        web: None,
                        ..Default::default()
                    };
                }
                let codegen_timestamp = "let flutter_rust_bridge::for_generated::Timestamp { s, ns } = flutter_rust_bridge::for_generated::decode_timestamp(self);";
                let codegen_naive =
                    "chrono::DateTime::from_timestamp(s, ns).expect(\"invalid or out-of-range datetime\").naive_utc()";
                let codegen_utc = format!("chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset({codegen_naive}, chrono::Utc)");
                let codegen_local = format!("chrono::DateTime::<chrono::Local>::from({codegen_utc})");
                let codegen_conversion = match mir {
                    MirTypeDelegateTime::Naive => codegen_naive,
                    MirTypeDelegateTime::Utc => codegen_utc.as_str(),
                    MirTypeDelegateTime::Local => codegen_local.as_str(),
                    // frb-coverage:ignore-start
                    MirTypeDelegateTime::Duration => unreachable!(),
                    // frb-coverage:ignore-end
                };
                Acc {
                    common: Some(format!("{codegen_timestamp}{codegen_conversion}")),
                    ..Default::default()
                }
            },
            // MirTypeDelegate::TimeList(_) => {
            //     Acc::distribute(
            //         Some(
            //             "let vec: Vec<i64> = self.cst_decode(); vec.into_iter().map(CstDecode::cst_decode).collect()".into()
            //         )
            //     )
            // }
            MirTypeDelegate::Uuid => Acc::distribute(
                Some(
                    "let single: Vec<u8> = self.cst_decode(); flutter_rust_bridge::for_generated::decode_uuid(single)".into(),
                ),
            ),
            // MirTypeDelegate::Uuids => Acc::distribute(
            //     Some(
            //         "let multiple: Vec<u8> = self.cst_decode(); flutter_rust_bridge::for_generated::decode_uuids(multiple)".into(),
            //     ),
            // ),
            MirTypeDelegate::Backtrace | MirTypeDelegate::AnyhowException | MirTypeDelegate::DynTrait(_) => Acc::new(|target| match target {
                TargetOrCommon::Common => None,
                TargetOrCommon::Io | TargetOrCommon::Web => Some("unimplemented!()".into()),
            }),
            MirTypeDelegate::Array(array) => {
                self.generate_skip_web_if_jsvalue(generate_decode_array(array))
            },
            MirTypeDelegate::Map(mir) => self.generate_skip_web_if_jsvalue(generate_decode_map(mir)),
            MirTypeDelegate::Set(mir) => self.generate_skip_web_if_jsvalue(generate_decode_set(mir)),
            MirTypeDelegate::StreamSink(_) => Acc {
                web: Some("StreamSink::deserialize(self)".into()),
                io: Some("let raw: String = self.cst_decode(); StreamSink::deserialize(raw)".into()),
                ..Default::default()
            },
            MirTypeDelegate::BigPrimitive(_) => Acc::distribute(
                Some(
                    "CstDecode::<String>::cst_decode(self).parse().unwrap()".into(),
                ),
            ),
            MirTypeDelegate::RustAutoOpaqueExplicit(_) => Acc {
                io: Some("flutter_rust_bridge::for_generated::rust_auto_opaque_explicit_decode(self.cst_decode())".into()),
                ..Default::default()
            },
            // Do not care about these unimplemented things
            // frb-coverage:ignore-start
            MirTypeDelegate::ProxyVariant(_)
            | MirTypeDelegate::ProxyEnum(_) =>
                Acc::distribute(Some(r#"unimplemented!("Not implemented in this codec, please use the other one")"#.to_string())),
            MirTypeDelegate::CastedPrimitive(_)
            | MirTypeDelegate::CustomSerDes(_)
            | MirTypeDelegate::Lifetimeable(_) => Acc::distribute(None),
            // frb-coverage:ignore-end
        }
    }

    fn generate_impl_decode_jsvalue_body(&self) -> Option<std::borrow::Cow<str>> {
        Some(match &self.mir {
            MirTypeDelegate::String => {
                "self.as_string().expect(\"non-UTF-8 string, or not a string\")".into()
            }
            MirTypeDelegate::Char => {
                "CstDecode::<String>::cst_decode(self).chars().next().unwrap()".into()
            }
            MirTypeDelegate::PrimitiveEnum (MirTypeDelegatePrimitiveEnum { repr, .. }) => format!(
                "(self.unchecked_into_f64() as {}).cst_decode()",
                WireRustCodecCstGenerator::new(repr.clone(), self.context).rust_wire_type(Target::Web)
            )
                .into(),
            // MirTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
            //     "flutter_rust_bridge::ZeroCopyBuffer(self.cst_decode())".into()
            // }
            MirTypeDelegate::Time(mir) => match mir {
                MirTypeDelegateTime::Duration => "chrono::Duration::milliseconds(CstDecode::<i64>::cst_decode(self))".into(),
                _ => "CstDecode::<i64>::cst_decode(self).cst_decode()".into(),
            },
            // MirTypeDelegate::TimeList(_) =>
            //     "self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::BigInt64Array>().to_vec().into_iter().map(CstDecode::cst_decode).collect()".into(),
            MirTypeDelegate::Uuid /*| MirTypeDelegate::Uuids*/ => {
                "self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Uint8Array>().to_vec().into_boxed_slice().cst_decode()"
                    .into()
            }
            MirTypeDelegate::Backtrace | MirTypeDelegate::AnyhowException | MirTypeDelegate::DynTrait(_) => "unimplemented!()".into(),
            MirTypeDelegate::Array(array) => generate_decode_array(array)
                .into(),
            MirTypeDelegate::Map(mir) => generate_decode_map(mir).into(),
            MirTypeDelegate::Set(mir) => generate_decode_set(mir).into(),
            MirTypeDelegate::StreamSink(_) => "StreamSink::deserialize(self.as_string().expect(\"should be a string\"))".into(),
            MirTypeDelegate::BigPrimitive(_) => "CstDecode::<String>::cst_decode(self).parse().unwrap()".into(),
            MirTypeDelegate::RustAutoOpaqueExplicit(_) =>
                "flutter_rust_bridge::for_generated::rust_auto_opaque_explicit_decode(self.cst_decode())".into(),
            // Do not care about these unimplemented things
            // frb-coverage:ignore-start
            MirTypeDelegate::ProxyVariant(_) | MirTypeDelegate::ProxyEnum(_) =>
                r#"unimplemented!("Not implemented in this codec, please use the other one")"#.into(),
            MirTypeDelegate::CastedPrimitive(_)
            | MirTypeDelegate::CustomSerDes(_)
            | MirTypeDelegate::Lifetimeable(_) => return None,
            // frb-coverage:ignore-end
        })
    }

    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        Default::default()
    }

    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn rust_wire_type(&self, target: Target) -> String {
        // frb-coverage:ignore-end
        match (&self.mir, target) {
            (MirTypeDelegate::String, Target::Web) => "String".into(),
            // (MirTypeDelegate::StringList, Target::Io) => "wire_cst_StringList".to_owned(),
            // (MirTypeDelegate::StringList, Target::Web) => JS_VALUE.into(),
            _ => WireRustCodecCstGenerator::new(self.mir.get_delegate(), self.context)
                .rust_wire_type(target),
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        WireRustCodecCstGenerator::new(self.mir.get_delegate(), self.context)
            .rust_wire_is_pointer(target)
    }
}

impl<'a> DelegateWireRustCodecCstGenerator<'a> {
    fn generate_skip_web_if_jsvalue(&self, acc: String) -> Acc<Option<String>> {
        if is_js_value(&self.mir.get_delegate()) {
            Acc {
                io: Some(acc),
                ..Default::default()
            }
        } else {
            Acc::distribute(Some(acc))
        }
    }
}

fn generate_decode_array(array: &MirTypeDelegateArray) -> String {
    format!(
        "let vec: Vec<{}> = self.cst_decode(); flutter_rust_bridge::for_generated::from_vec_to_array(vec)",
        array.inner().rust_api_type()
    )
}

fn generate_decode_map(mir: &MirTypeDelegateMap) -> String {
    format!(
        "let vec: Vec<({}, {})> = self.cst_decode(); vec.into_iter().collect()",
        mir.key.rust_api_type(),
        mir.value.rust_api_type(),
    )
}

fn generate_decode_set(mir: &MirTypeDelegateSet) -> String {
    format!(
        "let vec: Vec<{}> = self.cst_decode(); vec.into_iter().collect()",
        mir.inner.rust_api_type()
    )
}
