use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::generator::wire::dart::wire2api::ty::WireDartGeneratorWire2apiTrait;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegateArray};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;

impl<'a> WireDartGeneratorWire2apiTrait for DelegateWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        match &self.ir {
            IrTypeDelegate::Array(array) => match array {
                IrTypeDelegateArray::GeneralArray { general, .. } => format!(
                    r"return {}((raw as List<dynamic>).map(_wire2api_{}).toList());",
                    array.dart_api_type(),
                    general.safe_ident(),
                ),
                IrTypeDelegateArray::PrimitiveArray { .. } => format!(
                    r"return {}(_wire2api_{}(raw));",
                    array.dart_api_type(),
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
            | IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                gen_wire2api_simple_type_cast(&self.ir.dart_api_type())
            }
            IrTypeDelegate::StringList => {
                "return (raw as List<dynamic>).cast<String>();".to_owned()
            }
            IrTypeDelegate::PrimitiveEnum { ir, .. } => {
                format!("return {}.values[raw as int];", ir.dart_api_type()) // here `as int` is neccessary in strict dynamic mode
            }
            IrTypeDelegate::Time(ir) => {
                if !ir.is_duration() {
                    format!(
                        "return wire2apiTimestamp(ts: _wire2api_i64(raw), isUtc: {});",
                        ir.is_utc()
                    )
                } else {
                    "return wire2apiDuration(_wire2api_i64(raw));".to_owned()
                }
            }
            IrTypeDelegate::TimeList(ir) => {
                format!(
                    "return (raw as List<dynamic>).map(_wire2api_Chrono_{}).toList();",
                    ir.safe_ident()
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
