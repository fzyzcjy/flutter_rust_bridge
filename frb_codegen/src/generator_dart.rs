use convert_case::{Case, Casing};
use log::debug;

use crate::api_types::ApiType::*;
use crate::api_types::*;
use crate::others::*;

pub struct Output {
    pub header: String,
    pub api_class: String,
    pub other: String,
}

pub fn generate(
    api_file: &ApiFile,
    dart_api_class_name: &str,
    dart_api_impl_class_name: &str,
    dart_wire_class_name: &str,
) -> Output {
    let distinct_types = api_file.distinct_types();
    debug!("distinct_types={:?}", distinct_types);

    let dart_func_signatures_and_implementations = api_file
        .funcs
        .iter()
        .map(generate_api_func)
        .collect::<Vec<_>>();
    let dart_structs = distinct_types
        .iter()
        .filter_map(|ty| {
            if let StructRef(s) = ty {
                Some(s.get(api_file))
            } else {
                None
            }
        })
        .map(generate_api_struct)
        .collect::<Vec<_>>();
    let dart_api2wire_funcs = distinct_types
        .iter()
        .map(generate_api2wire_func)
        .collect::<Vec<_>>();
    let dart_api_fill_to_wire_funcs = distinct_types
        .iter()
        .map(|ty| generate_api_fill_to_wire_func(ty, api_file))
        .collect::<Vec<_>>();
    let dart_wire2api_funcs = distinct_types
        .iter()
        .map(|ty| generate_wire2api_func(ty, api_file))
        .collect::<Vec<_>>();

    let header = format!(
        "{}

        // ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas
        import 'dart:convert';
        import 'dart:typed_data';

        import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';",
        CODE_HEADER,
    );

    let api_class = format!(
        "abstract class {} extends FlutterRustBridgeBase<{}> {{
            factory {}(ffi.DynamicLibrary dylib) => {}.raw({}(dylib));

            {}.raw({} inner) : super(inner);

            {}
        }}

        {}

        // ------------------------- Implementation Details -------------------------
        ",
        dart_api_class_name,
        dart_wire_class_name,
        dart_api_class_name,
        dart_api_impl_class_name,
        dart_wire_class_name,
        dart_api_class_name,
        dart_wire_class_name,
        dart_func_signatures_and_implementations
            .iter()
            .map(|(sig, _)| sig.clone())
            .collect::<Vec<_>>()
            .join("\n\n"),
        dart_structs.join("\n\n"),
    );

    let other = format!(
        "/// Implementations for {}. Prefer using {} if possible; but this class allows more 
        /// flexible customizations (such as subclassing to create an initializer, a logger, or 
        /// a timer).
        class {} extends {} {{
            {}.raw({} inner) : super.raw(inner);

            {}

            // Section: api2wire
            {}

            // Section: api_fill_to_wire
            {}
        }}

        // Section: wire2api
        {}
        ",
        dart_api_class_name,
        dart_api_class_name,
        dart_api_impl_class_name,
        dart_api_class_name,
        dart_api_impl_class_name,
        dart_wire_class_name,
        dart_func_signatures_and_implementations
            .iter()
            .map(|(_, imp)| imp.clone())
            .collect::<Vec<_>>()
            .join("\n\n"),
        dart_api2wire_funcs.join("\n\n"),
        dart_api_fill_to_wire_funcs.join("\n\n"),
        dart_wire2api_funcs.join("\n\n"),
    );

    Output {
        header,
        api_class,
        other,
    }
}

fn generate_api_func(func: &ApiFunc) -> (String, String) {
    let raw_func_param_list = func
        .inputs
        .iter()
        .map(|input| {
            format!(
                "required {} {}",
                input.ty.dart_api_type(),
                input.name.dart_style()
            )
        })
        .collect::<Vec<_>>();

    let full_func_param_list = [raw_func_param_list, vec!["dynamic hint".to_string()]].concat();

    let wire_param_list = func
        .inputs
        .iter()
        .map(|input| {
            format!(
                "_api2wire_{}({})",
                &input.ty.safe_ident(),
                &input.name.dart_style()
            )
        })
        .collect::<Vec<_>>();

    let partial = format!(
        "{}<{}> {}({{ {} }})",
        func.mode.dart_return_type(),
        func.output.dart_api_type(),
        func.name.to_case(Case::Camel),
        full_func_param_list.join(","),
    );

    let execute_func_name = match func.mode {
        ApiFuncMode::Normal => "executeNormal",
        ApiFuncMode::Stream => "executeStream",
    };

    let signature = format!("{};", partial);

    let implementation = format!(
        "{} => {}(FlutterRustBridgeTask(
            debugName: '{}',
            callFfi: (port) => inner.{}(port, {}),
            parseSuccessData: _wire2api_{},
            hint: hint
        ));",
        partial,
        execute_func_name,
        func.name,
        func.wire_func_name(),
        wire_param_list.join(", "),
        func.output.safe_ident(),
    );

    (signature, implementation)
}

fn generate_api2wire_func(ty: &ApiType) -> String {
    let body = match ty {
        Primitive(_) => "return raw;".to_string(),
        Delegate(d) => match d {
            ApiTypeDelegate::String => {
                "return _api2wire_uint_8_list(utf8.encoder.convert(raw));".to_string()
            }
            ApiTypeDelegate::ZeroCopyBufferVecU8 => {
                "return _api2wire_uint_8_list(raw);".to_string()
            }
        },
        PrimitiveList(_) => {
            // NOTE Dart code *only* allocates memory. It never *release* memory by itself.
            // Instead, Rust receives that pointer and now it is in control of Rust.
            // Therefore, *never* continue to use this pointer after you have passed the pointer
            // to Rust.
            // NOTE WARN: Never use the [calloc] provided by Dart FFI to allocate any memory.
            // Instead, ask Rust to allocate some memory and return raw pointers. Otherwise,
            // memory will be allocated in one dylib (e.g. libflutter.so), and then be released
            // by another dylib (e.g. my_rust_code.so), especially in Android platform. It can be
            // undefined behavior.
            format!(
                "final ans = inner.new_{}(raw.length);
                ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
                return ans;",
                ty.safe_ident(),
            )
        }
        GeneralList(list) => {
            // NOTE 内存策略同PrimitiveList（比如Uint8List之类），见那边的注释
            format!(
                "final ans = inner.new_{}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    _api_fill_to_wire_{}(raw[i], ans.ref.ptr[i]);
                }}
                return ans;",
                ty.safe_ident(),
                list.inner.safe_ident(),
            )
        }
        Boxed(b) => {
            format!(
                "final ptr = inner.new_{}();
                _api_fill_to_wire_{}(raw, ptr.ref);
                return ptr;",
                ty.safe_ident(),
                b.inner.safe_ident(),
            )
        }
        // skip
        StructRef(_) => return "".to_string(),
        Optional(opt) => {
            let get_ptr = if let StructRef(inner) = &opt.inner {
                format!(
                    "final ptr = inner.new_{}();
                    _api_fill_to_wire_{}(raw, ptr.ref);
                    return ptr;",
                    ty.safe_ident(),
                    inner.safe_ident()
                )
            } else {
                format!("return _api2wire_{}(raw);", opt.inner.safe_ident())
            };
            format!("if (raw == null) return ffi.nullptr; {}", get_ptr)
        }
    };

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

fn generate_api_fill_to_wire_func(ty: &ApiType, api_file: &ApiFile) -> String {
    let body = match ty {
        StructRef(s) => s
            .get(api_file)
            .fields
            .iter()
            .map(|field| {
                format!(
                    "wireObj.{} = _api2wire_{}(apiObj.{});",
                    field.name.rust_style(),
                    field.ty.safe_ident(),
                    field.name.dart_style()
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        // skip
        Primitive(_) | Delegate(_) | PrimitiveList(_) | GeneralList(_) | Boxed(_) | Optional(_) => {
            return "".to_string();
        }
    };

    format!(
        "void _api_fill_to_wire_{}({} apiObj, {} wireObj) {{
            {}
        }}",
        ty.safe_ident(),
        ty.dart_api_type(),
        ty.dart_wire_type(),
        body,
    )
}

fn generate_wire2api_func(ty: &ApiType, api_file: &ApiFile) -> String {
    let gen_simple_type_cast = |s: &str| format!("return raw as {};", s);

    let body = match ty {
        Primitive(p) => gen_simple_type_cast(&p.dart_api_type()),
        Delegate(d) => match d {
            ApiTypeDelegate::String => gen_simple_type_cast(&d.dart_api_type()),
            ApiTypeDelegate::ZeroCopyBufferVecU8 => gen_simple_type_cast(&d.dart_api_type()),
        },
        PrimitiveList(list) => gen_simple_type_cast(&list.dart_api_type()),
        GeneralList(list) => format!(
            "return (raw as List<dynamic>).map((item) => _wire2api_{}(item)).toList();",
            list.inner.safe_ident()
        ),
        StructRef(s_ref) => {
            let s = s_ref.get(api_file);
            let inner = s
                .fields
                .iter()
                .enumerate()
                .map(|(idx, field)| {
                    format!(
                        "{}: _wire2api_{}(arr[{}]),",
                        field.name.dart_style(),
                        field.ty.safe_ident(),
                        idx
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");

            format!(
                "final arr = raw as List<dynamic>;
                if (arr.length != {}) throw Exception('unexpected arr length: expect {} but see ${{arr.length}}');
                return {}({});",
                s.fields.len(),
                s.fields.len(),
                s.name, inner,
            )
        }
        Boxed(_) => return "".to_string(),
        Optional(opt) => format!(
            "return raw == null ? null : _wire2api_{}(raw);",
            opt.inner.safe_ident()
        ),
    };

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

fn generate_api_struct(s: &ApiStruct) -> String {
    let field_declarations = s
        .fields
        .iter()
        .map(|f| format!("final {} {};", f.ty.dart_api_type(), f.name.dart_style()))
        .collect::<Vec<_>>()
        .join("\n");

    let constructor_params = s
        .fields
        .iter()
        .map(|f| {
            format!(
                "{}this.{},",
                if f.required { "required " } else { "" },
                f.name.dart_style()
            )
        })
        .collect::<Vec<_>>()
        .join("");

    format!(
        "class {} {{
            {}

            {}({{{}}});
        }}",
        s.name, field_declarations, s.name, constructor_params
    )
}
