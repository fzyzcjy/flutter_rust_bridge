use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::misc::gen_decode_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateArrayMode, IrTypeDelegatePrimitiveEnum, IrTypeDelegateTime,
};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for DelegateWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        match &self.ir {
            IrTypeDelegate::Array(array) => match &array.mode {
                IrTypeDelegateArrayMode::General(general) => format!(
                    r"return {}((raw as List<dynamic>).map(_dco_decode_{}).toList());",
                    ApiDartGenerator::new(self.ir.clone(), self.context.as_api_dart_context())
                        .dart_api_type(),
                    general.safe_ident(),
                ),
                IrTypeDelegateArrayMode::Primitive(_) => format!(
                    r"return {}(_dco_decode_{}(raw));",
                    ApiDartGenerator::new(self.ir.clone(), self.context.as_api_dart_context())
                        .dart_api_type(),
                    array.get_delegate().safe_ident(),
                ),
            },

            IrTypeDelegate::ZeroCopyBufferVecPrimitive(
                IrTypePrimitive::I64 | IrTypePrimitive::U64,
            ) => {
                format!(
                    "return _dco_decode_{}(raw);",
                    self.ir.get_delegate().safe_ident()
                )
            }
            IrTypeDelegate::String
            | IrTypeDelegate::Backtrace
            | IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                gen_decode_simple_type_cast(self.ir.clone().into(), self.context)
            }
            // IrTypeDelegate::StringList => {
            //     "return (raw as List<dynamic>).cast<String>();".to_owned()
            // }
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) => {
                format!(
                    "return {}.values[raw as int];",
                    ApiDartGenerator::new(ir.clone(), self.context.as_api_dart_context())
                        .dart_api_type()
                ) // here `as int` is neccessary in strict dynamic mode
            }
            IrTypeDelegate::Time(ir) => {
                if ir == &IrTypeDelegateTime::Duration {
                    "return dcoDecodeDuration(_dco_decode_i_64(raw).toInt());".to_owned()
                } else {
                    format!(
                        "return dcoDecodeTimestamp(ts: _dco_decode_i_64(raw).toInt(), isUtc: {is_utc});",
                        is_utc = matches!(ir, IrTypeDelegateTime::Naive | IrTypeDelegateTime::Utc)
                    )
                }
            }
            // IrTypeDelegate::TimeList(t) => {
            //     format!(
            //         "return (raw as List<dynamic>).map(_dco_decode_{}).toList();",
            //         IrTypeDelegate::Time(*t).safe_ident()
            //     )
            // }
            IrTypeDelegate::Uuid => {
                "return UuidValue.fromByteList(_dco_decode_list_prim_u_8(raw));".to_owned()
            }
            // IrTypeDelegate::Uuids =>
            //     "const kUuidSizeInBytes = 16;
            //     final bytes = _dco_decode_list_prim_u_8(raw);
            //     return List.generate(
            //       bytes.lengthInBytes ~/ kUuidSizeInBytes,
            //       (i) => UuidValue.fromByteList(Uint8List.view(bytes.buffer, i * kUuidSizeInBytes, kUuidSizeInBytes)),
            //       growable: false,
            //     );
            //     ".to_owned(),
            IrTypeDelegate::Anyhow => "return AnyhowException(raw as String);".to_owned(),
        }
    }
}
