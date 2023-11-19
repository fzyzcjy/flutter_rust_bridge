use crate::codegen::generator::acc::Acc;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::IrType;
use crate::enum_map;
use crate::utils::file_utils::create_dir_all_and_write;
use anyhow::bail;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::ops::Add;
use std::path::PathBuf;
use strum::IntoEnumIterator;

pub(crate) mod structs_macro;
pub(crate) mod target;
pub(crate) mod text_generator_utils;

/// In WASM, these types belong to the JS scope-local heap, **NOT** the Rust heap and
/// therefore do not implement [Send]. More specifically, these are types wasm-bindgen
/// can't handle yet.
pub fn is_js_value(ty: &IrType) -> bool {
    match ty {
        IrType::GeneralList(_)
        | IrType::OptionalList(_)
        | IrType::StructRef(_)
        | IrType::EnumRef(_)
        | IrType::RustOpaque(_)
        | IrType::DartOpaque(_)
        | IrType::Record(_) => true,
        IrType::Boxed(IrTypeBoxed { inner, .. }) => is_js_value(inner),
        IrType::Delegate(inner) => is_js_value(&inner.get_delegate()),
        IrType::Optional(inner) => is_js_value(&inner.inner),
        IrType::Primitive(_) | IrType::PrimitiveList(_) => false,
        IrType::Dynamic(_) | IrType::Unencodable(_) => unreachable!(),
    }
}

// TODO optimize the detailed type
pub(crate) struct OutputTexts(pub Vec<(PathBuf, String)>);
