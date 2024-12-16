use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::ty::delegate::{
    generate_set_to_list, generate_stream_sink_setup_and_serialize,
};
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegateArrayMode, MirTypeDelegatePrimitiveEnum, MirTypeDelegateTime,
};
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::primitive_list::MirTypePrimitiveList;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for DelegateWireDartCodecCstGenerator<'a> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        match &self.mir {
            MirTypeDelegate::Array(ref array) => match &array.mode {
                MirTypeDelegateArrayMode::General(_) => Acc::distribute(Some(format!(
                    "return cst_encode_{}(raw);",
                    array.get_delegate().safe_ident(),
                ))),
                MirTypeDelegateArrayMode::Primitive(_) => Acc {
                    io: Some(format!(
                        "final ans = wire.cst_new_{}({length});
                        ans.ref.ptr.asTypedList({length}).setAll(0, raw);
                        return ans;",
                        array.get_delegate().safe_ident(),
                        length = array.length,
                    )),
                    web: Some(format!(
                        "return {}.fromList(raw).jsify()!;",
                        ApiDartGenerator::new(
                            array.get_delegate(),
                            self.context.as_api_dart_context()
                        )
                        .dart_api_type()
                    )),
                    ..Default::default()
                },
            },

            MirTypeDelegate::String => Acc {
                io: Some(format!(
                    "return cst_encode_{}(utf8.encoder.convert(raw));",
                    uint8list_safe_ident(true)
                )),
                web: Some("return raw;".into()),
                ..Default::default()
            },
            MirTypeDelegate::Char => Acc {
                io: Some("return cst_encode_String(raw);".into()),
                web: Some("return cst_encode_String(raw);".into()),
                ..Default::default()
            },
            // MirTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
            //     // In this case, even though the body is the same, their types are different
            //     // and must be split.
            //     let body = format!(
            //         "return cst_encode_{}(raw);",
            //         self.mir.get_delegate().safe_ident()
            //     );
            //     Acc::distribute(Some(body))
            // }
            // MirTypeDelegate::StringList => Acc {
            //     io: Some(
            //         "final ans = wire.cst_new_StringList(raw.length);
            //         for (var i = 0; i < raw.length; i++){
            //             ans.ref.ptr[i] = cst_encode_String(raw[i]);
            //         }
            //         return ans;"
            //             .to_string(),
            //     ),
            //     web: Some("return raw;".into()),
            //     ..Default::default()
            // },
            MirTypeDelegate::PrimitiveEnum(MirTypeDelegatePrimitiveEnum { ref repr, .. }) => {
                format!("return cst_encode_{}(raw.index);", repr.safe_ident()).into()
            }
            MirTypeDelegate::Time(mir) => match mir {
                MirTypeDelegateTime::Utc
                | MirTypeDelegateTime::Local
                | MirTypeDelegateTime::Naive => Acc {
                    io: Some("return cst_encode_i_64(raw.microsecondsSinceEpoch);".into()),
                    web: Some(
                        "return cst_encode_i_64(BigInt.from(raw.millisecondsSinceEpoch));".into(),
                    ),
                    ..Default::default()
                },
                MirTypeDelegateTime::Duration => Acc {
                    io: Some("return cst_encode_i_64(raw.inMicroseconds);".into()),
                    web: Some("return cst_encode_i_64(BigInt.from(raw.inMilliseconds));".into()),
                    ..Default::default()
                },
            },
            // MirTypeDelegate::TimeList(t) => Acc::distribute(Some(format!(
            //     "final ans = Int64List(raw.length);
            //     for (var i=0; i < raw.length; ++i) ans[i] = cst_encode_{}(raw[i]);
            //     return cst_encode_list_prim_i_64(ans);",
            //     MirTypeDelegate::Time(*t).safe_ident()
            // ))),
            MirTypeDelegate::Uuid => Acc::distribute(Some(format!(
                "return cst_encode_{}(raw.toBytes());",
                uint8list_safe_ident(true)
            ))),
            // MirTypeDelegate::Uuids => Acc::distribute(Some(format!(
            //     "final builder = BytesBuilder();
            //     for (final element in raw) {{
            //       builder.add(element.toBytes());
            //     }}
            //     return cst_encode_{}(builder.toBytes());",
            //     uint8list_safe_ident()
            // ))),
            MirTypeDelegate::Backtrace
            | MirTypeDelegate::AnyhowException
            | MirTypeDelegate::DynTrait(_) => {
                Acc::distribute(Some("throw UnimplementedError();".to_string()))
            }
            MirTypeDelegate::Map(_) => Acc::distribute(Some(format!(
                "return cst_encode_{}(raw.entries.map((e) => (e.key, e.value)).toList());",
                self.mir.get_delegate().safe_ident()
            ))),
            MirTypeDelegate::Set(mir) => Acc::distribute(Some(format!(
                "return cst_encode_{}({});",
                self.mir.get_delegate().safe_ident(),
                generate_set_to_list(mir, self.context.as_api_dart_context(), "raw"),
            ))),
            MirTypeDelegate::StreamSink(mir) => Acc::distribute(Some(format!(
                "return cst_encode_{}({});",
                self.mir.get_delegate().safe_ident(),
                generate_stream_sink_setup_and_serialize(mir, "raw")
            ))),
            MirTypeDelegate::BigPrimitive(_) => Acc::distribute(Some(
                "return cst_encode_String(raw.toString());".to_string(),
            )),
            MirTypeDelegate::RustAutoOpaqueExplicit(_) => Acc::distribute(Some(format!(
                "return cst_encode_{}(raw);",
                self.mir.get_delegate().safe_ident(),
            ))),
            MirTypeDelegate::ProxyVariant(_)
            | MirTypeDelegate::ProxyEnum(_)
            | MirTypeDelegate::CastedPrimitive(_)
            | MirTypeDelegate::CustomSerDes(_)
            | MirTypeDelegate::Lifetimeable(_) =>
                Acc::distribute(Some("throw UnimplementedError('Not implemented in this codec, please use the other one');".to_string()))
        }
    }

    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn dart_wire_type(&self, target: Target) -> String {
        // frb-coverage:ignore-end
        match (&self.mir, target) {
            (MirTypeDelegate::String, Target::Web) => "String".into(),
            // (MirTypeDelegate::StringList, Target::Web) => "List<String>".into(),
            // (MirTypeDelegate::StringList, _) => "ffi.Pointer<wire_cst_StringList>".to_owned(),
            _ => WireDartCodecCstGenerator::new(self.mir.get_delegate(), self.context)
                .dart_wire_type(target),
        }
    }
}

fn uint8list_safe_ident(strict_dart_type: bool) -> String {
    MirTypePrimitiveList {
        primitive: MirTypePrimitive::U8,
        strict_dart_type,
    }
    .safe_ident()
}
