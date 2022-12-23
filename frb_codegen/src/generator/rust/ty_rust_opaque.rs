use std::borrow::Cow;

use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;
use crate::utils::BlockIndex;

use super::{ExternFuncCollector, NO_PARAMS};

type_rust_generator_struct!(TypeRustOpaqueGenerator, IrTypeRustOpaque);

impl TypeRustGeneratorTrait for TypeRustOpaqueGenerator<'_> {
    fn wire2api_body(&self) -> crate::target::Acc<Option<String>> {
        Acc {
            io: Some(
                "unsafe {
                support::opaque_from_dart(self.ptr as _)
            }"
                .into(),
            ),
            ..Default::default()
        }
    }

    /// Handles JsValue to Self conversions.
    fn wire2api_jsvalue(&self) -> Option<Cow<str>> {
        Some(
            r##"
            #[cfg(target_pointer_width = "64")]
            {
                compile_error!("64-bit pointers are not supported.");
            }
    
            unsafe {
                support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
            }"##
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

    fn wrap_obj(&self, obj: String, _wired_fallible_func: bool) -> String {
        obj
    }

    fn convert_to_dart(&self, obj: String) -> String {
        format!("{}.into_dart()", obj)
    }

    fn structs(&self) -> String {
        "".to_owned()
    }

    fn allocate_funcs(
        &self,
        collector: &mut ExternFuncCollector,
        _block_index: BlockIndex,
    ) -> Acc<Option<String>> {
        let rust_wire = self.ir.rust_wire_type(crate::target::Target::Io);

        Acc {
            io: Some(collector.generate(
                &format!("new_{}", self.ir.safe_ident()),
                NO_PARAMS,
                Some(&format!(
                    "{}{}",
                    self.ir.rust_wire_modifier(crate::target::Target::Io),
                    rust_wire
                )),
                &format!("{}::new_with_null_ptr()", rust_wire,),
                crate::target::Target::Io,
            )),
            ..Default::default()
        }
    }

    fn related_funcs(
        &self,
        collector: &mut ExternFuncCollector,
        _block_index: BlockIndex,
    ) -> Acc<Option<String>> {
        let mut generate_impl = |target| {
            vec![
                collector.generate(
                    &format!("drop_opaque_{}", self.ir.safe_ident()),
                    [("ptr: *const c_void", "")],
                    None,
                    &format!(
                        "unsafe {{Arc::<{}>::decrement_strong_count(ptr as _);}}",
                        self.ir.inner_rust
                    ),
                    target,
                ),
                collector.generate(
                    &format!("share_opaque_{}", self.ir.safe_ident()),
                    [("ptr: *const c_void", "")],
                    Some("*const c_void"),
                    &format!(
                        "unsafe {{Arc::<{}>::increment_strong_count(ptr as _); ptr}}",
                        self.ir.inner_rust
                    ),
                    target,
                ),
            ]
            .join("\n")
        };
        Acc {
            io: Some(generate_impl(crate::target::Target::Io)),
            wasm: Some(generate_impl(crate::target::Target::Wasm)),
            ..Default::default()
        }
    }

    fn impl_intodart(&self) -> String {
        "".to_owned()
    }

    fn new_with_nullptr(&self, _collector: &mut ExternFuncCollector) -> String {
        format!(
            "impl NewWithNullPtr for {} {{
                fn new_with_null_ptr() -> Self {{
                    Self {{ ptr: core::ptr::null() }}
                }}
            }}",
            self.ir.rust_wire_type(crate::target::Target::Io)
        )
    }

    fn imports(&self) -> Option<String> {
        None
    }
}
