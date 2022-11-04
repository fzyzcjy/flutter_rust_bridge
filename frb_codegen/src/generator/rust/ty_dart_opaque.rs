use std::borrow::Cow;

use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeDartOpaqueGenerator, IrTypeDartOpaque);

impl TypeRustGeneratorTrait for TypeDartOpaqueGenerator<'_> {
    fn wire2api_body(&self) -> crate::target::Acc<Option<String>> {
        Acc {
            io: Some(
                "unsafe { /* A */ }"
                .into(),
            ),
            ..Default::default()
        }
    }

    /// Handles JsValue to Self conversions.
    fn wire2api_jsvalue(&self) -> Option<Cow<str>> {
        #[cfg(target_arch = "wasm64")]
        {
            panic!("The wasm64 arch is not supported.");
        }

        Some(
            "unsafe {
                support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
            }"
            .into(),
        )
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        Some(vec!["ptr: *const core::ffi::c_void".to_owned()])
    }

    fn static_checks(&self) -> Option<String> {
        None
    }

    fn wrapper_struct(&self) -> Option<String> {
        None
    }

    fn self_access(&self, obj: String) -> String {
        obj
    }

    fn wrap_obj(&self, obj: String) -> String {
        obj
    }

    fn convert_to_dart(&self, obj: String) -> String {
        format!("{}.into_dart()", obj)
    }

    fn structs(&self) -> String {
        "".to_owned()
    }

    fn impl_intodart(&self) -> String {
        "".to_owned()
    }


    fn imports(&self) -> Option<String> {
        None
    }
}