use crate::codegen::ir::field::IrField;
use crate::codegen::ir::namespace::NamespacedName;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;
use log::warn;

pub(crate) fn check_field(
    struct_name: &NamespacedName,
    field: &IrField,
) -> Option<SanityCheckHint> {
    (!field.ty.cloned_getter_semantics_reasonable()).then(|| SanityCheckHint {
        name: format!("{}.{}", struct_name.rust_style(), field.name.rust_style()),
    })
}

pub(crate) fn report(hints: &[SanityCheckHint]) {
    if hints.is_empty() {
        return;
    }

    warn!(
        "To use the automatically generated getters of the following fields of opaque types, \
        it is suggested to read https://fzyzcjy.github.io/flutter_rust_bridge/guides/types/arbitrary/rust-auto-opaque/properties to know more details. \
        (Related fields: {})",
        hints.iter().map(|x| &x.name).join(", ")
    )
}

#[derive(Clone)]
pub(crate) struct SanityCheckHint {
    name: String,
}
