use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeEnumRefGenerator(IrTypeEnumRef);

impl TypeDartGeneratorTrait for TypeEnumRefGenerator {}
