use crate::codegen::ir::hir::hierarchical::function::HirFunction;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirEnum;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirStruct;
use crate::codegen::ir::hir::hierarchical::traits::{HirTrait, HirTraitImpl};
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

impl HirModule {
    pub(crate) fn visit<'a, F: FnMut(&'a HirModule)>(&'a self, f: &mut F) {
        f(self);
        for scope_module in self.content.modules.iter() {
            scope_module.visit(f);
        }
    }

    pub(crate) fn visit_mut<F: FnMut(&mut HirModule)>(&mut self, f: &mut F) {
        f(self);
        for scope_module in self.content.modules.iter_mut() {
            scope_module.visit_mut(f);
        }
    }
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
    pub type_alias: Vec<HirTypeAlias>,
    pub enums: Vec<HirEnum>,
    pub structs: Vec<HirStruct>,
    pub functions: Vec<HirFunction>,
    pub traits: Vec<HirTrait>,
    pub trait_impls: Vec<HirTraitImpl>,
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
