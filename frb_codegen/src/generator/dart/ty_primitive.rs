use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypePrimitiveGenerator(IrTypePrimitive);

impl TypeDartGeneratorTrait for TypePrimitiveGenerator {}
