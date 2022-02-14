use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeGeneralListGenerator(IrTypeGeneralList);

impl TypeRustGeneratorTrait for TypeGeneralListGenerator {}
