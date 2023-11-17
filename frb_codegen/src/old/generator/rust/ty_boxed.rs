use super::NO_PARAMS;
use crate::generator::rust::ty::*;
use crate::generator::rust::{generate_import, ExternFuncCollector};
use crate::ir::*;
use crate::target::Acc;
use crate::target::Target::*;
use crate::type_rust_generator_struct;
use crate::utils::misc::BlockIndex;

type_rust_generator_struct!(TypeBoxedGenerator, IrTypeBoxed);

impl TypeRustGeneratorTrait for TypeBoxedGenerator<'_> {
    fn allocate_funcs(
        &self,
        collector: &mut ExternFuncCollector,
        block_index: BlockIndex,
    ) -> Acc<Option<String>> {
        if self.ir.inner.is_array() {
            return Acc::default();
        }
        let func_name = format!("new_{}_{}", self.ir.safe_ident(), block_index);
        if self.ir.inner.is_primitive() {
            Acc {
                io: Some(collector.generate(
                    &func_name,
                    [(
                        format!("value: {}", self.ir.inner.rust_wire_type(Io)),
                        self.ir.inner.dart_wire_type(Io),
                    )],
                    Some(&format!("*mut {}", self.ir.inner.rust_wire_type(Io))),
                    "support::new_leak_box_ptr(value)",
                    Io,
                )),
                ..Default::default()
            }
        } else {
            Acc {
                io: Some(collector.generate(
                    &func_name,
                    NO_PARAMS,
                    Some(&[self.ir.rust_wire_modifier(Io), self.ir.rust_wire_type(Io)].concat()),
                    &format!(
                        "support::new_leak_box_ptr({}::new_with_null_ptr())",
                        self.ir.inner.rust_wire_type(Io)
                    ),
                    Io,
                )),
                ..Default::default()
            }
        }
    }

    fn imports(&self) -> Option<String> {
        generate_import(&self.ir.inner, self.context.ir_pack, self.context.config)
    }
}
