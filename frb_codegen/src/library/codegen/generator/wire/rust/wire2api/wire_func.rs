use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::wire2api::extern_func::{
    CodeWithExternFunc, ExternFunc, ExternFuncParam,
};
use crate::codegen::ir::func::{IrFunc, IrFuncMode};
use crate::codegen::ir::pack::IrPack;
use itertools::Itertools;
use std::convert::TryInto;

pub(crate) fn generate_wire_func(func: &IrFunc, ir_pack: &IrPack) -> Acc<CodeWithExternFunc> {
    let params = generate_params(func);
    let inner_func_params = generate_inner_func_params(func, ir_pack);
    let wrap_info_obj = generate_wrap_info_obj(func);
    let code_wire2api = generate_code_wire2api(func);
    let code_call_inner_func_result = generate_code_call_inner_func_result(func, inner_func_params);

    let (handler_func_name, return_type, code_closure) = match func.mode {
        IrFuncMode::Sync => (
            String::from("wrap_sync"),
            Some("support::WireSyncReturn"),
            format!(
                "{code_wire2api}
                    {code_call_inner_func_result}"
            ),
        ),
        IrFuncMode::Normal | IrFuncMode::Stream { .. } => {
            let output = if matches!(func.mode, IrFuncMode::Stream { .. }) {
                String::from("()")
            } else {
                func.output.intodart_type(ir_pack)
            };
            (
                format!("wrap::<_,_,_,{output},_>"),
                None,
                format!("{code_wire2api} move |task_callback| {code_call_inner_func_result}"),
            )
        }
    };

    let body = format!(
        "{HANDLER_NAME}.{handler_func_name}({wrap_info_obj}, move || {{ {code_closure} }})"
    );
    let redirect_body = generate_redirect_body(func);

    Acc::new(|target| match target {
        TargetOrCommon::Io | TargetOrCommon::Wasm => ExternFunc {
            func_name: func.wire_func_name(),
            params: params[target].iter().cloned().collect_vec(),
            return_type,
            body: redirect_body,
            target: target.try_into().unwrap(),
        }
        .into(),
        TargetOrCommon::Common => format!(
            "fn {name}_impl({params}) {return_type} {{ {body} }}",
            name = func.wire_func_name(),
            params = params.common.join(","),
            return_type = return_type.map(|t| format!("-> {t}")).unwrap_or_default(),
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

    if func.owner.is_non_static_method() {
        ans[0] = format!("&{}", ans[0]);
    }

    ans
}

fn generate_params(func: &IrFunc) -> Acc<Vec<ExternFuncParam>> {
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
                TargetOrCommon::Io | TargetOrCommon::Wasm => format!(
                    "{}: {}{}",
                    name,
                    field.ty.rust_wire_modifier(target),
                    field.ty.rust_wire_type(target)
                ),
            })
        })
        .collect();

    params
}

fn generate_wrap_info_obj(func: &IrFunc) -> String {
    format!(
        "WrapInfo{{ debug_name: \"{}\", port: {}, mode: FfiCallMode::{} }}",
        func.name,
        if func.mode.has_port_argument() {
            "Some(port_)"
        } else {
            "None"
        },
        func.mode.ffi_call_mode(),
    )
}

fn generate_code_wire2api(func: &IrFunc) -> String {
    func.inputs
        .iter()
        .map(|field| format!("let api_{0} = {0}.wire2api();", field.name.rust_style()))
        .collect_vec()
        .join("")
}

fn generate_code_call_inner_func_result(func: &IrFunc, inner_func_params: Vec<String>) -> String {
    let code_call_inner_func = if func.owner.is_non_static_method() || func.owner.is_static_method()
    {
        let method_name = if func.owner.is_non_static_method() {
            func.owner.method_name()
        } else if func.owner.is_static_method() {
            func.owner.static_method_name().unwrap()
        } else {
            panic!("{} is not a method, nor a static method.", func.name)
        };
        format!(
            r"{}::{}({})",
            func.owner.struct_name.unwrap(),
            method_name,
            inner_func_params.join(", ")
        )
    } else {
        format!("{}({})", func.name, inner_func_params.join(", "))
    };

    if func.fallible {
        code_call_inner_func
    } else {
        format!("Result::<_,()>::Ok({code_call_inner_func})")
    }
}

fn generate_redirect_body(func: &IrFunc) -> String {
    format!(
        "{}_impl({})",
        func.wire_func_name(),
        (func.mode.has_port_argument().then_some("port_"))
            .into_iter()
            .chain(func.inputs.iter().map(|arg| arg.name.rust_style()))
            .collect_vec()
            .join(","),
    )
}
