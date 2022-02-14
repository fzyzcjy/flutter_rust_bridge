use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeBoxedGenerator(IrTypeBoxed);

impl TypeRustGeneratorTrait for TypeBoxedGenerator {}
