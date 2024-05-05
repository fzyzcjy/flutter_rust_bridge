use crate::codegen::ir::func::OwnershipMode;
use crate::codegen::ir::ty::IrType;
use convert_case::Case;

pub(crate) fn generate_code_inner_decode() -> String {
    func.inputs
        .iter()
        .filter_map(|field| {
            if let IrType::RustAutoOpaque(o) = &field.inner.ty {
                if o.ownership_mode != OwnershipMode::Owned {
                    let mode = o.ownership_mode.to_string().to_case(Case::Snake);
                    let mutability = if o.ownership_mode == OwnershipMode::RefMut {
                        "mut "
                    } else {
                        ""
                    };
                    Some(format!(
                        "let {mutability}api_{name} = api_{name}.rust_auto_opaque_decode_{syncness}_{mode}(){maybe_await};\n",
                        name = field.inner.name.rust_style(),
                        syncness = if func.rust_async { "async" } else { "sync" },
                        maybe_await = if func.rust_async { ".await" } else { "" },
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .join("")
}
