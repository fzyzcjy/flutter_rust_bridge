use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeOptionalGenerator, IrTypeOptional);

impl TypeRustGeneratorTrait for TypeOptionalGenerator<'_> {
    fn wire2api_body(&self) -> String {
        "".to_string()
    }

    fn imports(&self) -> Option<String> {
        self.generate_import(&self.ir.inner, self.context.ir_file)
    }
}
