use crate::codegen::generator::api_dart::base::ApiDartGenerator;
use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::generator::wire::dart::wire2api::misc::gen_wire2api_simple_type_cast;
use crate::codegen::generator::wire::dart::wire2api::ty::WireDartGeneratorWire2apiTrait;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateArray, IrTypeDelegatePrimitiveEnum, IrTypeDelegateTime,
};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::library::codegen::generator::api_dart::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorWire2apiTrait for DelegateWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        match &self.ir {
            IrTypeDelegate::Array(array) => match array {
                IrTypeDelegateArray::GeneralArray { general, .. } => format!(
                    r"return {}((raw as List<dynamic>).map(_wire2api_{}).toList());",
                    ApiDartGenerator::new(self.ir.clone(), self.context.as_api_dart_context())
                        .dart_api_type(),
                    general.safe_ident(),
                ),
                IrTypeDelegateArray::PrimitiveArray { .. } => format!(
                    r"return {}(_wire2api_{}(raw));",
                    ApiDartGenerator::new(self.ir.clone(), self.context.as_api_dart_context())
                        .dart_api_type(),
                    array.get_delegate().safe_ident(),
                ),
            },

            IrTypeDelegate::ZeroCopyBufferVecPrimitive(
                IrTypePrimitive::I64 | IrTypePrimitive::U64,
            ) => {
                format!(
                    "return _wire2api_{}(raw);",
                    self.ir.get_delegate().safe_ident()
                )
            }
            IrTypeDelegate::String
            | IrTypeDelegate::Backtrace
            | IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => gen_wire2api_simple_type_cast(
                &ApiDartGenerator::new(self.ir.clone(), self.context.as_api_dart_context())
                    .dart_api_type(),
            ),
            IrTypeDelegate::StringList => {
                "return (raw as List<dynamic>).cast<String>();".to_owned()
            }
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) => {
                format!(
                    "return {}.values[raw as int];",
                    ApiDartGenerator::new(ir.clone(), self.context.as_api_dart_context())
                        .dart_api_type()
                ) // here `as int` is neccessary in strict dynamic mode
            }
            IrTypeDelegate::Time(ir) => {
                if ir == IrTypeDelegateTime::Duration {
                    "return wire2apiDuration(_wire2api_i64(raw));".to_owned()
                } else {
                    format!(
                        "return wire2apiTimestamp(ts: _wire2api_i64(raw), isUtc: {is_utc});",
                        is_utc = matches!(ir, IrTypeDelegateTime::Naive | IrTypeDelegateTime::Utc)
                    )
                }
            }
            IrTypeDelegate::TimeList(_) => {
                format!(
                    "return (raw as List<dynamic>).map(_wire2api_Chrono_{}).toList();",
                    self.ir.safe_ident()
                )
            }
            IrTypeDelegate::Uuid => {
                "return UuidValue.fromByteList(_wire2api_uint_8_list(raw));".to_owned()
            }
            IrTypeDelegate::Uuids => "
            final bytes = _wire2api_uint_8_list(raw);
            return wire2apiUuids(bytes);"
                .to_owned(),
            IrTypeDelegate::Anyhow => "return FrbAnyhowException(raw as String);".to_owned(),
        }
    }
}
