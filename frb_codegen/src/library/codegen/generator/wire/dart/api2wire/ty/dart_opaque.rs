use crate::codegen::generator::wire::dart::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::base::*;

impl<'a> WireDartGeneratorApi2wireTrait for DartOpaqueWireDartGenerator<'a> {
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
}
