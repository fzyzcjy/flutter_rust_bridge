use crate::codegen::ir::hir::misc::visibility::HirVisibility;
use crate::utils::namespace::Namespace;
use derivative::Derivative;
use itertools::concat;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct HirTreeModule {
    pub meta: HirTreeModuleMeta,
    pub modules: Vec<HirTreeModule>,
    #[serde(skip_serializing)]
    pub items: Vec<syn::Item>,
}

// This is surely used, but not counted by coverage tools
// frb-coverage:ignore-start
#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct HirTreeModuleMeta {
    pub parent_vis: Vec<HirVisibility>,
    pub vis: HirVisibility,
    pub namespace: Namespace,
}
// frb-coverage:ignore-end

impl HirTreeModule {
    // Have this method because also need get_mut etc
    pub(crate) fn get_module_index_by_name(&self, mod_name: &str) -> Option<usize> {
        (self.modules.iter().enumerate())
            .filter(|(_, m)| *m.meta.namespace.path().last().unwrap() == mod_name)
            .map(|(i, _)| i)
            .next()
    }

    pub(crate) fn get_module_by_name(&self, mod_name: &str) -> Option<&HirTreeModule> {
        self.get_module_index_by_name(mod_name)
            .map(|i| &self.modules[i])
    }

    pub(crate) fn get_module_nested(&self, mod_names: &[&str]) -> Option<&HirTreeModule> {
        let m = self.get_module_by_name(mod_names[0])?;
        if mod_names.len() == 1 {
            Some(m)
        } else {
            m.get_module_nested(&mod_names[1..])
        }
    }
}

impl HirTreeModuleMeta {
    pub(crate) fn parent_and_self_vis(&self) -> Vec<HirVisibility> {
        concat([self.parent_vis.clone(), vec![self.vis]])
    }

    pub(crate) fn is_public(&self) -> bool {
        (self.parent_and_self_vis().iter()).all(|x| *x == HirVisibility::Public)
    }

    pub(crate) fn is_private(&self) -> bool {
        (self.parent_and_self_vis().iter()).any(|x| *x == HirVisibility::Inherited)
    }
}
