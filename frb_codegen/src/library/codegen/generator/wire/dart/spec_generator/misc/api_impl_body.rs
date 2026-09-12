use crate::codegen::generator::api_dart;
use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::api_dart::spec_generator::function::{
    compute_params_str, ApiDartGeneratedFunction,
};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::WireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::misc::function::wire_func_name;
use crate::codegen::ir::mir::func::{MirFunc, MirFuncArgMode, MirFuncMode};
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use convert_case::{Case, Casing};
use itertools::Itertools;

pub(crate) fn generate_api_impl_normal_function(
    func: &MirFunc,
    context: WireDartGeneratorContext,
) -> anyhow::Result<WireDartOutputCode> {
    let dart2rust_codec = WireDartCodecEntrypoint::from(func.codec_mode_pack.dart2rust);

    let api_dart_func =
        api_dart::spec_generator::function::generate(func, context.as_api_dart_context())?;

    let const_meta_field_name =
        format!("k{}ConstMeta", func.name_dart_wire().to_case(Case::Pascal));

    let wire_func_name = wire_func_name(func);
    let inner_func_stmt = dart2rust_codec.generate_dart2rust_inner_func_stmt(func, &wire_func_name);
    let execute_func_name = generate_execute_func_name(func);

    let codec = generate_rust2dart_codec_object(func);
    let call_ffi_args = generate_call_ffi_args(func);
    let arg_values = generate_arg_values(func);

    let task_class = generate_task_class(func);

    let ApiDartGeneratedFunction {
        func_return_type,
        func_params,
        ..
    } = api_dart_func;
    let func_params_str = compute_params_str(&func_params, MirFuncArgMode::Named);
    let func_expr = format!(
        "{func_return_type} {func_name}({func_params_str})",
        func_name = func.name_dart_wire(),
    );

    let call_handler = format!(
        "handler.{execute_func_name}({task_class}(
            callFfi: ({call_ffi_args}) {{
              {inner_func_stmt}
            }},
            codec: {codec},
            constMeta: {const_meta_field_name},
            argValues: [{arg_values}],
            apiImpl: this,
        ))",
    );
    let function_implementation_body = if let Some(return_stream) = &api_dart_func.return_stream {
        let wrapped_call_handler = match func.mode {
            MirFuncMode::Normal => {
                if func.stream_dart_await {
                    format!("await {call_handler}")
                } else {
                    format!("unawaited({call_handler})")
                }
            }
            MirFuncMode::Sync => call_handler.clone(),
        };

        format!(
            "
            final {return_stream_name} = {return_stream_type}();
            {wrapped_call_handler};
            return {return_stream_name}.stream;
            ",
            return_stream_name = return_stream.field.name.dart_style(),
            return_stream_type = ApiDartGenerator::new(
                return_stream.field.ty.clone(),
                context.as_api_dart_context()
            )
            .dart_api_type(),
        )
    } else {
        format!("return {call_handler};")
    };
    let function_implementation = format!(
        "@override {func_expr} {maybe_async} {{ {function_implementation_body} }}",
        maybe_async = if func.mode != MirFuncMode::Sync
            && api_dart_func.return_stream.is_some()
            && func.stream_dart_await
        {
            "async "
        } else {
            ""
        },
    );

    let companion_field_implementation = generate_companion_field(func, &const_meta_field_name);

    Ok(WireDartOutputCode {
        api_class_body: format!("{func_expr};\n\n"),
        api_impl_class_body: format!(
            "{function_implementation}\n\n{companion_field_implementation}\n\n"
        ),
        ..Default::default()
    })
}

fn generate_execute_func_name(func: &MirFunc) -> &str {
    match func.mode {
        MirFuncMode::Normal => "executeNormal",
        MirFuncMode::Sync => "executeSync",
    }
}

fn generate_task_class(func: &MirFunc) -> &str {
    match func.mode {
        MirFuncMode::Normal => "NormalTask",
        MirFuncMode::Sync => "SyncTask",
    }
}

fn generate_companion_field(func: &MirFunc, const_meta_field_name: &str) -> String {
    format!(
        r#"
        TaskConstMeta get {const_meta_field_name} => const TaskConstMeta(
            debugName: "{}",
            argNames: [{}],
        );
        "#,
        func.name,
        func.inputs
            .iter()
            .map(|input| format!("\"{}\"", input.inner.name.dart_style()))
            .collect_vec()
            .join(", "),
    )
}

fn generate_call_ffi_args(func: &MirFunc) -> &str {
    if func.mode == MirFuncMode::Sync {
        ""
    } else {
        "port_"
    }
}

fn generate_arg_values(func: &MirFunc) -> String {
    (func.inputs.iter())
        .map(|input| input.inner.name.dart_style())
        .join(", ")
}

fn generate_rust2dart_codec_object(func: &MirFunc) -> String {
    let codec_mode = func.codec_mode_pack.rust2dart;
    let codec_name_pascal = codec_mode.delegate_or_self().to_string();
    let codec_name_snake = codec_name_pascal.to_case(Case::Snake);

    let parse_success_data = format!(
        "{codec_name_snake}_decode_{}",
        func.output.normal.safe_ident()
    );
    let parse_error_data = if let Some(error_output) = &func.output.error {
        format!("{codec_name_snake}_decode_{}", error_output.safe_ident())
    } else {
        "null".to_string()
    };

    format!(
        "
        {codec_name_pascal}Codec(
          decodeSuccessData: {parse_success_data},
          decodeErrorData: {parse_error_data},
        )
        "
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
    use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
    use crate::codegen::ir::mir::func::{
        MirFuncImplMode, MirFuncInput, MirFuncOutput, MirFuncOwnerInfo,
    };
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::MirType;
    use crate::utils::namespace::Namespace;

    fn func(mode: MirFuncMode, error: Option<MirType>) -> MirFunc {
        MirFunc {
            namespace: Namespace::default(),
            name: MirIdent::new("calculate".into(), None),
            id: None,
            inputs: vec![MirFuncInput {
                ownership_mode: None,
                inner: MirField {
                    ty: MirType::Primitive(MirTypePrimitive::I32),
                    name: MirIdent::new("input_value".into(), None),
                    is_final: false,
                    is_rust_public: None,
                    comments: vec![],
                    default: None,
                    settings: MirFieldSettings::default(),
                },
                needs_extend_lifetime: false,
            }],
            output: MirFuncOutput {
                normal: MirType::Primitive(MirTypePrimitive::I32),
                error,
            },
            owner: MirFuncOwnerInfo::Function,
            mode,
            stream_dart_await: false,
            rust_async: false,
            initializer: false,
            init_dart_code: None,
            arg_mode: MirFuncArgMode::Named,
            accessor: None,
            comments: vec![],
            codec_mode_pack: CodecModePack {
                dart2rust: CodecMode::Cst,
                rust2dart: CodecMode::Cst,
            },
            rust_call_code: None,
            rust_aop_after: None,
            impl_mode: MirFuncImplMode::Normal,
            src_lineno_pseudo: 0,
        }
    }

    /// Selects task, FFI, argument, and codec output fragments from function metadata.
    #[test]
    fn api_impl_helpers_cover_modes_arguments_and_fallibility() {
        let normal = func(
            MirFuncMode::Normal,
            Some(MirType::Primitive(MirTypePrimitive::Bool)),
        );
        let sync = func(MirFuncMode::Sync, None);

        assert_eq!(generate_execute_func_name(&normal), "executeNormal");
        assert_eq!(generate_task_class(&normal), "NormalTask");
        assert_eq!(generate_call_ffi_args(&normal), "port_");
        assert_eq!(generate_arg_values(&normal), "inputValue");
        assert_eq!(generate_execute_func_name(&sync), "executeSync");
        assert_eq!(generate_task_class(&sync), "SyncTask");
        assert_eq!(generate_call_ffi_args(&sync), "");
        assert_eq!(
            generate_rust2dart_codec_object(&normal),
            "\n        CstCodec(\n          decodeSuccessData: cst_decode_i_32,\n          decodeErrorData: cst_decode_bool,\n        )\n        "
        );
        assert_eq!(
            generate_rust2dart_codec_object(&sync),
            "\n        CstCodec(\n          decodeSuccessData: cst_decode_i_32,\n          decodeErrorData: null,\n        )\n        "
        );
    }
}
