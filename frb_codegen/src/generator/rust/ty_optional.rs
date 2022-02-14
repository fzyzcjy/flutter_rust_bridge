use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeOptionalGenerator(IrTypeOptional);

impl TypeRustGeneratorTrait for TypeOptionalGenerator {}
