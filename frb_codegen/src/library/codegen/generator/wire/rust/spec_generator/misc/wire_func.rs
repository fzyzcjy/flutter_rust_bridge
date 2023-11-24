use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::func::IrFuncOwnerInfoMethodMode::Instance;
use crate::codegen::ir::func::{IrFunc, IrFuncMode, IrFuncOwnerInfo, IrFuncOwnerInfoMethod};
use crate::codegen::ir::pack::IrPack;
use crate::library::codegen::generator::wire::rust::spec_generator::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::library::codegen::generator::wire::rust::spec_generator::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use crate::misc::consts::HANDLER_NAME;
use itertools::Itertools;
use std::convert::TryInto;

pub(crate) fn generate_wire_func(
    func: &IrFunc,
    context: WireRustGeneratorContext,
) -> Acc<WireRustOutputCode> {
    let ir_pack = context.ir_pack;
    let params = generate_params(func, context);
    let inner_func_params = generate_inner_func_params(func, ir_pack, context);
    let wrap_info_obj = generate_wrap_info_obj(func);
    let code_wire2api = generate_code_wire2api(func);
    let code_call_inner_func_result = generate_code_call_inner_func_result(func, inner_func_params);
    let handler_func_name = generate_handler_func_name(func, ir_pack, context);
    let return_type = generate_return_type(func);
    let code_closure = generate_code_closure(func, &code_wire2api, &code_call_inner_func_result);
    let func_name = wire_func_name(func);

    Acc::new(|target| match target {
        TargetOrCommon::Io | TargetOrCommon::Wasm => ExternFunc {
            func_name: func_name.clone(),
            params: params.clone().get(target),
            return_type: return_type.clone(),
            body: generate_redirect_body(func),
            target: target.try_into().unwrap(),
        }
        .into(),
        TargetOrCommon::Common => format!(
            "fn {func_name}_impl({params}) {return_type} {{ {body} }}",
            params = params
                .common
                .iter()
                .map(|param| param.rust_name_and_type())
                .join(","),
            return_type = return_type
                .clone()
                .map(|t| format!("-> {t}"))
                .unwrap_or_default(),
            body = format!(
                "{HANDLER_NAME}.{handler_func_name}({wrap_info_obj}, move || {{ {code_closure} }})"
            )
        )
        .into(),
    })
}

fn generate_inner_func_params(
    func: &IrFunc,
    ir_pack: &IrPack,
    context: WireRustGeneratorContext,
) -> Vec<String> {
    let mut ans = func
        .inputs
        .iter()
        .map(|field| format!("api_{}", field.name.rust_style()))
        .collect_vec();

    if let IrFuncMode::Stream { argument_index } = func.mode {
        ans.insert(
            argument_index,
            format!(
                "task_callback.stream_sink::<_,{}>()",
                WireRustGenerator::new(func.output.clone(), context).intodart_type(ir_pack)
            ),
        );
    }

    if matches!(&func.owner, IrFuncOwnerInfo::Method(IrFuncOwnerInfoMethod { mode, .. }) if mode == &Instance)
    {
        ans[0] = format!("&{}", ans[0]);
    }

    ans
}

fn generate_params(func: &IrFunc, context: WireRustGeneratorContext) -> Acc<Vec<ExternFuncParam>> {
    let mut params = if has_port_argument(func.mode) {
        Acc::new(|target| {
            let rust_type = match target {
                // NOTE Though in `io`, i64 == our MessagePort, but it will affect the cbindgen
                // and ffigen and make code tricker, so we manually write down "i64" here.
                TargetOrCommon::Io => "i64",
                TargetOrCommon::Common | TargetOrCommon::Wasm => "flutter_rust_bridge::MessagePort",
            }
            .to_owned();
            vec![ExternFuncParam {
                name: "port_".to_owned(),
                rust_type,
                dart_type: Some("NativePortType".to_owned()),
            }]
        })
    } else {
        Acc::default()
    };

    params += func
        .inputs
        .iter()
        .map(|field| {
            let name = field.name.rust_style().to_owned();
            Acc::new(|target| match target {
                TargetOrCommon::Common => ExternFuncParam {
                    name: name.clone(),
                    rust_type: format!(
                        "impl Wire2Api<{}> + core::panic::UnwindSafe",
                        field.ty.rust_api_type()
                    ),
                    dart_type: None,
                },
                TargetOrCommon::Io | TargetOrCommon::Wasm => {
                    let target: Target = target.try_into().unwrap();
                    let field_generator = WireRustGenerator::new(field.ty.clone(), context);
                    ExternFuncParam {
                        name: name.clone(),
                        rust_type: format!(
                            "{}{}",
                            field_generator.rust_wire_modifier(target),
                            field_generator.rust_wire_type(target)
                        ),
                        dart_type: None,
                    }
                }
            })
        })
        .collect();

    params
}

fn generate_wrap_info_obj(func: &IrFunc) -> String {
    format!(
        "flutter_rust_bridge::WrapInfo{{ debug_name: \"{name}\", port: {port}, mode: flutter_rust_bridge::FfiCallMode::{mode} }}",
        name = func.name.name,
        port = if has_port_argument(func.mode) {
            "Some(port_)"
        } else {
            "None"
        },
        mode = ffi_call_mode(func.mode),
    )
}

fn generate_code_wire2api(func: &IrFunc) -> String {
    func.inputs
        .iter()
        .map(|field| {
            format!(
                "let api_{name} = {name}.wire2api();",
                name = field.name.rust_style()
            )
        })
        .join("")
}

fn generate_code_call_inner_func_result(func: &IrFunc, inner_func_params: Vec<String>) -> String {
    let code_call_inner_func = match &func.owner {
        IrFuncOwnerInfo::Function => {
            format!(
                "{}({})",
                func.name.rust_style(),
                inner_func_params.join(", ")
            )
        }
        IrFuncOwnerInfo::Method(method) => {
            format!(
                r"{}::{}({})",
                method.enum_or_struct_name.rust_style(),
                method.actual_method_name,
                inner_func_params.join(", ")
            )
        }
    };

    if func.fallible() {
        code_call_inner_func
    } else {
        format!("Result::<_,()>::Ok({code_call_inner_func})")
    }
}

fn generate_handler_func_name(
    func: &IrFunc,
    ir_pack: &IrPack,
    context: WireRustGeneratorContext,
) -> String {
    match func.mode {
        IrFuncMode::Sync => "wrap_sync".to_owned(),
        IrFuncMode::Normal | IrFuncMode::Stream { .. } => {
            let output = if matches!(func.mode, IrFuncMode::Stream { .. }) {
                "()".to_owned()
            } else {
                WireRustGenerator::new(func.output.clone(), context).intodart_type(ir_pack)
            };
            format!("wrap::<_,_,_,{output},_>")
        }
    }
}

fn generate_return_type(func: &IrFunc) -> Option<String> {
    match func.mode {
        IrFuncMode::Sync => Some("flutter_rust_bridge::support::WireSyncReturn".to_owned()),
        IrFuncMode::Normal | IrFuncMode::Stream { .. } => None,
    }
}

fn generate_code_closure(
    func: &IrFunc,
    code_wire2api: &str,
    code_call_inner_func_result: &str,
) -> String {
    match func.mode {
        IrFuncMode::Sync => format!(
            "{code_wire2api}
                {code_call_inner_func_result}"
        ),
        IrFuncMode::Normal | IrFuncMode::Stream { .. } => {
            format!("{code_wire2api} move |task_callback| {code_call_inner_func_result}")
        }
    }
}

fn generate_redirect_body(func: &IrFunc) -> String {
    format!(
        "{}_impl({})",
        wire_func_name(func),
        has_port_argument(func.mode)
            .then_some("port_")
            .into_iter()
            .chain(func.inputs.iter().map(|arg| arg.name.rust_style()))
            .join(","),
    )
}

pub(crate) fn wire_func_name(func: &IrFunc) -> String {
    format!("wire_{}", func.name.name)
}

fn ffi_call_mode(mode: IrFuncMode) -> &'static str {
    match mode {
        IrFuncMode::Normal => "Normal",
        IrFuncMode::Sync => "Sync",
        IrFuncMode::Stream { .. } => "Stream",
    }
}
