use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::generator::wire::rust::wire2api::extern_func::{
    CodeWithExternFunc, ExternFunc, ExternFuncParam,
};
use crate::codegen::ir::func::IrFuncOwnerInfoMethodMode::Instance;
use crate::codegen::ir::func::{
    IrFunc, IrFuncMode, IrFuncOwnerInfo, IrFuncOwnerInfoMethod, IrFuncOwnerInfoMethodMode,
};
use crate::codegen::ir::pack::IrPack;
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use crate::misc::consts::HANDLER_NAME;
use itertools::Itertools;
use std::convert::TryInto;

pub(crate) fn generate_wire_func(
    func: &IrFunc,
    ir_pack: &IrPack,
    context: WireRustGeneratorContext,
) -> Acc<CodeWithExternFunc> {
    let params = generate_params(func, context);
    let inner_func_params = generate_inner_func_params(func, ir_pack);
    let wrap_info_obj = generate_wrap_info_obj(func);
    let code_wire2api = generate_code_wire2api(func);
    let code_call_inner_func_result = generate_code_call_inner_func_result(func, inner_func_params);
    let handler_func_name = generate_handler_func_name(func, ir_pack);
    let return_type = generate_return_type(func);
    let code_closure = generate_code_closure(func, &code_wire2api, &code_call_inner_func_result);

    Acc::new(|target| match target {
        TargetOrCommon::Io | TargetOrCommon::Wasm => ExternFunc {
            func_name: func.wire_func_name(),
            params: params[target].clone(),
            return_type,
            body: generate_redirect_body(func),
            target: target.try_into().unwrap(),
        }
        .into(),
        TargetOrCommon::Common => format!(
            "fn {name}_impl({params}) {return_type} {{ {body} }}",
            name = func.wire_func_name(),
            params = params.common.join(","),
            return_type = return_type.map(|t| format!("-> {t}")).unwrap_or_default(),
            body = format!(
                "{HANDLER_NAME}.{handler_func_name}({wrap_info_obj}, move || {{ {code_closure} }})"
            )
        )
        .into(),
    })
}

fn generate_inner_func_params(func: &IrFunc, ir_pack: &IrPack) -> Vec<String> {
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
                func.output.intodart_type(ir_pack)
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
    let mut params = if func.mode.has_port_argument() {
        Acc::new(|target| {
            let rust_type = match target {
                TargetOrCommon::Io => "i64",
                TargetOrCommon::Common | TargetOrCommon::Wasm => "MessagePort",
            }
            .to_owned();
            vec![ExternFuncParam {
                name: "port_".to_owned(),
                rust_type,
                dart_type: None,
            }]
        })
    } else {
        Acc::default()
    };

    params += func
        .inputs
        .iter()
        .map(|field| {
            let name = field.name.rust_style();
            Acc::new(|target| match target {
                TargetOrCommon::Common => format!(
                    "{}: impl Wire2Api<{}> + UnwindSafe",
                    name,
                    field.ty.rust_api_type()
                ),
                TargetOrCommon::Io | TargetOrCommon::Wasm => {
                    let target: Target = target.try_into().unwrap();
                    format!(
                        "{}: {}{}",
                        name,
                        WireRustGenerator::new(field.ty.clone(), context)
                            .rust_wire_modifier(target),
                        WireRustGenerator::new(field.ty.clone(), context).rust_wire_type(target)
                    )
                }
            })
        })
        .collect();

    params
}

fn generate_wrap_info_obj(func: &IrFunc) -> String {
    format!(
        "WrapInfo{{ debug_name: \"{name}\", port: {port}, mode: FfiCallMode::{mode} }}",
        name = func.name,
        port = if func.mode.has_port_argument() {
            "Some(port_)"
        } else {
            "None"
        },
        mode = func.mode.ffi_call_mode(),
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
        IrFuncOwnerInfo::Function => format!("{}({})", func.name, inner_func_params.join(", ")),
        IrFuncOwnerInfo::Method(method) => {
            format!(
                r"{}::{}({})",
                method.struct_name,
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

fn generate_handler_func_name(func: &IrFunc, ir_pack: &IrPack) -> String {
    match func.mode {
        IrFuncMode::Sync => "wrap_sync".to_owned(),
        IrFuncMode::Normal | IrFuncMode::Stream { .. } => {
            let output = if matches!(func.mode, IrFuncMode::Stream { .. }) {
                "()".to_owned()
            } else {
                func.output.intodart_type(ir_pack)
            };
            format!("wrap::<_,_,_,{output},_>")
        }
    }
}

fn generate_return_type(func: &IrFunc) -> Option<String> {
    match func.mode {
        IrFuncMode::Sync => Some("support::WireSyncReturn".to_owned()),
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
        func.wire_func_name(),
        func.mode
            .has_port_argument()
            .then_some("port_")
            .into_iter()
            .chain(func.inputs.iter().map(|arg| arg.name.rust_style()))
            .join(","),
    )
}
