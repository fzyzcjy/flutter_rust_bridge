use derivative::Derivative;
use std::collections::HashMap;
use std::path::PathBuf;
use syn::{Ident, ItemEnum, ItemStruct, Type};

#[derive(Clone, Debug)]
pub struct Module {
    pub(super) info: ModuleInfo,
    pub(super) scope: ModuleScope,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct ModuleInfo {
    pub(super) visibility: Visibility,
    pub(super) file_path: PathBuf,
    pub(super) module_path: Vec<String>,
    #[derivative(Debug = "ignore")]
    pub(super) source: ModuleSource,
}

/// Mirrors syn::Visibility, but can be created without a token
#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Restricted,
    // Not supported
    Inherited, // Usually means private
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

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Struct {
    ident: Ident,
    #[derivative(Debug = "ignore")]
    src: ItemStruct,
    visibility: Visibility,
    path: Vec<String>,
    mirror: bool,
}

#[derive(Clone, Derivative)]
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
    pub fn parse(info: ModuleInfo) -> Self {
        todo!()
    }

    pub fn collect_structs(&self) -> HashMap<String, &Struct> {
        self.collect_objects(|module| &module.scope.structs, |x| (x.ident.to_string(), x))
    }

    pub fn collect_enums(&self) -> HashMap<String, &Enum> {
        self.collect_objects(|module| &module.scope.enums, |x| (x.ident.to_string(), x))
    }

    pub fn collect_types(&self) -> HashMap<String, Type> {
        self.collect_objects(
            |module| &module.scope.type_alias,
            |x| (x.ident.clone(), x.target.clone()),
        )
    }

    fn collect_objects<T, F, G, K, V>(&self, f: F, extract_entry: G) -> HashMap<String, V>
    where
        F: Fn(&Module) -> &[T],
        G: Fn(&T) -> (String, V),
    {
        let mut ans = HashMap::new();
        self.visit_modules(&mut |module| {
            for item in f(module) {
                let (k, v) = extract_entry(item);
                ans.insert(k, v);
            }
        });
        ans
    }

    //noinspection RsNeedlessLifetimes
    fn visit_modules<'a, F: FnMut(&'a Module)>(&'a self, f: &mut F) {
        f(self);
        for scope_module in &self.scope.modules {
            scope_module.visit_modules(f);
        }
    }
}
