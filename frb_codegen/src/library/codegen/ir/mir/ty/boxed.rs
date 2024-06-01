use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};

crate::mir! {
pub struct MirTypeBoxed {
    /// if false, means that we automatically add it when transforming it - it does not exist in real api.
    pub exist_in_real_api: bool,
    pub inner: Box<MirType>,
}
}

impl MirTypeTrait for MirTypeBoxed {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        self.inner.visit_types(f, mir_context);
    }

    fn safe_ident(&self) -> String {
        format!(
            "box_{}{}",
            if self.exist_in_real_api {
                ""
            } else {
                "autoadd_"
            },
            self.inner.safe_ident()
        )
    }

    fn rust_api_type(&self) -> String {
        if self.exist_in_real_api {
            format!("Box<{}>", self.inner.rust_api_type())
        } else {
            self.inner.rust_api_type()
        }
    }

    fn cloned_getter_semantics_reasonable(&self) -> bool {
        self.inner.cloned_getter_semantics_reasonable()
    }
}
