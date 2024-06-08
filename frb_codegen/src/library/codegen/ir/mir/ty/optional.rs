use crate::codegen::ir::mir::ty::boxed::MirTypeBoxed;
use crate::codegen::ir::mir::ty::MirType::Boxed;
use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};

crate::mir! {
pub struct MirTypeOptional {
    pub inner: Box<MirType>,
}
}

impl MirTypeTrait for MirTypeOptional {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        self.inner.visit_types(f, mir_context);
    }

    fn safe_ident(&self) -> String {
        format!("opt_{}", self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("Option<{}>", self.inner.rust_api_type())
    }

    fn cloned_getter_semantics_reasonable(&self) -> bool {
        self.inner.cloned_getter_semantics_reasonable()
    }
}

impl MirTypeOptional {
    pub(crate) fn new(inner: MirType) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }

    pub(crate) fn new_with_boxed_wrapper(inner: MirType) -> Self {
        Self::new(Boxed(MirTypeBoxed {
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
