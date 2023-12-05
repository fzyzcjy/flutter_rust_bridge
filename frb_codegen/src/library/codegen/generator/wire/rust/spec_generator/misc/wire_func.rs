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
use crate::codegen::ir::ty::ownership::IrTypeOwnershipMode;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorDart2RustTrait;
use crate::library::codegen::generator::wire::rust::spec_generator::rust2dart::ty::WireRustGeneratorRust2DartTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use crate::misc::consts::HANDLER_NAME;
use convert_case::{Case, Casing};
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
    let code_wire2api = generate_code_wire2api(func, context);
    let code_inner_wire2api = generate_code_inner_wire2api(func);
    let code_call_inner_func_result = generate_code_call_inner_func_result(func, inner_func_params);
    let handler_func_name = generate_handler_func_name(func, ir_pack, context);
    let return_type = generate_return_type(func);
    let code_closure = generate_code_closure(
        func,
        &code_wire2api,
        &code_inner_wire2api,
        &code_call_inner_func_result,
    );
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
            "fn {func_name}_impl({params}) {return_type} {{
                {HANDLER_NAME}.{handler_func_name}({wrap_info_obj}, move || {{ {code_closure} }})
            }}",
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

fn generate_inner_func_params(
    func: &IrFunc,
    ir_pack: &IrPack,
    context: WireRustGeneratorContext,
) -> Vec<String> {
    let mut ans = func
        .inputs
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let mut ans = format!("api_{}", field.name.rust_style());
            if let IrType::RustAutoOpaque(o) = &field.ty {
                ans = match o.ownership_mode {
                    IrTypeOwnershipMode::Ref => format!("&{ans}"),
                    IrTypeOwnershipMode::RefMut => format!("&mut {ans}"),
                    _ => ans,
                };
            } else if index == 0 && matches!(&func.owner, IrFuncOwnerInfo::Method(IrFuncOwnerInfoMethod { mode, .. }) if mode == &Instance) {
                ans = format!("&{ans}");
            }
            ans
        })
        .collect_vec();

    if let IrFuncMode::Stream { argument_index } = func.mode {
        ans.insert(
            argument_index,
            format!(
                "context.rust2dart_context().stream_sink::<_,{}>()",
                WireRustGenerator::new(func.output.clone(), context).intodart_type(ir_pack)
            ),
        );
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
                TargetOrCommon::Common | TargetOrCommon::Wasm => {
                    "flutter_rust_bridge::for_generated::MessagePort"
                }
            }
            .to_owned();
            vec![ExternFuncParam {
                name: "port_".to_owned(),
                rust_type,
                dart_type: "NativePortType".to_owned(),
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
                        WireRustGenerator::new(field.ty.clone(), context)
                            .generate_wire_func_param_api_type()
                            .unwrap_or(field.ty.rust_api_type())
                    ),
                    dart_type: "THIS_TYPE_SHOULD_NOT_BE_USED".into(),
                },
                TargetOrCommon::Io | TargetOrCommon::Wasm => {
                    let target: Target = target.try_into().unwrap();
                    ExternFuncParam::new(name.clone(), target, &field.ty, context)
                }
            })
        })
        .collect();

    params
}

fn generate_wrap_info_obj(func: &IrFunc) -> String {
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

fn generate_code_wire2api(func: &IrFunc, context: WireRustGeneratorContext) -> String {
    func.inputs
        .iter()
        .map(|field| {
            let name = field.name.rust_style();
            let wire_func_call_wire2api = WireRustGenerator::new(field.ty.clone(), context)
                .generate_wire_func_call_wire2api();
            format!("let api_{name} = {wire_func_call_wire2api};")
        })
        .join("")
}

fn generate_code_inner_wire2api(func: &IrFunc) -> String {
    func.inputs
        .iter()
        .filter_map(|field| {
            if let IrType::RustAutoOpaque(o) = &field.ty {
                let mode = o.ownership_mode.to_string().to_case(Case::Snake);
                let mutability = if o.ownership_mode == IrTypeOwnershipMode::RefMut { "mut " } else {""};
                Some(format!(
                    "let {mutability}api_{name} = api_{name}.rust_auto_opaque_wire2api_{mode}()?;\n",
                    name = field.name.rust_style()
                ))
            } else {
                None
            }
        })
        .join("")
}

fn generate_code_call_inner_func_result(func: &IrFunc, inner_func_params: Vec<String>) -> String {
    let mut ans = match &func.owner {
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

    if func.rust_async {
        ans = format!("{ans}.await");
    }

    if matches!(&func.output, IrType::RustAutoOpaque(_)) {
        ans = format!("flutter_rust_bridge::for_generated::rust_auto_opaque_api2wire({ans})");
    }

    if !func.fallible() {
        let error_type = if (func.inputs.iter()).any(|x| matches!(x.ty, IrType::RustAutoOpaque(_)))
        {
            "anyhow::Error"
        } else {
            "()"
        };
        ans = format!("Result::<_,{error_type}>::Ok({ans})");
    }

    ans
}

fn generate_handler_func_name(
    func: &IrFunc,
    ir_pack: &IrPack,
    context: WireRustGeneratorContext,
) -> String {
    match func.mode {
        IrFuncMode::Sync => "wrap_sync".to_owned(),
        IrFuncMode::Normal | IrFuncMode::Stream { .. } => {
            let name = if func.rust_async {
                "wrap_async"
            } else {
                "wrap_normal"
            };

            let output = if matches!(func.mode, IrFuncMode::Stream { .. }) {
                "()".to_owned()
            } else {
                WireRustGenerator::new(func.output.clone(), context).intodart_type(ir_pack)
            };

            let generic_args = if func.rust_async {
                format!("<_,_,_,_,{output},_>")
            } else {
                format!("<_,_,_,{output},_>")
            };

            format!("{name}::{generic_args}")
        }
    }
}

fn generate_return_type(func: &IrFunc) -> Option<String> {
    match func.mode {
        IrFuncMode::Sync => Some("flutter_rust_bridge::for_generated::WireSyncReturn".to_owned()),
        IrFuncMode::Normal | IrFuncMode::Stream { .. } => None,
    }
}

fn generate_code_closure(
    func: &IrFunc,
    code_wire2api: &str,
    code_inner_wire2api: &str,
    code_call_inner_func_result: &str,
) -> String {
    match func.mode {
        IrFuncMode::Sync => {
            format!("{code_wire2api}{code_inner_wire2api}{code_call_inner_func_result}")
        }
        IrFuncMode::Normal | IrFuncMode::Stream { .. } => {
            let maybe_async_move = if func.rust_async { "async move" } else { "" };
            format!("{code_wire2api} move |context| {maybe_async_move} {{ {code_inner_wire2api} {code_call_inner_func_result} }}")
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
