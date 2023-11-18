use crate::generator::rust::ty::*;
use crate::generator::rust::ExternFuncCollector;
use crate::ir::*;
use crate::target::Acc;
use crate::target::Target;
use crate::type_rust_generator_struct;
use crate::utils::misc::BlockIndex;

type_rust_generator_struct!(TypePrimitiveListGenerator, IrTypePrimitiveList);

impl TypeRustGeneratorTrait for TypePrimitiveListGenerator<'_> {
    fn generate_allocate_funcs(
        &self,
        collector: &mut ExternFuncCollector,
        block_index: BlockIndex,
    ) -> Acc<Option<String>> {
        Acc {
            io: Some(collector.generate(
                &format!("new_{}_{}", self.ir.safe_ident(), block_index),
                [("len: i32", "int")],
                Some(&format!(
                    "{}{}",
                    self.ir.rust_wire_modifier(Target::Io),
                    self.ir.rust_wire_type(Target::Io)
                )),
                &format!(
                    "let ans = {} {{ ptr: support::new_leak_vec_ptr(Default::default(), len), len }};
                    support::new_leak_box_ptr(ans)",
                    self.ir.rust_wire_type(Target::Io),
                ),
                Target::Io,
            )),
            ..Default::default()
        }
    }
}
