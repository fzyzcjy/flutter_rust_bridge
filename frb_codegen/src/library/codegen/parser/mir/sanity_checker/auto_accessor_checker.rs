use crate::codegen::ir::mir::field::MirField;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::namespace::NamespacedName;
use itertools::Itertools;
use log::info;

pub(crate) fn check_field(
    struct_name: &NamespacedName,
    field: &MirField,
) -> Option<SanityCheckHint> {
    (!field.ty.cloned_getter_semantics_reasonable()).then(|| SanityCheckHint {
        name: format!("{}.{}", struct_name.rust_style(), field.name.rust_style()),
    })
}

pub(crate) fn report(hints: &[SanityCheckHint]) {
    if hints.is_empty() {
        return;
    }

    info!(
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

#[cfg(test)]
mod tests {
    use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
    use crate::codegen::ir::mir::ty::boxed::MirTypeBoxed;
    use crate::codegen::ir::mir::ty::dart_opaque::MirTypeDartOpaque;
    use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
    use crate::codegen::ir::mir::ty::dynamic::MirTypeDynamic;
    use crate::codegen::ir::mir::ty::optional::MirTypeOptional;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::rust_opaque::{
        MirRustOpaqueInner, MirTypeRustOpaque, RustOpaqueCodecMode,
    };
    use crate::codegen::ir::mir::ty::MirType;
    use crate::library::codegen::ir::mir::ty::MirTypeTrait;
    use crate::utils::namespace::Namespace;

    #[test]
    fn test_cloned_getter_semantics_reasonable() {
        assert!(MirType::Boxed(MirTypeBoxed {
            exist_in_real_api: true,
            inner: Box::new(MirType::Primitive(MirTypePrimitive::F32))
        })
        .cloned_getter_semantics_reasonable());

        assert!(MirType::RustOpaque(MirTypeRustOpaque {
            namespace: Namespace::new_raw("".to_owned()),
            inner: MirRustOpaqueInner(MirLifetimeAwareType::new("".to_owned())),
            codec: RustOpaqueCodecMode::Nom,
            dart_api_type: None,
            brief_name: true,
        })
        .cloned_getter_semantics_reasonable());

        assert!(MirType::Optional(MirTypeOptional {
            inner: Box::new(MirType::Primitive(MirTypePrimitive::F32))
        })
        .cloned_getter_semantics_reasonable());

        assert!(MirType::Dynamic(MirTypeDynamic).cloned_getter_semantics_reasonable());

        assert!(MirType::DartOpaque(MirTypeDartOpaque).cloned_getter_semantics_reasonable());

        assert!(MirType::Delegate(MirTypeDelegate::String).cloned_getter_semantics_reasonable());
        assert!(MirType::Delegate(MirTypeDelegate::Char).cloned_getter_semantics_reasonable());
    }
}
