use crate::generator::dart::ty::*;
use crate::generator::dart::{dart_comments, dart_metadata, GeneratedApiMethod};
use crate::target::Acc;
use crate::type_dart_generator_struct;
use crate::utils::method::FunctionName;
use crate::utils::misc::{dart_maybe_implements_exception, BlockIndex};

use crate::{ir::*, Opts};
use convert_case::{Case, Casing};

type_dart_generator_struct!(TypeStructRefGenerator, IrTypeStructRef);

impl TypeDartGeneratorTrait for TypeStructRefGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            wasm: self.context.config.wasm_enabled.then(|| {
                format!(
                    "return [{}];",
                    self.ir
                        .get(self.context.ir_file)
                        .fields
                        .iter()
                        .map(|field| {
                            let api2wire_func_name = format!("api2wire_{}", field.ty.safe_ident());
                            let prefix = if self.context.config.shared
                                || !self.is_type_shared_by_safe_ident(&field.ty)
                            {
                                ""
                            } else {
                                let shared_dart_api2wire_funcs = self
                                    .context
                                    .all_configs
                                    .get_dart_api2wire_funcs(BlockIndex::new_shared());
                                let shared_acc = shared_dart_api2wire_funcs.as_ref().unwrap();
                                // NOTE: search in `common`, not in `wasm`, even this method is in used for .web.dart
                                if (shared_acc.common).contains(&api2wire_func_name) {
                                    ""
                                } else {
                                    "_sharedPlatform."
                                }
                            };
                            format!(
                                "{prefix}{api2wire_func_name}(raw.{})",
                                field.name.dart_style()
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        let s = self.ir.get(self.context.ir_file);
        Some(
            s.fields
                .iter()
                .map(|field| {
                    api_fill_for_field(
                        &field.ty.safe_ident(),
                        &field.name.dart_style(),
                        field.name.rust_style(),
                        field.ty.is_struct_ref_or_enum_ref_or_record(),
                        self.context.config.shared,
                        self.is_type_shared_by_safe_ident(&field.ty),
                        self.context
                            .all_configs
                            .get_dart_api2wire_funcs(BlockIndex::new_shared()),
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    fn wire2api_body(&self) -> String {
        let src = self.ir.get(self.context.ir_file);
        let s = self.ir.get(self.context.ir_file);

        let mut methods = self
            .get_context()
            .all_configs
            .get_funcs(self.context.config.block_index, true)
            .into_iter()
            .filter(|f| {
                let f = FunctionName::deserialize(&f.name);
                f.is_method_for_struct(&src.name) || f.is_static_method_for_struct(&src.name)
            });
        let has_methods = methods.next().is_some();

        let mut inner = s
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let prefix = if self.context.config.shared {
                    ""
                } else if !self.is_type_shared_by_safe_ident(&field.ty) {
                    "_"
                } else {
                    "_sharedImpl."
                };
                format!(
                    "{}: {}wire2api_{}(arr[{}]),",
                    field.name.dart_style(),
                    prefix,
                    field.ty.safe_ident(),
                    idx
                )
            })
            .collect::<Vec<_>>();
        if has_methods && self.context.config.bridge_in_method {
            inner.insert(0, "bridge: this,".to_string());
        }

        let inner = inner.join("\n");
        let cast = "final arr = raw as List<dynamic>;".to_string();
        let safe_check = format!("if (arr.length != {}) throw Exception('unexpected arr length: expect {} but see ${{arr.length}}');", s.fields.len(), s.fields.len());
        format!(
            "{}
                {}
                return {}({});",
            cast, safe_check, s.name, inner,
        )
    }

    fn structs(&self) -> String {
        let src = self.ir.get(self.context.ir_file);
        let comments = dart_comments(&src.comments);
        let metadata = dart_metadata(&src.dart_metadata);

        let methods = self
            .context
            .all_configs
            .get_funcs(self.context.config.block_index, true)
            .into_iter()
            .filter(|f| {
                let f = FunctionName::deserialize(&f.name);
                f.is_method_for_struct(&src.name) || f.is_static_method_for_struct(&src.name)
            })
            .collect::<Vec<_>>();

        let has_methods = !methods.is_empty();
        let methods = methods
            .iter()
            .map(|func| {
                generate_api_method(
                    func,
                    src,
                    self.context.config.dart_api_class_name().to_string(),
                    self.context.config,
                )
            })
            .collect::<Vec<_>>();

        let methods_string = methods
            .iter()
            .map(|g| {
                format!(
                    "{}{}=>{};\n\n",
                    g.comments,
                    g.signature.clone(),
                    g.implementation.clone()
                )
            })
            .collect::<Vec<_>>()
            .concat();
        let extra_argument = if self.context.config.bridge_in_method {
            "required this.bridge,".to_string()
        } else {
            "".to_string()
        };
        let field_bridge = if self.context.config.bridge_in_method {
            format!(
                "final {} bridge;",
                self.context.config.dart_api_class_name(),
            )
        } else {
            String::new()
        };
        let private_constructor = if has_methods {
            format!("const {}._();", self.ir.name)
        } else {
            "".to_owned()
        };
        if src.using_freezed() {
            let mut constructor_params = src
                .fields
                .iter()
                .map(|field| {
                    let r#default = field.field_default(true, Some(self.context.config));
                    format!(
                        "{default} {} {} {},",
                        field.required_modifier(),
                        field.ty.dart_api_type(),
                        field.name.dart_style()
                    )
                })
                .collect::<Vec<_>>();
            if has_methods && self.context.config.bridge_in_method {
                constructor_params.insert(
                    0,
                    format!(
                        "required {} bridge,",
                        self.context.config.dart_api_class_name()
                    ),
                );
            }
            let constructor_params = constructor_params.join("");

            format!(
                "{comments}{meta}class {Name} with _${Name} {implements_exception} {{
                    {private_constructor}
                    const factory {Name}({{{}}}) = _{Name};
                    {}
                }}",
                constructor_params,
                methods_string,
                comments = comments,
                private_constructor = private_constructor,
                meta = metadata,
                Name = self.ir.name,
                implements_exception = dart_maybe_implements_exception(self.ir.is_exception),
            )
        } else {
            let mut field_declarations = src
                .fields
                .iter()
                .map(|f| {
                    let comments = dart_comments(&f.comments);
                    format!(
                        "{}{} {} {};",
                        comments,
                        if f.is_final { "final" } else { "" },
                        f.ty.dart_api_type(),
                        f.name.dart_style()
                    )
                })
                .collect::<Vec<_>>();
            if has_methods {
                field_declarations.insert(0, field_bridge);
            }
            let field_declarations = field_declarations.join("\n");
            let mut constructor_params = src
                .fields
                .iter()
                .map(|f| {
                    format!(
                        "{required}this.{} {default},",
                        f.name.dart_style(),
                        required = f.required_modifier(),
                        default = f.field_default(false, Some(self.context.config))
                    )
                })
                .collect::<Vec<_>>();
            if has_methods {
                constructor_params.insert(0, extra_argument);
            }

            let (left, right) = if constructor_params.is_empty() {
                ("", "")
            } else {
                ("{", "}")
            };
            let constructor_params = constructor_params.join("");
            let const_capable = if src.const_capable() { "const " } else { "" };

            format!(
                "{comments}{meta}class {Name} {implements_exception} {{
                    {}

                    {const}{Name}({}{}{});

                {}
            }}",
                field_declarations,
                left,
                constructor_params,
                right,
                methods_string,
                comments = comments,
                meta = metadata,
                Name = self.ir.name,
                const = const_capable,
                implements_exception = dart_maybe_implements_exception(self.ir.is_exception),
            )
        }
    }
}

#[inline]
pub(crate) fn api_fill_for_field(
    safe_ident: &str,
    dart_style: &str,
    rust_style: &str,
    is_struct: bool,
    ir_file_shared: bool,
    is_type_shared: bool,
    shared_dart_api2wire_funcs: Option<&Acc<String>>,
) -> String {
    if is_struct {
        format!(
            "_api_fill_to_wire_{}(apiObj.{}, wireObj.{});",
            safe_ident, dart_style, rust_style
        )
    } else {
        let api2wire_func_name = format!("api2wire_{}", safe_ident);
        let prefix = if ir_file_shared || !is_type_shared {
            ""
        } else {
            // NOTE: search in `common`, not in `io`, even this method is in used for .io.dart
            let shared_acc = shared_dart_api2wire_funcs.as_ref().unwrap();
            if (shared_acc.common).contains(&api2wire_func_name) {
                ""
            } else {
                "_sharedPlatform."
            }
        };

        format!(
            "wireObj.{} = {}{}(apiObj.{});",
            rust_style, prefix, api2wire_func_name, dart_style,
        )
    }
}

fn generate_api_method(
    func: &IrFunc,
    ir_struct: &IrStruct,
    dart_api_class_name: String,
    config: &Opts,
) -> GeneratedApiMethod {
    let f = FunctionName::deserialize(&func.name);
    let skip_count = usize::from(!f.is_static_method());
    let mut raw_func_param_list = func
        .inputs
        .iter()
        .skip(skip_count) //skip the first as it's the method 'self'
        .map(|input| {
            format!(
                "{required}{} {} {default}",
                input.ty.dart_api_type(),
                input.name.dart_style(),
                required = input.required_modifier(),
                default = input.field_default(false, Some(config))
            )
        })
        .collect::<Vec<_>>();

    if f.is_static_method() && config.bridge_in_method {
        raw_func_param_list.insert(0, format!("required {dart_api_class_name} bridge"));
    }

    let full_func_param_list = [raw_func_param_list, vec!["dynamic hint".to_string()]].concat();

    let static_function_name = f.method_name();
    let comments = dart_comments(&func.comments);

    let partial = format!(
        "{} {} {}({{ {} }})",
        if f.is_static_method() { "static" } else { "" },
        func.mode.dart_return_type(&func.output.dart_api_type()),
        if f.is_static_method() {
            if static_function_name == "new" {
                format!("new{}", ir_struct.name)
            } else {
                static_function_name.to_case(Case::Camel)
            }
        } else {
            f.method_name().to_case(Case::Camel)
        },
        full_func_param_list.join(","),
    );

    let signature = partial;

    let mut arg_names = func
        .inputs
        .iter()
        .skip(skip_count) //skip the first as it's the method 'self'
        .map(|input| format!("{}:{},", input.name.dart_style(), input.name.dart_style()))
        .collect::<Vec<_>>();

    let implementation = if f.is_static_method() {
        arg_names.push("hint: hint".to_string());
        let arg_names = arg_names.concat();
        format!(
            "{}.{}({})",
            config.get_dart_api_bridge_name(),
            func.name.clone().to_case(Case::Camel),
            arg_names
        )
    } else {
        let arg_names = arg_names.concat();
        format!(
            "{}.{}({}: this, {})",
            config.get_dart_api_bridge_name(),
            func.name.clone().to_case(Case::Camel),
            func.inputs[0].name.dart_style(),
            arg_names
        )
    };

    GeneratedApiMethod {
        signature,
        implementation,
        comments,
    }
}
