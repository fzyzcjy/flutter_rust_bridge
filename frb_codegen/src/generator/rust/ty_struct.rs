use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeStructRefGenerator(pub IrTypeStructRef);

impl TypeRustGeneratorTrait for TypeStructRefGenerator {
    fn wire2api_body(&self) -> String {
        let api_struct = self.0.get(api_file);
        let fields_str = &api_struct
            .fields
            .iter()
            .map(|field| {
                format!(
                    "{} self.{}.wire2api()",
                    if api_struct.is_fields_named {
                        field.name.rust_style().to_string() + ": "
                    } else {
                        String::new()
                    },
                    field.name.rust_style()
                )
            })
            .collect::<Vec<_>>()
            .join(",");

        let (left, right) = api_struct.brackets_pair();
        format!("{}{}{}{}", self.0.rust_api_type(), left, fields_str, right).into()
    }

    fn wire_struct_fields(&self) -> Vec<String> {
        let s = self.0.get(api_file);
        s.fields
            .iter()
            .map(|field| {
                format!(
                    "{}: {}{}",
                    field.name.rust_style(),
                    field.ty.rust_wire_modifier(),
                    field.ty.rust_wire_type()
                )
            })
            .collect()
    }

    fn impl_intodart(&self) -> String {
        let body = self
            .0
            .fields
            .iter()
            .map(|field| {
                format!(
                    "self.{}.into_dart()",
                    field.name_rust_style(self.0.is_fields_named)
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            "impl support::IntoDart for {} {{
                fn into_dart(self) -> support::DartCObject {{
                    vec![
                        {}
                    ].into_dart()
                }}
            }}
            impl support::IntoDartExceptPrimitive for {} {{}}
            ",
            self.0.name, body, self.0.name,
        )
    }

    fn new_with_nullptr(&self) -> String {
        let body = {
            self.0
                .fields
                .iter()
                .map(|field| {
                    format!(
                        "{}: {},",
                        field.name.rust_style(),
                        if field.ty.rust_wire_is_pointer() {
                            "core::ptr::null_mut()"
                        } else {
                            "Default::default()"
                        }
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        };
        format!(
            r#"impl NewWithNullPtr for {} {{
                    fn new_with_null_ptr() -> Self {{
                        Self {{ {} }}
                    }}
                }}
            "#,
            rust_wire_type, body,
        )
    }

    fn imports(&self) -> Option<String> {
        let api_struct = self.0.get(api_file);
        if api_struct.path.is_some() {
            Some(format!(
                "use {};",
                api_struct.path.as_ref().unwrap().join("::")
            ))
        } else {
            None
        }
    }
}
