use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::WireDartCodecCstGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::output_code::{
    DartApiImplClassMethod, WireDartOutputCode,
};
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::enumeration::MirVariantKind;
use crate::codegen::ir::mir::ty::{MirType, MirTypeTrait};
use itertools::Itertools;
use std::collections::HashSet;

pub(crate) fn generate(
    ty: &MirType,
    context: WireDartCodecCstGeneratorContext,
) -> WireDartOutputCode {
    if !needs_validation(ty, context.mir_pack) {
        return WireDartOutputCode::default();
    }
    let call = |ty: &MirType, value: &str| generate_call(ty, value, context.mir_pack);
    let body = match ty {
        MirType::RustOpaque(_) | MirType::RustAutoOpaqueImplicit(_) => {
            "if (raw.isDisposed) { throw DroppableDisposedException(raw.runtimeType.toString()); }"
                .to_owned()
        }
        MirType::Boxed(inner) => call(&inner.inner, "raw"),
        MirType::Optional(inner) => format!("if (raw != null) {{ {} }}", call(&inner.inner, "raw")),
        MirType::GeneralList(inner) => format!(
            "for (final item in raw) {{ {} }}",
            call(&inner.inner, "item")
        ),
        MirType::StructRef(inner) => inner
            .get(context.mir_pack)
            .fields
            .iter()
            .map(|field| call(&field.ty, &format!("raw.{}", field.name.dart_style())))
            .join("\n"),
        MirType::Record(inner) => inner
            .values
            .iter()
            .enumerate()
            .map(|(index, ty)| call(ty, &format!("raw.${}", index + 1)))
            .join("\n"),
        MirType::EnumRef(inner) => inner
            .get(context.mir_pack)
            .variants()
            .iter()
            .filter_map(|variant| {
                let MirVariantKind::Struct(inner) = &variant.kind else {
                    return None;
                };
                let body = inner
                    .fields
                    .iter()
                    .map(|field| call(&field.ty, &format!("raw.{}", field.name.dart_style())))
                    .join("\n");
                Some(format!("if (raw is {}) {{ {body} }}", variant.wrapper_name))
            })
            .join("\n"),
        MirType::Delegate(inner) => match inner {
            MirTypeDelegate::Map(inner) => format!(
                "for (final entry in raw.entries) {{ {} {} }}",
                call(&inner.key, "entry.key"),
                call(&inner.value, "entry.value")
            ),
            MirTypeDelegate::Set(inner) => format!(
                "for (final item in raw) {{ {} }}",
                call(&inner.inner, "item")
            ),
            MirTypeDelegate::Array(_) | MirTypeDelegate::RustAutoOpaqueExplicit(_) => {
                call(&inner.get_delegate(), "raw")
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    WireDartOutputCode {
        api_impl_class_methods: vec![DartApiImplClassMethod {
            signature: format!(
                "void cst_validate_{}({} raw)",
                ty.safe_ident(),
                ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type()
            ),
            body: Some(body),
        }],
        ..Default::default()
    }
}

pub(crate) fn generate_call(ty: &MirType, value: &str, pack: &MirPack) -> String {
    if needs_validation(ty, pack) {
        format!("cst_validate_{}({value});", ty.safe_ident())
    } else {
        String::new()
    }
}

fn needs_validation(ty: &MirType, pack: &MirPack) -> bool {
    contains_opaque(ty, pack, &mut HashSet::new())
}

fn contains_opaque(ty: &MirType, pack: &MirPack, visited: &mut HashSet<String>) -> bool {
    if !visited.insert(ty.safe_ident()) {
        return false;
    }
    let mut visit = |ty: &MirType| contains_opaque(ty, pack, visited);
    match ty {
        MirType::RustOpaque(_) | MirType::RustAutoOpaqueImplicit(_) => true,
        MirType::Boxed(inner) => visit(&inner.inner),
        MirType::Optional(inner) => visit(&inner.inner),
        MirType::GeneralList(inner) => visit(&inner.inner),
        MirType::StructRef(inner) => inner.get(pack).fields.iter().any(|field| visit(&field.ty)),
        MirType::Record(inner) => inner.values.iter().any(visit),
        MirType::EnumRef(inner) => inner
            .get(pack)
            .variants()
            .iter()
            .any(|variant| match &variant.kind {
                MirVariantKind::Value => false,
                MirVariantKind::Struct(inner) => inner.fields.iter().any(|field| visit(&field.ty)),
            }),
        MirType::Delegate(inner) => match inner {
            MirTypeDelegate::Map(inner) => visit(&inner.key) || visit(&inner.value),
            MirTypeDelegate::Set(inner) => visit(&inner.inner),
            MirTypeDelegate::Array(_) | MirTypeDelegate::RustAutoOpaqueExplicit(_) => {
                visit(&inner.get_delegate())
            }
            _ => false,
        },
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::test_utils;
    use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
    use crate::codegen::ir::mir::ty::delegate::MirTypeDelegateSet;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::rust_opaque::{
        MirRustOpaqueInner, MirTypeRustOpaque, RustOpaqueCodecMode,
    };
    use crate::utils::namespace::Namespace;

    /// Ignores a Rust-only opaque hasher when Dart elements contain no opaque values.
    #[test]
    fn primitive_set_with_opaque_hasher_needs_no_validation() {
        let ty = MirType::Delegate(MirTypeDelegate::Set(MirTypeDelegateSet {
            inner: Box::new(MirType::Primitive(MirTypePrimitive::I32)),
            hasher: Some(Box::new(opaque())),
        }));
        assert!(!needs_validation(&ty, &test_utils::pack()));
    }

    /// Detects an opaque element even when the container has no Rust-only hasher.
    #[test]
    fn opaque_set_elements_require_validation() {
        let ty = MirType::Delegate(MirTypeDelegate::Set(MirTypeDelegateSet {
            inner: Box::new(opaque()),
            hasher: None,
        }));
        assert!(needs_validation(&ty, &test_utils::pack()));
    }

    fn opaque() -> MirType {
        MirType::RustOpaque(MirTypeRustOpaque {
            namespace: Namespace::default(),
            inner: MirRustOpaqueInner(MirLifetimeAwareType::new("Handle".into())),
            codec: RustOpaqueCodecMode::Nom,
            dart_api_type: Some("Handle".into()),
            brief_name: false,
        })
    }
}
