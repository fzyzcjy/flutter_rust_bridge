use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::misc::gen_decode_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegateArrayMode, MirTypeDelegatePrimitiveEnum, MirTypeDelegateTime,
};
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for DelegateWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        match &self.mir {
            MirTypeDelegate::Array(array) => match &array.mode {
                MirTypeDelegateArrayMode::General(general) => format!(
                    r"return {}((raw as List<dynamic>).map(dco_decode_{}).toList());",
                    ApiDartGenerator::new(self.mir.clone(), self.context.as_api_dart_context())
                        .dart_api_type(),
                    general.safe_ident(),
                ),
                MirTypeDelegateArrayMode::Primitive(_) => format!(
                    r"return {}(dco_decode_{}(raw));",
                    ApiDartGenerator::new(self.mir.clone(), self.context.as_api_dart_context())
                        .dart_api_type(),
                    array.get_delegate().safe_ident(),
                ),
            },

            // MirTypeDelegate::ZeroCopyBufferVecPrimitive(
            //     MirTypePrimitive::I64 | MirTypePrimitive::U64,
            // ) => {
            //     format!(
            //         "return dco_decode_{}(raw);",
            //         self.mir.get_delegate().safe_ident()
            //     )
            // }
            MirTypeDelegate::String
            | MirTypeDelegate::Backtrace
            /*| MirTypeDelegate::ZeroCopyBufferVecPrimitive(_)*/ => {
                gen_decode_simple_type_cast(self.mir.clone().into(), self.context)
            }
            MirTypeDelegate::Char => {
                "return String.fromCharCode(raw);".to_owned()
            }
            // MirTypeDelegate::StringList => {
            //     "return (raw as List<dynamic>).cast<String>();".to_owned()
            // }
            MirTypeDelegate::PrimitiveEnum(MirTypeDelegatePrimitiveEnum { mir, .. }) => {
                format!(
                    "return {}.values[raw as int];",
                    ApiDartGenerator::new(mir.clone(), self.context.as_api_dart_context())
                        .dart_api_type()
                ) // here `as int` is neccessary in strict dynamic mode
            }
            MirTypeDelegate::Time(mir) => {
                if mir == &MirTypeDelegateTime::Duration {
                    "return dcoDecodeDuration(dco_decode_i_64(raw).toInt());".to_owned()
                } else {
                    format!(
                        "return dcoDecodeTimestamp(ts: dco_decode_i_64(raw).toInt(), isUtc: {is_utc});",
                        is_utc = matches!(mir, MirTypeDelegateTime::Naive | MirTypeDelegateTime::Utc)
                    )
                }
            }
            // MirTypeDelegate::TimeList(t) => {
            //     format!(
            //         "return (raw as List<dynamic>).map(dco_decode_{}).toList();",
            //         MirTypeDelegate::Time(*t).safe_ident()
            //     )
            // }
            MirTypeDelegate::Uuid => {
                "return UuidValue.fromByteList(dco_decode_list_prim_u_8_strict(raw));".to_owned()
            }
            // MirTypeDelegate::Uuids => ...,
            MirTypeDelegate::AnyhowException => "return AnyhowException(raw as String);".to_owned(),
            MirTypeDelegate::Map(_) => format!(
                "return Map.fromEntries(dco_decode_{}(raw).map((e) => MapEntry(e.$1, e.$2)));",
                self.mir.get_delegate().safe_ident(),
            ),
            MirTypeDelegate::Set(_) => format!(
                "return Set.from(dco_decode_{}(raw));",
                self.mir.get_delegate().safe_ident(),
            ),
            MirTypeDelegate::StreamSink(_) | MirTypeDelegate::DynTrait(_) => "throw UnimplementedError();".to_owned(),
            MirTypeDelegate::BigPrimitive(_) => {
                "return BigInt.parse(raw);".to_owned()
            }
            MirTypeDelegate::RustAutoOpaqueExplicit(mir) => format!(r"return dco_decode_{}(raw);", mir.inner.safe_ident()),
            MirTypeDelegate::ProxyVariant(_)
            | MirTypeDelegate::ProxyEnum(_)
            | MirTypeDelegate::CastedPrimitive(_)
            | MirTypeDelegate::CustomSerDes(_)
            | MirTypeDelegate::Lifetimeable(_) =>
                "throw UnimplementedError('Not implemented in this codec, please use the other one');".into(),
        }
    }
}
