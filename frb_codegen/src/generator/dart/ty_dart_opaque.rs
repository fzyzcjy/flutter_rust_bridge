use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeDartOpaqueGenerator, IrTypeDartOpaque);

impl TypeDartGeneratorTrait for TypeDartOpaqueGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc::default()
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        None
    }

    fn wire2api_body(&self) -> String {
        "".into()
    }

    fn structs(&self) -> String {
        "".into()
    }
}