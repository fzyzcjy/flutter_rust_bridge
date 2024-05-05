use crate::codegen::ir::func::{IrFunc, IrFuncInput, OwnershipMode};
use crate::codegen::ir::ty::rust_auto_opaque::IrTypeRustAutoOpaque;
use crate::codegen::ir::ty::IrType;
use convert_case::{Case, Casing};
use itertools::Itertools;

pub(crate) fn generate_code_inner_decode(func: &IrFunc) -> String {
    let interest_fields = filter_interest_fields(func);
    TODO
}

fn generate_decode_statement(field: &IrFuncInput) -> Option<String> {
    let mode = o.ownership_mode.to_string().to_case(Case::Snake);
    let mutability = if o.ownership_mode == OwnershipMode::RefMut {
        "mut "
    } else {
        ""
    };
    format!(
        "let {mutability}api_{name} = api_{name}.rust_auto_opaque_decode_{syncness}_{mode}(){maybe_await};\n",
        name = field.inner.name.rust_style(),
        syncness = if func.rust_async { "async" } else { "sync" },
        maybe_await = if func.rust_async { ".await" } else { "" },
    )
}

fn filter_interest_fields(func: &IrFunc) -> Vec<(&IrFuncInput, &IrTypeRustAutoOpaque)> {
    (func.inputs.iter())
        .filter_map(|field| {
            if let IrType::RustAutoOpaque(o) = &field.inner.ty {
                if o.ownership_mode != OwnershipMode::Owned {
                    Some((field, o))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect_vec()
}
