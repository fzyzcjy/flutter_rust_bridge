use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::ty::delegate::rust_decode_primitive_enum;
use crate::codegen::generator::misc::is_js_value;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateArray, IrTypeDelegateMap, IrTypeDelegatePrimitiveEnum,
    IrTypeDelegateSet, IrTypeDelegateTime,
};
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for DelegateWireRustCodecCstGenerator<'a> {
    fn generate_decoder_class(&self) -> Option<WireRustOutputCode> {
        None
    }

    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        match &self.ir {
            IrTypeDelegate::String => {
                Acc {
                    web: Some("self".into()),
                    io: Some("let vec: Vec<u8> = self.cst_decode(); String::from_utf8(vec).unwrap()".into()),
                    ..Default::default()
                }
            },
            IrTypeDelegate::Char => {
                Acc {
                    web: Some("CstDecode::<String>::cst_decode(self).chars().next().unwrap()".into()),
                    io: Some("CstDecode::<String>::cst_decode(self).chars().next().unwrap()".into()),
                    ..Default::default()
                }
            },
            // IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
            //     Acc::distribute(Some("flutter_rust_bridge::ZeroCopyBuffer(self.cst_decode())".into()))
            // },
            // IrTypeDelegate::StringList => general_list_impl_decode_body(),
            IrTypeDelegate::PrimitiveEnum (inner) => rust_decode_primitive_enum(inner, self.context.ir_pack, "self").into(),
            IrTypeDelegate::Time(ir) => {
                if ir == &IrTypeDelegateTime::Duration {
                    return Acc {
                        io: Some("chrono::Duration::microseconds(self)".into()),
                        web: None,
                        ..Default::default()
                    };
                }
                let codegen_timestamp = "let flutter_rust_bridge::for_generated::Timestamp { s, ns } = flutter_rust_bridge::for_generated::decode_timestamp(self);";
                let codegen_naive_date_time =
                    "chrono::DateTime::from_timestamp(s, ns).expect(\"invalid or out-of-range datetime\").naive_utc()";
                let codegen_naive_date = format!("{codegen_naive_date_time}.date()");
                let codegen_utc = format!("chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset({codegen_naive_date_time}, chrono::Utc)");
                let codegen_local = format!("chrono::DateTime::<chrono::Local>::from({codegen_utc})");
                let codegen_conversion = match ir {
                    IrTypeDelegateTime::NaiveDate => codegen_naive_date.as_str(),
                    IrTypeDelegateTime::NaiveDateTime => codegen_naive_date_time,
                    IrTypeDelegateTime::Utc => codegen_utc.as_str(),
                    IrTypeDelegateTime::Local => codegen_local.as_str(),
                    // frb-coverage:ignore-start
                    IrTypeDelegateTime::Duration => unreachable!(),
                    // frb-coverage:ignore-end
                };
                Acc {
                    common: Some(format!("{codegen_timestamp}{codegen_conversion}")),
                    ..Default::default()
                }
            },
            // IrTypeDelegate::TimeList(_) => {
            //     Acc::distribute(
            //         Some(
            //             "let vec: Vec<i64> = self.cst_decode(); vec.into_iter().map(CstDecode::cst_decode).collect()".into()
            //         )
            //     )
            // }
            IrTypeDelegate::Uuid => Acc::distribute(
                Some(
                    "let single: Vec<u8> = self.cst_decode(); flutter_rust_bridge::for_generated::decode_uuid(single)".into(),
                ),
            ),
            // IrTypeDelegate::Uuids => Acc::distribute(
            //     Some(
            //         "let multiple: Vec<u8> = self.cst_decode(); flutter_rust_bridge::for_generated::decode_uuids(multiple)".into(),
            //     ),
            // ),
            IrTypeDelegate::Backtrace | IrTypeDelegate::AnyhowException => Acc::new(|target| match target {
                TargetOrCommon::Common => None,
                TargetOrCommon::Io | TargetOrCommon::Web => Some("unimplemented!()".into()),
            }),
            IrTypeDelegate::Array(array) => {
                self.generate_skip_web_if_jsvalue(generate_decode_array(array))
            },
            IrTypeDelegate::Map(ir) => self.generate_skip_web_if_jsvalue(generate_decode_map(ir)),
            IrTypeDelegate::Set(ir) => self.generate_skip_web_if_jsvalue(generate_decode_set(ir)),
            IrTypeDelegate::StreamSink(_) => Acc {
                web: Some("StreamSink::deserialize(self)".into()),
                io: Some("let raw: String = self.cst_decode(); StreamSink::deserialize(raw)".into()),
                ..Default::default()
            },
            IrTypeDelegate::BigPrimitive(_) => Acc::distribute(
                Some(
                    "CstDecode::<String>::cst_decode(self).parse().unwrap()".into(),
                ),
            ),
            IrTypeDelegate::RustAutoOpaqueExplicit(_) => Acc {
                io: Some("flutter_rust_bridge::for_generated::rust_auto_opaque_explicit_decode(self.cst_decode())".into()),
                ..Default::default()
            },
        }
    }

    fn generate_impl_decode_jsvalue_body(&self) -> Option<std::borrow::Cow<str>> {
        Some(match &self.ir {
            IrTypeDelegate::String => {
                "self.as_string().expect(\"non-UTF-8 string, or not a string\")".into()
            }
            IrTypeDelegate::Char => {
                "CstDecode::<String>::cst_decode(self).chars().next().unwrap()".into()
            }
            IrTypeDelegate::PrimitiveEnum (IrTypeDelegatePrimitiveEnum { repr, .. }) => format!(
                "(self.unchecked_into_f64() as {}).cst_decode()",
                WireRustCodecCstGenerator::new(repr.clone(), self.context).rust_wire_type(Target::Web)
            )
                .into(),
            // IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
            //     "flutter_rust_bridge::ZeroCopyBuffer(self.cst_decode())".into()
            // }
            IrTypeDelegate::Time(ir) => match ir {
                IrTypeDelegateTime::Duration => "chrono::Duration::milliseconds(CstDecode::<i64>::cst_decode(self))".into(),
                _ => "CstDecode::<i64>::cst_decode(self).cst_decode()".into(),
            },
            // IrTypeDelegate::TimeList(_) =>
            //     "self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::BigInt64Array>().to_vec().into_iter().map(CstDecode::cst_decode).collect()".into(),
            IrTypeDelegate::Uuid /*| IrTypeDelegate::Uuids*/ => {
                "self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Uint8Array>().to_vec().into_boxed_slice().cst_decode()"
                    .into()
            }
            IrTypeDelegate::Backtrace | IrTypeDelegate::AnyhowException => "unimplemented!()".into(),
            IrTypeDelegate::Array(array) => generate_decode_array(array)
                .into(),
            IrTypeDelegate::Map(ir) => generate_decode_map(ir).into(),
            IrTypeDelegate::Set(ir) => generate_decode_set(ir).into(),
            IrTypeDelegate::StreamSink(_) => "StreamSink::deserialize(self.as_string().expect(\"should be a string\"))".into(),
            IrTypeDelegate::BigPrimitive(_) => "CstDecode::<String>::cst_decode(self).parse().unwrap()".into(),
            IrTypeDelegate::RustAutoOpaqueExplicit(_) =>
                "flutter_rust_bridge::for_generated::rust_auto_opaque_explicit_decode(self.cst_decode())".into(),
        })
    }

    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        Default::default()
    }

    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn rust_wire_type(&self, target: Target) -> String {
        // frb-coverage:ignore-end
        match (&self.ir, target) {
            (IrTypeDelegate::String, Target::Web) => "String".into(),
            // (IrTypeDelegate::StringList, Target::Io) => "wire_cst_StringList".to_owned(),
            // (IrTypeDelegate::StringList, Target::Web) => JS_VALUE.into(),
            _ => WireRustCodecCstGenerator::new(self.ir.get_delegate(), self.context)
                .rust_wire_type(target),
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        WireRustCodecCstGenerator::new(self.ir.get_delegate(), self.context)
            .rust_wire_is_pointer(target)
    }
}

impl<'a> DelegateWireRustCodecCstGenerator<'a> {
    fn generate_skip_web_if_jsvalue(&self, acc: String) -> Acc<Option<String>> {
        if is_js_value(&self.ir.get_delegate()) {
            Acc {
                io: Some(acc),
                ..Default::default()
            }
        } else {
            Acc::distribute(Some(acc))
        }
    }
}

fn generate_decode_array(array: &IrTypeDelegateArray) -> String {
    format!(
        "let vec: Vec<{}> = self.cst_decode(); flutter_rust_bridge::for_generated::from_vec_to_array(vec)",
        array.inner().rust_api_type()
    )
}

fn generate_decode_map(ir: &IrTypeDelegateMap) -> String {
    format!(
        "let vec: Vec<({}, {})> = self.cst_decode(); vec.into_iter().collect()",
        ir.key.rust_api_type(),
        ir.value.rust_api_type(),
    )
}

fn generate_decode_set(ir: &IrTypeDelegateSet) -> String {
    format!(
        "let vec: Vec<{}> = self.cst_decode(); vec.into_iter().collect()",
        ir.inner.rust_api_type()
    )
}
