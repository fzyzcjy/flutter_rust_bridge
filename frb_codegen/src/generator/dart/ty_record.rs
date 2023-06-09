use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeRecordGenerator, IrTypeRecord);

impl TypeDartGeneratorTrait for TypeRecordGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            wasm: self.context.config.wasm_enabled.then(|| {
                let values = self
                    .ir
                    .values
                    .iter()
                    .enumerate()
                    .map(|(idx, value)| {
                        format!("api2wire_{}(raw.${})", value.safe_ident(), idx + 1)
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                format!("return [{values}];")
            }),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        let values = self
            .ir
            .values
            .iter()
            .enumerate()
            .map(|(idx, value)| {
                if value.is_struct() {
                    format!(
                        "_api_fill_to_wire_{}(apiObj.${}, wireObj.field{idx});",
                        value.safe_ident(),
                        idx + 1
                    )
                } else {
                    format!(
                        "wireObj.field{idx} = api2wire_{}(apiObj.${});",
                        value.safe_ident(),
                        idx + 1
                    )
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        Some(values)
    }

    fn wire2api_body(&self) -> String {
        let len = self.ir.values.len();
        let values = self
            .ir
            .values
            .iter()
            .enumerate()
            .map(|(idx, value)| format!("_wire2api_{}(arr[{idx}])", value.safe_ident()))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "final arr = raw as List<dynamic>;
            if (arr.length != {len}) {{
                throw Exception('Expected {len} elements, got ${{arr.length}}');
            }}
            return ({values},);"
        )
    }
}
