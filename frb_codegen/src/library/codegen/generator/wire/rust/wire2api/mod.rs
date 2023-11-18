mod impl_wire2api_trait;
mod misc;
pub(crate) mod ty;

pub(crate) fn generate_impl_new_with_nullptr(&self) -> String {
    todo!()
}

fn generate_impl_new_with_nullptr_misc(&self) -> &'static str {
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

fn generate_impl_new_with_nullptr_func(&mut self, ty: &IrType, ir_pack: &IrPack) -> String {
    TypeRustGenerator::new(ty.clone(), ir_pack, self.config)
        .new_with_nullptr(&mut self.extern_func_collector)
}
