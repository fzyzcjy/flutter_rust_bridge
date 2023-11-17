use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::ir::ty::IrType;

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
    WireRustGenerator::new(ty, context)
        .wire2api_body()
        .map(|body, target| {
            body.map(|body| {
                format!(
                    "impl Wire2Api<{api}> for {}{} {{
                        fn wire2api(self) -> {api} {{
                            {body}
                        }}
                    }}",
                    ty.rust_wire_modifier(target),
                    ty.rust_wire_type(target),
                    api = ty.rust_api_type(),
                )
            })
            .unwrap_or_default()
        })
}
