use crate::codegen::parser::source_graph::modules::{Enum, Module, Struct};
use log::{debug};
use std::collections::HashMap;
use std::fmt::Debug;
use syn::Type;

impl Module {
    pub fn collect_structs(&self) -> HashMap<String, &Struct> {
        self.collect_objects(
            |module| &module.scope.structs,
            |x| (x.0.ident.to_string(), x),
        )
    }

    pub fn collect_enums(&self) -> HashMap<String, &Enum> {
        self.collect_objects(|module| &module.scope.enums, |x| (x.0.ident.to_string(), x))
    }

    pub fn collect_types(&self) -> HashMap<String, Type> {
        self.collect_objects(
            |module| &module.scope.type_alias,
            |x| (x.ident.clone(), x.target.clone()),
        )
    }

    fn collect_objects<'a, T: 'a, F, G, V: 'a>(
        &'a self,
        f: F,
        extract_entry: G,
    ) -> HashMap<String, V>
    where
        F: Fn(&Module) -> &[T],
        G: Fn(&'a T) -> (String, V),
        V: Debug,
    {
        let mut ans = HashMap::new();
        self.visit_modules(&mut |module| {
            for item in f(module) {
                let (key, value) = extract_entry(item);
                if let Some(old_value) = ans.get(&key) {
                    debug!("Same key={key} has multiple values: {old_value:?} (thrown away) and {value:?} (used). This may or may not be a problem.");
                }
                let _old_value = ans.insert(key, value);
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
