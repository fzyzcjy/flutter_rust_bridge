use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeStructRefGenerator(IrTypeStructRef);

impl TypeRustGeneratorTrait for TypeStructRefGenerator {}
