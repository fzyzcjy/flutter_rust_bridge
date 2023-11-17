use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::common::ty::WireRustGeneratorCommonTrait;

impl<'a> WireRustGeneratorCommonTrait for RecordWireRustGenerator<'a> {
    // TODO https://github.com/fzyzcjy/yplusplus/issues/11145#issuecomment-1816273032
    // fn wrapper_struct_name(&self) -> Option<String> {
    //     self.as_struct_generator().wrapper_struct_name()
    // }

    // TODO old code has this, but I think do not need?
    // fn generate_static_checks(&self) -> Option<String> {
    //     self.as_struct_generator().static_checks()
    // }
}
