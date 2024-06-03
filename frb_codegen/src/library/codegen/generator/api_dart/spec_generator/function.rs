use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::generator::api_dart::spec_generator::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_imports_which_types_and_funcs_use,
};
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::func::{IrFunc, IrFuncMode};
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegateStreamSink};
use crate::codegen::ir::ty::IrType;
use crate::if_then_some;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::utils::basic_code::DartBasicHeaderCode;
use convert_case::{Case, Casing};
use itertools::Itertools;
use serde::Serialize;
use crate::utils::dart_keywords::make_string_keyword_safe;

#[derive(Debug, Serialize)]
pub(crate) struct ApiDartGeneratedFunction {
    pub(crate) namespace: Namespace,
    pub(crate) header: DartBasicHeaderCode,
    pub(crate) func_comments: String,
    pub(crate) func_expr: String,
    pub(crate) func_impl: String,
    pub(crate) func_params: Vec<ApiDartGeneratedFunctionParam>,
    pub(crate) func_return_type: String,
    pub(crate) src_lineno: usize,
    pub(crate) return_stream: Option<ReturnStreamInfo>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ApiDartGeneratedFunctionParam {
    pub(crate) full: String,
    pub(crate) type_str: String,
    pub(crate) name_str: String,
}

pub(crate) fn generate(
    func: &IrFunc,
    context: ApiDartGeneratorContext,
) -> anyhow::Result<ApiDartGeneratedFunction> {
    let return_stream = compute_return_stream(func);
    let (func_params, func_params_str) = generate_params(
        func,
        context,
        context.config.dart_enums_style,
        &return_stream,
    );
    let func_return_type = generate_function_dart_return_type(
        func,
        &ApiDartGenerator::new(func.output.normal.clone(), context).dart_api_type(),
        &return_stream,
        context,
    );

    let func_expr = format!(
        "{func_return_type} {func_name}({func_params_str})",
        func_name = make_string_keyword_safe(func.name.name.to_case(Case::Camel)),
    );

    let func_comments = generate_dart_comments(&func.comments);

    let func_impl = generate_func_impl(
        func,
        &context.config.dart_entrypoint_class_name,
        &return_stream,
    );

    let header = generate_header(func, context)?;

    Ok(ApiDartGeneratedFunction {
        namespace: func.name.namespace.clone(),
        header,
        func_comments,
        func_expr,
        func_impl,
        func_params,
        func_return_type,
        src_lineno: func.src_lineno,
        return_stream,
    })
}

#[derive(Debug, Serialize)]
pub(crate) struct ReturnStreamInfo {
    pub field: IrField,
    pub ty: IrTypeDelegateStreamSink,
}

fn compute_return_stream(func: &IrFunc) -> Option<ReturnStreamInfo> {
    let stream_sink_vars = (func.inputs.iter())
        .filter_map(|input| {
            if_then_some!(
                let IrType::Delegate(IrTypeDelegate::StreamSink(ty)) = &input.inner.ty,
                ReturnStreamInfo { field: input.inner.to_owned(), ty: ty.clone() }
            )
        })
        .collect_vec();
    if stream_sink_vars.len() == 1 {
        Some(stream_sink_vars.into_iter().next().unwrap())
    } else {
        None
    }
}

fn generate_params(
    func: &IrFunc,
    context: ApiDartGeneratorContext,
    dart_enums_style: bool,
    return_stream: &Option<ReturnStreamInfo>,
) -> (Vec<ApiDartGeneratedFunctionParam>, String) {
    let mut params = (func.inputs.iter())
        .filter(|field| Some(&field.inner.name) != return_stream.as_ref().map(|s| &s.field.name))
        .map(|input| {
            let required = generate_field_required_modifier(&input.inner);
            let r#default = generate_field_default(&input.inner, false, dart_enums_style);
            let type_str = ApiDartGenerator::new(input.inner.ty.clone(), context).dart_api_type();
            let name_str = input.inner.name.dart_style();
            ApiDartGeneratedFunctionParam {
                full: format!("{required}{type_str} {name_str} {default}"),
                type_str,
                name_str,
            }
        })
        .collect_vec();
    params.push(ApiDartGeneratedFunctionParam {
        full: "dynamic hint".to_string(),
        type_str: "dynamic".to_string(),
        name_str: "hint".to_string(),
    });

    let mut params_str = params.iter().map(|x| &x.full).join(", ");
    if !params_str.is_empty() {
        params_str = format!("{{{params_str}}}");
    }
    (params, params_str)
}

fn generate_func_impl(
    func: &IrFunc,
    dart_entrypoint_class_name: &str,
    return_stream: &Option<ReturnStreamInfo>,
) -> String {
    let func_name = &func.name.name.to_case(Case::Camel);
    let param_names: Vec<String> = [
        ((func.inputs.iter())
            .filter(|field| {
                Some(&field.inner.name) != return_stream.as_ref().map(|s| &s.field.name)
            })
            .map(|input| input.inner.name.dart_style()))
        .collect_vec(),
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

fn generate_function_dart_return_type(
    func: &IrFunc,
    inner: &str,
    return_stream: &Option<ReturnStreamInfo>,
    context: ApiDartGeneratorContext,
) -> String {
    let mut inner = inner.to_owned();

    if let Some(return_stream) = return_stream {
        inner = format!(
            "Stream<{}>",
            ApiDartGenerator::new(return_stream.ty.inner.clone(), context).dart_api_type()
        );
    }

    let return_future = if return_stream.is_some() {
        func.stream_dart_await
    } else {
        func.mode != IrFuncMode::Sync
    };
    if return_future {
        inner = format!("Future<{inner}>");
    }

    inner
}
