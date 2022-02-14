use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeOptionalGenerator(IrTypeOptional);

impl TypeRustGeneratorTrait for TypeOptionalGenerator {
    fn wire2api_body(&self) -> String {
        "".to_string()
    }

    fn imports(&self) -> Option<String> {
        self.generate_import(&self.0.inner, api_file)
    }
}
