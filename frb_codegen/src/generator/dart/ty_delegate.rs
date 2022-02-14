use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeDelegateGenerator(IrTypeDelegate);

impl TypeDartGeneratorTrait for TypeDelegateGenerator {}
