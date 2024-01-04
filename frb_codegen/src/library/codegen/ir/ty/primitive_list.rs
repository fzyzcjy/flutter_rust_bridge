use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypePrimitiveList {
    pub primitive: IrTypePrimitive,
    pub direction: PrimitveListLocation,
}

#[derive(Copy, strum_macros::Display)]
pub(crate) enum PrimitveListLocation {
    Rust2Dart,
    Dart2Rust,
}
}

impl IrTypeTrait for IrTypePrimitiveList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        IrType::Primitive(self.primitive.clone()).visit_types(f, ir_context);
    }

    fn safe_ident(&self) -> String {
        format!(
            "list_prim_{}_{}",
            self.primitive.safe_ident(),
            self.direction
        )
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.primitive.rust_api_type())
    }
}
