pub(crate) mod array;
pub(crate) mod concrete;
mod dart_fn;
mod enum_or_struct;
pub(crate) mod enumeration;
pub(crate) mod misc;
pub(crate) mod optional;
pub(crate) mod path;
pub(crate) mod path_data;
pub(crate) mod primitive;
pub(crate) mod reference;
pub(crate) mod rust_auto_opaque;
mod rust_opaque;
pub(crate) mod structure;
pub(crate) mod tuple;
pub(crate) mod ty;
pub(crate) mod unencodable;

use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::{IrEnumPool, IrStructPool};
use crate::codegen::ir::ty::enumeration::{IrEnum, IrEnumIdent};
use crate::codegen::ir::ty::structure::{IrStruct, IrStructIdent};
use crate::codegen::ir::ty::{IrContext, IrType};
use crate::codegen::parser::source_graph::modules::{Enum, Struct};
use crate::codegen::parser::type_parser::array::ArrayParserInfo;
use crate::codegen::parser::type_parser::enum_or_struct::EnumOrStructParserInfo;
use crate::codegen::parser::type_parser::rust_auto_opaque::RustAutoOpaqueParserInfo;
use crate::codegen::parser::type_parser::rust_opaque::RustOpaqueParserInfo;
use std::collections::HashMap;
use syn::Type;

pub(crate) struct TypeParser<'a> {
    src_structs: HashMap<String, &'a Struct>,
    src_enums: HashMap<String, &'a Enum>,
    src_types: HashMap<String, Type>,
    struct_parser_info: EnumOrStructParserInfo<IrStructIdent, IrStruct>,
    enum_parser_info: EnumOrStructParserInfo<IrEnumIdent, IrEnum>,
    rust_opaque_parser_info: RustOpaqueParserInfo,
    rust_auto_opaque_parser_info: RustAutoOpaqueParserInfo,
    array_parser_info: ArrayParserInfo,
}

impl<'a> TypeParser<'a> {
    pub(crate) fn new(
        src_structs: HashMap<String, &'a Struct>,
        src_enums: HashMap<String, &'a Enum>,
        src_types: HashMap<String, Type>,
    ) -> Self {
        TypeParser {
            src_structs,
            src_enums,
            src_types,
            struct_parser_info: EnumOrStructParserInfo::new(),
            enum_parser_info: EnumOrStructParserInfo::new(),
            rust_opaque_parser_info: RustOpaqueParserInfo::new(),
            rust_auto_opaque_parser_info: RustAutoOpaqueParserInfo::new(),
            array_parser_info: Default::default(),
        }
    }

    pub(crate) fn consume(self) -> (IrStructPool, IrEnumPool) {
        (
            self.struct_parser_info.object_pool,
            self.enum_parser_info.object_pool,
        )
    }

    pub(crate) fn parse_type(
        &mut self,
        ty: &Type,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<IrType> {
        TypeParserWithContext::new(self, context).parse_type(ty)
    }

    pub(crate) fn transform_type_rust_auto_opaque(
        &mut self,
        ty: &IrType,
        context: &TypeParserParsingContext,
    ) -> IrType {
        TypeParserWithContext::new(self, context).transform_type_rust_auto_opaque(ty)
    }

    pub(crate) fn check_candidate_rust_auto_opaque(
        &mut self,
        ty: &IrType,
        context: &TypeParserParsingContext,
    ) -> bool {
        TypeParserWithContext::new(self, context).check_candidate_rust_auto_opaque(ty)
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
    pub(crate) location: ParsingLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParsingLocation {
    Param,
    Return,
    Misc,
}

impl IrContext for TypeParser<'_> {
    fn struct_pool(&self) -> &IrStructPool {
        &self.struct_parser_info.object_pool
    }

    fn enum_pool(&self) -> &IrEnumPool {
        &self.enum_parser_info.object_pool
    }
}
