use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::parser::source_graph::modules::{Enum, Module, Struct};
use std::collections::HashMap;
use syn::Type;

impl Module {
    pub fn collect_structs(&self) -> HashMap<NamespacedName, &Struct> {
        self.collect_objects(
            |module| &module.scope.structs,
            |x| (TODO /*x.0.ident.to_string()*/, x),
        )
    }

    pub fn collect_enums(&self) -> HashMap<NamespacedName, &Enum> {
        self.collect_objects(
            |module| &module.scope.enums,
            |x| (TODO /*x.0.ident.to_string()*/, x),
        )
    }

    pub fn collect_types(&self) -> HashMap<NamespacedName, Type> {
        self.collect_objects(
            |module| &module.scope.type_alias,
            |x| (TODO /*x.ident.clone()*/, x.target.clone()),
        )
    }

    fn collect_objects<'a, T: 'a, F, G, V: 'a>(
        &'a self,
        f: F,
        extract_entry: G,
    ) -> HashMap<NamespacedName, V>
    where
        F: Fn(&Module) -> &[T],
        G: Fn(&'a T) -> (NamespacedName, V),
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
