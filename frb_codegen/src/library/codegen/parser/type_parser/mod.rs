pub(crate) mod array;
pub(crate) mod concrete;
mod enum_or_struct;
pub(crate) mod enumeration;
pub(crate) mod misc;
pub(crate) mod optional;
pub(crate) mod path;
pub(crate) mod path_data;
pub(crate) mod primitive;
pub(crate) mod structure;
pub(crate) mod tuple;
pub(crate) mod ty;
pub(crate) mod unencodable;
pub(crate) mod vec;

use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::codegen::ir::pack::{IrEnumPool, IrStructPool};
use crate::codegen::ir::ty::enumeration::{IrEnum, IrEnumIdent};
use crate::codegen::ir::ty::structure::{IrStruct, IrStructIdent};
use crate::codegen::parser::source_graph::modules::{Enum, Struct};
use crate::codegen::parser::type_parser::enum_or_struct::EnumOrStructParserInfo;
use std::collections::{HashMap, HashSet};
use syn::Type;

pub(crate) struct TypeParser<'a> {
    src_structs: HashMap<String, &'a Struct>,
    src_enums: HashMap<String, &'a Enum>,
    src_types: HashMap<String, Type>,
    struct_parser_info: EnumOrStructParserInfo<IrStructIdent, IrStruct>,
    enum_parser_info: EnumOrStructParserInfo<IrEnumIdent, IrEnum>,
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
        }
    }

    pub(crate) fn consume(self) -> (IrStructPool, IrEnumPool) {
        (
            self.struct_parser_info.object_pool,
            self.enum_parser_info.object_pool,
        )
    }
}

pub(crate) struct TypeParserWithContext<'a> {
    pub inner: TypeParser<'a>,
    pub context: TypeParserParsingContext,
}

pub(crate) struct TypeParserParsingContext {
    pub(crate) initiated_namespace: Namespace,
}
