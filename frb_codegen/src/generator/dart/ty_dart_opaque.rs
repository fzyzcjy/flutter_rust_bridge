use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeDartOpaqueGenerator, IrTypeDartOpaque);

impl TypeDartGeneratorTrait for TypeDartOpaqueGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let body = "return inner.new_dart_opaque(raw, dropPort);";
        Acc {
            io: Some(body.to_owned()),
            wasm: Some(body.to_owned()),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        None
    }

    fn wire2api_body(&self) -> String {
        "return _platform.inner.get_dart_object(raw);".into()
    }

    fn structs(&self) -> String {
        "".into()
    }
}
