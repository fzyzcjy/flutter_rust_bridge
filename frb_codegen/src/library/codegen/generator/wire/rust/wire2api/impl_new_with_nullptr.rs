use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::generator::wire::rust::wire2api::extern_func::{
    CodeWithExternFunc, ExternFunc,
};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use itertools::Itertools;

pub(crate) fn generate_impl_new_with_nullptr(
    types: &[IrType],
    context: &WireRustGeneratorContext,
) -> CodeWithExternFunc {
    let misc = CodeWithExternFunc::code(generate_impl_new_with_nullptr_misc().to_string());

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

pub(super) fn generate_impl_new_with_nullptr_code_block(
    rust_wire_type: &str,
    body: &str,
    impl_default: bool,
) -> String {
    format!(
        "impl NewWithNullPtr for {rust_wire_type} {{
            fn new_with_null_ptr() -> Self {{
                {body}
            }}
        }}"
    ) + &(if impl_default {
        format!(
            "
            impl Default for {rust_wire_type} {{
                fn default() -> Self {{
                    Self::new_with_null_ptr()
                }}
            }}"
        )
    } else {
        "".to_string()
    })
}
