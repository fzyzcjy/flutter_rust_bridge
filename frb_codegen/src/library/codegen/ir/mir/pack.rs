use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::mir::func::{MirFunc, MirFuncImplMode};
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::ir::mir::ty::enumeration::{MirEnum, MirEnumIdent};
use crate::codegen::ir::mir::ty::structure::{MirStruct, MirStructIdent};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::misc::skip::IrSkip;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::basic_code::general_code::GeneralDartCode;
use crate::utils::namespace::NamespacedName;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

pub type MirStructPool = HashMap<MirStructIdent, MirStruct>;
pub type MirEnumPool = HashMap<MirEnumIdent, MirEnum>;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MirPack {
    pub funcs_all: Vec<MirFunc>, // Do not direct use, but use things like `funcs_with_impl`
    pub struct_pool: MirStructPool,
    pub enum_pool: MirEnumPool,
    pub dart_code_of_type: HashMap<String, GeneralDartCode>,
    pub existing_handler: Option<NamespacedName>,
    pub skips: Vec<IrSkip>,
    pub trait_impls: Vec<MirTraitImpl>,
    pub extra_rust_output_code: String,
    pub extra_dart_output_code: GeneralDartCode,
}

impl MirPack {
    pub(crate) fn funcs_with_impl(&self) -> Vec<MirFunc> {
        (self.funcs_all.iter())
            .filter(|f| f.impl_mode == MirFuncImplMode::Normal)
            .cloned()
            .collect()
    }

    #[allow(clippy::type_complexity)]
    pub fn distinct_types(
        &self,
        filter_func: Option<Box<dyn Fn(&MirFunc) -> bool>>,
    ) -> Vec<MirType> {
        let mut gatherer = DistinctTypeGatherer::new();
        self.visit_types(&mut |ty| gatherer.add(ty), &filter_func);
        gatherer.gather()
    }

    /// [f] returns [true] if it wants to stop going to the *children* of this subtree
    fn visit_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        filter_func: &Option<impl Fn(&MirFunc) -> bool>,
    ) {
        for func in &self.funcs_all {
            if filter_func.is_some() && !filter_func.as_ref().unwrap()(func) {
                continue;
            }
            func.visit_types(f, self)
        }
    }
}

/// Some information derivable from `MirPack`, but may be expensive to compute,
/// so we compute once and cache them.
pub(crate) struct MirPackComputedCache {
    // pub(crate) distinct_input_types: Vec<MirType>,
    // pub(crate) distinct_output_types: Vec<MirType>,
    pub(crate) distinct_types: Vec<MirType>,
    pub(crate) distinct_types_for_codec: HashMap<CodecMode, Vec<MirType>>,
}

impl MirPackComputedCache {
    pub fn compute(mir_pack: &MirPack) -> Self {
        // let distinct_input_types = mir_pack.distinct_types(true, false);
        // let distinct_output_types = mir_pack.distinct_types(false, true);
        let distinct_types = mir_pack.distinct_types(None);
        let distinct_types_for_codec = CodecMode::iter()
            .map(|codec| {
                (
                    codec,
                    mir_pack.distinct_types(Some(Box::new(move |f: &MirFunc| {
                        (f.codec_mode_pack.all().iter()).any(|c| *c == codec)
                    }))),
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
    ans: Vec<MirType>,
}

impl DistinctTypeGatherer {
    pub fn new() -> Self {
        Self {
            seen_idents: HashSet::default(),
            ans: vec![],
        }
    }

    pub(crate) fn add(&mut self, ty: &MirType) -> bool {
        let ident = ty.safe_ident();
        let contains = self.seen_idents.contains(&ident);
        if !contains {
            self.seen_idents.insert(ident);
            self.ans.push(ty.clone());
        }
        contains
    }

    pub(crate) fn gather(self) -> Vec<MirType> {
        self.ans
            .into_iter()
            // make the output change less when input change
            .sorted_by_cached_key(|ty| ty.safe_ident())
            .collect()
    }
}
