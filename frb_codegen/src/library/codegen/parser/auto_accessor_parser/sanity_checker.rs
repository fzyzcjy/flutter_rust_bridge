use crate::codegen::ir::field::IrField;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::ty::IrType;
use log::warn;

pub(super) fn sanity_check_field(struct_name: &NamespacedName, field: &IrField) {
    if matches!(field.ty, IrType::RustAutoOpaqueImplicit(_)) {
        warn!(
            "It is suggested (but not required) to wrap field `{struct_name}.{field_name}` with `RustAutoOpaque<..>`. \
            Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/types/arbitrary/rust-auto-opaque/properties for more details.",
            struct_name = struct_name.rust_style(),
            field_name = field.name.rust_style(),
        );
    }
}
