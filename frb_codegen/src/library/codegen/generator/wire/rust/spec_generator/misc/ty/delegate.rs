use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

impl<'a> WireRustGeneratorMiscTrait for DelegateWireRustGenerator<'a> {
    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn wrapper_struct_name(&self) -> Option<String> {
        // frb-coverage:ignore-end
        if let MirTypeDelegate::PrimitiveEnum(enu) = &self.mir {
            WireRustGenerator::new(enu.mir.clone(), self.context).wrapper_struct_name()
        } else {
            None
        }
    }

    fn generate_wire_func_call_decode_type(&self) -> Option<String> {
        if let MirTypeDelegate::ProxyEnum(mir) = &self.mir {
            Some(mir.get_delegate().rust_api_type())
        } else {
            None
        }
    }
}
