use crate::codegen::ir::hir::misc::serializers::serialize_vec_syn;
use crate::codegen::ir::hir::misc::visibility::HirVisibility;
use crate::utils::namespace::Namespace;
use derivative::Derivative;
use itertools::concat;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct HirTreeModule {
    pub meta: HirTreeModuleMeta,
    pub modules: Vec<HirTreeModule>,
    #[serde(serialize_with = "serialize_vec_syn")]
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
        if mod_names.is_empty() {
            Some(self)
        } else {
            let m = self.get_module_by_name(mod_names[0])?;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn module(
        namespace: &str,
        vis: HirVisibility,
        parent_vis: Vec<HirVisibility>,
    ) -> HirTreeModule {
        HirTreeModule {
            meta: HirTreeModuleMeta {
                parent_vis,
                vis,
                namespace: Namespace::new_raw(namespace.to_owned()),
            },
            modules: vec![],
            items: vec![],
        }
    }

    /// Finds modules by their final path component and traverses nested paths.
    #[test]
    fn finds_modules_by_name_and_nested_path() {
        let leaf = module("crate::outer::leaf", HirVisibility::Public, vec![]);
        let outer = HirTreeModule {
            modules: vec![leaf],
            ..module("crate::outer", HirVisibility::Public, vec![])
        };
        let root = HirTreeModule {
            modules: vec![outer],
            ..module("crate", HirVisibility::Public, vec![])
        };

        assert!(root.get_module_by_name("outer").is_some());
        assert_eq!(
            root.get_module_nested(&["outer", "leaf"])
                .map(|module| module.meta.namespace.path()),
            Some(vec!["crate", "outer", "leaf"])
        );
        assert!(root.get_module_nested(&[]).is_some());
        assert!(root.get_module_nested(&["outer", "missing"]).is_none());
    }

    /// Reports a module public only when its own and parent visibilities are public.
    #[test]
    fn computes_effective_public_visibility() {
        let public = HirTreeModuleMeta {
            parent_vis: vec![HirVisibility::Public, HirVisibility::Public],
            vis: HirVisibility::Public,
            namespace: Namespace::new_raw("crate::public".to_owned()),
        };
        let private_parent = HirTreeModuleMeta {
            parent_vis: vec![HirVisibility::Public, HirVisibility::Inherited],
            vis: HirVisibility::Public,
            namespace: Namespace::new_raw("crate::private_parent".to_owned()),
        };
        let restricted_self = HirTreeModuleMeta {
            parent_vis: vec![HirVisibility::Public],
            vis: HirVisibility::Restricted,
            namespace: Namespace::new_raw("crate::restricted".to_owned()),
        };

        assert_eq!(
            public.parent_and_self_vis(),
            vec![
                HirVisibility::Public,
                HirVisibility::Public,
                HirVisibility::Public
            ]
        );
        assert!(public.is_public());
        assert!(!private_parent.is_public());
        assert!(!restricted_self.is_public());
    }
}
