use crate::codegen::generator::api_dart::base::ApiDartGenerator;
use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::generator::wire::dart::wire2api::misc::gen_wire2api_simple_type_cast;
use crate::codegen::generator::wire::dart::wire2api::ty::WireDartGeneratorWire2apiTrait;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrType::*;
use crate::library::codegen::generator::api_dart::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorWire2apiTrait for BoxedWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        match &*self.ir.inner {
            StructRef(_)
            | DartOpaque(_)
            | RustOpaque(_)
            | EnumRef(_)
            | Primitive(IrTypePrimitive::I64 | IrTypePrimitive::U64 | IrTypePrimitive::Usize)
            | Delegate(IrTypeDelegate::Array(_) | IrTypeDelegate::PrimitiveEnum { .. }) => {
                format!("return _wire2api_{}(raw);", self.ir.inner.safe_ident())
            }
            Delegate(IrTypeDelegate::Time(time)) => gen_wire2api_chrono(time),
            _ => gen_wire2api_simple_type_cast(
                &ApiDartGenerator::new(self.ir.clone(), self.context.as_api_dart_context())
                    .dart_api_type(),
            ),
        }
    }
}
