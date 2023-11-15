use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeUnencodable {
    pub string: String,
    pub segments: Vec<NameComponent>,
}

/// A component of a fully qualified name and any type arguments for it
pub struct NameComponent {
    pub ident: String,
    pub args: Option<Args>,
}

pub enum Args {
    Generic(Vec<IrType>),
    Signature(Vec<IrType>),
}
}

impl IrTypeTrait for IrTypeUnencodable {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_pack: &IrPack) {}

    fn safe_ident(&self) -> String { unimplemented!() }
}
