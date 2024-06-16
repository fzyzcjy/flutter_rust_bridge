use crate::codegen::ir::mir::func::{MirFunc, MirFuncInput};
use itertools::Itertools;

pub(super) fn generate_code_inner_decode(func: &MirFunc, inner: &str) -> String {
    let object_create_static_ref = (func.inputs.iter())
        .map(|field| {
            generate_illegal_static_reference(&format!(
                "api_{name}",
                name = get_variable_name(field)
            ))
        })
        .join("");

    format!("{object_create_static_ref}{inner}")
}

pub(super) fn generate_illegal_static_reference(var_name: &str) -> String {
    format!(
        "let {var_name}_illegal_static_reference = unsafe {{
            flutter_rust_bridge::for_generated::ouroboros_change_lifetime(&{var_name})
        }};"
    )
}

fn get_variable_name(field: &MirFuncInput) -> String {
    field.inner.name.rust_style()
}
