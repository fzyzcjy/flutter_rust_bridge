use crate::{ir::IrTypeDynamic, target::Acc, type_rust_generator_struct};

use super::TypeRustGeneratorTrait;

type_rust_generator_struct!(TypeDynamicGenerator, IrTypeDynamic);

impl TypeRustGeneratorTrait for TypeDynamicGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        Default::default()
    }
}
