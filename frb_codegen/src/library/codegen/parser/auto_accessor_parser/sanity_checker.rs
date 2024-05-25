use crate::codegen::ir::field::IrField;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::IrType;
use log::warn;

pub(super) fn check_field(
    struct_name: &NamespacedName,
    field: &IrField,
) -> Option<SanityCheckHint> {
    does_need_hint(&field.ty).then(|| SanityCheckHint {
        name: format!("{}.{}", struct_name.rust_style(), field.name.rust_style()),
    })
}

pub(super) fn report(hints: &[SanityCheckHint]) {
    if hints.is_empty() {
        return;
    }

    warn!(
        "To use the automatically generated getters of the following fields, \
        it is suggested to read https://fzyzcjy.github.io/flutter_rust_bridge/guides/types/arbitrary/rust-auto-opaque/properties to know more details. \
        (Related fields: {})",
        hints.iter().map(|x| &x.name).join(", ")
    )
}

pub(super) struct SanityCheckHint {
    name: String,
}

fn does_need_hint(ty: &IrType) -> bool {
    ty.roughly_dart_mutable()
}
