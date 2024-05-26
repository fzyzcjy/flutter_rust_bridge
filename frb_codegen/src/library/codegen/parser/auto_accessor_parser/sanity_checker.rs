use crate::codegen::ir::field::IrField;
use crate::codegen::ir::namespace::NamespacedName;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;
use log::warn;

pub(super) fn check_field(
    struct_name: &NamespacedName,
    field: &IrField,
) -> Option<SanityCheckHint> {
    (!field.ty.cloned_getter_semantics_reasonable()).then(|| SanityCheckHint {
        name: format!("{}.{}", struct_name.rust_style(), field.name.rust_style()),
    })
}

pub(super) fn report(hints: &[SanityCheckHint]) {
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
pub(super) struct SanityCheckHint {
    name: String,
}

#[cfg(test)]
mod tests {
    use crate::codegen::ir::namespace::Namespace;
    use crate::codegen::ir::ty::boxed::IrTypeBoxed;
    use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
    use crate::codegen::ir::ty::delegate::IrTypeDelegate;
    use crate::codegen::ir::ty::dynamic::IrTypeDynamic;
    use crate::codegen::ir::ty::optional::IrTypeOptional;
    use crate::codegen::ir::ty::primitive::IrTypePrimitive;
    use crate::codegen::ir::ty::rust_opaque::{
        IrRustOpaqueInner, IrTypeRustOpaque, RustOpaqueCodecMode,
    };
    use crate::codegen::ir::ty::IrType;
    use crate::library::codegen::ir::ty::IrTypeTrait;

    #[test]
    fn test_cloned_getter_semantics_reasonable() {
        assert!(IrType::Boxed(IrTypeBoxed {
            exist_in_real_api: true,
            inner: Box::new(IrType::Primitive(IrTypePrimitive::F32))
        })
        .cloned_getter_semantics_reasonable());

        assert!(IrType::RustOpaque(IrTypeRustOpaque {
            namespace: Namespace::new_raw("".to_owned()),
            inner: IrRustOpaqueInner("".to_owned()),
            codec: RustOpaqueCodecMode::Nom,
            brief_name: true,
        })
        .cloned_getter_semantics_reasonable());

        assert!(IrType::Optional(IrTypeOptional {
            inner: Box::new(IrType::Primitive(IrTypePrimitive::F32))
        })
        .cloned_getter_semantics_reasonable());

        assert!(IrType::Dynamic(IrTypeDynamic).cloned_getter_semantics_reasonable());

        assert!(IrType::DartOpaque(IrTypeDartOpaque).cloned_getter_semantics_reasonable());

        assert!(IrType::Delegate(IrTypeDelegate::String).cloned_getter_semantics_reasonable());
        assert!(IrType::Delegate(IrTypeDelegate::Char).cloned_getter_semantics_reasonable());
    }
}
