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
    #[derivative(Debug = "ignore")]
    // TODO why so many `Option`?
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
    #[derivative(Debug = "ignore")]
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
    #[derivative(Debug = "ignore")]
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
    pub fn collect_structs(&self) -> HashMap<String, &Struct> {
        self.collect_objects(|module| &module.scope.as_ref().unwrap().structs, |x| x.ident.to_string())
    }

    pub fn collect_enums(&self) -> HashMap<String, &Enum> {
        self.collect_objects(|module| &module.scope.as_ref().unwrap().enums, |x| x.ident.to_string())
    }

    pub fn collect_type_aliases(&self) -> HashMap<String, &TypeAlias> {
        self.collect_objects(|module| &module.scope.as_ref().unwrap().type_alias, |x| x.ident.clone())
    }

    fn collect_objects<T, F, G>(&self, f: F, extract_ident: G) -> HashMap<String, &T>
        where F: Fn(&Module) -> &[T],
              G: Fn(&T) -> String,
    {
        let mut ans = HashMap::new();
        self.visit_modules(&mut |module| {
            for item in f(module) {
                ans.insert(extract_ident(item), item);
            }
        });
        ans
    }

    fn visit_modules<F: FnMut(&Module)>(&self, f: &mut F) {
        f(self);
        for scope_module in &self.scope.as_ref().unwrap().modules {
            scope_module.visit_modules(f);
        }
    }
}