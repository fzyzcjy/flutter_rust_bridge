use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypePrimitiveListGenerator(IrTypePrimitiveList);

impl TypeDartGeneratorTrait for TypePrimitiveListGenerator {}
