use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::misc::gen_decode_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::mir::ty::delegate::IrTypeDelegate;
use crate::codegen::mir::ty::primitive::IrTypePrimitive;
use crate::codegen::mir::ty::IrType;
use crate::codegen::mir::ty::IrType::*;
use crate::library::codegen::mir::ty::IrTypeTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for BoxedWireDartCodecDcoGenerator<'a> {
    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn generate_impl_decode_body(&self) -> String {
        // frb-coverage:ignore-end
        match &*self.ir.inner {
            StructRef(_)
            | DartOpaque(_)
            | RustOpaque(_)
            | IrType::Delegate(IrTypeDelegate::RustAutoOpaqueExplicit(_))
            | RustAutoOpaqueImplicit(_)
            | EnumRef(_)
            | Primitive(
                IrTypePrimitive::I64
                | IrTypePrimitive::Isize
                | IrTypePrimitive::U64
                | IrTypePrimitive::Usize,
            )
            | Delegate(IrTypeDelegate::Array(_) | IrTypeDelegate::PrimitiveEnum { .. }) => {
                format!("return dco_decode_{}(raw);", self.ir.inner.safe_ident())
            }
            // TODO merge with above
            Delegate(IrTypeDelegate::Time(time)) => {
                format!("return dco_decode_Chrono_{}(raw);", time)
            }
            _ => gen_decode_simple_type_cast(self.ir.clone().into(), self.context),
        }
    }
}
