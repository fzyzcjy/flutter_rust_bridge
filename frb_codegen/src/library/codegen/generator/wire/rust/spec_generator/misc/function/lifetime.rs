use crate::codegen::ir::mir::func::{MirFunc, MirFuncInput};
use itertools::Itertools;

pub(super) fn generate_code_inner_decode(func: &MirFunc, inner: &str) -> String {
    let interest_inputs = (func.inputs.iter())
        .filter(|field| field.needs_extend_lifetime)
        .collect_vec();

    let object_static_ref = (interest_inputs.iter())
        .map(|field| {
            generate_illegal_static_reference(&format!(
                "api_{name}",
                name = get_variable_name(field)
            ))
        })
        .join("");

    let guard_static_ref = (interest_inputs.iter())
        .map(|field| {
            let static_ref = generate_illegal_static_reference(&format!(
                "api_guard_{name}",
                name = get_variable_name(field)
            ));
            format!(
                "let api_{name}_guard = Arc::new(api_{name}_guard);
                {static_ref}",
                name = get_variable_name(field)
            )
        })
        .join("");

    format!("{object_static_ref}{inner}{guard_static_ref}")
}

pub(super) fn generate_illegal_static_reference(var_name: &str) -> String {
    format!(
        "let {var_name}_illegal_static_ref = unsafe {{
            flutter_rust_bridge::for_generated::ouroboros_change_lifetime(&{var_name})
        }};"
    )
}

fn get_variable_name(field: &MirFuncInput) -> String {
    field.inner.name.rust_style()
}
