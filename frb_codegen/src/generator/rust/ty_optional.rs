use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeOptionalGenerator(pub IrTypeOptional);

impl TypeRustGeneratorTrait for TypeOptionalGenerator {
    fn wire2api_body(&self) -> String {
        "".to_string()
    }

    fn imports(&self) -> Option<String> {
        self.generate_import(&self.0.inner, ir_file)
    }
}
