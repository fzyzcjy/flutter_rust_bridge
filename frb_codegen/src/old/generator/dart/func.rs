use super::*;

#[derive(Debug)]
pub(crate) struct GeneratedApiFunc {
    pub(crate) signature: String,
    pub(crate) implementation: String,
    pub(crate) comments: String,
    pub(crate) companion_field_signature: String,
    pub(crate) companion_field_implementation: String,
}

pub(crate) fn generate_api_func(
    func: &IrFunc,
    ir_pack: &IrPack,
    common_api2wire_body: &str,
    dart_enums_style: bool,
) -> GeneratedApiFunc {
    // NOTE have moved a part of it

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
        .collect_vec();

    let wire_param_list = [
        if has_port_argument(func.mode) {
            vec!["port_".to_owned()]
        } else {
            vec![]
        },
        (0..prepare_args.len())
            .map(|index| format!("arg{index}"))
            .collect_vec(),
    ]
    .concat();

    let execute_func_name = match func.mode {
        IrFuncMode::Normal => "_platform.executeNormal",
        IrFuncMode::Sync => "_platform.executeSync",
        IrFuncMode::Stream { .. } => "_platform.executeStream",
    };

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
            .collect_vec()
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
    let parse_success_data = if (f.is_static_method()
        && f.struct_name().unwrap() == {
        if let IrType::StructRef(IrTypeStructRef { name, .. }) = &func.output {
            name.clone()
        } else {
            String::new()
        }
    })
        // If struct has a method with first element `input0`
        || (input_0_struct_name.is_some() && MethodNamingUtil::has_methods(input_0_struct_name.unwrap(), ir_pack))
        //If output is a struct with methods
        || (func_output_struct_name.is_some()
        && MethodNamingUtil::has_methods(func_output_struct_name.unwrap(), ir_pack))
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
