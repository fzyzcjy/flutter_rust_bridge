use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;
use crate::utils::misc::BlockIndex;

type_dart_generator_struct!(TypeRecordGenerator, IrTypeRecord);

impl TypeDartGeneratorTrait for TypeRecordGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let values = self
            .ir
            .values
            .iter()
            .enumerate()
            .map(|(idx, ty)| format!("api2wire_{}(raw.${})", ty.safe_ident(), idx + 1))
            .collect::<Vec<_>>()
            .join(",");
        Acc {
            wasm: Some(format!("return [{values}];")),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        let ir = self.ir.inner.get(self.context.ir_file);
        let values = ir
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                super::ty_struct::api_fill_for_field(
                    &field.ty.safe_ident(),
                    &format!("${}", idx + 1),
                    field.name.rust_style(),
                    field.ty.is_struct_ref_or_enum_ref_or_record(),
                    self.context.config.shared,
                    self.is_type_shared_by_safe_ident(&field.ty),
                    self.context
                        .all_configs
                        .get_dart_api2wire_funcs(BlockIndex::new_shared()),
                )
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
            .map(|(idx, ty)| format!("_wire2api_{}(arr[{idx}])", ty.safe_ident()))
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
