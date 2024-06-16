use crate::codegen::ir::mir::func::MirFunc;

pub(super) fn generate_code_inner_decode(func: &MirFunc, inner: &str) -> String {

}

pub(super) fn generate_illegal_static_reference(var_name: &str) -> String {
    format!(
        "let {var_name}_illegal_static_reference = unsafe {{
            flutter_rust_bridge::for_generated::ouroboros_change_lifetime(&{var_name})
        }};"
    )
}
