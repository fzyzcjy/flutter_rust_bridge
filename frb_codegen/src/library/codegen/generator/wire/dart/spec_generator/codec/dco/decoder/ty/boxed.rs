use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::misc::gen_decode_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::*;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for BoxedWireDartCodecDcoGenerator<'a> {
    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn generate_impl_decode_body(&self) -> String {
        // frb-coverage:ignore-end
        match &*self.mir.inner {
            StructRef(_)
            | DartOpaque(_)
            | RustOpaque(_)
            | MirType::Delegate(MirTypeDelegate::RustAutoOpaqueExplicit(_))
            | RustAutoOpaqueImplicit(_)
            | EnumRef(_)
            | Primitive(
                MirTypePrimitive::I64
                | MirTypePrimitive::Isize
                | MirTypePrimitive::U64
                | MirTypePrimitive::Usize,
            )
            | Delegate(MirTypeDelegate::Array(_) | MirTypeDelegate::PrimitiveEnum { .. }) => {
                format!("return dco_decode_{}(raw);", self.mir.inner.safe_ident())
            }
            // TODO merge with above
            Delegate(MirTypeDelegate::Time(time)) => {
                format!("return dco_decode_Chrono_{}(raw);", time)
            }
            _ => gen_decode_simple_type_cast(self.mir.clone().into(), self.context),
        }
    }
}
