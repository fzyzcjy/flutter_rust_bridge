use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
use crate::codegen::ir::early_generator::proxied_type::IrEarlyGeneratorProxiedType;
use crate::codegen::ir::early_generator::trait_def_info::IrEarlyGeneratorTraitDefInfo;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatEnum;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStruct;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::mir::custom_ser_des::MirCustomSerDes;
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
use crate::codegen::parser::mir::ParseMode;
use crate::utils::basic_code::general_code::GeneralDartCode;
use crate::utils::namespace::Namespace;
use std::collections::HashMap;
use syn::Type;

pub(crate) mod array;
pub(crate) mod concrete;
pub(crate) mod custom_ser_des;
mod dart_fn;
mod enum_or_struct;
pub(crate) mod enumeration;
pub(crate) mod external_impl;
pub(crate) mod generics;
pub(crate) mod lifetimeable;
pub(crate) mod misc;
mod namespace;
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
    pub(super) proxied_types: Vec<IrEarlyGeneratorProxiedType>,
    pub(super) trait_def_infos: Vec<IrEarlyGeneratorTraitDefInfo>,
    pub(super) custom_ser_des_infos: Vec<MirCustomSerDes>,
    dart_code_of_type: HashMap<String, GeneralDartCode>,
    struct_parser_info: EnumOrStructParserInfo<MirStructIdent, MirStruct>,
    enum_parser_info: EnumOrStructParserInfo<MirEnumIdent, MirEnum>,
    rust_opaque_parser_info: RustOpaqueParserInfo,
    rust_auto_opaque_parser_info: RustAutoOpaqueParserInfo,
    array_parser_info: ArrayParserInfo,
    has_logged_lifetimeable: bool,
}

impl<'a> TypeParser<'a> {
    pub(crate) fn new_from_pack(ir_pack: &'a IrEarlyGeneratorPack) -> Self {
        Self::new(
            ir_pack.hir_flat_pack.structs_map(),
            ir_pack.hir_flat_pack.enums_map(),
            ir_pack.hir_flat_pack.traits_map(),
            ir_pack.hir_flat_pack.types_map(),
            ir_pack.proxied_types.clone(),
            ir_pack.trait_def_infos.clone(),
        )
    }

    fn new(
        src_structs: HashMap<String, &'a HirFlatStruct>,
        src_enums: HashMap<String, &'a HirFlatEnum>,
        src_traits: HashMap<String, &'a HirFlatTrait>,
        src_types: HashMap<String, Type>,
        proxied_types: Vec<IrEarlyGeneratorProxiedType>,
        trait_def_infos: Vec<IrEarlyGeneratorTraitDefInfo>,
    ) -> Self {
        TypeParser {
            src_structs,
            src_enums,
            src_traits,
            src_types,
            proxied_types,
            trait_def_infos,
            custom_ser_des_infos: Default::default(),
            dart_code_of_type: HashMap::new(),
            struct_parser_info: EnumOrStructParserInfo::new(),
            enum_parser_info: EnumOrStructParserInfo::new(),
            rust_opaque_parser_info: RustOpaqueParserInfo::default(),
            rust_auto_opaque_parser_info: RustAutoOpaqueParserInfo::default(),
            array_parser_info: Default::default(),
            has_logged_lifetimeable: false,
        }
    }

    pub(crate) fn consume(self) -> (MirStructPool, MirEnumPool, HashMap<String, GeneralDartCode>) {
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

#[derive(Clone)]
pub(crate) struct TypeParserParsingContext {
    pub(crate) initiated_namespace: Namespace,
    pub(crate) func_attributes: FrbAttributes,
    pub(crate) struct_or_enum_attributes: Option<FrbAttributes>,
    // TODO if still not used later, rm it
    #[allow(dead_code)]
    pub(crate) rust_output_path_namespace: Namespace,
    pub(crate) default_stream_sink_codec: CodecMode,
    pub(crate) default_rust_opaque_codec: RustOpaqueCodecMode,
    pub(crate) owner: Option<MirFuncOwnerInfo>,
    pub(crate) enable_lifetime: bool,
    pub(crate) type_64bit_int: bool,
    pub(crate) forbid_type_self: bool,
    pub(crate) parse_mode: ParseMode,
}

impl TypeParserParsingContext {
    pub(crate) fn with_struct_or_enum_attributes(&self, x: FrbAttributes) -> Self {
        Self {
            struct_or_enum_attributes: Some(x),
            ..self.clone()
        }
    }
}

impl MirContext for TypeParser<'_> {
    fn struct_pool(&self) -> &MirStructPool {
        &self.struct_parser_info.object_pool
    }

    fn enum_pool(&self) -> &MirEnumPool {
        &self.enum_parser_info.object_pool
    }
}
