use std::borrow::Cow;

use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;
use crate::utils::misc::BlockIndex;

use super::{ExternFuncCollector, NO_PARAMS};

type_rust_generator_struct!(TypeRustOpaqueGenerator, IrTypeRustOpaque);

impl TypeRustGeneratorTrait for TypeRustOpaqueGenerator<'_> {
    fn generate_related_funcs(&self) -> Acc<Option<CodeWithExternFunc>> {
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
            io: Some(generate_impl(Target::Io)),
            wasm: Some(generate_impl(Target::Wasm)),
            ..Default::default()
        }
    }
}
