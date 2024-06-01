use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};

crate::mir! {
pub struct MirTypeDynamic;
}

impl MirTypeTrait for MirTypeDynamic {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        _f: &mut F,
        _mir_context: &impl MirContext,
    ) {
    }

    fn safe_ident(&self) -> String {
        "dartabi".to_owned()
    }

    fn rust_api_type(&self) -> String {
        "flutter_rust_bridge::for_generated::DartAbi".to_owned()
    }

    fn cloned_getter_semantics_reasonable(&self) -> bool {
        true
    }
}
