use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::dart::spec_generator::wire_class::io::common::generate_wire_class_header;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use itertools::Itertools;

pub(crate) fn generate(
    config: &GeneratorWireDartInternalConfig,
    rust_extern_funcs: &[ExternFunc],
) -> anyhow::Result<WireDartOutputCode> {
    let wire_class_header =
        generate_wire_class_header(&config.dart_output_class_name_pack.wire_class_name);
    let body = rust_extern_funcs.iter().map(|f| generate_func(f)).join("");

    let code = format!(
        "
        {wire_class_header}
        {body}
        }}
        "
    );
    Ok(code.into())
}

fn generate_func(func: &ExternFunc) -> String {
    // Only know how to generate this currently
    if !func.needs_ffigen
        && func.return_type.is_none()
        && func.params.len() == 1
        && func.params[0].rust_type == "*const std::ffi::c_void"
    {
        let name = &func.params[0].name;
        format!(
            "
            void {name}(
              ffi.Pointer<ffi.Void> ptr,
            ) {{
              return _{name}(
                ptr,
              );
            }}

            late final _{name}Ptr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>('{name}');
            late final _{name} = _{name}Ptr.asFunction<void Function(ffi.Pointer<ffi.Void>)>();
            "
        )
    } else {
        unreachable!(
            "Do not understand how to generate this func without ffigen yet (func={func:?})"
        )
    }
}
