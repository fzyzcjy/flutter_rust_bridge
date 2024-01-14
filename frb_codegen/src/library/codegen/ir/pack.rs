use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrEnumIdent};
use crate::codegen::ir::ty::structure::{IrStruct, IrStructIdent};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

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
    fn distinct_types(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
        filter_func: Option<Box<dyn Fn(&IrFunc) -> bool>>,
    ) -> Vec<IrType> {
        let mut gatherer = DistinctTypeGatherer::new();
        self.visit_types(
            &mut |ty| gatherer.add(ty),
            include_func_inputs,
            include_func_output,
            filter_func,
        );
        gatherer.gather()
    }

    /// [f] returns [true] if it wants to stop going to the *children* of this subtree
    fn visit_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        include_func_inputs: bool,
        include_func_output: bool,
        filter_func: Option<impl Fn(&IrFunc) -> bool>,
    ) {
        for func in &self.funcs {
            if filter_func.is_none() || !filter_func.unwrap()(func) {
                continue;
            }
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
    pub(crate) distinct_types_for_codec: HashMap<CodecMode, Vec<IrType>>,
}

impl IrPackComputedCache {
    pub fn compute(ir_pack: &IrPack) -> Self {
        // let distinct_input_types = ir_pack.distinct_types(true, false);
        // let distinct_output_types = ir_pack.distinct_types(false, true);
        let distinct_types = ir_pack.distinct_types(true, true, None);
        let distinct_types_for_codec = CodecMode::iter()
            .map(|codec| {
                (
                    codec,
                    ir_pack.distinct_types(
                        true,
                        true,
                        Some(Box::new(|f: &IrFunc| {
                            // currently quite coarse
                            f.codec_mode_pack.dart2rust == codec
                                || f.codec_mode_pack.rust2dart == codec
                        })),
                    ),
                )
            })
            .collect();
        Self {
            // distinct_input_types,
            // distinct_output_types,
            distinct_types,
            distinct_types_for_codec,
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
