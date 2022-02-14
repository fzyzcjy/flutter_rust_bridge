use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeStructRefGenerator(IrTypeStructRef);

impl TypeDartGeneratorTrait for TypeStructRefGenerator {}
