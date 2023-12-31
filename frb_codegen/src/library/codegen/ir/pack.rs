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
pub(crate) struct IrPack {
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
        let mut gatherer = DistinctTypeGatherer::new();
        self.visit_types(
            &mut |ty| gatherer.add(ty),
            include_func_inputs,
            include_func_output,
        );
        gatherer.gather()
    }

    /// [f] returns [true] if it wants to stop going to the *children* of this subtree
    fn visit_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        include_func_inputs: bool,
        include_func_output: bool,
    ) {
        for func in &self.funcs {
            func.visit_types(f, include_func_inputs, include_func_output, self)
        }
    }
}

/// Some information derivable from `IrPack`, but may be expensive to compute,
/// so we compute once and cache them.
pub(crate) struct IrPackComputedCache {
    // pub(crate) distinct_input_types: Vec<IrType>,
    // pub(crate) distinct_output_types: Vec<IrType>,
    pub(crate) distinct_types: Vec<IrType>,
}

impl IrPackComputedCache {
    pub fn compute(ir_pack: &IrPack) -> Self {
        // let distinct_input_types = ir_pack.distinct_types(true, false);
        // let distinct_output_types = ir_pack.distinct_types(false, true);
        let distinct_types = ir_pack.distinct_types(true, true);
        Self {
            // distinct_input_types,
            // distinct_output_types,
            distinct_types,
        }
    }
}

pub(crate) struct DistinctTypeGatherer {
    seen_idents: HashSet<String>,
    ans: Vec<IrType>,
}

impl DistinctTypeGatherer {
    pub fn new() -> Self {
        Self {
            seen_idents: HashSet::default(),
            ans: vec![],
        }
    }

    pub(crate) fn add(&mut self, ty: &IrType) -> bool {
        let ident = ty.safe_ident();
        let contains = self.seen_idents.contains(&ident);
        if !contains {
            self.seen_idents.insert(ident);
            self.ans.push(ty.clone());
        }
        contains
    }

    pub(crate) fn gather(self) -> Vec<IrType> {
        self.ans
            .into_iter()
            // make the output change less when input change
            .sorted_by_key(|ty| ty.safe_ident())
            .collect()
    }
}
