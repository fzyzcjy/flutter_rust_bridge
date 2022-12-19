use crate::generator::rust::ty::*;
use crate::generator::rust::ExternFuncCollector;
use crate::ir::*;
use crate::target::Acc;
use crate::target::Target;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeStructRefGenerator, IrTypeStructRef);

impl TypeRustGeneratorTrait for TypeStructRefGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        let api_struct = self.ir.get(self.context.ir_file);
        let fields: Acc<Vec<_>> = api_struct
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let field_name = field.name.rust_style();
                let field_ = if api_struct.is_fields_named {
                    format!("{field_name}: ")
                } else {
                    String::new()
                };

                Acc {
                    wasm: format!("{field_} self_.get({idx}).wire2api()"),
                    io: format!("{field_} self.{field_name}.wire2api()"),
                    ..Default::default()
                }
            })
            .collect();

        let (left, right) = api_struct.brackets_pair();
        let rust_api_type = self.ir.rust_api_type();
        Acc {
            io: Some(format!(
                "
                {rust_api_type}{left}{fields}{right}
                ",
                fields = fields.io.join(","),
            )),
            wasm: Some(format!(
                "
                let self_ = self.dyn_into::<JsArray>().unwrap();
                assert_eq!(self_.length(), {len}, \"Expected {len} elements, got {{}}\", self_.length());
                {rust_api_type}{left}{fields}{right}
                ",
                fields = fields.wasm.join(","),
                len = api_struct.fields.len(),
            )),
            ..Default::default()
        }
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        let s = self.ir.get(self.context.ir_file);
        Some(
            s.fields
                .iter()
                .map(|field| {
                    format!(
                        "{}: {}{}",
                        field.name.rust_style(),
                        field.ty.rust_wire_modifier(Target::Io),
                        field.ty.rust_wire_type(Target::Io)
                    )
                })
                .collect(),
        )
    }

    fn static_checks(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_file);
        src.wrapper_name.as_ref()?;

        let var = if src.is_fields_named {
            src.name.clone()
        } else {
            // let bindings cannot shadow tuple structs
            format!("{}_", src.name)
        };
        let checks = src
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                format!(
                    "let _: {} = {}.{};\n",
                    field.ty.rust_api_type(),
                    var,
                    if src.is_fields_named {
                        field.name.to_string()
                    } else {
                        i.to_string()
                    },
                )
            })
            .collect::<Vec<_>>()
            .join("");
        Some(format!(
            "{{ let {} = None::<{}>.unwrap(); {} }} ",
            var, src.name, checks
        ))
    }

    fn wrapper_struct(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_file);
        src.wrapper_name.as_ref().cloned()
    }

    fn wrap_obj(&self, obj: String, wired_fallible_func: bool) -> String {
        match self.wrapper_struct() {
            Some(wrapper) => {
                if wired_fallible_func {
                    format!("Ok({}({}?))", wrapper, obj)
                } else {
                    format!("{}({})", wrapper, obj)
                }
            }

            None => obj,
        }
    }

    fn impl_intodart(&self) -> String {
        let src = self.ir.get(self.context.ir_file);

        let unwrap = match &src.wrapper_name {
            Some(_) => ".0",
            None => "",
        };
        let body = src
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let field_ref = if src.is_fields_named {
                    field.name.rust_style().to_string()
                } else {
                    i.to_string()
                };
                let gen = TypeRustGenerator::new(
                    field.ty.clone(),
                    self.context.ir_file,
                    self.context.config,
                );
                // wired_fallible is always false here, this parameter is only used for generate_wire_func
                gen.convert_to_dart(gen.wrap_obj(format!("self{}.{}", unwrap, field_ref), false))
            })
            .collect::<Vec<_>>()
            .join(",\n");

        let name = match &src.wrapper_name {
            Some(wrapper) => wrapper,
            None => &src.name,
        };
        format!(
            "impl support::IntoDart for {} {{
                fn into_dart(self) -> support::DartAbi {{
                    vec![
                        {}
                    ].into_dart()
                }}
            }}
            impl support::IntoDartExceptPrimitive for {} {{}}
            ",
            name, body, name,
        )
    }

    fn new_with_nullptr(&self, _collector: &mut ExternFuncCollector) -> String {
        let src = self.ir.get(self.context.ir_file);

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
            self.ir.rust_wire_type(Target::Io),
            body,
        )
    }

    fn imports(&self) -> Option<String> {
        let api_struct = self.ir.get(self.context.ir_file);
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
