use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;

impl<'a> WireRustGeneratorMiscTrait for DelegateWireRustGenerator<'a> {
    fn wrapper_struct_name(&self) -> Option<String> {
        if let IrTypeDelegate::PrimitiveEnum(enu) = &self.ir {
            WireRustGenerator::new(enu.ir.clone(), self.context).wrapper_struct_name()
        } else {
            None
        }
    }
}
