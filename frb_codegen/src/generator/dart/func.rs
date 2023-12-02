use crate::utils::misc::BlockIndex;

use super::*;

#[derive(Debug)]
pub(crate) struct GeneratedApiMethod {
    pub comments: String,
    pub signature: String,
    pub implementation: String,
}

pub(crate) fn generate_api_func(
    func: &IrFunc,
    config: &Opts,
    all_configs: &AllConfigs,
) -> GeneratedApiFunc {
    let raw_func_param_list = func
        .inputs
        .iter()
        .map(|input| {
            format!(
                "{required}{} {} {default}",
                input.ty.dart_api_type(),
                input.name.dart_style(),
                required = input.required_modifier(),
                default = input.field_default(false, None),
            )
        })
        .collect::<Vec<_>>();
    let full_func_param_list = [raw_func_param_list, vec!["dynamic hint".to_owned()]].concat();
    let prepare_args = func
        .inputs
        .iter()
        .enumerate()
        .map(|(index, input)| {
            // edge case: ffigen performs its own bool-to-int conversions
            if let Primitive(IrTypePrimitive::Bool) = input.ty {
                format!("var arg{index} = {};", input.name.dart_style())
            } else {
                let func = format!("api2wire_{}", input.ty.safe_ident());
                let prefix = get_api2wire_prefix(&func, config, &input.ty, true, all_configs);
                format!(
                    "var arg{index} = {prefix}{func}({});",
                    &input.name.dart_style()
                )
            }
        })
        .collect::<Vec<_>>();

    let wire_param_list = [
        if func.mode.has_port_argument() {
            vec!["port_".to_owned()]
        } else {
            vec![]
        },
        (0..prepare_args.len())
            .map(|index| format!("arg{index}"))
            .collect_vec(),
    ]
    .concat();

    let func_expr = format!(
        "{} {}({{ {} }})",
        func.mode.dart_return_type(&func.output.dart_api_type()),
        func.name.to_case(Case::Camel),
        full_func_param_list.join(","),
    );

    let execute_func_name = match func.mode {
        IrFuncMode::Normal => "_platform.executeNormal",
        IrFuncMode::Sync => "_platform.executeSync",
        IrFuncMode::Stream { .. } => "_platform.executeStream",
    };

    let const_meta_field_name = format!("k{}ConstMeta", func.name.to_case(Case::Pascal));

    let signature = format!("{func_expr};");

    let comments = dart_comments(&func.comments);

    let task_common_args = format!(
        "
        constMeta: {},
        argValues: [{}],
        hint: hint,
        ",
        const_meta_field_name,
        func.inputs
            .iter()
            .map(|input| input.name.dart_style())
            .collect::<Vec<_>>()
            .join(", "),
    );

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
    let mut parse_success_data = if (f.is_static_method()
        && f.struct_name().unwrap() == {
            if let IrType::StructRef(IrTypeStructRef { name, .. }) = &func.output {
                name.clone()
            } else {
                String::new()
            }
        })
        // If struct has a method with first element `input0`
        || (input_0_struct_name.is_some() && MethodNamingUtil::has_methods(input_0_struct_name.unwrap(), config,all_configs))
        //If output is a struct with methods
        || (func_output_struct_name.is_some()
            && MethodNamingUtil::has_methods(func_output_struct_name.unwrap(), config,all_configs))
    {
        format!("(d) => _wire2api_{}(d)", func.output.safe_ident())
    } else {
        format!("_wire2api_{}", func.output.safe_ident())
    };

    let mut parse_error_data = if let Some(error_output) = &func.error_output {
        format!("_wire2api_{}", error_output.safe_ident())
    } else {
        "null".to_string()
    };

    if config.shared {
        parse_success_data = parse_success_data.replace("_wire2api_", "wire2api_");
        parse_error_data = parse_error_data.replace("_wire2api_", "wire2api_");
    } else if all_configs.is_type_shared(&func.output, true) {
        parse_success_data = parse_success_data.replace("_wire2api_", "_sharedImpl.wire2api_");
        parse_error_data = parse_error_data.replace("_wire2api_", "_sharedImpl.wire2api_");
    }

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
        func.wire_func_name(),
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

    let companion_field_signature =
        format!("FlutterRustBridgeTaskConstMeta get {const_meta_field_name};");

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
            .collect::<Vec<_>>()
            .join(", "),
    );

    GeneratedApiFunc {
        signature,
        implementation,
        comments,
        companion_field_signature,
        companion_field_implementation,
    }
}

pub(crate) fn get_api2wire_prefix(
    api2wire_func: &str,
    config: &Opts,
    ir_type: &IrType,
    for_dart_common_file: bool,
    all_configs: &AllConfigs,
) -> String {
    let dart_api2wire_funcs = all_configs.get_dart_api2wire_funcs(config.block_index);
    let common_api2wire_body = if let Some(dart_api2wire_funcs) = dart_api2wire_funcs {
        dart_api2wire_funcs.common.clone()
    } else {
        "".into()
    };
    let shared_dart_api2wire_funcs = all_configs.get_dart_api2wire_funcs(BlockIndex::new_shared());

    let prefix = if common_api2wire_body.contains(api2wire_func) {
        ""
    } else {
        match shared_dart_api2wire_funcs {
            // multi-blocks case with `shared_dart_api2wire_funcs` initialized
            Some(shared_dart_api2wire_funcs) => {
                if config.shared {
                    if for_dart_common_file {
                        "_platform."
                    } else {
                        ""
                    }
                } else if all_configs.is_type_shared(ir_type, true) {
                    let shared_common_api2wire_body = &shared_dart_api2wire_funcs.common;
                    if shared_common_api2wire_body.contains(api2wire_func) {
                        ""
                    } else {
                        "_sharedPlatform."
                    }
                } else if common_api2wire_body.contains(api2wire_func) {
                    ""
                } else if for_dart_common_file {
                    "_platform."
                } else {
                    ""
                }
            }
            // single block case, or multi-blocks case with `shared_dart_api2wire_funcs` NOT initilaized yet
            _ => {
                if for_dart_common_file {
                    "_platform."
                } else {
                    ""
                }
            }
        }
    };

    prefix.into()
}

pub(crate) fn get_api_to_fill_wire_prefix(
    config: &Opts,
    ir_type: &IrType,
    all_configs: &AllConfigs,
) -> String {
    if config.shared {
        ""
    } else if all_configs.is_type_shared(ir_type, true) {
        "_sharedPlatform"
    } else {
        "_"
    }
    .into()
}

pub(crate) fn generate_opaque_getters(ty: &IrType) -> GeneratedApiFunc {
    let signature = format!(
        "
    DropFnType get dropOpaque{0};
    ShareFnType get shareOpaque{0};
    OpaqueTypeFinalizer get {0}Finalizer;
    ",
        ty.dart_api_type(),
    );

    let implementation = format!(
        "
        DropFnType get dropOpaque{0} => _platform.inner.drop_opaque_{0};
        ShareFnType get shareOpaque{0} => _platform.inner.share_opaque_{0};
        OpaqueTypeFinalizer get {0}Finalizer => _platform.{0}Finalizer;
        ",
        ty.dart_api_type()
    );

    GeneratedApiFunc {
        signature,
        implementation,
        comments: String::new(),
        companion_field_signature: String::new(),
        companion_field_implementation: String::new(),
    }
}
