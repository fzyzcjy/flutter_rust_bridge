use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::{
    WireRustCodecCstGenerator, WireRustCodecCstGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

pub(crate) fn generate_impl_new_with_nullptr(
    types: &[MirType],
    context: WireRustCodecCstGeneratorContext,
) -> Vec<WireRustOutputCode> {
    let mut ans = vec![];

    ans.extend(types.iter().filter_map(|ty| {
        WireRustCodecCstGenerator::new(ty.clone(), context).generate_impl_new_with_nullptr()
    }));

    ans
}

pub(crate) fn generate_impl_new_with_nullptr_code_block(
    mir: impl Into<MirType>,
    context: WireRustCodecCstGeneratorContext,
    body: &str,
) -> String {
    let rust_wire_type =
        WireRustCodecCstGenerator::new(mir.into(), context).rust_wire_type(Target::Io);

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
