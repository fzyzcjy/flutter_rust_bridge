use crate::codegen::ir::func::{IrFunc, IrFuncInput, OwnershipMode};
use crate::codegen::ir::ty::rust_auto_opaque::IrTypeRustAutoOpaque;
use crate::codegen::ir::ty::IrType;
use convert_case::{Case, Casing};
use itertools::Itertools;

pub(crate) fn generate_code_inner_decode(func: &IrFunc) -> String {
    let interest_fields = filter_interest_fields(func);
    if interest_fields.is_empty() {
        return "".to_owned();
    }

    let declarations = (interest_fields.iter())
        .map(|(field, _ty)| {
            format!(
                "let mut api_{name}_decoded = None;\n",
                name = get_variable_name(field)
            )
        })
        .join("");

    let var_orders = (interest_fields.iter().enumerate())
        .map(|(index, (field, ty))| {
            let mutable = (ty.ownership_mode == OwnershipMode::RefMut).to_string();
            format!(
                "api_{name}.rust_auto_opaque_lock_order_info({index}, {mutable})",
                name = get_variable_name(field)
            )
        })
        .join(", ");

    let match_arms = (interest_fields.iter().enumerate())
        .map(|(index, (field, ty))| {
            format!("{index} => {},", generate_decode_statement(func, field, ty))
        })
        .join("\n");

    let unwraps = (interest_fields.iter())
        .map(|(field, ty)| {
            let mutability = if ty.ownership_mode == OwnershipMode::RefMut {
                "mut "
            } else {
                ""
            };
            format!(
                "let {mutability}api_{name} = api_{name}_decoded.unwrap();\n",
                name = get_variable_name(field)
            )
        })
        .join("");

    format!(
        "{declarations}let decode_indices_ = flutter_rust_bridge::for_generated::rust_auto_opaque_decode_compute_order(vec![{var_orders}]);
        for i in decode_indices_ {{
            match i {{
                {match_arms}
                _ => unreachable!(),
            }}
        }}
        {unwraps}"
    )
}

fn generate_decode_statement(
    func: &IrFunc,
    field: &IrFuncInput,
    ty: &IrTypeRustAutoOpaque,
) -> String {
    let mode = ty.ownership_mode.to_string().to_case(Case::Snake);
    format!(
        "api_{name}_decoded = Some(api_{name}.rust_auto_opaque_decode_{syncness}_{mode}(){maybe_await})",
        name = get_variable_name(field),
        syncness = if func.rust_async { "async" } else { "sync" },
        maybe_await = if func.rust_async { ".await" } else { "" },
    )
}

fn filter_interest_fields(func: &IrFunc) -> Vec<(&IrFuncInput, &IrTypeRustAutoOpaque)> {
    (func.inputs.iter())
        .filter_map(|field| {
            if let IrType::RustAutoOpaque(ty) = &field.inner.ty {
                if ty.ownership_mode != OwnershipMode::Owned {
                    Some((field, ty))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect_vec()
}

fn get_variable_name(field: &IrFuncInput) -> &str {
    field.inner.name.rust_style()
}
