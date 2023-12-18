use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeDartOpaqueGenerator, IrTypeDartOpaque);

impl TypeDartGeneratorTrait for TypeDartOpaqueGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(
                "
                inner.dartApi.initApi();
                final ptr = inner.new_DartOpaque();
                _api_fill_to_wire_DartOpaque(raw, ptr);
                return ptr;
                "
                .to_owned(),
            ),
            wasm: Some("return[raw, dropPort];".to_owned()),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        Some(
            "wireObj.handle = inner.new_dart_opaque(apiObj);
        wireObj.port = dropPort;"
                .to_owned(),
        )
    }

    fn wire2api_body(&self) -> String {
        "return _platform.inner.get_dart_object(raw);".into()
    }

    fn structs(&self) -> String {
        "".into()
    }
}
