use std::borrow::Cow;

use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeDartOpaqueGenerator, IrTypeDartOpaque);

impl TypeRustGeneratorTrait for TypeDartOpaqueGenerator<'_> {
    fn wire2api_body(&self) -> crate::target::Acc<Option<String>> {
        Acc {
            io: Some("unsafe {DartOpaque::new((*self).handle, (*self).port)}".into()),
            wasm: Some(
                "let data = self.dyn_into::<JsArray>().unwrap();
                DartOpaque::new(data.get(0), data.get(1).dyn_into().unwrap())"
                    .into(),
            ),
            ..Default::default()
        }
    }

    /// Handles JsValue to Self conversions.
    fn wire2api_jsvalue(&self) -> Option<Cow<str>> {
        None
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        Some(vec![
            "port: i64".to_owned(),
            "handle: *mut _Dart_Handle".to_owned(),
        ])
    }

    fn static_checks(&self) -> Option<String> {
        None
    }

    fn wrapper_struct(&self) -> Option<String> {
        None
    }

    fn self_access(&self, _obj: String) -> String {
        "".to_owned()
    }

    fn wrap_obj(&self, _obj: String) -> String {
        "".to_owned()
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

    fn allocate_funcs(
        &self,
        collector: &mut super::ExternFuncCollector,
        _block_index: crate::utils::BlockIndex,
    ) -> Acc<Option<String>> {
        let func_name = "new_DartOpaque";
        Acc {
            io: Some(collector.generate(
                func_name,
                [("handle: *mut _Dart_Handle", ""), ("port: i64", "")],
                Some("*mut wire_DartOpaque"),
                "
                let handle = unsafe {Dart_NewPersistentHandle_DL_Trampolined(handle)};
                support::new_leak_box_ptr(wire_DartOpaque {port, handle})",
                crate::target::Target::Io,
            )),
            ..Default::default()
        }
    }

    fn imports(&self) -> Option<String> {
        None
    }
}
