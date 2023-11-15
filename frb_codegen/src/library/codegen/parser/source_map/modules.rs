use std::collections::HashMap;
use std::path::PathBuf;
use derivative::Derivative;
use syn::{Visibility, Ident, ItemEnum, ItemStruct, Type};

#[derive(Clone)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Module {
    pub visibility: Visibility,
    pub file_path: PathBuf,
    pub module_path: Vec<String>,
    #[derivative(Debug="ignore")]
    pub source: Option<ModuleSource>,
    pub scope: Option<ModuleScope>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub path: Vec<String>,
    pub visibility: Visibility,
}

#[derive(Debug, Clone)]
pub enum ModuleSource {
    File(syn::File),
    ModuleInFile(Vec<syn::Item>),
}

#[derive(Clone)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Struct {
    pub ident: Ident,
    #[derivative(Debug="ignore")]
    pub src: ItemStruct,
    pub visibility: Visibility,
    pub path: Vec<String>,
    pub mirror: bool,
}

#[derive(Clone)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Enum {
    pub ident: Ident,
    #[derivative(Debug="ignore")]
    pub src: ItemEnum,
    pub visibility: Visibility,
    pub path: Vec<String>,
    pub mirror: bool,
}

#[derive(Clone, Debug)]
pub struct TypeAlias {
    pub ident: String,
    pub target: Type,
}

#[derive(Debug, Clone)]
pub struct ModuleScope {
    pub modules: Vec<Module>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
    pub imports: Vec<Import>,
    pub type_alias: Vec<TypeAlias>,
}

impl Module {
    pub fn collect_structs<'a>(&'a self, container: &mut HashMap<String, &'a Struct>) {
        let scope = self.scope.as_ref().unwrap();
        for scope_struct in &scope.structs {
            container.insert(scope_struct.ident.to_string(), scope_struct);
        }
        for scope_module in &scope.modules {
            scope_module.collect_structs(container);
        }
    }

    pub fn collect_structs_to_vec(&self) -> HashMap<String, &Struct> {
        let mut ans = HashMap::new();
        self.collect_structs(&mut ans);
        ans
    }

    pub fn collect_enums<'a>(&'a self, container: &mut HashMap<String, &'a Enum>) {
        let scope = self.scope.as_ref().unwrap();
        for scope_enum in &scope.enums {
            container.insert(scope_enum.ident.to_string(), scope_enum);
        }
        for scope_module in &scope.modules {
            scope_module.collect_enums(container);
        }
    }

    pub fn collect_enums_to_vec(&self) -> HashMap<String, &Enum> {
        let mut ans = HashMap::new();
        self.collect_enums(&mut ans);
        ans
    }

    pub fn collect_types(&self, container: &mut HashMap<String, Type>) {
        let scope = self.scope.as_ref().unwrap();
        for scope_type in &scope.type_alias {
            container.insert(scope_type.ident.to_string(), scope_type.target.clone());
        }
        for scope_module in &scope.modules {
            scope_module.collect_types(container);
        }
    }

    pub fn collect_types_to_pool(&self) -> HashMap<String, Type> {
        let mut ans = HashMap::new();
        self.collect_types(&mut ans);
        ans
    }
}