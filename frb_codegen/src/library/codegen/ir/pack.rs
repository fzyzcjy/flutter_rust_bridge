use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrEnumIdent};
use crate::codegen::ir::ty::structure::{IrStruct, IrStructIdent};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub type IrStructPool = HashMap<IrStructIdent, IrStruct>;
pub type IrEnumPool = HashMap<IrEnumIdent, IrEnum>;

#[derive(Debug, Clone, serde::Serialize)]
pub struct IrPack {
    pub funcs: Vec<IrFunc>,
    pub struct_pool: IrStructPool,
    pub enum_pool: IrEnumPool,
    pub has_executor: bool,
}

impl IrPack {
    pub(crate) fn distinct_types(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
    ) -> Vec<IrType> {
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

    /// [f] returns [true] if it wants to stop going to the *children* of this subtree
    fn visit_types<F: FnMut(&IrType) -> bool>(
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
                if let Some(error_output) = &func.error_output {
                    error_output.visit_types(f, self);
                }
            }
        }
    }
}

/// Some information derivable from `IrPack`, but may be expensive to compute,
/// so we compute once and cache them.
pub(crate) struct IrPackComputedCache {
    pub(crate) distinct_input_types: Vec<IrType>,
    pub(crate) distinct_output_types: Vec<IrType>,
    pub(crate) input_and_output_types: Vec<IrType>,
}

impl IrPackComputedCache {
    pub fn compute(ir_pack: &IrPack) -> Self {
        let distinct_input_types = ir_pack.distinct_types(true, false);
        let distinct_output_types = ir_pack.distinct_types(false, true);
        let input_and_output_types = distinct_input_types
            .iter()
            .cloned()
            .chain(distinct_output_types.iter().cloned())
            .collect_vec();
        Self {
            distinct_input_types,
            distinct_output_types,
            input_and_output_types,
        }
    }
}
