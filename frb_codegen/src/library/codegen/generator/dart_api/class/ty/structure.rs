use crate::codegen::generator::dart_api::base::*;
use crate::codegen::generator::dart_api::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::dart_api::class::ty::DartApiGeneratorClassTrait;
use crate::codegen::generator::dart_api::internal_config::GeneratorDartApiInternalConfig;
use crate::codegen::generator::dart_api::misc::{
    generate_dart_comments, generate_dart_maybe_implements_exception, generate_dart_metadata,
};
use crate::codegen::ir::func::{
    IrFunc, IrFuncMode, IrFuncOwnerInfo, IrFuncOwnerInfoMethod, IrFuncOwnerInfoMethodMode,
};
use crate::codegen::ir::ty::structure::IrStruct;
use crate::library::codegen::generator::dart_api::decl::DartApiGeneratorDeclTrait;
use convert_case::{Case, Casing};

impl<'a> DartApiGeneratorClassTrait for StructRefDartApiGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        let comments = generate_dart_comments(&src.comments);
        let metadata = generate_dart_metadata(&src.dart_metadata);

        let ir_pack = self.context.ir_pack;

        let methods = ir_pack
            .funcs
            .iter()
            .filter(|f| {
                matches!(&f.owner, IrFuncOwnerInfo::Method(IrFuncOwnerInfoMethod{ struct_name, .. }) if struct_name == &src.name)
            })
            .collect::<Vec<_>>();

        let has_methods = !methods.is_empty();
        let methods = methods
            .iter()
            .map(|func| {
                generate_api_method(
                    func,
                    src,
                    self.context.config.dart_api_class_name.to_string(),
                    &self.context,
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
        let extra_argument = if self.context.config.use_bridge_in_method {
            "required this.bridge,".to_string()
        } else {
            "".to_string()
        };
        let field_bridge = if self.context.config.use_bridge_in_method {
            format!("final {} bridge;", self.context.config.dart_api_class_name,)
        } else {
            String::new()
        };
        let private_constructor = if has_methods {
            format!("const {}._();", self.ir.ident.0)
        } else {
            "".to_owned()
        };
        Some(if src.using_freezed() {
            let mut constructor_params = src
                .fields
                .iter()
                .map(|field| {
                    let r#default =
                        generate_field_default(field, true, self.context.config.dart_enums_style);
                    format!(
                        "{default} {} {} {},",
                        generate_field_required_modifier(field),
                        DartApiGenerator::new(field.ty.clone(), self.context.clone())
                            .dart_api_type(),
                        field.name.dart_style()
                    )
                })
                .collect::<Vec<_>>();
            if has_methods && self.context.config.use_bridge_in_method {
                constructor_params.insert(
                    0,
                    format!(
                        "required {} bridge,",
                        self.context.config.dart_api_class_name
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
                Name = self.ir.ident.0,
                implements_exception =
                    generate_dart_maybe_implements_exception(self.ir.is_exception),
            )
        } else {
            let mut field_declarations = src
                .fields
                .iter()
                .map(|f| {
                    let comments = generate_dart_comments(&f.comments);
                    format!(
                        "{}{} {} {};",
                        comments,
                        if f.is_final { "final" } else { "" },
                        DartApiGenerator::new(f.ty.clone(), self.context.clone()).dart_api_type(),
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
                        required = generate_field_required_modifier(f),
                        default =
                            generate_field_default(f, false, self.context.config.dart_enums_style)
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
            let const_capable = src.fields.iter().all(|field| field.is_final);

            format!(
                "{comments}{meta}class {Name} {implements_exception} {{
                    {}
        
                    {maybe_const}{Name}({}{}{});
        
                {}
            }}",
                field_declarations,
                left,
                constructor_params,
                right,
                methods_string,
                comments = comments,
                meta = metadata,
                Name = self.ir.ident.0,
                maybe_const = if const_capable { "const " } else { "" },
                implements_exception =
                    generate_dart_maybe_implements_exception(self.ir.is_exception),
            )
        })
    }
}

#[derive(Debug)]
struct GeneratedApiMethod {
    comments: String,
    signature: String,
    implementation: String,
}

fn generate_api_method(
    func: &IrFunc,
    ir_struct: &IrStruct,
    dart_api_class_name: String,
    context: &DartApiGeneratorContext,
) -> GeneratedApiMethod {
    let method_info = if let IrFuncOwnerInfo::Method(info) = &func.owner {
        info
    } else {
        unimplemented!("should not happen")
    };
    let is_static_method = method_info.mode == IrFuncOwnerInfoMethodMode::Static;

    let skip_count = usize::from(!is_static_method);
    let mut raw_func_param_list = func
        .inputs
        .iter()
        .skip(skip_count) //skip the first as it's the method 'self'
        .map(|input| {
            format!(
                "{required}{} {} {default}",
                DartApiGenerator::new(input.ty.clone(), context.clone()).dart_api_type(),
                input.name.dart_style(),
                required = generate_field_required_modifier(input),
                default = generate_field_default(input, false, context.config.dart_enums_style)
            )
        })
        .collect::<Vec<_>>();

    if is_static_method && context.config.use_bridge_in_method {
        raw_func_param_list.insert(0, format!("required {dart_api_class_name} bridge"));
    }

    let full_func_param_list = [raw_func_param_list, vec!["dynamic hint".to_string()]].concat();

    let comments = generate_dart_comments(&func.comments);

    let partial = format!(
        "{} {} {}({{ {} }})",
        if is_static_method { "static" } else { "" },
        generate_function_dart_return_type(
            &func.mode,
            &DartApiGenerator::new(func.output.clone(), context.clone()).dart_api_type()
        ),
        if is_static_method {
            if method_info.actual_method_name == "new" {
                format!("new{}", ir_struct.name)
            } else {
                method_info.actual_method_name.to_case(Case::Camel)
            }
        } else {
            method_info.actual_method_name.to_case(Case::Camel)
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

    let implementation = if is_static_method {
        arg_names.push("hint: hint".to_string());
        let arg_names = arg_names.concat();
        format!(
            "{}.{}({})",
            context.config.dart_api_instance_name,
            func.name.clone().to_case(Case::Camel),
            arg_names
        )
    } else {
        let arg_names = arg_names.concat();
        format!(
            "{}.{}({}: this, {})",
            context.config.dart_api_instance_name,
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

fn generate_function_dart_return_type(func_mode: &IrFuncMode, inner: &str) -> String {
    match func_mode {
        IrFuncMode::Normal => format!("Future<{inner}>"),
        IrFuncMode::Sync => inner.to_string(),
        IrFuncMode::Stream { .. } => format!("Stream<{inner}>"),
    }
}
