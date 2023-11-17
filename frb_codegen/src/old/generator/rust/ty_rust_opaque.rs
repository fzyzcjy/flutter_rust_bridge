use std::borrow::Cow;

use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;
use crate::utils::misc::BlockIndex;

use super::{ExternFuncCollector, NO_PARAMS};

type_rust_generator_struct!(TypeRustOpaqueGenerator, IrTypeRustOpaque);

impl TypeRustGeneratorTrait for TypeRustOpaqueGenerator<'_> {
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
                    "{}{rust_wire}",
                    self.ir.rust_wire_modifier(crate::target::Target::Io),
                )),
                &format!("{rust_wire}::new_with_null_ptr()"),
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
            [
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
}
