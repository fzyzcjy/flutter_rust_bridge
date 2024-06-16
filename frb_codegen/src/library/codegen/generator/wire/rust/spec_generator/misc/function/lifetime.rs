use crate::codegen::ir::mir::func::{MirFunc, MirFuncInput};
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::MirType;
use itertools::Itertools;

pub(super) fn generate_code_inner_decode(func: &MirFunc, inner: &str) -> String {
    let interest_inputs = (func.inputs.iter())
        .filter(|field| is_interest_field(field))
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
                "api_{name}_guard",
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

fn is_interest_field(field: &MirFuncInput) -> bool {
    field.needs_extend_lifetime
}

fn get_variable_name(field: &MirFuncInput) -> String {
    field.inner.name.rust_style()
}

pub(crate) fn generate_inner_func_arg(raw: &str, field: &MirFuncInput) -> String {
    if is_interest_field(field) {
        format!("{raw}_illegal_static_ref")
    } else {
        raw.to_owned()
    }
}

pub(super) fn generate_code_postprocess_inner_output(func: &MirFunc) -> String {
    if !matches!(
        &func.output.normal,
        MirType::Delegate(MirTypeDelegate::Lifetimeable(_))
    ) {
        return "".to_owned();
    }

    let dependencies = (func.inputs.iter())
        .filter(|field| is_interest_field(field))
        .map(get_variable_name)
        .map(|field_name| {
            format!(
                "flutter_rust_bridge::for_generated::LifetimeableDependency::new_guard_lockable(
                    Box::new(api_{field_name}_guard.clone()),
                    Box::new(api_{field_name}.clone()),
                )"
            )
        })
        .join(", ");
    format!(
        "let output_ok = RustAutoOpaque::new(Lifetimeable::new(output_ok, vec![{dependencies}]));"
    )
}
