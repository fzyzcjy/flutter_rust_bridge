use crate::generator::dart::dart_comments;
use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeEnumRefGenerator(pub IrTypeEnumRef);

impl TypeDartGeneratorTrait for TypeEnumRefGenerator {
    fn api2wire_body(&self) -> String {
        if !self.0.is_struct {
            "return raw.index;".to_owned()
        } else {
            "".to_string()
        }
    }

    fn api_fill_to_wire_body(&self) -> String {
        if self.0.is_struct {
            self.0
                .get(api_file)
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    if let IrVariantKind::Value = &variant.kind {
                        format!(
                            "if (apiObj is {}) {{ wireObj.tag = {}; return; }}",
                            variant.name, idx
                        )
                    } else {
                        let r = format!("wireObj.kind.ref.{}.ref", variant.name);
                        let body: Vec<_> = match &variant.kind {
                            IrVariantKind::Struct(st) => st
                                .fields
                                .iter()
                                .map(|field| {
                                    format!(
                                        "{}.{} = _api2wire_{}(apiObj.{});",
                                        r,
                                        field.name.rust_style(),
                                        field.ty.safe_ident(),
                                        field.name.dart_style()
                                    )
                                })
                                .collect(),
                            _ => unreachable!(),
                        };
                        format!(
                            "if (apiObj is {0}) {{
                            wireObj.tag = {1};
                            wireObj.kind = inner.inflate_{2}_{0}();
                            {3}
                        }}",
                            variant.name,
                            idx,
                            self.0.name,
                            body.join("\n")
                        )
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            "".to_string()
        }
    }

    fn wire2api_body(&self) -> String {
        if self.0.is_struct {
            let enu = self.0.get(api_file);
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
                                let val = format!(
                                    "_wire2api_{}(raw[{}]),",
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
                            .join(""),
                    };
                    format!("case {}: return {}({});", idx, variant.name, args)
                })
                .collect::<Vec<_>>();
            format!(
                "switch (raw[0]) {{
                    {}
                    default: throw Exception(\"unreachable\");
                }}",
                variants.join("\n"),
            )
        } else {
            format!("return {}.values[raw];", self.0.name)
        }
    }

    fn structs(&self) -> String {
        let comments = dart_comments(&self.0.comments);
        if self.0.is_struct() {
            let variants = self
                .0
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
                                types.join(",")
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
                        self.0.name,
                        variant.name.dart_style(),
                        args,
                        variant.name.rust_style(),
                    )
                })
                .collect::<Vec<_>>();
            format!(
                "@freezed
                class {0} with _${0} {{
                    {1}
                }}",
                self.0.name,
                variants.join("\n")
            )
        } else {
            let variants = self
                .0
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
                comments, self.0.name, variants
            )
        }
    }
}
