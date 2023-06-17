use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

use super::TypeStructRefGenerator;

type_dart_generator_struct!(TypeRecordGenerator, IrTypeRecord);

impl TypeDartGeneratorTrait for TypeRecordGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        TypeStructRefGenerator {
            ir: self.ir.inner.clone(),
            context: self.context.clone(),
        }
        .api2wire_body()
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        TypeStructRefGenerator {
            ir: self.ir.inner.clone(),
            context: self.context.clone(),
        }
        .api_fill_to_wire_body()
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
