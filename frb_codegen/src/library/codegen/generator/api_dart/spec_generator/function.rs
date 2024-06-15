use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::generator::api_dart::spec_generator::class::field::generate_field_default;
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_imports_which_types_and_funcs_use,
};
use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::func::{MirFunc, MirFuncArgMode, MirFuncMode};
use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateStreamSink};
use crate::codegen::ir::mir::ty::MirType;
use crate::if_then_some;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::utils::basic_code::dart_header_code::DartHeaderCode;
use crate::utils::namespace::Namespace;
use itertools::Itertools;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct ApiDartGeneratedFunction {
    pub(crate) namespace: Namespace,
    pub(crate) header: DartHeaderCode,
    pub(crate) func_comments: String,
    pub(crate) func_expr: String,
    pub(crate) func_impl: String,
    pub(crate) func_params: Vec<ApiDartGeneratedFunctionParam>,
    pub(crate) func_return_type: String,
    pub(crate) src_lineno_pseudo: usize,
    pub(crate) return_stream: Option<ReturnStreamInfo>,
}

#[derive(Debug, Serialize, Clone)]
pub(crate) struct ApiDartGeneratedFunctionParam {
    pub(crate) is_required: bool,
    pub(crate) type_str: String,
    pub(crate) name_str: String,
    pub(crate) default_value: String,
}

impl ApiDartGeneratedFunctionParam {
    pub(crate) fn full(&self, arg_mode: MirFuncArgMode) -> String {
        let ApiDartGeneratedFunctionParam {
            is_required,
            type_str,
            name_str,
            default_value,
        } = &self;

        match arg_mode {
            MirFuncArgMode::Positional => format!("{type_str} {name_str}"),
            MirFuncArgMode::Named => format!(
                "{required}{type_str} {name_str} {default_value}",
                required = if *is_required { "required " } else { "" }
            ),
        }
    }
}

pub(crate) fn generate(
    func: &MirFunc,
    context: ApiDartGeneratorContext,
) -> anyhow::Result<ApiDartGeneratedFunction> {
    let return_stream = compute_return_stream(func);
    let func_params = generate_params(
        func,
        context,
        context.config.dart_enums_style,
        &return_stream,
    );
    let func_params_str = compute_params_str(&func_params, func.arg_mode);
    let func_return_type = generate_function_dart_return_type(
        func,
        &ApiDartGenerator::new(func.output.normal.clone(), context).dart_api_type(),
        &return_stream,
        context,
    );

    let func_expr = format!(
        "{func_return_type} {func_name}({func_params_str})",
        func_name = func.name_dart_api(),
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
        src_lineno_pseudo: func.src_lineno_pseudo,
        return_stream,
    })
}

#[derive(Debug, Serialize)]
pub(crate) struct ReturnStreamInfo {
    pub field: MirField,
    pub ty: MirTypeDelegateStreamSink,
}

fn compute_return_stream(func: &MirFunc) -> Option<ReturnStreamInfo> {
    let stream_sink_vars = (func.inputs.iter())
        .filter_map(|input| {
            if_then_some!(
                let MirType::Delegate(MirTypeDelegate::StreamSink(ty)) = &input.inner.ty,
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
    func: &MirFunc,
    context: ApiDartGeneratorContext,
    dart_enums_style: bool,
    return_stream: &Option<ReturnStreamInfo>,
) -> Vec<ApiDartGeneratedFunctionParam> {
    let params = (func.inputs.iter())
        .filter(|field| Some(&field.inner.name) != return_stream.as_ref().map(|s| &s.field.name))
        .map(|input| {
            let type_str = ApiDartGenerator::new(input.inner.ty.clone(), context).dart_api_type();
            let name_str = input.inner.name.dart_style();
            let default_value = generate_field_default(&input.inner, false, dart_enums_style);
            ApiDartGeneratedFunctionParam {
                is_required: !input.inner.is_optional(),
                type_str,
                name_str,
                default_value,
            }
        })
        .collect_vec();
    // params.push(ApiDartGeneratedFunctionParam {
    //     full: "dynamic hint".to_string(),
    //     type_str: "dynamic".to_string(),
    //     name_str: "hint".to_string(),
    // });

    params
}

pub(crate) fn compute_params_str(
    params: &[ApiDartGeneratedFunctionParam],
    mode: MirFuncArgMode,
) -> String {
    let mut params_str = params.iter().map(|x| x.full(mode)).join(", ");
    if !params_str.is_empty() && mode == MirFuncArgMode::Named {
        params_str = format!("{{{params_str}}}");
    }
    params_str
}

fn generate_func_impl(
    func: &MirFunc,
    dart_entrypoint_class_name: &str,
    return_stream: &Option<ReturnStreamInfo>,
) -> String {
    let func_name = &func.name_dart_wire();
    let param_names: Vec<String> = [
        ((func.inputs.iter())
            .filter(|field| {
                Some(&field.inner.name) != return_stream.as_ref().map(|s| &s.field.name)
            })
            .map(|input| input.inner.name.dart_style()))
        .collect_vec(),
        // vec!["hint".to_owned()],
    ]
    .concat();
    let param_forwards = param_names
        .iter()
        .map(|name| format!("{name}: {name}"))
        .join(", ");
    format!("{dart_entrypoint_class_name}.instance.api.{func_name}({param_forwards})")
}

fn generate_header(
    func: &MirFunc,
    context: ApiDartGeneratorContext,
) -> anyhow::Result<DartHeaderCode> {
    Ok(DartHeaderCode {
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
    func: &MirFunc,
    inner: &str,
    return_stream: &Option<ReturnStreamInfo>,
    context: ApiDartGeneratorContext,
) -> String {
    let mut inner = inner.to_owned();

    if let Some(return_stream) = return_stream {
        inner = format!(
            "Stream<{}>",
            ApiDartGenerator::new(return_stream.ty.inner_ok.clone(), context).dart_api_type()
        );
    }

    let return_future = if return_stream.is_some() {
        func.stream_dart_await
    } else {
        func.mode != MirFuncMode::Sync
    };
    if return_future {
        inner = format!("Future<{inner}>");
    }

    inner
}
