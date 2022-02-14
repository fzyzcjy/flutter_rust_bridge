use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeOptionalGenerator(IrTypeOptional);

impl TypeDartGeneratorTrait for TypeOptionalGenerator {}
