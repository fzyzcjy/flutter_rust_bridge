use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeDelegateGenerator(IrTypeDelegate);

impl TypeRustGeneratorTrait for TypeDelegateGenerator {}
