use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeBoxedGenerator(IrTypeBoxed);

impl TypeDartGeneratorTrait for TypeBoxedGenerator {}
