mod ty;
mod ty_boxed;
mod ty_delegate;
mod ty_enum;
mod ty_general_list;
mod ty_optional;
mod ty_primitive;
mod ty_primitive_list;
mod ty_struct;

pub use ty::*;
pub use ty_boxed::*;
pub use ty_delegate::*;
pub use ty_enum::*;
pub use ty_general_list::*;
pub use ty_optional::*;
pub use ty_primitive::*;
pub use ty_primitive_list::*;
pub use ty_struct::*;

use convert_case::{Case, Casing};
use log::debug;

use crate::ir::IrType::*;
use crate::ir::*;
use crate::others::*;

pub struct Output {
    pub file_prelude: DartBasicCode,
    pub decl_code: DartBasicCode,
    pub impl_code: DartBasicCode,
}

pub fn generate(
    api_file: &IrFile,
    dart_api_class_name: &str,
    dart_api_impl_class_name: &str,
    dart_wire_class_name: &str,
) -> Output {
    let distinct_types = api_file.distinct_types(true, true);
    let distinct_input_types = api_file.distinct_types(true, false);
    let distinct_output_types = api_file.distinct_types(false, true);
    debug!("distinct_input_types={:?}", distinct_input_types);
    debug!("distinct_output_types={:?}", distinct_output_types);

    let dart_func_signatures_and_implementations = api_file
        .funcs
        .iter()
        .map(generate_api_func)
        .collect::<Vec<_>>();
    let dart_structs = distinct_types
        .iter()
        .map(|ty| TODO.structs())
        .collect::<Vec<_>>();
    let dart_api2wire_funcs = distinct_input_types
        .iter()
        .map(generate_api2wire_func)
        .collect::<Vec<_>>();
    let dart_api_fill_to_wire_funcs = distinct_input_types
        .iter()
        .map(|ty| generate_api_fill_to_wire_func(ty, api_file))
        .collect::<Vec<_>>();
    let dart_wire2api_funcs = distinct_output_types
        .iter()
        .map(|ty| generate_wire2api_func(ty, api_file))
        .collect::<Vec<_>>();

    let needs_freezed = distinct_types
        .iter()
        .any(|ty| matches!(ty, EnumRef(e) if e.is_struct));
    let freezed_header = if needs_freezed {
        DartBasicCode {
            import: "import 'package:freezed_annotation/freezed_annotation.dart';".to_string(),
            part: "part 'bridge_generated.freezed.dart';".to_string(),
            body: "".to_string(),
        }
    } else {
        DartBasicCode::default()
    };

    let common_header = DartBasicCode {
        import: "import 'dart:convert';
            import 'dart:typed_data';"
            .to_string(),
        part: "".to_string(),
        body: "".to_string(),
    };

    let decl_body = format!(
        "abstract class {} {{
            {}
        }}

        {}
        ",
        dart_api_class_name,
        dart_func_signatures_and_implementations
            .iter()
            .map(|(sig, _, comm)| format!("{}{}", comm, sig))
            .collect::<Vec<_>>()
            .join("\n\n"),
        dart_structs.join("\n\n"),
    );

    let impl_body = format!(
        "class {} extends FlutterRustBridgeBase<{}> implements {} {{
            factory {}(ffi.DynamicLibrary dylib) => {}.raw({}(dylib));

            {}.raw({} inner) : super(inner);

            {}

            // Section: api2wire
            {}

            // Section: api_fill_to_wire
            {}
        }}

        // Section: wire2api
        {}
        ",
        dart_api_impl_class_name,
        dart_wire_class_name,
        dart_api_class_name,
        dart_api_impl_class_name,
        dart_api_impl_class_name,
        dart_wire_class_name,
        dart_api_impl_class_name,
        dart_wire_class_name,
        dart_func_signatures_and_implementations
            .iter()
            .map(|(_, imp, _)| imp.clone())
            .collect::<Vec<_>>()
            .join("\n\n"),
        dart_api2wire_funcs.join("\n\n"),
        dart_api_fill_to_wire_funcs.join("\n\n"),
        dart_wire2api_funcs.join("\n\n"),
    );

    let decl_code = &common_header
        + &freezed_header
        + &DartBasicCode {
            import: "".to_string(),
            part: "".to_string(),
            body: decl_body,
        };

    let impl_code = &common_header
        + &DartBasicCode {
            import: "import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';".to_string(),
            part: "".to_string(),
            body: impl_body,
        };

    let file_prelude = DartBasicCode {
        import: format!("{}
            
                // ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import
                ",
                        CODE_HEADER
        ),
        part: "".to_string(),
        body: "".to_string(),
    };

    Output {
        file_prelude,
        decl_code,
        impl_code,
    }
}

fn generate_api_func(func: &IrFunc) -> (String, String, String) {
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
                if let IrType::Primitive(IrTypePrimitive::Bool) = input.ty {
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
        IrFuncMode::Stream => "executeStream",
    };

    let signature = format!("{};", partial);

    let comments = dart_comments(&func.comments);

    let task_common_args = format!(
        "
        constMeta: const FlutterRustBridgeTaskConstMeta(
            debugName: \"{}\",
            argNames: [{}],
        ),
        argValues: [{}],
        hint: hint,
        ",
        func.name,
        func.inputs
            .iter()
            .map(|input| format!("\"{}\"", input.name.dart_style()))
            .collect::<Vec<_>>()
            .join(", "),
        func.inputs
            .iter()
            .map(|input| input.name.dart_style())
            .collect::<Vec<_>>()
            .join(", "),
    );

    let implementation = match func.mode {
        IrFuncMode::Sync => format!(
            "{} => {}(FlutterRustBridgeSyncTask(
            callFfi: () => inner.{}({}),
            {}
        ));",
            partial,
            execute_func_name,
            func.wire_func_name(),
            wire_param_list.join(", "),
            task_common_args,
        ),
        _ => format!(
            "{} => {}(FlutterRustBridgeTask(
            callFfi: (port_) => inner.{}({}),
            parseSuccessData: _wire2api_{},
            {}
        ));",
            partial,
            execute_func_name,
            func.wire_func_name(),
            wire_param_list.join(", "),
            func.output.safe_ident(),
            task_common_args,
        ),
    };

    (signature, implementation, comments)
}

fn generate_api2wire_func(ty: &IrType) -> String {
    let body = TODO.api2wire_body();

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
}

fn generate_api_fill_to_wire_func(ty: &IrType, api_file: &IrFile) -> String {
    let body = TODO.api_fill_to_wire_body();

    let target_wire_type = match ty {
        Optional(inner) => &inner.inner,
        it @ _ => it,
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
}

fn generate_wire2api_func(ty: &IrType, api_file: &IrFile) -> String {
    let body = TODO.wire2api_body();

    format!(
        "{} _wire2api_{}(dynamic raw) {{
            {}
        }}
        ",
        ty.dart_api_type(),
        ty.safe_ident(),
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
