use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorDart2RustTrait;

pub(crate) fn generate_impl_new_with_nullptr(
    types: &[IrType],
    context: WireRustGeneratorContext,
) -> Vec<WireRustOutputCode> {
    let mut ans = vec![];

    ans.push(generate_impl_new_with_nullptr_misc().to_string().into());

    ans.extend(types.iter().filter_map(|ty| {
        WireRustGenerator::new(ty.clone(), context).generate_impl_new_with_nullptr()
    }));

    ans
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

pub(crate) fn generate_impl_new_with_nullptr_code_block(
    ir: impl Into<IrType>,
    context: WireRustGeneratorContext,
    body: &str,
    impl_default: bool,
) -> String {
    let rust_wire_type = WireRustGenerator::new(ir.into(), context).rust_wire_type(Target::Io);

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
