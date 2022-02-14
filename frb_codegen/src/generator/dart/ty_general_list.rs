use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeGeneralListGenerator(IrTypeGeneralList);

impl TypeDartGeneratorTrait for TypeGeneralListGenerator {}
