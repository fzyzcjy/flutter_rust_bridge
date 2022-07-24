use crate::generator::rust::ty::*;
use crate::generator::rust::{generate_import, ExternFuncCollector};
use crate::ir::*;
use crate::type_rust_generator_struct;
use crate::utils::BlockIndex;

type_rust_generator_struct!(TypeBoxedGenerator, IrTypeBoxed);

impl TypeRustGeneratorTrait for TypeBoxedGenerator<'_> {
    fn wire2api_body(&self) -> Option<String> {
        let IrTypeBoxed {
            inner: box_inner,
            exist_in_real_api,
        } = &self.ir;
        if self.context.wasm() && box_inner.is_js_value() {
            return None;
        }
        Some(match (box_inner.as_ref(), exist_in_real_api) {
            (IrType::Primitive(_), false) => "unsafe { *support::box_from_leak_ptr(self) }".into(),
            (IrType::Primitive(_), true) => "unsafe { support::box_from_leak_ptr(self) }".into(),
            _ => {
                format!(
                    "let wrap = unsafe {{ support::box_from_leak_ptr(self) }};
                    Wire2Api::<{}>::wire2api(*wrap).into()",
                    box_inner.rust_api_type()
                )
            }
        })
    }

    fn wrapper_struct(&self) -> Option<String> {
        let src = TypeRustGenerator::new(
            *self.ir.inner.clone(),
            self.context.ir_file,
            self.context.config,
        );
        src.wrapper_struct()
    }

    fn self_access(&self, obj: String) -> String {
        format!("(*{})", obj)
    }

    fn wrap_obj(&self, obj: String) -> String {
        let src = TypeRustGenerator::new(
            *self.ir.inner.clone(),
            self.context.ir_file,
            self.context.config,
        );
        src.wrap_obj(self.self_access(obj))
    }

    fn allocate_funcs(
        &self,
        collector: &mut ExternFuncCollector,
        block_index: BlockIndex,
    ) -> String {
        let wasm = false;
        if self.ir.inner.is_primitive() {
            collector.generate(
                &format!("new_{}_{}", self.ir.safe_ident(), block_index),
                &[&format!("value: {}", self.ir.inner.rust_wire_type(wasm))],
                Some(&format!("*mut {}", self.ir.inner.rust_wire_type(wasm))),
                "support::new_leak_box_ptr(value)",
            )
        } else {
            collector.generate(
                &format!("new_{}_{}", self.ir.safe_ident(), block_index),
                &[],
                Some(
                    &[
                        self.ir.rust_wire_modifier(wasm),
                        self.ir.rust_wire_type(wasm),
                    ]
                    .concat(),
                ),
                &format!(
                    "support::new_leak_box_ptr({}::new_with_null_ptr())",
                    self.ir.inner.rust_wire_type(wasm)
                ),
            )
        }
    }

    fn imports(&self) -> Option<String> {
        generate_import(&self.ir.inner, self.context.ir_file, self.context.config)
    }
}
