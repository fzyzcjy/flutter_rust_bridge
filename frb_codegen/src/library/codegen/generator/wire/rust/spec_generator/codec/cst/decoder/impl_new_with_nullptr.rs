use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::{
    WireRustCodecCstGenerator, WireRustCodecCstGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

pub(crate) fn generate_impl_new_with_nullptr(
    types: &[IrType],
    context: WireRustCodecCstGeneratorContext,
) -> Vec<WireRustOutputCode> {
    let mut ans = vec![];

    ans.push(generate_impl_new_with_nullptr_misc().to_string().into());

    ans.extend(types.iter().filter_map(|ty| {
        WireRustCodecCstGenerator::new(ty.clone(), context).generate_impl_new_with_nullptr()
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
    context: WireRustCodecCstGeneratorContext,
    body: &str,
) -> String {
    let rust_wire_type =
        WireRustCodecCstGenerator::new(ir.into(), context).rust_wire_type(Target::Io);

    format!(
        "impl NewWithNullPtr for {rust_wire_type} {{
            fn new_with_null_ptr() -> Self {{
                {body}
            }}
        }}
        impl Default for {rust_wire_type} {{
            fn default() -> Self {{
                Self::new_with_null_ptr()
            }}
        }}"
    )
}
