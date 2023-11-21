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

use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::pack::{IrEnumPool, IrStructPool};
use crate::codegen::parser::source_graph::modules::{Enum, Struct};
use std::collections::{HashMap, HashSet};
use syn::Type;

pub(crate) struct TypeParser<'a> {
    src_structs: HashMap<String, &'a Struct>,
    src_enums: HashMap<String, &'a Enum>,
    src_types: HashMap<String, Type>,
    parsing_or_parsed_structs: HashSet<NamespacedName>,
    struct_pool: IrStructPool,
    parsing_or_parsed_enums: HashSet<NamespacedName>,
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
            parsing_or_parsed_structs: HashSet::new(),
            parsing_or_parsed_enums: HashSet::new(),
        }
    }

    pub(crate) fn consume(self) -> (IrStructPool, IrEnumPool) {
        (self.struct_pool, self.enum_pool)
    }
}
