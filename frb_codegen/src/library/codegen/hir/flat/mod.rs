use crate::codegen::hir::hierarchical::struct_or_enum::{Enum, Struct};
use std::collections::HashMap;
use syn::Type;

pub(crate) struct HirFlatCrate<'a> {
    pub structs: HashMap<String, &'a Struct>,
    pub enums: HashMap<String, &'a Enum>,
    pub types: HashMap<String, Type>,
}
