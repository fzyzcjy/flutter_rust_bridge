use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypePrimitiveList {
    pub primitive: IrTypePrimitive,
}
}

impl IrTypeTrait for IrTypePrimitiveList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        IrType::Primitive(self.primitive.clone()).visit_types(f, ir_context);

        // SSE codec needs i32 for length
        IrType::Primitive(IrTypePrimitive::I32).visit_types(f, ir_context);
    }

    fn safe_ident(&self) -> String {
        format!("list_prim_{}", self.primitive.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.primitive.rust_api_type())
    }
}
