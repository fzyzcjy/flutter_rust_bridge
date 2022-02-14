use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeEnumRefGenerator(IrTypeEnumRef);

impl TypeRustGeneratorTrait for TypeEnumRefGenerator {}
