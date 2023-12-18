use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::generator::api_dart::spec_generator::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_function_dart_return_type,
    generate_imports_which_types_and_funcs_use,
};
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::namespace::Namespace;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::utils::basic_code::DartBasicHeaderCode;
use convert_case::{Case, Casing};
use itertools::Itertools;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct ApiDartGeneratedFunction {
    pub(crate) namespace: Namespace,
    pub(crate) header: DartBasicHeaderCode,
    pub(crate) func_comments: String,
    pub(crate) func_expr: String,
    pub(crate) func_impl: String,
    pub(crate) src_lineno: usize,
}

pub(crate) fn generate(
    func: &IrFunc,
    context: ApiDartGeneratorContext,
) -> anyhow::Result<ApiDartGeneratedFunction> {
    let params = generate_params(func, context, context.config.dart_enums_style);

    let func_expr = format!(
        "{func_return_type} {func_name}({params})",
        func_name = func.name.name.to_case(Case::Camel),
        func_return_type = generate_function_dart_return_type(
            &func.mode,
            &ApiDartGenerator::new(func.output.clone(), context).dart_api_type()
        ),
    );

    let func_comments = generate_dart_comments(&func.comments);

    let func_impl = generate_func_impl(func, &context.config.dart_entrypoint_class_name);

    let header = generate_header(func, context)?;

    Ok(ApiDartGeneratedFunction {
        namespace: func.name.namespace.clone(),
        header,
        func_comments,
        func_expr,
        func_impl,
        src_lineno: func.src_lineno,
    })
}

fn generate_params(
    func: &IrFunc,
    context: ApiDartGeneratorContext,
    dart_enums_style: bool,
) -> String {
    let mut params = func
        .inputs
        .iter()
        .map(|input| {
            let required = generate_field_required_modifier(input);
            let r#default = generate_field_default(input, false, dart_enums_style);
            let type_str = ApiDartGenerator::new(input.ty.clone(), context).dart_api_type();
            let name_str = input.name.dart_style();
            format!("{required}{type_str} {name_str} {default}")
        })
        .collect_vec();
    params.push("dynamic hint".to_owned());

    let mut params = params.join(", ");
    if !params.is_empty() {
        params = format!("{{{params}}}");
    }
    params
}

fn generate_func_impl(func: &IrFunc, dart_entrypoint_class_name: &str) -> String {
    let func_name = &func.name.name.to_case(Case::Camel);
    let param_names: Vec<String> = [
        (func.inputs.iter().map(|input| input.name.dart_style())).collect_vec(),
        vec!["hint".to_owned()],
    ]
    .concat();
    let param_forwards = param_names
        .iter()
        .map(|name| format!("{name}: {name}"))
        .join(", ");
    format!("{dart_entrypoint_class_name}.instance.api.{func_name}({param_forwards})")
}

fn generate_header(
    func: &IrFunc,
    context: ApiDartGeneratorContext,
) -> anyhow::Result<DartBasicHeaderCode> {
    Ok(DartBasicHeaderCode {
        import: generate_imports_which_types_and_funcs_use(
            &func.name.namespace.clone(),
            &None,
            &Some(&vec![func]),
            context,
        )?,
        ..Default::default()
    })
}
