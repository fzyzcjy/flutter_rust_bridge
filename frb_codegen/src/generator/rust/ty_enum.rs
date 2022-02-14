use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeEnumRefGenerator(pub IrTypeEnumRef);

impl TypeRustGeneratorTrait for TypeEnumRefGenerator {
    fn wire2api_body(&self) -> String {
        if self.0.is_struct {
            let enu = self.0.get(ir_file);
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
            .into()
        } else {
            let enu = self.0.get(ir_file);
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
            .into()
        }
    }

    fn structs(&self) -> String {
        let src = self.0.get(file);
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
                    self.0.name,
                    variant.name,
                    fields.join("\n")
                )
            })
            .collect::<Vec<_>>();
        let union_fields = src
            .variants()
            .iter()
            .map(|variant| format!("{0}: *mut {1}_{0},", variant.name, self.0.name))
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
            self.0.rust_wire_type(),
            self.0.name,
            union_fields.join("\n"),
            variant_structs.join("\n\n")
        )
    }

    fn impl_intodart(&self) -> String {
        if self.0.is_struct {
            let variants = self
                .0
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    let tag = format!("{}.into_dart()", idx);
                    match &variant.kind {
                        IrVariantKind::Value => {
                            format!("Self::{} => vec![{}],", variant.name, tag)
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
                                "Self::{}{}{}{} => vec![{}],",
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
                    match self {{
                        {}
                    }}.into_dart()
                }}
            }}
            ",
                self.0.name,
                variants.join("\n")
            )
        } else {
            let variants = self
                .0
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| format!("Self::{} => {},", variant.name, idx))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "impl support::IntoDart for {} {{
                fn into_dart(self) -> support::DartCObject {{
                    match self {{
                        {}
                    }}.into_dart()
                }}
            }}
            ",
                self.0.name, variants
            )
        }
    }

    fn new_with_nullptr(&self) -> String {
        fn init_of(ty: &IrType) -> &str {
            if ty.rust_wire_is_pointer() {
                "core::ptr::null_mut()"
            } else {
                "Default::default()"
            }
        }
        let inflators = self
            .0
            .variants()
            .iter()
            .filter_map(|variant| {
                let typ = format!("{}_{}", self.0.name, variant.name);
                let body: Vec<_> = if let IrVariantKind::Struct(st) = &variant.kind {
                    st.fields
                        .iter()
                        .map(|field| format!("{}: {}", field.name.rust_style(), init_of(&field.ty)))
                        .collect()
                } else {
                    return None;
                };
                Some(self.extern_func_collector.generate(
                    &format!("inflate_{}", typ),
                    &[],
                    Some(&format!("*mut {}Kind", self.0.name)),
                    &format!(
                        "support::new_leak_box_ptr({}Kind {{
                        {}: support::new_leak_box_ptr({} {{
                            {}
                        }})
                    }})",
                        self.0.name,
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
            rust_wire_type,
            inflators.join("\n\n")
        )
    }

    fn imports(&self) -> Option<String> {
        let api_enum = self.0.get(ir_file);
        Some(format!("use {};", api_enum.path.join("::")))
    }
}
