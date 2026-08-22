use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
use crate::codegen::ir::mir::extra_type::MirExtraType;
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
    pub extra_types_all: Vec<MirExtraType>,
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
        filter: Option<Box<dyn Fn(&CodecModePack) -> bool>>,
    ) -> Vec<MirType> {
        let mut gatherer = DistinctTypeGatherer::new();
        self.visit_types(&mut |ty| gatherer.add(ty), &filter);
        gatherer.gather()
    }

    /// [f] returns [true] if it wants to stop going to the *children* of this subtree
    #[allow(clippy::type_complexity)]
    fn visit_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        filter: &Option<Box<dyn Fn(&CodecModePack) -> bool>>,
    ) {
        for func in &self.funcs_all {
            if filter.is_some() && !filter.as_ref().unwrap()(&func.codec_mode_pack) {
                continue;
            }
            func.visit_types(f, self)
        }

        for extra_ty in &self.extra_types_all {
            if filter.is_some() && !filter.as_ref().unwrap()(&extra_ty.codec_mode_pack) {
                continue;
            }
            extra_ty.ty.visit_types(f, self);
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
                    mir_pack.distinct_types(Some(Box::new(move |codec_mode_pack| {
                        codec_mode_pack.all().contains(&codec)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::mir::func::{
        MirFuncArgMode, MirFuncMode, MirFuncOutput, MirFuncOwnerInfo,
    };
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::utils::namespace::Namespace;

    fn codec_mode_pack(dart2rust: CodecMode, rust2dart: CodecMode) -> CodecModePack {
        CodecModePack {
            dart2rust,
            rust2dart,
        }
    }

    fn func(name: &str, impl_mode: MirFuncImplMode, codec_mode_pack: CodecModePack) -> MirFunc {
        MirFunc {
            namespace: Namespace::default(),
            name: MirIdent::new(name.into(), None),
            id: None,
            inputs: vec![],
            output: MirFuncOutput {
                normal: MirType::Primitive(MirTypePrimitive::I32),
                error: None,
            },
            owner: MirFuncOwnerInfo::Function,
            mode: MirFuncMode::Sync,
            stream_dart_await: false,
            rust_async: false,
            initializer: false,
            init_dart_code: None,
            arg_mode: MirFuncArgMode::Positional,
            accessor: None,
            comments: vec![],
            codec_mode_pack,
            rust_call_code: None,
            rust_aop_after: None,
            impl_mode,
            src_lineno_pseudo: 0,
        }
    }

    fn pack() -> MirPack {
        MirPack {
            funcs_all: vec![
                func(
                    "implemented",
                    MirFuncImplMode::Normal,
                    codec_mode_pack(CodecMode::Cst, CodecMode::Dco),
                ),
                func(
                    "declaration_only",
                    MirFuncImplMode::NoImpl,
                    codec_mode_pack(CodecMode::Sse, CodecMode::Pde),
                ),
            ],
            extra_types_all: vec![MirExtraType {
                ty: MirType::Primitive(MirTypePrimitive::U8),
                codec_mode_pack: codec_mode_pack(CodecMode::Sse, CodecMode::Pde),
            }],
            struct_pool: HashMap::new(),
            enum_pool: HashMap::new(),
            dart_code_of_type: HashMap::new(),
            existing_handler: None,
            skips: vec![],
            trait_impls: vec![],
            extra_rust_output_code: String::new(),
            extra_dart_output_code: GeneralDartCode::default(),
        }
    }

    fn idents(types: Vec<MirType>) -> Vec<String> {
        types.into_iter().map(|ty| ty.safe_ident()).collect()
    }

    /// Excludes declaration-only functions while preserving implemented function order.
    #[test]
    fn filters_functions_to_only_normal_implementations() {
        let functions = pack().funcs_with_impl();

        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name.rust_style(false), "implemented");
    }

    /// Collects sorted unique types and applies codec-mode filters to functions and extras.
    #[test]
    fn collects_distinct_types_for_each_codec_mode() {
        let cache = MirPackComputedCache::compute(&pack());

        assert_eq!(idents(cache.distinct_types), ["i_32", "u_8", "unit"]);
        assert_eq!(
            idents(cache.distinct_types_for_codec[&CodecMode::Cst].clone()),
            ["i_32", "unit"]
        );
        assert_eq!(
            idents(cache.distinct_types_for_codec[&CodecMode::Dco].clone()),
            ["i_32", "unit"]
        );
        assert_eq!(
            idents(cache.distinct_types_for_codec[&CodecMode::Sse].clone()),
            ["i_32", "u_8", "unit"]
        );
        assert_eq!(
            idents(cache.distinct_types_for_codec[&CodecMode::Pde].clone()),
            ["i_32", "u_8", "unit"]
        );
    }
}
