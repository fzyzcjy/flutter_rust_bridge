use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use itertools::Itertools;

mod impl_wire2api_trait;
mod misc;
pub(crate) mod ty;

pub(crate) fn generate_impl_new_with_nullptr(
    types: &[IrType],
    context: &WireRustGeneratorContext,
) -> String {
    let funcs = types
        .iter()
        .map(|ty| {
            WireRustGenerator::new(ty.clone(), context.clone()).generate_impl_new_with_nullptr()
        })
        .collect_vec();

    format!(
        "{}\n{}",
        generate_impl_new_with_nullptr_misc(),
        funcs.join("\n\n")
    )
}

fn generate_impl_new_with_nullptr_misc() -> &'static str {
    "pub trait NewWithNullPtr {
        fn new_with_null_ptr() -> Self;
    }

    impl<T> NewWithNullPtr for *mut T {
        fn new_with_null_ptr() -> Self {
            std::ptr::null_mut()
        }
    }
    "
}
