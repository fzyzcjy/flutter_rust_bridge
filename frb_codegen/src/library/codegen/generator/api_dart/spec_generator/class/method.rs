use crate::codegen::generator::api_dart::spec_generator::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_function_dart_return_type,
};
use crate::codegen::ir::func::{
    IrFunc, IrFuncOwnerInfo, IrFuncOwnerInfoMethod, IrFuncOwnerInfoMethodMode,
};
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::structure::IrStruct;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use convert_case::{Case, Casing};
use itertools::Itertools;

pub(crate) fn generate_api_methods(
    generalized_class_name: &NamespacedName,
    context: ApiDartGeneratorContext,
) -> Vec<String> {
    (context.ir_pack.funcs.iter())
        .filter(|f| {
            matches!(&f.owner, IrFuncOwnerInfo::Method(IrFuncOwnerInfoMethod{ enum_or_struct_name, .. }) if enum_or_struct_name == generalized_class_name)
        })
        .map(|func| generate_api_method(func, &generalized_class_name.name, context))
        .collect_vec()
}

fn generate_api_method(
    func: &IrFunc,
    generalized_class_name: &str,
    context: ApiDartGeneratorContext,
) -> String {
    let method_info = if let IrFuncOwnerInfo::Method(info) = &func.owner {
        info
    } else {
        unreachable!()
    };
    let is_static_method = method_info.mode == IrFuncOwnerInfoMethodMode::Static;

    // skip the first as it's the method 'self'
    let skip_count = usize::from(!is_static_method);

    let params = generate_params(func, context, is_static_method, skip_count);
    let comments = generate_dart_comments(&func.comments);
    let signature = generate_signature(func, generalized_class_name, context, method_info, params);
    let arg_names = generate_arg_names(func, is_static_method, skip_count).concat();
    let implementation = generate_implementation(func, context, is_static_method, arg_names);

    format!("{comments}{signature}=>{implementation};\n\n")
}

fn generate_params(
    func: &IrFunc,
    context: ApiDartGeneratorContext,
    _is_static_method: bool,
    skip_count: usize,
) -> Vec<String> {
    let mut ans = func
        .inputs
        .iter()
        .skip(skip_count)
        .map(|input| {
            let required = generate_field_required_modifier(input);
            let default = generate_field_default(input, false, context.config.dart_enums_style);
            let type_str = ApiDartGenerator::new(input.ty.clone(), context).dart_api_type();
            let name_str = input.name.dart_style();
            format!("{required}{type_str} {name_str} {default}")
        })
        .collect_vec();

    ans.push("dynamic hint".to_string());

    ans
}

fn generate_signature(
    func: &IrFunc,
    generalized_class_name: &str,
    context: ApiDartGeneratorContext,
    method_info: &IrFuncOwnerInfoMethod,
    func_params: Vec<String>,
) -> String {
    let is_static_method = method_info.mode == IrFuncOwnerInfoMethodMode::Static;
    let maybe_static = if is_static_method { "static" } else { "" };
    let return_type = generate_function_dart_return_type(
        &func.mode,
        &ApiDartGenerator::new(func.output.clone(), context).dart_api_type(),
    );
    let method_name = if is_static_method && method_info.actual_method_name == "new" {
        format!("new{}", generalized_class_name)
    } else {
        method_info.actual_method_name.to_case(Case::Camel)
    };
    let func_params = func_params.join(",");

    format!("{maybe_static} {return_type} {method_name}({{ {func_params} }})")
}

fn generate_arg_names(func: &IrFunc, is_static_method: bool, skip_count: usize) -> Vec<String> {
    let mut ans = func
        .inputs
        .iter()
        .skip(skip_count)
        .map(|input| format!("{}:{},", input.name.dart_style(), input.name.dart_style()))
        .collect_vec();
    if is_static_method {
        ans.push("hint: hint".to_string());
    }
    ans
}

fn generate_implementation(
    func: &IrFunc,
    context: ApiDartGeneratorContext,
    is_static_method: bool,
    arg_names: String,
) -> String {
    let dart_entrypoint_class_name = &context.config.dart_entrypoint_class_name;
    let dart_api_instance = format!("{dart_entrypoint_class_name}.instance.api");

    let func_name = func.name.name.clone().to_case(Case::Camel);

    if is_static_method {
        format!("{dart_api_instance}.{func_name}({arg_names})")
    } else {
        let extra_arg_name = func.inputs[0].name.dart_style();
        format!("{dart_api_instance}.{func_name}({extra_arg_name}: this, {arg_names})")
    }
}
