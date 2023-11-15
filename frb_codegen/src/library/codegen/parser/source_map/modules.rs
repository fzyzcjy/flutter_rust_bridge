use std::collections::HashMap;
use std::path::PathBuf;
use derivative::Derivative;
use syn::Visibility;

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