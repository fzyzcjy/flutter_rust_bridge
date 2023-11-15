pub(crate) mod misc;

use crate::codegen::ir::pack::{IrEnumPool, IrStructPool};
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::source_graph::modules::{Enum, Struct};
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

    pub(crate) fn parse_type(&mut self, ty: &Type) -> IrType {
        todo!()
    }

    pub(crate) fn convert_path_to_ir_type(
        &mut self,
        type_path: &TypePath,
    ) -> Result<IrType, String> {
        todo!()
    }

    pub(crate) fn resolve_alias<'b: 'a>(&self, ty: &'b Type) -> &Type {
        todo!()
    }
}
