use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::common::ty::WireRustGeneratorCommonTrait;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::delegate::IrTypeDelegatePrimitiveEnum;
use crate::forward_delegate_primitive_enum;

impl<'a> WireRustGeneratorCommonTrait for DelegateWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        match (&self.ir, target) {
            (IrTypeDelegate::String, Target::Wasm) => "String".into(),
            (IrTypeDelegate::StringList, Target::Io) => "wire_StringList".to_owned(),
            (IrTypeDelegate::StringList, Target::Wasm) => JS_VALUE.into(),
            _ => WireRustGenerator::new(self.ir.get_delegate().clone(), self.context)
                .rust_wire_type(target),
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        WireRustGenerator::new(self.ir.get_delegate(), self.context).rust_wire_is_pointer(target)
    }

    fn wrapper_struct_name(&self) -> Option<String> {
        if let IrTypeDelegate::PrimitiveEnum(enu) = &self.ir {
            WireRustGenerator::new(enu.ir.clone(), self.context).wrapper_struct_name()
        } else {
            None
        }

        // TODO https://github.com/fzyzcjy/yplusplus/issues/11145#issuecomment-1816273032
        // forward_delegate_primitive_enum!(self, wrapper_struct_name(), None)
    }

    // TODO old code has this, but I think do not need?
    // fn generate_static_checks(&self) -> Option<String> {
    //     forward_delegate_primitive_enum!(self, static_checks(), None)
    // }

    // TODO rm this, since we will visit all sub-types to generate
    // fn generate_imports(&self) -> Option<Vec<String>> {
    //     forward_delegate_primitive_enum!(self, imports(), None)
    // }
}
