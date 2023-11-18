use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::wire2api::extern_func::ExternFunc;
use crate::codegen::ir::func::{IrFunc, IrFuncMode};
use crate::codegen::ir::pack::IrPack;
use itertools::Itertools;

pub(crate) fn generate_wire_func(func: &IrFunc, ir_pack: &IrPack) -> Acc<String> {
    let mut params = if func.mode.has_port_argument() {
        Acc {
            io: vec!["port_: i64".to_owned()],
            wasm: vec!["port_: MessagePort".to_owned()],
            common: vec!["port_: MessagePort".to_owned()],
        }
    } else {
        Acc::default()
    };

    params += func
        .inputs
        .iter()
        .map(|field| {
            let name = field.name.rust_style();
            Acc::new(|target| match target {
                Common => format!(
                    "{}: impl Wire2Api<{}> + UnwindSafe",
                    name,
                    field.ty.rust_api_type()
                ),
                Io | Wasm => format!(
                    "{}: {}{}",
                    name,
                    field.ty.rust_wire_modifier(target),
                    field.ty.rust_wire_type(target)
                ),
            })
        })
        .collect();

    let mut inner_func_params = [
        vec![],
        func.inputs
            .iter()
            .map(|field| format!("api_{}", field.name.rust_style()))
            .collect_vec(),
    ]
    .concat();
    if let IrFuncMode::Stream { argument_index } = func.mode {
        inner_func_params.insert(
            argument_index,
            format!(
                "task_callback.stream_sink::<_,{}>()",
                func.output.intodart_type(ir_pack)
            ),
        );
    }
    let wrap_info_obj = format!(
        "WrapInfo{{ debug_name: \"{}\", port: {}, mode: FfiCallMode::{} }}",
        func.name,
        if func.mode.has_port_argument() {
            "Some(port_)"
        } else {
            "None"
        },
        func.mode.ffi_call_mode(),
    );

    let code_wire2api = func
        .inputs
        .iter()
        .map(|field| format!("let api_{0} = {0}.wire2api();", field.name.rust_style()))
        .collect_vec()
        .join("");

    let code_call_inner_func = if func.owner.is_non_static_method() || func.owner.is_static_method()
    {
        let method_name = if func.owner.is_non_static_method() {
            inner_func_params[0] = format!("&{}", inner_func_params[0]);
            FunctionName::deserialize(&func.name).method_name()
        } else if func.owner.is_static_method() {
            FunctionName::deserialize(&func.name)
                .static_method_name()
                .unwrap()
        } else {
            panic!("{} is not a method, nor a static method.", func.name)
        };
        format!(
            r"{}::{}({})",
            struct_name.unwrap(),
            method_name,
            inner_func_params.join(", ")
        )
    } else {
        format!("{}({})", func.name, inner_func_params.join(", "))
    };
    let code_call_inner_func_result = if func.fallible {
        code_call_inner_func
    } else {
        format!("Result::<_,()>::Ok({code_call_inner_func})")
    };

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
    let redirect_body = format!(
        "{}_impl({})",
        func.wire_func_name(),
        (func.mode.has_port_argument().then_some("port_"))
            .into_iter()
            .chain(func.inputs.iter().map(|arg| arg.name.rust_style()))
            .collect_vec()
            .join(","),
    );

    Acc::new(|target| match target {
        Io | Wasm => ExternFunc {
            func_name: func.wire_func_name(),
            params: if target == Target::Wasm {
                &params.wasm[..]
            } else {
                &params.io[..]
            }
            .iter()
            .map(|item| (item, ""))
            .collect_vec(),
            return_type,
            body: &redirect_body,
            target,
        },
        Common => format!(
            "fn {}_impl({}) {} {{ {} }}",
            func.wire_func_name(),
            params.common.join(","),
            return_type.map(|t| format!("-> {t}")).unwrap_or_default(),
            body,
        ),
    })
}
