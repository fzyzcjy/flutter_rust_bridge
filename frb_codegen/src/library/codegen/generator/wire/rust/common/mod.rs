use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::common::ty::WireRustGeneratorCommonTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;
use std::collections::HashSet;

mod misc;
pub(crate) mod ty;

pub(crate) fn generate_wrapper_struct(
    ty: &IrType,
    context: WireRustGeneratorContext,
) -> Option<String> {
    // the generated wrapper structs need to be public for the StreamSinkTrait impl to work
    WireRustGenerator::new(ty.clone(), context)
        .wrapper_struct_name()
        .map(|wrapper_struct_name| {
            format!(
                r###"
                #[derive(Clone)]
                pub struct {}({});
                "###,
                wrapper_struct_name,
                ty.rust_api_type(),
            )
        })
}

pub(crate) fn generate_static_checks(
    types: &[IrType],
    context: WireRustGeneratorContext,
) -> String {
    let raw = types
        .iter()
        .filter_map(|ty| WireRustGenerator::new(ty.clone(), context).generate_static_checks())
        .collect_vec();

    if raw.is_empty() {
        return "".to_owned();
    }

    let mut lines = vec![];
    lines.push("const _: fn() = || {".to_owned());
    lines.extend(raw);
    lines.push("};".to_owned());
    lines.join("\n")
}

pub(crate) fn generate_imports(types: &[IrType], context: WireRustGeneratorContext) -> String {
    let rust_wire_mod = &context.config.rust_wire_mod;
    let imports_misc = format!(
        r#"
        use crate::{rust_wire_mod}::*;
        use flutter_rust_bridge::*;
        use core::panic::UnwindSafe;
        use std::sync::Arc;
        use std::ffi::c_void;
        use flutter_rust_bridge::rust2dart::IntoIntoDart;
        "#
    );

    let imports_from_types = types
        .iter()
        .flat_map(|ty| WireRustGenerator::new(ty.clone(), context).generate_imports())
        .flatten()
        // Don't include imports from the API file
        .filter(|import| !import.starts_with(&format!("use crate::{rust_wire_mod}::")))
        // de-duplicate
        .collect::<HashSet<String>>()
        .into_iter()
        .join("\n");

    imports_misc + &imports_from_types
}
