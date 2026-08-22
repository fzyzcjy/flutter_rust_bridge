use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};
use crate::utils::namespace::{Namespace, NamespacedName};

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
        format!("TraitDef_{}", self.name.name)
    }

    fn rust_api_type(&self) -> String {
        self.name.name.clone()
    }

    fn self_namespace(&self) -> Option<Namespace> {
        Some(self.name.namespace.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Uses the unqualified trait name while retaining its namespace.
    #[test]
    fn trait_definition_uses_unqualified_api_name() {
        let ty = MirTypeTraitDef {
            name: NamespacedName::new(
                Namespace::new_raw("crate::api".to_owned()),
                "Service".to_owned(),
            ),
        };
        assert_eq!(ty.safe_ident(), "TraitDef_Service");
        assert_eq!(ty.rust_api_type(), "Service");
        assert_eq!(
            ty.self_namespace(),
            Some(Namespace::new_raw("crate::api".to_owned()))
        );
    }
}
