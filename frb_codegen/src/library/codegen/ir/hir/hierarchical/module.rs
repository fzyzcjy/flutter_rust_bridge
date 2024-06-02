use crate::codegen::ir::hir::hierarchical::function::HirFunction;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirEnum;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirStruct;
use crate::codegen::ir::hir::hierarchical::type_alias::HirTypeAlias;
use crate::utils::namespace::Namespace;
use derivative::Derivative;
use itertools::concat;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct HirModule {
    pub meta: HirModuleMeta,
    pub content: HirModuleContent,
    // avoid too big debug dump
    #[serde(skip_serializing)]
    pub raw: Vec<String>,
}

#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct HirModuleMeta {
    pub parent_vis: Vec<HirVisibility>,
    pub vis: HirVisibility,
    pub namespace: Namespace,
}

impl HirModuleMeta {
    pub(crate) fn parent_and_self_vis(&self) -> Vec<HirVisibility> {
        concat([self.parent_vis.clone(), vec![self.vis]])
    }

    pub(crate) fn is_public(&self) -> bool {
        (self.parent_and_self_vis().iter()).all(|x| *x == HirVisibility::Public)
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct HirModuleContent {
    pub modules: Vec<HirModule>,
    pub enums: Vec<HirEnum>,
    pub structs: Vec<HirStruct>,
    // pub imports: Vec<Import>, // not implemented yet
    pub type_alias: Vec<HirTypeAlias>,
    pub functions: Vec<HirFunction>,
}

/// Mirrors syn::Visibility, but can be created without a token
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum HirVisibility {
    Public,
    Restricted,
    // Not supported
    Inherited, // Usually means private
}

impl HirModuleContent {
    pub(crate) fn get_module_index_by_name(&self, mod_name: &str) -> Option<usize> {
        self.modules
            .iter()
            .enumerate()
            .filter(|(_, m)| *m.meta.namespace.path().last().unwrap() == mod_name)
            .map(|(i, _)| i)
            .next()
    }

    pub(crate) fn get_module_by_name(&self, mod_name: &str) -> Option<&HirModule> {
        self.get_module_index_by_name(mod_name)
            .map(|i| &self.modules[i])
    }

    pub(crate) fn get_mut_module_by_name(&mut self, mod_name: &str) -> Option<&mut HirModule> {
        self.get_module_index_by_name(mod_name)
            .map(|i| self.modules.get_mut(i).unwrap())
    }

    pub(crate) fn remove_module_by_name(&mut self, mod_name: &str) -> Option<HirModule> {
        self.get_module_index_by_name(mod_name)
            .map(|index| self.modules.remove(index))
    }

    pub(crate) fn get_module_nested(&self, mod_names: &[&str]) -> Option<&HirModule> {
        let m = self.get_module_by_name(mod_names[0])?;
        if mod_names.len() == 1 {
            Some(m)
        } else {
            m.content.get_module_nested(&mod_names[1..])
        }
    }
}
