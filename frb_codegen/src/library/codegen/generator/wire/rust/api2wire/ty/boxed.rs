use crate::codegen::generator::wire::rust::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::generator::wire::rust::base::*;

impl<'a> WireRustGeneratorApi2wireTrait for BoxedWireRustGenerator<'a> {
    fn hello(&self) -> Option<String> {
        todo!()
    }
}
