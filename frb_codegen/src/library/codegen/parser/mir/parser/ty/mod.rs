use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatEnum;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStruct;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::mir::func::MirFuncOwnerInfo;
use crate::codegen::ir::mir::pack::{MirEnumPool, MirStructPool};
use crate::codegen::ir::mir::ty::enumeration::{MirEnum, MirEnumIdent};
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::mir::ty::structure::{MirStruct, MirStructIdent};
use crate::codegen::ir::mir::ty::MirContext;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::ty::array::ArrayParserInfo;
use crate::codegen::parser::mir::parser::ty::enum_or_struct::EnumOrStructParserInfo;
use crate::codegen::parser::mir::parser::ty::rust_auto_opaque_implicit::RustAutoOpaqueParserInfo;
use crate::codegen::parser::mir::parser::ty::rust_opaque::RustOpaqueParserInfo;
use crate::utils::namespace::Namespace;
use std::collections::HashMap;
use syn::Type;

pub(crate) mod array;
pub(crate) mod concrete;
mod dart_fn;
mod enum_or_struct;
pub(crate) mod enumeration;
pub(crate) mod external_impl;
pub(crate) mod misc;
pub(crate) mod optional;
pub(crate) mod path;
pub(crate) mod path_data;
pub(crate) mod primitive;
pub(crate) mod result;
pub(crate) mod rust_auto_opaque_explicit;
pub(crate) mod rust_auto_opaque_implicit;
mod rust_opaque;
pub(crate) mod slice;
pub(crate) mod structure;
pub(crate) mod trait_def;
pub(crate) mod trait_object;
pub(crate) mod tuple;
#[allow(clippy::module_inception)]
pub(crate) mod ty;
pub(crate) mod unencodable;

pub(crate) struct TypeParser<'a> {
    src_structs: HashMap<String, &'a HirFlatStruct>,
    src_enums: HashMap<String, &'a HirFlatEnum>,
    pub(super) src_traits: HashMap<String, &'a HirFlatTrait>,
    src_types: HashMap<String, Type>,
    pub(super) proxied_types: Vec<MirType>,
    dart_code_of_type: HashMap<String, String>,
    struct_parser_info: EnumOrStructParserInfo<MirStructIdent, MirStruct>,
    enum_parser_info: EnumOrStructParserInfo<MirEnumIdent, MirEnum>,
    rust_opaque_parser_info: RustOpaqueParserInfo,
    rust_auto_opaque_parser_info: RustAutoOpaqueParserInfo,
    array_parser_info: ArrayParserInfo,
}

impl<'a> TypeParser<'a> {
    pub(crate) fn new_from_pack(ir_pack: &'a IrEarlyGeneratorPack) -> Self {
        Self::new(
            ir_pack.hir_flat_pack.structs_map(),
            ir_pack.hir_flat_pack.enums_map(),
            ir_pack.hir_flat_pack.traits_map(),
            ir_pack.hir_flat_pack.types_map(),
            ir_pack.proxied_types.clone(),
        )
    }

    fn new(
        src_structs: HashMap<String, &'a HirFlatStruct>,
        src_enums: HashMap<String, &'a HirFlatEnum>,
        src_traits: HashMap<String, &'a HirFlatTrait>,
        src_types: HashMap<String, Type>,
        proxied_types: Vec<MirType>,
    ) -> Self {
        TypeParser {
            src_structs,
            src_enums,
            src_traits,
            src_types,
            proxied_types,
            dart_code_of_type: HashMap::new(),
            struct_parser_info: EnumOrStructParserInfo::new(),
            enum_parser_info: EnumOrStructParserInfo::new(),
            rust_opaque_parser_info: RustOpaqueParserInfo::default(),
            rust_auto_opaque_parser_info: RustAutoOpaqueParserInfo::default(),
            array_parser_info: Default::default(),
        }
    }

    pub(crate) fn consume(self) -> (MirStructPool, MirEnumPool, HashMap<String, String>) {
        (
            self.struct_parser_info.object_pool,
            self.enum_parser_info.object_pool,
            self.dart_code_of_type,
        )
    }

    pub(crate) fn parse_type(
        &mut self,
        ty: &Type,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<MirType> {
        TypeParserWithContext::new(self, context).parse_type(ty)
    }

    pub(crate) fn transform_rust_auto_opaque(
        &mut self,
        ty_raw: &MirTypeRustAutoOpaqueImplicit,
        transform: impl FnOnce(&str) -> String,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<MirType> {
        TypeParserWithContext::new(self, context).transform_rust_auto_opaque(ty_raw, transform)
    }
}

pub(crate) struct TypeParserWithContext<'a, 'b, 'c> {
    pub inner: &'b mut TypeParser<'a>,
    pub context: &'c TypeParserParsingContext,
}

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub fn new(inner: &'b mut TypeParser<'a>, context: &'c TypeParserParsingContext) -> Self {
        Self { inner, context }
    }
}

pub(crate) struct TypeParserParsingContext {
    pub(crate) initiated_namespace: Namespace,
    pub(crate) func_attributes: FrbAttributes,
    pub(crate) default_stream_sink_codec: CodecMode,
    pub(crate) default_rust_opaque_codec: RustOpaqueCodecMode,
    pub(crate) owner: Option<MirFuncOwnerInfo>,
}

impl MirContext for TypeParser<'_> {
    fn struct_pool(&self) -> &MirStructPool {
        &self.struct_parser_info.object_pool
    }

    fn enum_pool(&self) -> &MirEnumPool {
        &self.enum_parser_info.object_pool
    }
}
