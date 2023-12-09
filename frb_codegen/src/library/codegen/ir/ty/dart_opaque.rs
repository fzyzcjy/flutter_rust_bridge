use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeDartOpaque;
}

impl IrTypeTrait for IrTypeDartOpaque {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        self.get_delegate().visit_types(f, ir_context)
    }

    fn safe_ident(&self) -> String {
        "DartOpaque".to_owned()
    }

    fn rust_api_type(&self) -> String {
        "flutter_rust_bridge::DartOpaque".to_owned()
    }
}

impl IrTypeDartOpaque {
    pub(crate) fn get_delegate(&self) -> IrType {
        IrType::Primitive(IrTypePrimitive::Usize)
    }
}
