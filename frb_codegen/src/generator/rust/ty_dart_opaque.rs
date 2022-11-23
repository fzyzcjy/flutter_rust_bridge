use std::borrow::Cow;

use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;
use crate::utils::BlockIndex;
use super::ExternFuncCollector;

type_rust_generator_struct!(TypeDartOpaqueGenerator, IrTypeDartOpaque);

impl TypeRustGeneratorTrait for TypeDartOpaqueGenerator<'_> {
    fn wire2api_body(&self) -> crate::target::Acc<Option<String>> {
        Acc {
            io: Some(
                "let data = unsafe{support::box_from_leak_ptr(self)};
            unsafe { DartOpaque::new(data.handle, data.port) }"
                    .into(),
            ),
            wasm: Some(
                "*unsafe{support::box_from_leak_ptr::<DartOpaque>(self as _)}"
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
            wasm: Some(collector.generate(
                func_name,
                [("handle: JsValue", ""), ("port: MessagePort", "")],
                Some("usize"),
                "support::new_leak_box_ptr(DartOpaque::new(handle, port)) as _",
                crate::target::Target::Wasm,
            )),
            ..Default::default()
        }
    }

    fn related_funcs(
        &self,
        collector: &mut ExternFuncCollector,
        _block_index: BlockIndex,
    ) -> Acc<Option<String>> {
        Acc {
            io: Some(
                vec![
                    collector.generate(
                        &format!("drop_{}", self.ir.safe_ident()),
                        [("ptr: usize", "")],
                        None,
                        "unsafe{Dart_DeletePersistentHandle_DL_Trampolined(ptr as _);}",
                        crate::target::Target::Io,
                    ),
                    collector.generate(
                        &format!("get_{}", self.ir.safe_ident()),
                        [("ptr: usize", "")],
                        Some("*mut _Dart_Handle"),
                        "unsafe{Dart_HandleFromPersistent_DL_Trampolined(ptr as _)}",
                        crate::target::Target::Io,
                    ),
                ]
                .join("\n"),
            ),
            wasm: Some(
                vec![
                    collector.generate(
                        &format!("drop_{}", self.ir.safe_ident()),
                        [("ptr: usize", "")],
                        None,
                        "unsafe{drop(support::box_from_leak_ptr::<JsValue>(ptr as _))}",
                        crate::target::Target::Wasm,
                    ),
                    collector.generate(
                        &format!("get_{}", self.ir.safe_ident()),
                        [("ptr: usize", "")],
                        Some("JsValue"),
                        "*unsafe{support::box_from_leak_ptr(ptr as _)}",
                        crate::target::Target::Wasm,
                    ),
                ]
                .join("\n"),
            ),
            ..Default::default()
        }
    }

    fn imports(&self) -> Option<String> {
        None
    }
}
