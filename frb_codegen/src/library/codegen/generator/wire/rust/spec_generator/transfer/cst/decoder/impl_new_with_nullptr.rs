use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGenerator;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::{
    WireRustTransferCstGenerator, WireRustTransferCstGeneratorContext,
};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustTransferCstGeneratorDecoderTrait;

pub(crate) fn generate_impl_new_with_nullptr(
    types: &[IrType],
    context: WireRustTransferCstGeneratorContext,
) -> Vec<WireRustOutputCode> {
    let mut ans = vec![];

    ans.push(generate_impl_new_with_nullptr_misc().to_string().into());

    ans.extend(types.iter().filter_map(|ty| {
        WireRustTransferCstGenerator::new(ty.clone(), context).generate_impl_new_with_nullptr()
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
    context: WireRustTransferCstGeneratorContext,
    body: &str,
    impl_default: bool,
) -> String {
    let rust_wire_type =
        WireRustTransferCstGenerator::new(ir.into(), context).rust_wire_type(Target::Io);

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
