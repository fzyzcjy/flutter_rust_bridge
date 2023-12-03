use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeRustAutoOpaque {
    // TODO
}
}

impl IrTypeTrait for IrTypeRustAutoOpaque {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_pack: &IrPack) {}

    fn safe_ident(&self) -> String {
        todo!()
    }

    fn rust_api_type(&self) -> String {
        todo!()
    }

    fn self_namespace(&self) -> Option<Namespace> {
        todo!()
    }
}
