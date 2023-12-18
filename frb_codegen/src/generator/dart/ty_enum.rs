use itertools::Itertools;

use crate::generator::dart::dart_comments;
use crate::generator::dart::func::get_api2wire_prefix;
use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;
use crate::utils::misc::dart_maybe_implements_exception;

const BACKTRACE_IDENT: &str = "backtrace";

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
                            let api2wire_func = format!("api2wire_{}", field.ty.safe_ident());
                            let prefix = get_api2wire_prefix(
                                &api2wire_func,
                                self.context.config,
                                &field.ty,
                                false,
                                self.get_context().all_configs,
                            );
                            format!(
                                ",{}api2wire_{}(raw.{})",
                                prefix,
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
                "{variants}

                throw Exception('unreachable');"
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
                                    let api2wire_func =
                                        format!("api2wire_{}", field.ty.safe_ident());
                                    let prefix = get_api2wire_prefix(
                                        &api2wire_func,
                                        self.context.config,
                                        &field.ty,
                                        false,
                                        self.get_context().all_configs,
                                    );
                                    format!(
                                        "var pre_{} = {}api2wire_{}(apiObj.{});",
                                        field.name.rust_style(),
                                        prefix,
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
                    IrVariantKind::Struct(st) => {
                        let prefix = self.get_private_prefix();
                        st.fields
                            .iter()
                            .enumerate()
                            .map(|(idx, field)| {
                                let val = format!(
                                    "{}wire2api_{}(raw[{}]),",
                                    prefix,
                                    field.ty.safe_ident(),
                                    idx + 1
                                );
                                if st.is_fields_named {
                                    format!("{}: {}", field.name.dart_style(), val)
                                } else {
                                    val
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("")
                    }
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
                    let has_backtrace = matches!(&variant.kind, IrVariantKind::Struct(IrStruct {
                        is_fields_named: true,
                        fields,
                        ..
                    }) if fields.iter().any(|field| field.name.raw == BACKTRACE_IDENT));

                    let args = match &variant.kind {
                        IrVariantKind::Value => "".to_owned(),
                        IrVariantKind::Struct(IrStruct {
                            is_fields_named: false,
                            fields,
                            ..
                        }) => {
                            let split = optional_boundary_index(fields);
                            let types = fields
                                .iter()
                                .map(|field| {
                                    // If no split, default values are not valid.
                                    let default = split
                                        .is_some()
                                        .then(|| {
                                            field.field_default(true, Some(self.context.config))
                                        })
                                        .unwrap_or_default();
                                    format!(
                                        "{comments} {default} {} {},",
                                        field.ty.dart_api_type(),
                                        field.name.dart_style(),
                                        comments = dart_comments(&field.comments),
                                        default = default
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
                                        "{comments} {default} {required}{} {} ,",
                                        field.ty.dart_api_type(),
                                        field.name.dart_style(),
                                        required = field.required_modifier(),
                                        comments = dart_comments(&field.comments),
                                        default =
                                            field.field_default(true, Some(self.context.config)),
                                    )
                                })
                                .collect::<Vec<_>>();
                            format!("{{ {} }}", fields.join(""))
                        }
                    };

                    let implements_exception = if self.ir.is_exception && has_backtrace {
                        "@Implements<FrbBacktracedException>()"
                    } else {
                        ""
                    };

                    format!(
                        "{} {}const factory {}.{}({}) = {};",
                        implements_exception,
                        dart_comments(&variant.comments),
                        self.ir.name,
                        variant.name.dart_style(),
                        args,
                        variant.wrapper_name.rust_style(),
                    )
                })
                .collect::<Vec<_>>();
            let sealed = if self.context.config.dart3 {
                "sealed"
            } else {
                ""
            };
            format!(
                "@freezed
                {sealed} class {0} with _${0} {1} {{
                    {2}
                }}",
                self.ir.name,
                dart_maybe_implements_exception(self.ir.is_exception),
                variants.join("\n")
            )
        } else {
            let variants = src
                .variants()
                .iter()
                .map(|variant| {
                    let variant_name = if self.context.config.dart_enums_style {
                        crate::utils::misc::make_string_keyword_safe(variant.name.dart_style())
                    } else {
                        variant.name.rust_style().to_string()
                    };

                    format!("{}{},", dart_comments(&variant.comments), variant_name)
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
