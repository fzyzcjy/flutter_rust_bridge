use crate::{ir::IrTypeDynamic, target::Acc, type_dart_generator_struct};

use super::TypeDartGeneratorTrait;

type_dart_generator_struct!(TypeDynamicGenerator, IrTypeDynamic);

impl TypeDartGeneratorTrait for TypeDynamicGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc::default()
    }
    fn wire2api_body(&self) -> String {
        "return raw;".into()
    }
}
