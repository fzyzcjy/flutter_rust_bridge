use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypePrimitiveGenerator(IrTypePrimitive);

impl TypeRustGeneratorTrait for TypePrimitiveGenerator {
    fn wire2api_body(&self) -> String {
        "self".into()
    }
}
