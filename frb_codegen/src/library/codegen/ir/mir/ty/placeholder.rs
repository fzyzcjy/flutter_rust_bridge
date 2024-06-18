use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};

crate::mir! {
#[derive(strum_macros::Display)]
pub enum MirTypePlaceholder {
    SelfButNotAllowed,
}
}

impl MirTypeTrait for MirTypePlaceholder {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        _f: &mut F,
        _mir_context: &impl MirContext,
    ) {
    }

    fn safe_ident(&self) -> String {
        format!("Placeholder_{}", self)
    }

    fn rust_api_type(&self) -> String {
        "NOT_USED".to_owned()
    }
}
