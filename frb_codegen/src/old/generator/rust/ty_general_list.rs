use crate::generator::rust::ty::*;
use crate::generator::rust::{generate_import, generate_list_allocate_func, ExternFuncCollector};
use crate::ir::*;
use crate::target::{Acc, Target};
use crate::type_rust_generator_struct;
use crate::utils::misc::BlockIndex;

type_rust_generator_struct!(TypeGeneralListGenerator, IrTypeGeneralList);

impl TypeGeneralListGenerator<'_> {}

impl TypeRustGeneratorTrait for TypeGeneralListGenerator<'_> {
    fn allocate_funcs(
        &self,
        collector: &mut ExternFuncCollector,
        _: BlockIndex,
    ) -> Acc<Option<String>> {
        Acc {
            io: Some(generate_list_allocate_func(
                collector,
                &self.ir.safe_ident(),
                &self.ir,
                &self.ir.inner,
                self.context.config.block_index,
            )),
            ..Default::default()
        }
    }

    fn imports(&self) -> Option<String> {
        generate_import(&self.ir.inner, self.context.ir_pack, self.context.config)
    }
}
