use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::ir::ty::IrType;

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
