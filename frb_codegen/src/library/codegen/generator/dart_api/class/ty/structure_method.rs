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
use itertools::Itertools;

#[derive(Debug)]
struct GeneratedApiMethod {
    comments: String,
    signature: String,
    implementation: String,
}

pub(super) fn generate_api_method(
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
        .collect_vec();

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
        .collect_vec();

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
