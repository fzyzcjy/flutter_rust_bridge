use crate::generator::dart::dart_comments;
use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeStructRefGenerator(pub IrTypeStructRef);

impl TypeDartGeneratorTrait for TypeStructRefGenerator {
    fn api2wire_body(&self) -> String {
        "".to_string()
    }

    fn api_fill_to_wire_body(&self) -> String {
        let s = self.0.get(api_file);
        s.fields
            .iter()
            .map(|field| {
                format!(
                    "wireObj.{} = _api2wire_{}(apiObj.{});",
                    field.name.rust_style(),
                    field.ty.safe_ident(),
                    field.name.dart_style()
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn wire2api_body(&self) -> String {
        let s = self.0.get(api_file);
        let inner = s
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                format!(
                    "{}: _wire2api_{}(arr[{}]),",
                    field.name.dart_style(),
                    field.ty.safe_ident(),
                    idx
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "final arr = raw as List<dynamic>;
                if (arr.length != {}) throw Exception('unexpected arr length: expect {} but see ${{arr.length}}');
                return {}({});",
            s.fields.len(),
            s.fields.len(),
            s.name, inner,
        )
    }

    fn structs(&self) -> String {
        let field_declarations = self
            .0
            .fields
            .iter()
            .map(|f| {
                let comments = dart_comments(&f.comments);
                format!(
                    "{}final {} {};",
                    comments,
                    f.ty.dart_api_type(),
                    f.name.dart_style()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let constructor_params = self
            .0
            .fields
            .iter()
            .map(|f| {
                format!(
                    "{}this.{},",
                    f.ty.dart_required_modifier(),
                    f.name.dart_style()
                )
            })
            .collect::<Vec<_>>()
            .join("");

        let comments = dart_comments(&self.0.comments);

        format!(
            "{}class {} {{
            {}

            {}({{{}}});
        }}",
            comments, self.0.name, field_declarations, self.0.name, constructor_params
        )
    }
}
