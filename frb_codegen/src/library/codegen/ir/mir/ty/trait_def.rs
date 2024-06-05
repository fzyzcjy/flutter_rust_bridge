use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};
use crate::utils::namespace::NamespacedName;
use convert_case::{Case, Casing};

crate::mir! {
pub struct MirTypeTraitDef {
    pub name: NamespacedName,
}
}

impl MirTypeTrait for MirTypeTraitDef {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        _f: &mut F,
        _mir_context: &impl MirContext,
    ) {
    }

    fn safe_ident(&self) -> String {
        format!("TraitDef_{}", self.name)
    }

    fn rust_api_type(&self) -> String {
        self.name.name.clone()
    }
}
