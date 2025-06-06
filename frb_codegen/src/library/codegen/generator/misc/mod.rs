use crate::codegen::ir::mir::ty::boxed::MirTypeBoxed;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::MirType;

pub(crate) mod comments;
pub(crate) mod path_texts;
pub(crate) mod struct_or_record;
pub(crate) mod structs_macro;
pub(crate) mod target;
pub(crate) mod text_generator_utils;

/// In WASM, these types belong to the JS scope-local heap, **NOT** the Rust heap and
/// therefore do not implement [Send]. More specifically, these are types wasm-bindgen
/// can't handle yet.
pub fn is_js_value(ty: &MirType) -> bool {
    match ty {
        MirType::GeneralList(_)
        | MirType::StructRef(_)
        | MirType::EnumRef(_)
        | MirType::RustAutoOpaqueImplicit(_)
        | MirType::RustOpaque(_)
        | MirType::Delegate(MirTypeDelegate::RustAutoOpaqueExplicit(_))
        | MirType::DartOpaque(_)
        | MirType::DartFn(_)
        | MirType::Record(_) => true,
        MirType::Boxed(MirTypeBoxed { inner, .. }) => is_js_value(inner),
        MirType::Delegate(inner) => is_js_value(&inner.get_delegate()),
        MirType::Optional(inner) => is_js_value(&inner.inner),
        MirType::Primitive(_) | MirType::PrimitiveList(_) => false,
        // frb-coverage:ignore-start
        MirType::Dynamic(_) | MirType::TraitDef(_) => unreachable!(),
        // frb-coverage:ignore-end
    }
}

pub(crate) fn generate_code_header() -> &'static str {
    concat!(
        "// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ ",
        env!("CARGO_PKG_VERSION"),
        "."
    )
}
