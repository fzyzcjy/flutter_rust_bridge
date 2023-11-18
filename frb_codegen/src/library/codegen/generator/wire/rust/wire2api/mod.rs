use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::generator::wire::rust::wire2api::extern_func::{
    CodeWithExternFunc, ExternFunc,
};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use itertools::Itertools;

mod extern_func;
mod impl_wire2api_trait;
mod misc;
pub(crate) mod ty;

pub(crate) fn generate_impl_new_with_nullptr(
    types: &[IrType],
    context: &WireRustGeneratorContext,
) -> CodeWithExternFunc {
    let misc = CodeWithExternFunc {
        code: generate_impl_new_with_nullptr_misc().to_string(),
        ..Default::default()
    };

    let funcs: Vec<CodeWithExternFunc> = types
        .iter()
        .filter_map(|ty| {
            WireRustGenerator::new(ty.clone(), context.clone()).generate_impl_new_with_nullptr()
        })
        .collect_vec();

    misc + funcs.into_iter().fold(Default::default(), |a, b| a + b)
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
