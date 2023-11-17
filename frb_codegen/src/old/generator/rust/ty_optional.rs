use crate::generator::rust::generate_import;
use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeOptionalGenerator, IrTypeOptional);

impl TypeRustGeneratorTrait for TypeOptionalGenerator<'_> {
    fn imports(&self) -> Option<String> {
        generate_import(&self.ir.inner, self.context.ir_pack, self.context.config)
    }
}
