use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypePrimitiveList {
    pub primitive: IrTypePrimitive,
    pub strict_dart_type: bool,
}
}

impl IrTypeTrait for IrTypePrimitiveList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        IrType::Primitive(self.primitive.clone()).visit_types(f, ir_context);

        if !self.strict_dart_type {
            IrType::PrimitiveList(IrTypePrimitiveList {
                strict_dart_type: true,
                ..self.clone()
            })
            .visit_types(f, ir_context);
        }
    }

    fn safe_ident(&self) -> String {
        format!(
            "list_prim_{}_{}",
            self.primitive.safe_ident(),
            if self.strict_dart_type {
                "strict"
            } else {
                "loose"
            }
        )
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.primitive.rust_api_type())
    }
}
