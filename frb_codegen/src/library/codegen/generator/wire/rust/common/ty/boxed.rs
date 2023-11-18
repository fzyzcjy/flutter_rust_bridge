use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::common::ty::WireRustGeneratorCommonTrait;

impl<'a> WireRustGeneratorCommonTrait for BoxedWireRustGenerator<'a> {
    // TODO https://github.com/fzyzcjy/yplusplus/issues/11145#issuecomment-1816273032
    // fn wrapper_struct_name(&self) -> Option<String> {
    //     let src = TypeRustGenerator::new(
    //         *self.ir.inner.clone(),
    //         self.context.ir_pack,
    //         self.context.config,
    //     );
    //     src.wrapper_struct_name()
    // }

    fn generate_imports(&self) -> Option<Vec<String>> {
        generate_import(&self.ir.inner, self.context.ir_pack, self.context.config)
    }
}
