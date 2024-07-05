use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};
use crate::codegen::ir::mir::ty::boxed::MirTypeBoxed;
use crate::codegen::ir::mir::ty::MirType::Boxed;

crate::mir! {
pub struct MirTypeFuture {
    pub output: Box<MirType>,
}
}


impl MirTypeTrait for MirTypeFuture {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        self.output.visit_types(f, mir_context);
    }

    fn safe_ident(&self) -> String {
        format!("fut_{}", self.output.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("impl Future<{}>", self.output.rust_api_type())
    }

    fn cloned_getter_semantics_reasonable(&self) -> bool {
        self.output.cloned_getter_semantics_reasonable()
    }
}

impl MirTypeFuture {
    pub(crate) fn new(output: MirType) -> Self {
        Self {
            output: Box::new(output),
        }
    }

    pub(crate) fn get_delegate(&self) -> MirType {
        self.output.clone().into()
    }

    pub(crate) fn new_with_boxed_wrapper(output: MirType) -> Self {
        Self::new(Boxed(MirTypeBoxed {
            exist_in_real_api: false,
            inner: Box::new(output),
        }))
    }

    pub(crate) fn is_primitive(&self) -> bool {
        matches!(&*self.output, Boxed(boxed) if !boxed.exist_in_real_api && boxed.inner.is_primitive())
    }

    pub(crate) fn is_boxed_primitive(&self) -> bool {
        matches!(&*self.output, Boxed(boxed) if boxed.exist_in_real_api && boxed.inner.is_primitive())
    }
}
