use std::collections::HashMap;
use std::path::PathBuf;
use derivative::Derivative;
use syn::{Visibility, Ident, ItemEnum, ItemStruct, Type};

#[derive(Clone)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Module {
    visibility: Visibility,
    file_path: PathBuf,
    module_path: Vec<String>,
    #[derivative(Debug="ignore")]
    source: Option<ModuleSource>,
    scope: Option<ModuleScope>,
}

#[derive(Debug, Clone)]
pub struct Import {
    path: Vec<String>,
    visibility: Visibility,
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
    ident: Ident,
    #[derivative(Debug="ignore")]
    src: ItemStruct,
    visibility: Visibility,
    path: Vec<String>,
    mirror: bool,
}

#[derive(Clone)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Enum {
    ident: Ident,
    #[derivative(Debug="ignore")]
    src: ItemEnum,
    visibility: Visibility,
    path: Vec<String>,
    mirror: bool,
}

#[derive(Clone, Debug)]
pub struct TypeAlias {
    ident: String,
    target: Type,
}

#[derive(Debug, Clone)]
pub struct ModuleScope {
    modules: Vec<Module>,
    enums: Vec<Enum>,
    structs: Vec<Struct>,
    imports: Vec<Import>,
    type_alias: Vec<TypeAlias>,
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