use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::dart::spec_generator::wire_class::io::common::generate_wire_class_header;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::utils::basic_code::dart_header_code::DartHeaderCode;
use itertools::Itertools;

pub(crate) fn generate(
    config: &GeneratorWireDartInternalConfig,
    rust_extern_funcs: &[ExternFunc],
) -> anyhow::Result<WireDartOutputCode> {
    let wire_class_name = &config.dart_output_class_name_pack.wire_class_name;
    let wire_class_header = generate_wire_class_header(wire_class_name);
    let class_body = (rust_extern_funcs.iter())
        .filter(|f| f.target == Target::Io)
        .map(|f| generate_func(f, &config.c_symbol_prefix))
        .join("");

    let body = format!(
        "
        {wire_class_header}
            /// Holds the symbol lookup function.
            final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
                _lookup;
  
            /// The symbols are looked up in [dynamicLibrary].
            {wire_class_name}(ffi.DynamicLibrary dynamicLibrary)
                : _lookup = dynamicLibrary.lookup;

            {class_body}
        }}
        "
    );
    Ok(WireDartOutputCode {
        header: DartHeaderCode {
            import: "import 'dart:ffi' as ffi;\n".to_owned(),
            ..Default::default()
        },
        body,
        ..Default::default()
    })
}

fn generate_func(func: &ExternFunc, c_symbol_prefix: &str) -> String {
    // Only know how to generate this currently
    if !func.needs_ffigen
        && func.return_type.is_none()
        && func.params.len() == 1
        && func.params[0]
            == (ExternFuncParam {
                name: "ptr".to_string(),
                rust_type: "*const std::ffi::c_void".to_owned(),
                dart_type: "int".to_string(),
            })
    {
        let name = &func.partial_func_name;
        format!(
            "
            void {name}(
              ffi.Pointer<ffi.Void> ptr,
            ) {{
              return _{name}(
                ptr,
              );
            }}

            late final _{name}Ptr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>('{c_symbol_prefix}{name}');
            late final _{name} = _{name}Ptr.asFunction<void Function(ffi.Pointer<ffi.Void>)>();
            "
        )
    } else {
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        unreachable!(
            "Do not understand how to generate this func without ffigen yet (func={func:?})"
        )
        // frb-coverage:ignore-end
    }
}
