use crate::codegen::generator::api_dart;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::ir::func::{IrFunc, IrFuncMode, IrFuncOwnerInfo};
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::structure::IrTypeStructRef;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{Primitive, StructRef};
use itertools::Itertools;

pub(crate) fn generate_dispatcher_api_functions(
    func: &IrFunc,
    context: WireDartGeneratorContext,
) -> WireDartOutputCode {
    let api_dart_func =
        api_dart::spec_generator::function::generate(func, context.as_api_dart_context());

    let prepare_args = generate_prepare_args(func);
    let wire_param_list = generate_wire_param_list(func, prepare_args.len());
    let execute_func_name = generate_execute_func_name(func);
    let task_common_args = generate_task_common_args(func);

    let input_0 = func.inputs.get(0).as_ref().map(|x| &x.ty);
    let input_0_struct_name = if let Some(StructRef(IrTypeStructRef { name, .. })) = &input_0 {
        Some(name)
    } else {
        None
    };
    let f = FunctionName::deserialize(&func.name);
    let func_output_struct_name = if let StructRef(IrTypeStructRef { name, .. }) = &func.output {
        Some(name)
    } else {
        None
    };
    let parse_success_data = if (f.is_static_method()
        && f.struct_name().unwrap() == {
        if let IrType::StructRef(IrTypeStructRef { name, .. }) = &func.output {
            name.clone()
        } else {
            String::new()
        }
    })
        // If struct has a method with first element `input0`
        || (input_0_struct_name.is_some() && has_methods_for_struct_name(input_0_struct_name.unwrap(), context.ir_pack))
        //If output is a struct with methods
        || (func_output_struct_name.is_some()
        && has_methods_for_struct_name(func_output_struct_name.unwrap(), context.ir_pack))
    {
        format!("(d) => _wire2api_{}(d)", func.output.safe_ident())
    } else {
        format!("_wire2api_{}", func.output.safe_ident())
    };

    let parse_error_data = if let Some(error_output) = &func.error_output {
        format!("_wire2api_{}", error_output.safe_ident())
    } else {
        "null".to_string()
    };

    let is_sync = matches!(func.mode, IrFuncMode::Sync);
    let implementation = format!(
        "{} {{
            {}
            return {}({task}(
            callFfi: ({args}) => _platform.inner.{}({}),
            parseSuccessData: {},
            parseErrorData: {},
            {}
        ));}}",
        func_expr,
        prepare_args.join("\n"),
        execute_func_name,
        wire_func_name(func),
        wire_param_list.join(", "),
        parse_success_data,
        parse_error_data,
        task_common_args,
        task = if is_sync {
            "FlutterRustBridgeSyncTask"
        } else {
            "FlutterRustBridgeTask"
        },
        args = if is_sync { "" } else { "port_" },
    );

    let companion_field_implementation = format!(
        "
        FlutterRustBridgeTaskConstMeta get {const_meta_field_name} => const FlutterRustBridgeTaskConstMeta(
            debugName: \"{}\",
            argNames: [{}],
        );
        ",
        func.name,
        func.inputs
            .iter()
            .map(|input| format!("\"{}\"", input.name.dart_style()))
            .collect_vec()
            .join(", "),
    );

    WireDartOutputCode {
        dispatcher_body: todo!(),
        ..Default::default()
    }
}

fn generate_prepare_args(func: &IrFunc) -> Vec<String> {
    func.inputs
        .iter()
        .enumerate()
        .map(|(index, input)| {
            // edge case: ffigen performs its own bool-to-int conversions
            if let Primitive(IrTypePrimitive::Bool) = input.ty {
                format!("var arg{index} = {};", input.name.dart_style())
            } else {
                let func = format!("api2wire_{}", input.ty.safe_ident());
                format!(
                    "var arg{index} = {}{}({});",
                    if common_api2wire_body.contains(&func) {
                        ""
                    } else {
                        "_platform."
                    },
                    func,
                    &input.name.dart_style()
                )
            }
        })
        .collect_vec()
}

fn generate_wire_param_list(func: &IrFunc, num_prepare_args: usize) -> Vec<String> {
    [
        if has_port_argument(func.mode) {
            vec!["port_".to_owned()]
        } else {
            vec![]
        },
        (0..num_prepare_args)
            .map(|index| format!("arg{index}"))
            .collect_vec(),
    ]
    .concat()
}

fn generate_execute_func_name(func: &IrFunc) -> &str {
    match func.mode {
        IrFuncMode::Normal => "_platform.executeNormal",
        IrFuncMode::Sync => "_platform.executeSync",
        IrFuncMode::Stream { .. } => "_platform.executeStream",
    }
}

fn generate_task_common_args(func: &IrFunc) -> String {
    format!(
        "
        constMeta: {},
        argValues: [{}],
        hint: hint,
        ",
        const_meta_field_name,
        func.inputs
            .iter()
            .map(|input| input.name.dart_style())
            .collect_vec()
            .join(", "),
    )
}

fn has_methods_for_struct_name(struct_name: &str, ir_pack: &IrPack) -> bool {
    (ir_pack.funcs.iter()).any(|f| matches!(&f.owner, IrFuncOwnerInfo::Method(_)))
}

pub(crate) fn generate_dispatcher_opaque_getters() -> WireDartOutputCode {
    WireDartOutputCode {
        dispatcher_body: todo!(),
        ..Default::default()
    }
}
