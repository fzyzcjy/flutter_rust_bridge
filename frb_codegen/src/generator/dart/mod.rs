mod ty;
mod ty_boxed;
mod ty_delegate;
mod ty_enum;
mod ty_general_list;
mod ty_optional;
mod ty_primitive;
mod ty_primitive_list;
mod ty_struct;
mod ty_sync_return;

use std::collections::HashSet;

pub use ty::*;
pub use ty_boxed::*;
pub use ty_delegate::*;
pub use ty_enum::*;
pub use ty_general_list::*;
pub use ty_optional::*;
pub use ty_primitive::*;
pub use ty_primitive_list::*;
pub use ty_struct::*;
pub use ty_sync_return::*;

use convert_case::{Case, Casing};
use log::debug;

use crate::ir::IrType::*;
use crate::ir::*;
use crate::method_utils::{FunctionName, MethodNamingUtil};
use crate::others::*;
use crate::utils::BlockIndex;

pub struct Output {
    pub file_prelude: DartBasicCode,
    pub decl_code: DartBasicCode,
    pub impl_code: DartBasicCode,
}

pub fn generate(
    ir_file: &IrFile,
    dart_api_class_name: &str,
    dart_api_impl_class_name: &str,
    dart_wire_class_name: &str,
    dart_output_file_root: &str,
    block_index: BlockIndex,
) -> (Output, bool) {
    let DartApiSpec {
        dart_funcs,
        dart_structs,
        dart_api2wire_funcs,
        dart_api_fill_to_wire_funcs,
        dart_wire2api_funcs,
        needs_freezed,
    } = get_dart_api_spec_from_ir_file(ir_file, block_index, dart_api_class_name);
    let common_header = generate_common_header();

    let decl_code = generate_dart_declaration_code(
        &common_header,
        generate_freezed_header(dart_output_file_root, needs_freezed),
        generate_import_header(get_dart_imports(ir_file)),
        generate_dart_declaration_body(dart_api_class_name, &dart_funcs, &dart_structs),
    );

    let impl_code = generate_dart_implementation_code(
        &common_header,
        generate_dart_implementation_body(
            &dart_funcs,
            &dart_api2wire_funcs,
            &dart_api_fill_to_wire_funcs,
            &dart_wire2api_funcs,
            dart_api_impl_class_name,
            dart_wire_class_name,
            dart_api_class_name,
        ),
    );

    let file_prelude = generate_file_prelude();

    (
        Output {
            file_prelude,
            decl_code,
            impl_code,
        },
        needs_freezed,
    )
}

struct DartApiSpec {
    dart_funcs: Vec<GeneratedApiFunc>,
    dart_structs: Vec<String>,
    dart_api2wire_funcs: Vec<String>,
    dart_api_fill_to_wire_funcs: Vec<String>,
    dart_wire2api_funcs: Vec<String>,
    needs_freezed: bool,
}

fn get_dart_api_spec_from_ir_file(
    ir_file: &IrFile,
    block_index: BlockIndex,
    dart_api_class_name: &str,
) -> DartApiSpec {
    let distinct_types = ir_file.distinct_types(true, true);
    let distinct_input_types = ir_file.distinct_types(true, false);
    let distinct_output_types = ir_file.distinct_types(false, true);
    debug!("distinct_input_types={:?}", distinct_input_types);
    debug!("distinct_output_types={:?}", distinct_output_types);

    let dart_funcs = ir_file
        .funcs
        .iter()
        .map(|f| generate_api_func(f, ir_file))
        .collect::<Vec<_>>();
    let dart_structs = distinct_types
        .iter()
        .map(|ty| {
            TypeDartGenerator::new(ty.clone(), ir_file, Some(dart_api_class_name.to_string()))
                .structs()
        })
        .collect::<Vec<_>>();

    let dart_api2wire_funcs = distinct_input_types
        .iter()
        .map(|ty| generate_api2wire_func(ty, ir_file, block_index))
        .collect::<Vec<_>>();
    let dart_api_fill_to_wire_funcs = distinct_input_types
        .iter()
        .map(|ty| generate_api_fill_to_wire_func(ty, ir_file))
        .collect::<Vec<_>>();
    let dart_wire2api_funcs = distinct_output_types
        .iter()
        .map(|ty| generate_wire2api_func(ty, ir_file, dart_api_class_name))
        .collect::<Vec<_>>();

    let needs_freezed = distinct_types.iter().any(|ty| match ty {
        EnumRef(_) => true,
        StructRef(st) if st.freezed => true,
        _ => false,
    });

    DartApiSpec {
        dart_funcs,
        dart_structs,
        dart_api2wire_funcs,
        dart_api_fill_to_wire_funcs,
        dart_wire2api_funcs,
        needs_freezed,
    }
}

fn generate_freezed_header(dart_output_file_root: &str, needs_freezed: bool) -> DartBasicCode {
    if needs_freezed {
        DartBasicCode {
            import: "import 'package:freezed_annotation/freezed_annotation.dart';".to_string(),
            part: format!("part '{}.freezed.dart';", dart_output_file_root),
            body: "".to_string(),
        }
    } else {
        DartBasicCode::default()
    }
}

fn generate_import_header(imports: HashSet<&IrDartImport>) -> DartBasicCode {
    if !imports.is_empty() {
        DartBasicCode {
            import: imports
                .iter()
                .map(|it| match &it.alias {
                    Some(alias) => format!("import '{}' as {};", it.uri, alias),
                    _ => format!("import '{}';", it.uri),
                })
                .collect::<Vec<_>>()
                .join("\n"),
            part: "".to_string(),
            body: "".to_string(),
        }
    } else {
        DartBasicCode::default()
    }
}

fn generate_common_header() -> DartBasicCode {
    DartBasicCode {
        import: "import 'dart:convert';
            import 'dart:typed_data';"
            .to_string(),
        part: "".to_string(),
        body: "".to_string(),
    }
}

fn get_dart_imports(ir_file: &IrFile) -> HashSet<&IrDartImport> {
    ir_file
        .struct_pool
        .values()
        .flat_map(|s| s.dart_metadata.iter().flat_map(|it| &it.library))
        .collect()
}

fn generate_dart_declaration_body(
    dart_api_class_name: &str,
    dart_funcs: &[GeneratedApiFunc],
    dart_structs: &[String],
) -> String {
    format!(
        "abstract class {} {{
            {}
        }}

        {}
        ",
        dart_api_class_name,
        dart_funcs
            .iter()
            .map(|func| format!(
                "{}{}\n\n{}",
                func.comments, func.signature, func.companion_field_signature,
            ))
            .collect::<Vec<_>>()
            .join("\n\n"),
        dart_structs.join("\n\n"),
    )
}

fn generate_dart_implementation_body(
    dart_funcs: &[GeneratedApiFunc],
    dart_api2wire_funcs: &[String],
    dart_api_fill_to_wire_funcs: &[String],
    dart_wire2api_funcs: &[String],
    dart_api_impl_class_name: &str,
    dart_wire_class_name: &str,
    dart_api_class_name: &str,
) -> String {
    format!(
        "class {dart_api_impl_class_name} extends FlutterRustBridgeBase<{dart_wire_class_name}> implements {dart_api_class_name} {{
            factory {dart_api_impl_class_name}(ffi.DynamicLibrary dylib) => {dart_api_impl_class_name}.raw({dart_wire_class_name}(dylib));

            {dart_api_impl_class_name}.raw({dart_wire_class_name} inner) : super(inner);

            {}

            // Section: api2wire
            {}

            // Section: api_fill_to_wire
            {}
        }}

        // Section: wire2api
        {}
        ",
        dart_funcs
            .iter()
            .map(|func| format!(
                "{}\n\n{}",
                func.implementation, func.companion_field_implementation,
            ))
            .collect::<Vec<_>>()
            .join("\n\n"),
        dart_api2wire_funcs.join("\n\n"),
        dart_api_fill_to_wire_funcs.join("\n\n"),
        dart_wire2api_funcs.join("\n\n"),
        dart_api_impl_class_name = dart_api_impl_class_name,
        dart_wire_class_name = dart_wire_class_name,
        dart_api_class_name = dart_api_class_name,
    )
}

fn generate_dart_declaration_code(
    common_header: &DartBasicCode,
    freezed_header: DartBasicCode,
    import_header: DartBasicCode,
    declaration_body: String,
) -> DartBasicCode {
    common_header
        + &freezed_header
        + &import_header
        + &DartBasicCode {
            import: "".to_string(),
            part: "".to_string(),
            body: declaration_body,
        }
}

fn generate_dart_implementation_code(
    common_header: &DartBasicCode,
    implementation_body: String,
) -> DartBasicCode {
    common_header
        + &DartBasicCode {
            import: "import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';".to_string(),
            part: "".to_string(),
            body: implementation_body,
        }
}

fn generate_file_prelude() -> DartBasicCode {
    DartBasicCode {
        import: format!("{}

                // ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports
                ",
                        CODE_HEADER
        ),
        part: "".to_string(),
        body: "".to_string(),
    }
}

#[derive(Debug)]
struct GeneratedApiFunc {
    signature: String,
    implementation: String,
    comments: String,
    companion_field_signature: String,
    companion_field_implementation: String,
}

#[derive(Debug)]
struct GeneratedApiMethod {
    signature: String,
    implementation: String,
}

fn generate_api_func(func: &IrFunc, ir_file: &IrFile) -> GeneratedApiFunc {
    let raw_func_param_list = func
        .inputs
        .iter()
        .map(|input| {
            format!(
                "{}{} {}",
                input.ty.dart_required_modifier(),
                input.ty.dart_api_type(),
                input.name.dart_style()
            )
        })
        .collect::<Vec<_>>();
    let full_func_param_list = [raw_func_param_list, vec!["dynamic hint".to_string()]].concat();

    let wire_param_list = [
        if func.mode.has_port_argument() {
            vec!["port_".to_string()]
        } else {
            vec![]
        },
        func.inputs
            .iter()
            .map(|input| {
                // edge case: ffigen performs its own bool-to-int conversions
                if let Primitive(IrTypePrimitive::Bool) = input.ty {
                    input.name.dart_style()
                } else {
                    format!(
                        "_api2wire_{}({})",
                        &input.ty.safe_ident(),
                        &input.name.dart_style()
                    )
                }
            })
            .collect::<Vec<_>>(),
    ]
    .concat();

    let partial = format!(
        "{} {}({{ {} }})",
        func.mode.dart_return_type(&func.output.dart_api_type()),
        func.name.to_case(Case::Camel),
        full_func_param_list.join(","),
    );

    let execute_func_name = match func.mode {
        IrFuncMode::Normal => "executeNormal",
        IrFuncMode::Sync => "executeSync",
        IrFuncMode::Stream { .. } => "executeStream",
    };

    let const_meta_field_name = format!("k{}ConstMeta", func.name.to_case(Case::Pascal));

    let signature = format!("{};", partial);

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
    let parse_sucess_data = if (f.is_static_method()
        && f.struct_name().unwrap() == {
            if let IrType::StructRef(IrTypeStructRef { name, freezed: _ }) = &func.output {
                name.clone()
            } else {
                "".to_string()
            }
        })
        // If struct has a method with first element `input0`
        || (input_0_struct_name.is_some() && MethodNamingUtil::has_methods(input_0_struct_name.unwrap(), ir_file))
        //If output is a struct with methods
        || (func_output_struct_name.is_some()
            && MethodNamingUtil::has_methods(func_output_struct_name.unwrap(), ir_file))
    {
        format!("(d) => _wire2api_{}(this, d)", func.output.safe_ident())
    } else {
        format!("_wire2api_{}", func.output.safe_ident())
    };

    let implementation = match func.mode {
        IrFuncMode::Sync => format!(
            "{} => {}(FlutterRustBridgeSyncTask(
            callFfi: () => inner.{}({}),
            parseSuccessData: {},
            {}
        ));",
            partial,
            execute_func_name,
            func.wire_func_name(),
            wire_param_list.join(", "),
            parse_sucess_data,
            task_common_args,
        ),
        _ => format!(
            "{} => {}(FlutterRustBridgeTask(
            callFfi: (port_) => inner.{}({}),
            parseSuccessData: {},
            {}
        ));",
            partial,
            execute_func_name,
            func.wire_func_name(),
            wire_param_list.join(", "),
            parse_sucess_data,
            task_common_args,
        ),
    };

    let companion_field_signature = format!(
        "FlutterRustBridgeTaskConstMeta get {};",
        const_meta_field_name,
    );

    let companion_field_implementation = format!(
        "
        FlutterRustBridgeTaskConstMeta get {} => const FlutterRustBridgeTaskConstMeta(
            debugName: \"{}\",
            argNames: [{}],
        );
        ",
        const_meta_field_name,
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

fn generate_api2wire_func(ty: &IrType, ir_file: &IrFile, block_index: BlockIndex) -> String {
    if let Some(body) = TypeDartGenerator::new(ty.clone(), ir_file, None).api2wire_body(block_index)
    {
        format!(
            "{} _api2wire_{}({} raw) {{
            {}
        }}
        ",
            ty.dart_wire_type(),
            ty.safe_ident(),
            ty.dart_api_type(),
            body,
        )
    } else {
        "".to_string()
    }
}

fn generate_api_fill_to_wire_func(ty: &IrType, ir_file: &IrFile) -> String {
    if let Some(body) = TypeDartGenerator::new(ty.clone(), ir_file, None).api_fill_to_wire_body() {
        let target_wire_type = match ty {
            Optional(inner) => &inner.inner,
            it => it,
        };

        format!(
            "void _api_fill_to_wire_{}({} apiObj, {} wireObj) {{
            {}
        }}",
            ty.safe_ident(),
            ty.dart_api_type(),
            target_wire_type.dart_wire_type(),
            body,
        )
    } else {
        "".to_string()
    }
}

fn generate_wire2api_func(ty: &IrType, ir_file: &IrFile, dart_api_class_name: &str) -> String {
    let extra_argument = if matches!(ty, StructRef(IrTypeStructRef { name, freezed: _ }) if MethodNamingUtil::has_methods(name, ir_file))
    {
        format!("{} bridge,", dart_api_class_name)
    } else {
        "".to_string()
    };
    let body = TypeDartGenerator::new(ty.clone(), ir_file, None).wire2api_body();
    format!(
        "{} _wire2api_{}({}{} raw) {{
            {}
        }}
        ",
        ty.dart_api_type(),
        ty.safe_ident(),
        extra_argument,
        ty.dart_param_type(),
        body,
    )
}

fn gen_wire2api_simple_type_cast(s: &str) -> String {
    format!("return raw as {};", s)
}

/// A trailing newline is included if comments is not empty.
fn dart_comments(comments: &[IrComment]) -> String {
    let mut comments = comments
        .iter()
        .map(IrComment::comment)
        .collect::<Vec<_>>()
        .join("\n");
    if !comments.is_empty() {
        comments.push('\n');
    }
    comments
}

fn dart_metadata(metadata: &[IrDartAnnotation]) -> String {
    let mut metadata = metadata
        .iter()
        .map(|it| match &it.library {
            Some(IrDartImport {
                alias: Some(alias), ..
            }) => format!("@{}.{}", alias, it.content),
            _ => format!("@{}", it.content),
        })
        .collect::<Vec<_>>()
        .join("\n");
    if !metadata.is_empty() {
        metadata.push('\n');
    }
    metadata
}
