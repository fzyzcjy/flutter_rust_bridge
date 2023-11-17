use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;
use crate::library::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

mod misc;
pub(crate) mod ty;

pub(crate) fn generate_impl_wire2api() {
    todo!()
}

fn generate_impl_wire2api_misc() -> &'static str {
    r#"pub trait Wire2Api<T> {
        fn wire2api(self) -> T;
    }

    impl<T, S> Wire2Api<Option<T>> for *mut S
    where
        *mut S: Wire2Api<T>
    {
        fn wire2api(self) -> Option<T> {
            (!self.is_null()).then(|| self.wire2api())
        }
    }"#
}

fn generate_wire2api_for_type(ty: IrType, context: WireRustGeneratorContext) -> Acc<String> {
    let generator = WireRustGenerator::new(ty, context);
    generator.wire2api_body().map(|body, target| {
        body.map(|body| {
            format!(
                "impl Wire2Api<{api}> for {}{} {{
                        fn wire2api(self) -> {api} {{
                            {body}
                        }}
                    }}",
                generator.rust_wire_modifier(target),
                generator.rust_wire_type(target),
                api = ty.rust_api_type(),
            )
        })
        .unwrap_or_default()
    })
}
