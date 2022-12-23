use itertools::Itertools;

use crate::generator::dart::dart_comments;
use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeEnumRefGenerator, IrTypeEnumRef);

impl TypeDartGeneratorTrait for TypeEnumRefGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let variants = (self.ir.get(self.context.ir_file).variants())
            .iter()
            .enumerate()
            .map(|(idx, variant)| {
                let fields = match &variant.kind {
                    IrVariantKind::Value => vec![],
                    IrVariantKind::Struct(st) => (st.fields)
                        .iter()
                        .map(|field| {
                            format!(
                                ",api2wire_{}(raw.{})",
                                field.ty.safe_ident(),
                                field.name.dart_style()
                            )
                        })
                        .collect(),
                }
                .join("");
                format!(
                    "if (raw is {variant}) {{
                        return [{} {}];
                    }}",
                    idx,
                    fields,
                    variant = variant.wrapper_name.rust_style(),
                )
            })
            .join("\n");

        Acc {
            wasm: Some(format!(
                "{}

                throw Exception('unreachable');",
                variants,
            )),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        Some(
            self.ir
                .get(self.context.ir_file)
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    if let IrVariantKind::Value = &variant.kind {
                        format!(
                            "if (apiObj is {}) {{ wireObj.tag = {}; return; }}",
                            variant.wrapper_name, idx
                        )
                    } else {
                        let pre_field: Vec<_> = match &variant.kind {
                            IrVariantKind::Struct(st) => st
                                .fields
                                .iter()
                                .map(|field| {
                                    format!(
                                        "var pre_{} = api2wire_{}(apiObj.{});",
                                        field.name.rust_style(),
                                        field.ty.safe_ident(),
                                        field.name.dart_style()
                                    )
                                })
                                .collect(),
                            _ => unreachable!(),
                        };
                        let r = format!("wireObj.kind.ref.{}.ref", variant.name);
                        let body: Vec<_> = match &variant.kind {
                            IrVariantKind::Struct(st) => st
                                .fields
                                .iter()
                                .map(|field| {
                                    format!(
                                        "{}.{name} = pre_{name};",
                                        r,
                                        name = field.name.rust_style(),
                                    )
                                })
                                .collect(),
                            _ => unreachable!(),
                        };
                        format!(
                            "if (apiObj is {5}) {{
                                {3}
                                wireObj.tag = {1};
                                wireObj.kind = inner.inflate_{2}_{0}();
                                {4}
                                return;
                            }}",
                            variant.name,
                            idx,
                            self.ir.name,
                            pre_field.join("\n"),
                            body.join("\n"),
                            variant.wrapper_name
                        )
                    }
                })
                .join("\n"),
        )
    }

    fn wire2api_body(&self) -> String {
        let enu = self.ir.get(self.context.ir_file);
        assert!(enu.is_struct());
        let variants = enu
            .variants()
            .iter()
            .enumerate()
            .map(|(idx, variant)| {
                let args = match &variant.kind {
                    IrVariantKind::Value => "".to_owned(),
                    IrVariantKind::Struct(st) => st
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(idx, field)| {
                            let val =
                                format!("_wire2api_{}(raw[{}]),", field.ty.safe_ident(), idx + 1);
                            if st.is_fields_named {
                                format!("{}: {}", field.name.dart_style(), val)
                            } else {
                                val
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(""),
                };
                format!("case {}: return {}({});", idx, variant.wrapper_name, args)
            })
            .collect::<Vec<_>>();
        format!(
            "switch (raw[0]) {{
                {}
                default: throw Exception(\"unreachable\");
            }}",
            variants.join("\n"),
        )
    }

    fn structs(&self) -> String {
        let src = self.ir.get(self.context.ir_file);

        let comments = dart_comments(&src.comments);
        if src.is_struct() {
            let variants = src
                .variants()
                .iter()
                .map(|variant| {
                    let args = match &variant.kind {
                        IrVariantKind::Value => "".to_owned(),
                        IrVariantKind::Struct(IrStruct {
                            is_fields_named: false,
                            fields,
                            ..
                        }) => {
                            let types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();
                            let split = optional_boundary_index(&types);
                            let types = fields
                                .iter()
                                .map(|field| {
                                    format!(
                                        "{}{} {},",
                                        dart_comments(&field.comments),
                                        field.ty.dart_api_type(),
                                        field.name.dart_style()
                                    )
                                })
                                .collect::<Vec<_>>();
                            if let Some(idx) = split {
                                let before = &types[..idx];
                                let after = &types[idx..];
                                format!("{}[{}]", before.join(""), after.join(""))
                            } else {
                                types.join("")
                            }
                        }
                        IrVariantKind::Struct(st) => {
                            let fields = st
                                .fields
                                .iter()
                                .map(|field| {
                                    format!(
                                        "{}{}{} {},",
                                        dart_comments(&field.comments),
                                        field.ty.dart_required_modifier(),
                                        field.ty.dart_api_type(),
                                        field.name.dart_style()
                                    )
                                })
                                .collect::<Vec<_>>();
                            format!("{{ {} }}", fields.join(""))
                        }
                    };
                    format!(
                        "{}const factory {}.{}({}) = {};",
                        dart_comments(&variant.comments),
                        self.ir.name,
                        variant.name.dart_style(),
                        args,
                        variant.wrapper_name.rust_style(),
                    )
                })
                .collect::<Vec<_>>();
            format!(
                "@freezed
                class {0} with _${0} {{
                    {1}
                }}",
                self.ir.name,
                variants.join("\n")
            )
        } else {
            let variants = src
                .variants()
                .iter()
                .map(|variant| {
                    format!(
                        "{}{},",
                        dart_comments(&variant.comments),
                        variant.name.rust_style()
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "{}enum {} {{
                    {}
                }}",
                comments, self.ir.name, variants
            )
        }
    }
}
