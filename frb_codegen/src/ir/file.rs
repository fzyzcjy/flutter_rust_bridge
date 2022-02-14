use crate::ir::*;
use std::collections::{HashMap, HashSet};

pub type ApiStructPool = HashMap<String, ApiStruct>;
pub type ApiEnumPool = HashMap<String, ApiEnum>;

#[derive(Debug, Clone)]
pub struct ApiFile {
    pub funcs: Vec<ApiFunc>,
    pub struct_pool: ApiStructPool,
    pub enum_pool: ApiEnumPool,
    pub has_executor: bool,
}

impl ApiFile {
    /// [f] returns [true] if it wants to stop going to the *children* of this subtree
    pub fn visit_types<F: FnMut(&ApiType) -> bool>(
        &self,
        f: &mut F,
        include_func_inputs: bool,
        include_func_output: bool,
    ) {
        for func in &self.funcs {
            if include_func_inputs {
                for field in &func.inputs {
                    field.ty.visit_types(f, self);
                }
            }
            if include_func_output {
                func.output.visit_types(f, self);
            }
        }
    }

    pub fn distinct_types(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
    ) -> Vec<ApiType> {
        let mut seen_idents = HashSet::new();
        let mut ans = Vec::new();
        self.visit_types(
            &mut |ty| {
                let ident = ty.safe_ident();
                let contains = seen_idents.contains(&ident);
                if !contains {
                    seen_idents.insert(ident);
                    ans.push(ty.clone());
                }
                contains
            },
            include_func_inputs,
            include_func_output,
        );

        // make the output change less when input change
        ans.sort_by_key(|ty| ty.safe_ident());

        ans
    }
}
