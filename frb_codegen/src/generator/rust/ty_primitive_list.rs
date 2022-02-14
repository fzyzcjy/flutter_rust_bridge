use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypePrimitiveListGenerator(IrTypePrimitiveList);

impl TypeRustGeneratorTrait for TypePrimitiveListGenerator {}
