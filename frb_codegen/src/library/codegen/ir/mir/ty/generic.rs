use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};
use crate::utils::namespace::Namespace;

crate::mir! {
pub struct MirTypeGeneric {
    pub param_name: String,
}
}

impl MirTypeTrait for MirTypeGeneric {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        _f: &mut F,
        _mir_context: &impl MirContext,
    ) {
        // Generic types have no children
    }

    fn safe_ident(&self) -> String {
        self.param_name.clone()
    }

    fn rust_api_type(&self) -> String {
        self.param_name.clone()
    }

    fn self_namespace(&self) -> Option<Namespace> {
        None
    }

    fn should_ignore(&self, _mir_context: &impl MirContext) -> bool {
        false
    }
}

