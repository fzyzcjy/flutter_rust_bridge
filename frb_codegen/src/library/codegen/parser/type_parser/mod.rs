pub(crate) mod array;
pub(crate) mod enumeration;
pub(crate) mod misc;
pub(crate) mod path;
pub(crate) mod path_data;
pub(crate) mod structure;
pub(crate) mod tuple;
pub(crate) mod ty;

use crate::codegen::ir::pack::{IrEnumPool, IrStructPool};
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::source_graph::modules::{Enum, Struct};
use crate::codegen::parser::type_parser::misc::convert_ident_str;
use std::collections::{HashMap, HashSet};
use syn::{Type, TypePath};

pub(crate) struct TypeParser<'a> {
    src_structs: HashMap<String, &'a Struct>,
    src_enums: HashMap<String, &'a Enum>,
    src_types: HashMap<String, Type>,
    parsing_or_parsed_struct_names: HashSet<String>,
    struct_pool: IrStructPool,
    parsed_enums: HashSet<String>,
    enum_pool: IrEnumPool,
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
            struct_pool: HashMap::new(),
            enum_pool: HashMap::new(),
            parsing_or_parsed_struct_names: HashSet::new(),
            parsed_enums: HashSet::new(),
        }
    }

    pub(crate) fn consume(self) -> (IrStructPool, IrEnumPool) {
        (self.struct_pool, self.enum_pool)
    }

    pub(crate) fn resolve_alias<'b: 'a>(&self, ty: &'b Type) -> &Type {
        self.get_alias_type(ty).unwrap_or(ty)
    }

    pub(crate) fn get_alias_type(&self, ty: &Type) -> Option<&Type> {
        convert_ident_str(ty).and_then(|key| self.src_types.get(&key))
    }
}
