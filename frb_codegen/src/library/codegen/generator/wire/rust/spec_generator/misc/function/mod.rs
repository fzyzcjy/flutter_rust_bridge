pub(crate) mod lifetime;
pub(crate) mod lockable;

use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::WireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::mir::func::{MirFunc, MirFuncMode, MirFuncOwnerInfo};
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::misc::consts::HANDLER_NAME;
use convert_case::{Case, Casing};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryInto;

pub(crate) fn generate_wire_func(
    func: &MirFunc,
    context: WireRustGeneratorContext,
) -> Acc<WireRustOutputCode> {
    let dart2rust_codec = WireRustCodecEntrypoint::from(func.codec_mode_pack.dart2rust);

    let params = dart2rust_codec.generate_func_params(func, context);
    let inner_func_args = generate_inner_func_args(func);
    let wrap_info_obj = generate_wrap_info_obj(func);
    let code_decode = dart2rust_codec.generate_func_call_decode(func, context);
    let code_inner_decode = generate_code_inner_decode(func);
    let code_call_inner_func_result = generate_code_call_inner_func_result(func, inner_func_args);
    let code_postprocess_inner_output = generate_code_postprocess_inner_output(func);
    let handler_func_name = generate_handler_func_name(func);
    let return_type = generate_return_type(func);
    let code_closure = generate_code_closure(
        func,
        &code_decode,
        &code_inner_decode,
        &code_call_inner_func_result,
        &code_postprocess_inner_output,
    );
    let func_name = wire_func_name(func);

    Acc::new(|target| match target {
        TargetOrCommon::Io | TargetOrCommon::Web => ExternFunc {
            partial_func_name: func_name.clone(),
            params: params.clone().get(target),
            return_type: return_type.clone(),
            body: generate_redirect_body(func, &params.common),
            target: target.try_into().unwrap(),
            needs_ffigen: true,
        }
        .into(),
        TargetOrCommon::Common => format!(
            "fn {func_name}_impl({params}) {return_type} {{
                {HANDLER_NAME}.{handler_func_name}({wrap_info_obj}, move || {{ {code_closure} }})
            }}",
            HANDLER_NAME = HANDLER_NAME,
            params = params
                .common
                .iter()
                .map(|param| param.rust_name_and_type())
                .join(","),
            return_type = return_type
                .clone()
                .map(|t| format!("-> {t}"))
                .unwrap_or_default(),
        )
        .into(),
    })
}

fn generate_inner_func_args(func: &MirFunc) -> Vec<String> {
    (func.inputs.iter())
        .map(|field| {
            let ownership = lockable::generate_inner_func_arg_ownership(field);
            let ans = format!(
                "{ownership}api_{name}",
                name = field.inner.name.rust_style()
            );
            let ans = lockable::generate_inner_func_arg(&ans, field);

            lifetime::generate_inner_func_arg(&ans, field)
        })
        .collect_vec()
}

fn generate_wrap_info_obj(func: &MirFunc) -> String {
    format!(
        "flutter_rust_bridge::for_generated::TaskInfo{{ debug_name: \"{name}\", port: {port}, mode: flutter_rust_bridge::for_generated::FfiCallMode::{mode} }}",
        name = func.name.name,
        port = if has_port_argument(func.mode) {
            "Some(port_)"
        } else {
            "None"
        },
        mode = ffi_call_mode(func.mode),
    )
}

fn generate_code_inner_decode(func: &MirFunc) -> String {
    lifetime::generate_code_inner_decode(func, &lockable::generate_code_inner_decode(func))
}

fn generate_code_call_inner_func_result(func: &MirFunc, inner_func_args: Vec<String>) -> String {
    let mut ans = (func.rust_call_code.clone()).unwrap_or_else(|| {
        match &func.owner {
            MirFuncOwnerInfo::Function => {
                format!("{}({})", func.name.rust_style(), inner_func_args.join(", "))
            }
            MirFuncOwnerInfo::Method(method) => {
                let owner_ty_name = method.owner_ty_name().unwrap().rust_style();
                // For simplicity, remove all generics currently
                lazy_static! {
                    static ref REGEX: Regex = Regex::new(r#"<(.+)>"#).unwrap();
                }
                let stripped_name = REGEX.replace_all(&owner_ty_name, "").to_string();

                format!(
                    r"{stripped_name}::{}({})",
                    method.actual_method_name,
                    inner_func_args.join(", ")
                )
            }
        }
    });

    if func.rust_async {
        ans = format!("{ans}.await");
    }

    if func.output.normal == MirType::Primitive(MirTypePrimitive::Unit)
        && func.output.error.is_none()
    {
        ans = format!("{{ {ans}; }}");
    }

    if !func.fallible() {
        ans = format!("Result::<_,()>::Ok({ans})");
    }

    ans = format!("let output_ok = {ans}?;");

    ans
}

fn generate_code_postprocess_inner_output(func: &MirFunc) -> String {
    lifetime::generate_code_postprocess_inner_output(func)
}

fn generate_handler_func_name(func: &MirFunc) -> String {
    let codec = format!(
        "flutter_rust_bridge::for_generated::{}Codec",
        func.codec_mode_pack.rust2dart.delegate_or_self()
    );

    match func.mode {
        MirFuncMode::Sync => format!("wrap_sync::<{codec},_>"),
        MirFuncMode::Normal => {
            let name = if func.rust_async {
                "wrap_async"
            } else {
                "wrap_normal"
            };

            let generic_args = if func.rust_async {
                format!("<{codec},_,_,_>")
            } else {
                format!("<{codec},_,_>")
            };

            format!("{name}::{generic_args}")
        }
    }
}

fn generate_return_type(func: &MirFunc) -> Option<String> {
    match func.mode {
        MirFuncMode::Sync => Some(format!(
            "flutter_rust_bridge::for_generated::WireSyncRust2Dart{}",
            func.codec_mode_pack.rust2dart.delegate_or_self(),
        )),
        MirFuncMode::Normal => None,
    }
}

fn generate_code_closure(
    func: &MirFunc,
    code_decode: &str,
    code_inner_decode: &str,
    code_call_inner_func_result: &str,
    code_postprocess_inner_output: &str,
) -> String {
    let codec_mode = func.codec_mode_pack.rust2dart.delegate_or_self();
    let codec = (codec_mode).to_string().to_case(Case::Snake);

    let code_aop_after = func
        .rust_aop_after
        .as_deref()
        .unwrap_or("")
        .replace("{}", "api_that_guard");

    let code_inner = format!(
        "{code_inner_decode} {code_call_inner_func_result} {code_postprocess_inner_output} {code_aop_after} Ok(output_ok)"
    );

    let err_type = (func.output.error.as_ref())
        .map(|e| e.rust_api_type())
        .unwrap_or("()".to_owned());

    let transform_result_func = format!(
        "transform_result_{codec}::<{generic_prefix}, {err_type}>",
        generic_prefix = match codec_mode {
            CodecMode::Dco => "_, _",
            CodecMode::Sse => "_",
            _ => unreachable!(),
        }
    );

    match func.mode {
        MirFuncMode::Sync => {
            format!(
                "{code_decode}
                {transform_result_func}((move || {{
                    {code_inner}
                }})())"
            )
        }
        MirFuncMode::Normal => {
            let maybe_async_move = if func.rust_async { "async move" } else { "" };
            let maybe_await = if func.rust_async { ".await" } else { "" };
            format!(
                "{code_decode} move |context| {maybe_async_move} {{
                    {transform_result_func}((move || {maybe_async_move} {{
                        {code_inner}
                    }})(){maybe_await})
                }}"
            )
        }
    }
}

fn generate_redirect_body(func: &MirFunc, params: &[ExternFuncParam]) -> String {
    format!(
        "{}_impl({})",
        wire_func_name(func),
        params.iter().map(|x| x.name.clone()).join(", "),
    )
}

pub(crate) fn wire_func_name(func: &MirFunc) -> String {
    let name = &func.name;
    format!("wire__{}__{}", name.namespace.safe_ident(), name.name)
}

fn ffi_call_mode(mode: MirFuncMode) -> &'static str {
    match mode {
        MirFuncMode::Normal => "Normal",
        MirFuncMode::Sync => "Sync",
    }
}
