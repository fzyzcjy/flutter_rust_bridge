use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::IrType::Boxed;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeOptional {
    pub inner: Box<IrType>,
}
}

impl IrTypeTrait for IrTypeOptional {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        self.inner.visit_types(f, ir_context);
    }

    fn safe_ident(&self) -> String {
        format!("opt_{}", self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("Option<{}>", self.inner.rust_api_type())
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

    pub(crate) fn is_primitive(&self) -> bool {
        matches!(&*self.inner, Boxed(boxed) if !boxed.exist_in_real_api && boxed.inner.is_primitive())
    }

    pub(crate) fn is_boxed_primitive(&self) -> bool {
        matches!(&*self.inner, Boxed(boxed) if boxed.exist_in_real_api && boxed.inner.is_primitive())
    }
}
