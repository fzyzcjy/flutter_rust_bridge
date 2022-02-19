use crate::generator::rust::ty::*;
use crate::generator::rust::ExternFuncCollector;
use crate::ir::*;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeEnumRefGenerator, IrTypeEnumRef);

impl TypeRustGeneratorTrait for TypeEnumRefGenerator<'_> {
    fn wire2api_body(&self) -> Option<String> {
        let enu = self.ir.get(self.context.ir_file);
        Some(if self.ir.is_struct {
            let variants = enu
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| match &variant.kind {
                    IrVariantKind::Value => {
                        format!("{} => {}::{},", idx, enu.name, variant.name)
                    }
                    IrVariantKind::Struct(st) => {
                        let fields: Vec<_> = st
                            .fields
                            .iter()
                            .map(|field| {
                                if st.is_fields_named {
                                    format!("{0}: ans.{0}.wire2api()", field.name.rust_style())
                                } else {
                                    format!("ans.{}.wire2api()", field.name.rust_style())
                                }
                            })
                            .collect();
                        let (left, right) = st.brackets_pair();
                        format!(
                            "{} => unsafe {{
                                        let ans = support::box_from_leak_ptr(self.kind);
                                        let ans = support::box_from_leak_ptr(ans.{2});
                                        {}::{2}{3}{4}{5}
                                    }}",
                            idx,
                            enu.name,
                            variant.name,
                            left,
                            fields.join(","),
                            right
                        )
                    }
                })
                .collect::<Vec<_>>();
            format!(
                "match self.tag {{
                        {}
                        _ => unreachable!(),
                    }}",
                variants.join("\n"),
            )
        } else {
            let variants = enu
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| format!("{} => {}::{},", idx, enu.name, variant.name))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "match self {{
                        {}
                        _ => unreachable!(\"Invalid variant for {}: {{}}\", self),
                    }}",
                variants, enu.name
            )
        })
    }

    fn structs(&self) -> String {
        let src = self.ir.get(self.context.ir_file);
        if !src.is_struct() {
            return "".to_owned();
        }
        let variant_structs = src
            .variants()
            .iter()
            .map(|variant| {
                let fields = match &variant.kind {
                    IrVariantKind::Value => vec![],
                    IrVariantKind::Struct(s) => s
                        .fields
                        .iter()
                        .map(|field| {
                            format!(
                                "{}: {}{},",
                                field.name.rust_style(),
                                field.ty.rust_wire_modifier(),
                                field.ty.rust_wire_type()
                            )
                        })
                        .collect(),
                };
                format!(
                    "#[repr(C)]
                    #[derive(Clone)]
                    pub struct {}_{} {{ {} }}",
                    self.ir.name,
                    variant.name,
                    fields.join("\n")
                )
            })
            .collect::<Vec<_>>();
        let union_fields = src
            .variants()
            .iter()
            .map(|variant| format!("{0}: *mut {1}_{0},", variant.name, self.ir.name))
            .collect::<Vec<_>>();
        format!(
            "#[repr(C)]
            #[derive(Clone)]
            pub struct {0} {{ tag: i32, kind: *mut {1}Kind }}

            #[repr(C)]
            pub union {1}Kind {{
                {2}
            }}

            {3}",
            self.ir.rust_wire_type(),
            self.ir.name,
            union_fields.join("\n"),
            variant_structs.join("\n\n")
        )
    }

    fn wrapper_struct(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_file);
        src.wrapper_name.as_ref().cloned()
    }

    fn impl_intodart(&self) -> String {
        let src = self.ir.get(self.context.ir_file);

        let name = match &src.wrapper_name {
            Some(wrapper) => wrapper,
            None => &src.name,
        };
        let (self_ref, self_path): (&str, &str) = if src.wrapper_name.is_some() {
            ("self.0", &src.name)
        } else {
            ("self", "Self")
        };
        if self.ir.is_struct {
            let variants = src
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    let tag = format!("{}.into_dart()", idx);
                    match &variant.kind {
                        IrVariantKind::Value => {
                            format!("{}::{} => vec![{}],", self_path, variant.name, tag)
                        }
                        IrVariantKind::Struct(s) => {
                            let fields = Some(tag)
                                .into_iter()
                                .chain(s.fields.iter().map(|field| {
                                    format!("{}.into_dart()", field.name.rust_style())
                                }))
                                .collect::<Vec<_>>();
                            let pattern = s
                                .fields
                                .iter()
                                .map(|field| field.name.rust_style().to_owned())
                                .collect::<Vec<_>>();
                            let (left, right) = s.brackets_pair();
                            format!(
                                "{}::{}{}{}{} => vec![{}],",
                                self_path,
                                variant.name,
                                left,
                                pattern.join(","),
                                right,
                                fields.join(",")
                            )
                        }
                    }
                })
                .collect::<Vec<_>>();
            format!(
                "impl support::IntoDart for {} {{
                fn into_dart(self) -> support::DartCObject {{
                    match {} {{
                        {}
                    }}.into_dart()
                }}
            }}
            ",
                name,
                self_ref,
                variants.join("\n")
            )
        } else {
            let variants = src
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| format!("{}::{} => {},", self_path, variant.name, idx))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "impl support::IntoDart for {} {{
                fn into_dart(self) -> support::DartCObject {{
                    match {} {{
                        {}
                    }}.into_dart()
                }}
            }}
            ",
                name, self_ref, variants
            )
        }
    }

    fn new_with_nullptr(&self, collector: &mut ExternFuncCollector) -> String {
        if !self.ir.is_struct {
            return "".to_string();
        }

        fn init_of(ty: &IrType) -> &str {
            if ty.rust_wire_is_pointer() {
                "core::ptr::null_mut()"
            } else {
                "Default::default()"
            }
        }

        let src = self.ir.get(self.context.ir_file);

        let inflators = src
            .variants()
            .iter()
            .filter_map(|variant| {
                let typ = format!("{}_{}", self.ir.name, variant.name);
                let body: Vec<_> = if let IrVariantKind::Struct(st) = &variant.kind {
                    st.fields
                        .iter()
                        .map(|field| format!("{}: {}", field.name.rust_style(), init_of(&field.ty)))
                        .collect()
                } else {
                    return None;
                };
                Some(collector.generate(
                    &format!("inflate_{}", typ),
                    &[],
                    Some(&format!("*mut {}Kind", self.ir.name)),
                    &format!(
                        "support::new_leak_box_ptr({}Kind {{
                        {}: support::new_leak_box_ptr({} {{
                            {}
                        }})
                    }})",
                        self.ir.name,
                        variant.name.rust_style(),
                        typ,
                        body.join(",")
                    ),
                ))
            })
            .collect::<Vec<_>>();
        format!(
            "impl NewWithNullPtr for {} {{
                fn new_with_null_ptr() -> Self {{
                    Self {{
                        tag: -1,
                        kind: core::ptr::null_mut(),
                    }}
                }}
            }}
            {}",
            self.ir.rust_wire_type(),
            inflators.join("\n\n")
        )
    }

    fn imports(&self) -> Option<String> {
        let api_enum = self.ir.get(self.context.ir_file);
        Some(format!("use {};", api_enum.path.join("::")))
    }
}
