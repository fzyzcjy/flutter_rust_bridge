use crate::generator::rust::generate_impl_into_into_dart;
use crate::generator::rust::ty::*;
use crate::generator::rust::ExternFuncCollector;
use crate::ir::*;
use crate::target::Acc;
use crate::target::Target;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeStructRefGenerator, IrTypeStructRef);

impl TypeRustGeneratorTrait for TypeStructRefGenerator<'_> {
    fn new_with_nullptr(&self, _collector: &mut ExternFuncCollector) -> String {
        let src = self.ir.get(self.context.ir_pack);

        let body = {
            src.fields
                .iter()
                .map(|field| {
                    format!(
                        "{}: {},",
                        field.name.rust_style(),
                        if field.ty.rust_wire_is_pointer(Target::Io) {
                            "core::ptr::null_mut()".to_owned()
                        } else if field.ty.is_rust_opaque() || field.ty.is_dart_opaque() {
                            format!(
                                "{}::new_with_null_ptr()",
                                field.ty.rust_wire_type(Target::Io)
                            )
                        } else {
                            "Default::default()".to_owned()
                        }
                    )
                })
                .collect_vec()
                .join("\n")
        };
        format!(
            r#"impl NewWithNullPtr for {} {{
                    fn new_with_null_ptr() -> Self {{
                        Self {{ {} }}
                    }}
                }}

                impl Default for {} {{
                    fn default() -> Self {{
                        Self::new_with_null_ptr()
                    }}
                }}
            "#,
            self.ir.rust_wire_type(Target::Io),
            body,
            self.ir.rust_wire_type(Target::Io),
        )
    }

    fn imports(&self) -> Option<String> {
        let api_struct = self.ir.get(self.context.ir_pack);
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
