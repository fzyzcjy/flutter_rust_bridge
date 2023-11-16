use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::IrType::Boxed;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeOptional {
    pub inner: Box<IrType>,
}
}

impl IrTypeTrait for IrTypeOptional {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack) {
        self.inner.visit_types(f, ir_pack);
    }

    fn safe_ident(&self) -> String {
        format!("opt_{}", self.inner.safe_ident())
    }
}

impl IrTypeOptional {
    pub(crate) fn new(inner: IrType) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }

    pub(crate) fn new_with_boxed_wrapper(inner: IrType) -> Self {
        Self::new(Boxed(IrTypeBoxed {
            exist_in_real_api: false,
            inner: Box::new(inner),
        }))
    }
}
